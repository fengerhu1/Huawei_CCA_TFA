
#include <stdint.h>
#include <string.h>
#include <mm/mm.h>
#include <mm/mmu_def.h>
#include <mm/buddy_allocator.h>
// #include <mm/tzc400.h>
#include <common/def.h>
#include <assert.h>

#define PGD_SHIFT   39
#define PUD_SHIFT   30
#define PMD_SHIFT   21
#define PTE_SHIFT   12
#define PTRS_PER_LEVEL  512

uint64_t current_cpu_stack_sps[PHYSICAL_CORE_NUM] = {0};

extern void activate_mmu(void);
extern void clear_sctlr_el2(void);
extern void enable_hyp_mode2(void);
extern void activate_mmu_disable_cache(void);

uint64_t _boot_pt_l0_0[BIT(L0_BITS)] __attribute__((__aligned__(BIT(L0_PTP_BITS))));
uint64_t _boot_pt_l1_0[BIT(L1_BITS)] __attribute__((__aligned__(BIT(L1_PTP_BITS))));
uint64_t _boot_pt_l2_0[BIT(L2_BITS)] __attribute__((__aligned__(BIT(L2_PTP_BITS))));
uint64_t _boot_pt_l2_1[BIT(L2_BITS)] __attribute__((__aligned__(BIT(L2_PTP_BITS))));
uint64_t pgtable_l3[BIT(L2_BITS)] __attribute__((__aligned__(BIT(L3_PTP_BITS))));
int init_pt = 0;
// static void init_percpu_stack(void) {
//     int i = 0;
//     for (; i < PHYSICAL_CORE_NUM; i++) {
//         uint64_t stack_page = (uint64_t)bd_alloc(PAGE_SIZE, 12);
//         current_cpu_stack_sps[i] = stack_page + PAGE_SIZE;
//     }
// }

extern void activate_mmu(void);


vaddr_t phys_to_virt(paddr_t phys) {
    return (vaddr_t)phys;
}

paddr_t virt_to_phys(vaddr_t virt) {
    return (paddr_t)virt;
}

static void initialize_boot_page_table(void) {
	uint64_t i, j;

	print_info("[BOOT] init boot page table: _boot_pt_l0_0=0x%lx _boot_pt_l1_0=0x%lx stack %lx\r\n",
	       _boot_pt_l0_0, _boot_pt_l1_0, (unsigned long)(&i));

    memset(_boot_pt_l0_0, 0, sizeof(uint64_t) * BIT(L0_BITS));
    memset(_boot_pt_l1_0, 0, sizeof(uint64_t) * BIT(L1_BITS));
	_boot_pt_l0_0[0] = ((uintptr_t) _boot_pt_l1_0) | BIT(1) | BIT(0);

    // TODO:
    // Currently, we only map 32GB memory
    for (i = GET_L1_INDEX(0); i < GET_L1_INDEX(RMM_MAP_MEMORY_RANGE); i++) {
        _boot_pt_l1_0[i] = (i << HP_1G_BLOCK_SHIFT)
            | BIT(10)	/* bit[10]: access flag */
            | (3 << 8)  /* bit[9-8]: inner shareable */
                        /* bit[7-6] data access permission bit */
            | (0 << 5)  /* bit[5] non-secure bit */
            | (4 << 2)	/* bit[4-2]: MT_NORMAL */
                        /* bit[1]: block (0) table (1) */
            | BIT(0);	/* bit[0]: valid */
    }

    for (i = GET_L1_INDEX(2*RMM_MAP_MEMORY_RANGE); i < GET_L1_INDEX(3*RMM_MAP_MEMORY_RANGE); i++) {
        _boot_pt_l1_0[i] = ((i-GET_L1_INDEX(2*RMM_MAP_MEMORY_RANGE)) << HP_1G_BLOCK_SHIFT)
            | BIT(10)	/* bit[10]: access flag */
            | (3 << 8)  /* bit[9-8]: inner shareable */
                        /* bit[7-6] data access permission bit */
            | (1 << 5)  /* bit[5] non-secure bit */
            | (4 << 2)	/* bit[4-2]: MT_NORMAL */
                        /* bit[1]: block (0) table (1) */
            | BIT(0);	/* bit[0]: valid */
    }

    /* FIXME: map for the temporary page */
    _boot_pt_l1_0[0] = ((uintptr_t) _boot_pt_l2_1) | BIT(1) | BIT(0);
    _boot_pt_l1_0[GET_L1_INDEX(RMM_MAP_MEMORY_RANGE)] = ((uintptr_t) _boot_pt_l2_0) | BIT(1) | BIT(0);
    _boot_pt_l2_0[0] = ((uintptr_t) pgtable_l3) | BIT(1) | BIT(0);
    /* End */

    // for uart 0x1c0a0000
    j = GET_L2_INDEX(0x1c000000);
    _boot_pt_l2_1[j] = (j << HP_2M_BLOCK_SHIFT)
             | BIT(10)	/* bit[10]: access flag */
             | (0 << 8)  /* bit[9-8]: inner shareable */
             /* bit[7-6] data access permission bit */
             | (1 << 5) /* bit[5] non-secure bit */
             | (0 << 2)	/* bit[4-2]: MT_DEVICE_nGnRnE */
                         /* bit[1]: block (0) table (1) */
             | BIT(0);	/* bit[0]: valid */
   
    print_info("here j: %lx\n", &j);
    init_pt = 1;
}

void mm_primary_init_tfa(void) {
    initialize_boot_page_table();
    activate_mmu();
}

void mm_primary_init_qemu(void) {
    unsigned long tcr_el2_value;
    asm volatile("mrs %0, tcr_el2" : "=r" (tcr_el2_value));
    print_info("[BOOT] tcr_el2 test %lx\r\n", tcr_el2_value);

    unsigned long sctlr_el2_value;
    asm volatile("mrs %0, sctlr_el2" : "=r" (sctlr_el2_value));

    unsigned long* ttbr0_el2_value;
    asm volatile("mrs %0, ttbr0_el2" : "=r" (ttbr0_el2_value));
    print_info("[BOOT] sctlr_el2 %lx ttbr0_el2_value %lx\r\n", sctlr_el2_value, ttbr0_el2_value);
    ttbr0_el2_value = (unsigned long)ttbr0_el2_value & 0xfffffffffffffffe;
    print_info("[BOOT] sctlr_el2 %lx ttbr0_el2_value %lx\r\n", sctlr_el2_value, ttbr0_el2_value);

    unsigned long va = RMM_MAP_MEMORY_RANGE;
    unsigned long pud_index = (va >> PUD_SHIFT) & (PTRS_PER_LEVEL - 1);
    unsigned long pmd_index = (va >> PMD_SHIFT) & (PTRS_PER_LEVEL - 1);
    unsigned long pte_index = (va >> PTE_SHIFT) & (PTRS_PER_LEVEL - 1);

    print_info("[Boot] pud_index %lx pmd_index %lx pte_index %lx\n", pud_index, pmd_index, pte_index);
    print_info("[Boot] ttbr0 index %d entry %lx\n", pud_index, ttbr0_el2_value[pud_index]);

    ttbr0_el2_value[pud_index] = ((uintptr_t) _boot_pt_l1_0) | BIT(1) | BIT(0);
    _boot_pt_l1_0[pmd_index] = ((uintptr_t) pgtable_l3) | BIT(1) | BIT(0);

    va = 0x1c000000;
    pud_index = (va >> PUD_SHIFT) & (PTRS_PER_LEVEL - 1);
    pmd_index = (va >> PMD_SHIFT) & (PTRS_PER_LEVEL - 1);

    uint64_t* l1_pa = ((ttbr0_el2_value[pud_index] >> 12) & 0x0fffffffff) << 12;
	print_info("Debug: l1_pa %lx ttbr0_el2_value %lx entry %lx %lx\n", l1_pa, ttbr0_el2_value, ttbr0_el2_value[pud_index], l1_pa[pmd_index]);
    l1_pa[pmd_index] = (pmd_index << HP_2M_BLOCK_SHIFT)
             | BIT(10)	/* bit[10]: access flag */
             | (0 << 8)  /* bit[9-8]: inner shareable */
             /* bit[7-6] data access permission bit */
             | (1 << 5) /* bit[5] non-secure bit */
             | (0 << 2)	/* bit[4-2]: MT_DEVICE_nGnRnE */
                         /* bit[1]: block (0) table (1) */
             | BIT(0);	/* bit[0]: valid */

    // uint64_t* l1_pa = ((ttbr0_el2_value[0] >> 12) & 0x0fffffffff) << 12;
	// print_info("Debug: l1_pa %lx ttbr0_el2_value %lx entry %lx %lx\n", l1_pa, ttbr0_el2_value, ttbr0_el2_value[0], ttbr0_el2_value[1]);
    // uint64_t* l2_pa = ((l1_pa[0b1111000] >> 12) & 0x0fffffffff) << 12;
    // print_info("Debug: l2_pa %lx entry %lx\n", l2_pa, l1_pa[0b1111000]);
	// uint64_t* l3_pa = ((l2_pa[0] >> 12) & 0x0fffffffff) << 12;
	// print_info("Debug: l3_pa %lx entry %lx\n", l3_pa, l2_pa[0]);
}

void mm_primary_init(void) {
    mm_primary_init_qemu();
}

int tmp = 0;

void mm_secondary_init(void) {
    print_info("mm_secondary_init before mmu init\n");
    if (init_pt == 0) {
        print_info("mm_secondary_init init_pt is 0\n");
        initialize_boot_page_table();
    }
    //FIXME
    activate_mmu();

    print_info("mm_secondary_init after activate mmu\n");
}

void virt_primary_init(void) {
    /* Turn on hypervisor related configuration */
    // print_info("begin to enable virt2\n");
    enable_hyp_mode2();
    // print_info("end of enable_hyp_mode2\n");

    /* Initialize all VMs statically */
    // init_vms();

    /* Turn on stage2 mmu for VMs */
    // activate_stage2_mmu();
}

void virt_secondary_init(void) {
    /* Turn on hypervisor related configuration */
    // print_info("begin to enable secondary virt2\n");
    enable_hyp_mode2();
    // print_info("end of secondary enable_hyp_mode2\n");

    /* Initialize all VMs statically */
    // init_vms();

    /* Turn on stage2 mmu for VMs */
    // activate_stage2_mmu();
}

void debug_print() {
    print_info("debug\n");
}
