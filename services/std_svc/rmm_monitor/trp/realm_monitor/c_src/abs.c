#include <buffer.h>
#include <assert.h>
#include <sysreg.h>
#include <stdbool.h>
#include <table.h>
#include <barriers.h>
#include <abs.h>
#include <mm/mmu_def.h>

extern uint64_t pgtable_l3[];
extern uint64_t _boot_pt_l0_0[];
extern uint64_t _boot_pt_l1_0[];
extern uint64_t _boot_pt_l2_0[];
extern void print_info(const char *fmt, ...);

void realm_printf(const char *output) {
	print_info("%s\n", output);
}


uint64_t *slot_to_pgte(enum BufferSlot slot)
{
	unsigned long idx;

	assert(slot < NR_CPU_SLOTS);
	idx = cpuid() * NR_CPU_SLOTS + slot;

	return &(((struct pgtable*)pgtable_l3)->pgte[idx]);
}

uint64_t *va_to_pgte(void *va)
{
	unsigned long offset, idx;

	offset = (unsigned long)va - RMM_MAP_MEMORY_RANGE;
	assert(GRANULE_ALIGNED(offset));

	idx = offset / GRANULE_SIZE;
	// if (idx > (NR_CPU_SLOTS * MAX_CPUS))
	// 	print_info("%s: va = %#lx, idx = %lu, max = %lu\n", __func__,
	// 		(unsigned long)va, idx, (NR_CPU_SLOTS * MAX_CPUS));
	assert(idx < (NR_CPU_SLOTS * MAX_CPUS));

	return &(((struct pgtable*)pgtable_l3)->pgte[idx]);
}

void *buffer_map(enum BufferSlot slot, unsigned long addr, bool ns)
{
	uint64_t pgte = addr;
	uint64_t *pgtep = slot_to_pgte(slot);

	assert(GRANULE_ALIGNED(addr));


	pgte |= PGTE_SLOT;
	if (ns)
		pgte |= PGTE_NS;

	// print_info("Debug: buffer_map addr is %x slot is %x pgte %lx\n", addr, slot, pgte);
	pgte_write(pgtep,  pgte);
	dsb(ishst);
	isb();

	return slot_to_va(slot);
}

void buffer_unmap(void *buf)
{
	uint64_t *pgtep = va_to_pgte(buf);
	// print_info("Debug: buffer_unmap addr is %lx\n", buf);

	compiler_barrier();

	pgte_write(pgtep, 0);
	dsb(ishst);

	asm volatile("tlbi alle2is\n" : : : "memory");
	dsb(ish);
	isb();
}

bool memcpy_ns_read(void *dest, const void *ns_src, unsigned long size);
bool memcpy_ns_write(void *dest, const void *ns_src, unsigned long size);

bool is_ns_slot(enum BufferSlot slot)
{
	return slot == SLOT_NS;
}

bool ns_buffer_write(enum BufferSlot slot,
		     unsigned int offset,
		     unsigned int size,
		     void *src)
{
	assert(is_ns_slot(slot));
	assert(ALIGNED(size, 8));
	assert(ALIGNED(offset, 8));
	assert(ALIGNED(src, 8));

	offset &= ~GRANULE_MASK;
	assert(offset + size <= GRANULE_SIZE);

	return memcpy_ns_write(slot_to_va(slot) + offset, src, size);
}

bool ns_buffer_read(enum BufferSlot slot,
		    unsigned int offset,
		    unsigned int size,
		    void *dest)
{
	assert(is_ns_slot(slot));
	assert(ALIGNED(size, 8));
	assert(ALIGNED(offset, 8));
	assert(ALIGNED(dest, 8));

	offset &= ~GRANULE_MASK;
	assert(offset + size <= GRANULE_SIZE);

	// print_info("ns_buffer_read \n");
	// print_info("ns_buffer_read va %lx\n", slot_to_va(slot));
	// print_info("ns_buffer_read va_value %lx\n", *(unsigned long *)slot_to_va(slot));

	return memcpy_ns_read(dest, slot_to_va(slot) + offset, size);
}

bool ns_buffer_read_data(enum BufferSlot slot, void *data)
{
	return ns_buffer_read(SLOT_NS, 0, GRANULE_SIZE, data);
}

bool ns_buffer_read_rec_run(enum BufferSlot slot, void *data)
{
	return ns_buffer_read(SLOT_NS, 0, sizeof(struct rec_run), data);
}

bool ns_buffer_write_rec_run(enum BufferSlot slot, void *r)
{
	return ns_buffer_write(SLOT_NS, 0, sizeof(struct rec_run), r);
}

bool ns_buffer_read_rec_params(enum BufferSlot slot, void *data)
{
	return ns_buffer_read(SLOT_NS, 0, sizeof(struct rec_params), data);
}

void ns_buffer_unmap(enum BufferSlot slot)
{
	assert(is_ns_slot(slot));
	buffer_unmap(slot_to_va(slot));
}

void *realm_memset(void *dst, int ch, size_t size) {
	char *d;
	uint64_t i;

	d = (char *)dst;
	for (i = 0; i < size; ++i)
		d[i] = ch;
    return dst;
}

void memzero(unsigned long buf, unsigned long size) 
{
	// print_info("Debug: memzero %lx\n", buf);
	uint64_t* l1_pa = ((_boot_pt_l0_0[0] >> 12) & ((1<<48) - 1)) << 12;
	// print_info("Debug: l1_pa %lx _boot_pt_l1_0 %lx\n", l1_pa, _boot_pt_l1_0);
	uint64_t* l2_pa = ((l1_pa[4] >> 12) & ((1<<48) - 1)) << 12;
	// print_info("Debug: GET_L1_INDEX(RMM_MAP_MEMORY_RANGE) %x\n", GET_L1_INDEX(RMM_MAP_MEMORY_RANGE));
	// print_info("Debug: l2_pa %lx _boot_pt_l2_0 %lx\n", l2_pa, _boot_pt_l2_0);
	uint64_t* l3_pa = ((l2_pa[0] >> 12) & ((1<<48) - 1)) << 12;
	// print_info("Debug: l3_pa %lx pgtable_l3 %lx\n", l3_pa, pgtable_l3);
	// print_info("Debug: memzero l3_pa entry[%d]: %lx\n", (buf >> 12) & 0b111111111, l3_pa[(buf >> 12) & 0b111111111]);
	// print_info("Debug: size0 %lx\n", size);
	// char tmp = ((char *)buf)[0];
	// print_info("Debug: read test2 %x\n", tmp);
	// ((char *)buf)[0] = 1;
	// print_info("Debug: write test\n");

	realm_memset((char *)buf, 0, size);
}

static inline void stage2_tlbi_ipa(unsigned long ipa, unsigned long size)
{
	while (size) {
		asm volatile("tlbi ipas2e1is, %0 \n" : : "r" (ipa >> 12) : "memory");
		size -= GRANULE_SIZE;
		ipa += GRANULE_SIZE;
	}
	dsb(ish);
	asm volatile("tlbi vmalle1is\n" : : : "memory");
	dsb(ish);
	isb();
}

void clean_realm_stage2()
{
	asm volatile("tlbi vmalls12e1\n" : : : "memory");
	dsb(nsh);
	isb();
}

void barrier() 
{
	dsb(ish);
}

void invalidate_block(unsigned long addr)
{
	barrier();
	stage2_tlbi_ipa(addr, GRANULE_SIZE);
}

void invalidate_pages_in_block(unsigned long addr)
{
	barrier();
	stage2_tlbi_ipa(addr, BLOCK_L2_SIZE);
}

void invalidate_page(unsigned long addr)
{
	barrier();
	stage2_tlbi_ipa(addr, GRANULE_SIZE);
}

/*
 * Atomically set bit @bit in value pointed to by @val with release semantics.
 */
void c_atomic_bit_set_release_64(unsigned long *bitmap, unsigned long idx, unsigned long bit)
{
	uint64_t *loc = &bitmap[idx];
	uint64_t mask = (1UL << bit);

	asm volatile(
	"	stsetl %[mask], %[loc]\n"
	: [loc] "+Q" (*loc)
	: [mask] "r" (mask)
	: "memory"
	);
}

/*
 * Test bit @bit in value pointed to by @loc with acquire semantics.
 */
bool c_test_bit_acquire_64(unsigned long *bitmap, unsigned long idx, unsigned long bit)
{
	uint64_t *loc = &bitmap[idx];
	uint64_t _val;
	uint64_t mask = (1UL << bit);

	asm volatile(
	"	ldar %[_val], %[loc]\n"
	: [_val] "=r" (_val)
	: [loc] "Q" (*loc)
	: "memory"
	);

	return _val & mask;
}

/*
 * Returns the index of the first bit set in @bitmap from @start inclusive.
 * The index is zero-based from LSB (0) to MSB (63).
 * When no such bits are set, returns BITS_PER_UL (64).
 */
unsigned long find_next_set_bit(unsigned long bitmap,
					      unsigned long start)
{
	unsigned long mask = (~0UL) << start;
	unsigned long ns;

	if (start >= 64)
		return 64;

	bitmap &= mask;

	asm(
	"	rbit %[bitmap], %[bitmap]\n"
	"	clz %[ns], %[bitmap]\n"
	: [bitmap] "+r" (bitmap), [ns] "=r" (ns)
	);

	return ns;
}

void asm_isb() {
	isb();
}
