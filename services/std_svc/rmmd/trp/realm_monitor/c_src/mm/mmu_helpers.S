#include <mm/mmu_def.h>
#include <common/asm.h>

/* SCTLR_EL2 System Control Register aarch64 */

#define SCTLR_EL2_EE                BIT(25)     /* Endianness of data accesses at EL2, and stage 1 translation table walks in the EL2&0 translation regime */
#define SCTLR_EL2_WXN               BIT(19)     /* Write permission implies XN (Execute-never) */
#define SCTLR_EL2_I                 BIT(12)     /* Instruction access Cacheability control, for accesses at EL2 */
#define SCTLR_EL2_SA                BIT(3)      /* SP Alignment check */
#define SCTLR_EL2_C                 BIT(2)      /* Cacheability control for data accesses */
#define SCTLR_EL2_A                 BIT(1)      /* Alignment check enable */
#define SCTLR_EL2_M                 BIT(0)      /* MMU enable for EL2 stage 1 address translation */

/* 
#define SCTLR_EL2 (SCTLR_EL2_M | \
				 SCTLR_EL2_C  | \
				 SCTLR_EL2_I)
*/

#define TCR_T0SZ(x)       ((64 - (x)))
#define TCR_T1SZ(x)       ((64 - (x)) << 16)
#define TCR_TxSZ(x)       (TCR_T0SZ(x) | TCR_T1SZ(x))

#define TCR_IRGN0_WBWC    (1 << 8)
#define TCR_IRGN_NC       ((0 << 8) | (0 << 24))
#define TCR_IRGN_WBWA     ((1 << 8) | (1 << 24))
#define TCR_IRGN_WT       ((2 << 8) | (2 << 24))
#define TCR_IRGN_WBnWA    ((3 << 8) | (3 << 24))
#define TCR_IRGN_MASK     ((3 << 8) | (3 << 24))

#define TCR_ORGN0_WBWC    (1 << 10)
#define TCR_ORGN_NC       ((0 << 10) | (0 << 26))
#define TCR_ORGN_WBWA     ((1 << 10) | (1 << 26))
#define TCR_ORGN_WT       ((2 << 10) | (2 << 26))
#define TCR_ORGN_WBnWA    ((3 << 10) | (3 << 26))
#define TCR_ORGN_MASK     ((3 << 10) | (3 << 26))

#define TCR_SH0_ISH       (3 << 12)

#define TCR_TG0_4K        (0 << 14)
#define TCR_TG0_64K       (1 << 14)
#define TCR_TG1_4K        (2 << 30)
#define TCR_TG1_64K       (3 << 30)

#define TCR_PS_4G         (0 << 16)
#define TCR_PS_64G        (1 << 16)
#define TCR_PS_1T         (2 << 16)
#define TCR_PS_4T         (3 << 16)
#define TCR_PS_16T        (4 << 16)
#define TCR_PS_256T       (5 << 16)

/* bits are reserved as 1 */
#define TCR_EL2_RES1      ((1 << 23) | (1 << 31))
#define TCR_ASID16        (1 << 36)

#define UL(x) x##UL

#define TCR_SH0_SHIFT 12
#define TCR_SH0_MASK (UL(3) << TCR_SH0_SHIFT)
#define TCR_SH0_INNER (UL(3) << TCR_SH0_SHIFT)
#define TCR_SH1_SHIFT 28
#define TCR_SH1_MASK (UL(3) << TCR_SH1_SHIFT)
#define TCR_SH1_INNER (UL(3) << TCR_SH1_SHIFT)

#define TCR_SHARED (TCR_SH0_INNER | TCR_SH1_INNER)

#define TCR_TBI0 (UL(1) << 37)
#define TCR_A1   (UL(1) << 22)


#define ID_AA64PFR0_EL2_GIC     (0b1111 << 24)

#define MT_DEVICE_nGnRnE  0
#define MT_DEVICE_nGnRE   1
#define MT_DEVICE_GRE     2
#define MT_NORMAL_NC      3
#define MT_NORMAL         4
#define MAIR(_attr, _mt)  ((_attr) << ((_mt) * 8))

#define HCR_EL2_MIOCNCE             BIT(38)   /* Mismatched inner/outer cacheable non-coherency enable For EL1&0 */  
#define HCR_SEL2_TEA		        BIT(37)   /* Route synchronous external abort exceptions to EL2 */
#define HCR_SEL2_TERR	            BIT(36)   /* Trap error record access to EL2 */
#define HCR_SEL2_TLOR	            BIT(35)   /* Trap LOR registers */ 
#define HCR_SEL2_E2H	        	BIT(34)   /* EL2 host: whether OS is running in EL2 */
#define HCR_SEL2_ID	                BIT(33)   /* Stage 2 instruction access cacheability disable */
#define HCR_SEL2_CD	            	BIT(32)   /* Stage 2 data access cacheability disable */
#define HCR_SEL2_RW		            BIT(31)   /* Execution state control for lower exception levels */
#define HCR_SEL2_TRVM	            BIT(30)   /* Trap reads of virtual memory controls */
#define HCR_SEL2_HCD		        BIT(29)   /* HVC instruction diable */
#define HCR_SEL2_TDZ		        BIT(28)   /* Trap DC ZVA instrucitons */
#define HCR_SEL2_TGE		        BIT(27)   /* Trap general exceptions from EL0 */
#define HCR_SEL2_TVM		        BIT(26)   /* Trap writes of virtual memory controls */
#define HCR_SEL2_TTLB	            BIT(25)   /* Trap TLB maintenance instructions */
#define HCR_SEL2_TPU		        BIT(24)   /* Trap cache maintenance instructions that operate to the point of unification */
#define HCR_SEL2_TPC		        BIT(23)   /* Trap data or unified cache maintenance instructions that operate to the point of coherency */
#define HCR_SEL2_TSW		        BIT(22)   /* Trap data or unified cache maintenance instructions that operate by Set/Way */
#define HCR_SEL2_TAC		        BIT(21)   /* Trap Auxiliary Control Registers */
#define HCR_SEL2_TIDCP	            BIT(20)
#define HCR_SEL2_TSC	            BIT(19)   /* Trap SMC instructions */
#define HCR_SEL2_TID3	            BIT(18)   /* Trap ID group 3 */
#define HCR_SEL2_TID2	            BIT(17)   /* Trap ID group 2 */
#define HCR_SEL2_TID1	            BIT(16)   /* Trap ID group 1 */
#define HCR_SEL2_TID0	            BIT(15)   /* Trap ID group 0 */
#define HCR_SEL2_TWE	            BIT(14)   /* Traps EL0 and EL1 execution of WFE instructions to EL2, from both Execution states. */
#define HCR_SEL2_TWI	            BIT(13)   /* Traps EL0 and EL1 execution of WFI instructions to EL2, from both Execution states. */
#define HCR_SEL2_DC		            BIT(12)  
#define HCR_SEL2_BSU	            (3 << 10) /* Barrier Shareability upgrade */
#define HCR_SEL2_BSU_IS	            BIT(10) 
#define HCR_SEL2_FB		            BIT(9)    /* Force broadcast */
#define HCR_SEL2_VSE	            BIT(8)    /* Virtual SError interrupt */ 
#define HCR_SEL2_VI		            BIT(7)    /* Virtual IRQ Interrupt */
#define HCR_SEL2_VF		            BIT(6)    /* Virtual FIQ Interrupt */
#define HCR_SEL2_AMO	            BIT(5)    /* Physical SError Interrupt routing */
#define HCR_SEL2_IMO	            BIT(4)    /* Physical IRQ Routing */
#define HCR_SEL2_FMO	            BIT(3)    /* Physical FIQ Routing */
#define HCR_SEL2_PTW	            BIT(2)    /* Protected Table Walk */
#define HCR_SEL2_SWIO	            BIT(1)    /* Set/Way Invalidation Override */
#define HCR_SEL2_VM		            BIT(0)    /* Virtualization enable */


.macro enable_hcr2 hcr tmp
    mov     \tmp, #0
    /* Enable MMU */
    orr     \tmp, \tmp, #HCR_SEL2_VM
    orr     \tmp, \tmp, #HCR_SEL2_RW
    orr     \tmp, \tmp, #HCR_SEL2_FMO
    orr     \tmp, \tmp, #HCR_SEL2_TSC
    orr     \tmp, \tmp, #HCR_SEL2_E2H
//    orr     \tmp, \tmp, #HCR_SEL2_HCD
    msr     \hcr, \tmp
    isb
.endm

BEGIN_FUNC(enable_hyp_mode2)
    enable_hcr2 hcr_el2, x10
END_FUNC(enable_hyp_mode2)

.extern _boot_pt_l0_0
.extern _boot_pt_l0_1


LOCAL_FUNC_BEGIN(flush_dcache)
    dcache  cisw
    ret
LOCAL_FUNC_END(flush_dcache)

LOCAL_FUNC_BEGIN(invalidate_dcache)
    dcache  isw
    ret
LOCAL_FUNC_END(invalidate_dcache)

LOCAL_FUNC_BEGIN(invalidate_icache)
    ic      iallu
    dsb     nsh
    isb
    ret
LOCAL_FUNC_END(invalidate_icache)

BEGIN_FUNC(flush_dcache_and_tlb)
    stp     x29, x30, [sp, #-16]!
    mov     x29, sp

    tlbi    alle2
    dsb     sy
    isb
    
    ic      ialluis

    bl      flush_dcache

    ldp     x29, x30, [sp], #16
    ret
END_FUNC(flush_dcache_and_tlb)

.macro enable_mmu sctlr tmp
	mrs     \tmp, \sctlr
    /* Enable MMU */
	orr     \tmp, \tmp, #SCTLR_EL2_M
    /* Disable alignment checking */
	bic     \tmp, \tmp, #SCTLR_EL2_A
	bic     \tmp, \tmp, #SCTLR_EL2_SA
	/* Data accesses Cacheable */
    orr     \tmp, \tmp, #SCTLR_EL2_C
    /* Instruction access Cacheable */
	orr     \tmp, \tmp, #SCTLR_EL2_I
	msr     \sctlr, \tmp
	isb
.endm

.macro enable_mmu_disable_cache sctlr tmp
	mrs     \tmp, \sctlr
    /* Enable MMU */
	orr     \tmp, \tmp, #SCTLR_EL2_M
    /* Disable alignment checking */
	bic     \tmp, \tmp, #SCTLR_EL2_A
	bic     \tmp, \tmp, #SCTLR_EL2_SA
	/* Data accesses Cacheable */
    orr     \tmp, \tmp, #SCTLR_EL2_C
    /* Instruction access Cacheable */
	orr     \tmp, \tmp, #SCTLR_EL2_I
	msr     \sctlr, \tmp
	isb
.endm

.macro disable_mmu sctlr tmp
	mrs     \tmp, \sctlr
    /* Disable MMU */
	bic     \tmp, \tmp, #SCTLR_EL2_M
    /* Disable alignment checking */
	bic     \tmp, \tmp, #SCTLR_EL2_A
	bic     \tmp, \tmp, #SCTLR_EL2_SA
	bic     \tmp, \tmp, #SCTLR_EL2_C
    /* Disable Instruction Cache */
	bic     \tmp, \tmp, #SCTLR_EL2_I
	msr     \sctlr, \tmp
	isb
.endm

.macro flush_dcache
    MOV    r1, #0                        // Initialize segment counter outer_loop

    MOV    r0, #0                        // Initialize line counter inner_loop

    ORR    r2, r1, r0                    // Generate segment and line address

    MCR    p15, 0, r2, c7, c14, 2        // Clean and flush the line

    ADD    r0, r0, #0x20                 // Increment to next line

    CMP    r0, #0x400                    // Complete all entries in one segment?

    BNE    inner_loop                    // If not branch back to inner_loop

    ADD    r1, r1, #0x40000000           // Increment segment counter

    CMP    r1, #0x0                      // Complete all segments

    BNE    outer_loop                    // If not branch back to outer_loop
.endm


BEGIN_FUNC(activate_mmu)
    /* We call nested functions, follow the ABI. */
    stp     x29, x30, [sp, #-16]!
    mov     x29, sp

    bl      flush_dcache

    /* Ensure I-cache, D-cache and mmu are disabled for EL1/Stage1 */
    disable_mmu sctlr_el2 , x8

    /*
     * Invalidate the local I-cache so that any instructions fetched
     * speculatively are discarded.
     */
    bl      invalidate_icache

    /*
     *   DEVICE_nGnRnE      000     00000000
     *   DEVICE_nGnRE       001     00000100
     *   DEVICE_GRE         010     00001100
     *   NORMAL_NC          011     01000100
     *   NORMAL             100     11111111
     */
    ldr     x5, =MAIR(0x00, MT_DEVICE_nGnRnE) |\
                 MAIR(0x04, MT_DEVICE_nGnRE) |\
                 MAIR(0x0c, MT_DEVICE_GRE) |\
                 MAIR(0x44, MT_NORMAL_NC) |\
                 MAIR(0xff, MT_NORMAL)
    msr     mair_el2, x5

    ldr     x10, =TCR_TxSZ(48) | TCR_IRGN_WBWA | TCR_ORGN_WBWA | TCR_TG0_4K | TCR_TG1_4K | TCR_ASID16 | TCR_SHARED | TCR_PS_1T

    mrs     x9, ID_AA64MMFR0_EL1
    bfi     x10, x9, #32, #3
    msr     tcr_el2, x10

    /* Setup page tables */
    adrp    x8, _boot_pt_l0_0
    msr     ttbr0_el2, x8
//    adrp    x8, _boot_pt_l1_1
//    msr     ttbr1_el2, x8
    isb

    /* invalidate all TLB entries for EL2 */
    tlbi    vmalle1is
    dsb     ish
    isb

    enable_mmu sctlr_el2 , x8

    ldp     x29, x30, [sp], #16
    ret
END_FUNC(activate_mmu)


BEGIN_FUNC(activate_mmu_disable_cache)
    /* We call nested functions, follow the ABI. */
    stp     x29, x30, [sp, #-16]!
    mov     x29, sp

    bl      flush_dcache

    /* Ensure I-cache, D-cache and mmu are disabled for EL1/Stage1 */
    disable_mmu sctlr_el2 , x8

    /*
     * Invalidate the local I-cache so that any instructions fetched
     * speculatively are discarded.
     */
    bl      invalidate_icache

    /*
     *   DEVICE_nGnRnE      000     00000000
     *   DEVICE_nGnRE       001     00000100
     *   DEVICE_GRE         010     00001100
     *   NORMAL_NC          011     01000100
     *   NORMAL             100     11111111
     */
    ldr     x5, =MAIR(0x00, MT_DEVICE_nGnRnE) |\
                 MAIR(0x04, MT_DEVICE_nGnRE) |\
                 MAIR(0x0c, MT_DEVICE_GRE) |\
                 MAIR(0x44, MT_NORMAL_NC) |\
                 MAIR(0xff, MT_NORMAL)
    msr     mair_el2, x5

    ldr     x10, =TCR_TxSZ(48) | TCR_IRGN_WBWA | TCR_ORGN_WBWA | TCR_TG0_4K | TCR_TG1_4K | TCR_ASID16 | TCR_SHARED | TCR_PS_1T

    mrs     x9, ID_AA64MMFR0_EL1
    bfi     x10, x9, #32, #3
    msr     tcr_el2, x10

    /* Setup page tables */
    adrp    x8, _boot_pt_l0_0
    msr     ttbr0_el2, x8
//    adrp    x8, _boot_pt_l1_1
//    msr     ttbr1_el2, x8
    isb

    /* invalidate all TLB entries for EL2 */
    tlbi    vmalle1is
    dsb     ish
    isb

    enable_mmu_disable_cache sctlr_el2 , x8

    ldp     x29, x30, [sp], #16
    ret
END_FUNC(activate_mmu_disable_cache)
