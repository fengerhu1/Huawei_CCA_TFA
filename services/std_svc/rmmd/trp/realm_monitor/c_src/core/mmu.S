/*
 * (C) COPYRIGHT 2019 ARM Limited or its affiliates.
 * ALL RIGHTS RESERVED
 */
#include <assembler.h>
#include <sizes.h>
#include <platform.h>
#include <pagetable.h>
#include <mmu.h>
#include <sysreg.h>

#define CODE_ATTRS	(AP(RO) 			|\
			AF				|\
			ATTRIDX(MAIR_NIOWBNTRW_IDX) 	|\
			SH(INNER_SHAREABLE))

#define RO_ATTRS	(AP(RO) 			|\
			XN				|\
			PXN				|\
			AF				|\
			ATTRIDX(MAIR_NIOWBNTRW_IDX) 	|\
			SH(INNER_SHAREABLE))


#define RW_ATTRS	(AP(RW) 			|\
			XN				|\
			PXN				|\
			AF				|\
			ATTRIDX(MAIR_NIOWBNTRW_IDX) 	|\
			SH(INNER_SHAREABLE))

#define IO_ATTRS	(AP(RW) 			|\
			XN				|\
			PXN				|\
			AF				|\
			ATTRIDX(MAIR_DEV_NGNRNE_IDX) 	|\
			SH(NON_SHAREABLE))

#define SCTLR_EL2		(SCTLR_EL2_M   /* MMU enabled */ \
				/* SCTLR_EL2_A = 0 (No alignment checks) */ | \
				 SCTLR_EL2_C   /* Data accesses are cacheable
						* as per translation tables */ | \
				 SCTLR_EL2_SA  /* SP aligned at EL2 */ | \
				 SCTLR_EL2_SA0  /* SP Alignment check enable for EL0 */ \
				/* SCTLR_EL2_CP15BEN = 0 (EL0 using AArch32:
				 * EL0 execution of the CP15DMB, CP15DSB, and
				 * CP15ISB instructions is UNDEFINED. */ \
				/* SCTLR_EL2_NAA = 0 (unaligned MA fault at EL2 and EL0) */ \
				/* SCTLR_EL2_ITD = 0 (A32 Only) */ | \
				 SCTLR_EL2_SED /* A32 Only, RES1 for non-A32 systems */ \
				/* SCTLR_EL2_EOS TODO: ARMv8.5-CSEH, otherwise RES1 */ | \
				 SCTLR_EL2_I	 /* I$ is ON for EL2 and EL0 */ | \
				 SCTLR_EL2_DZE   /* Do not trap DC ZVA */ | \
				 SCTLR_EL2_UCT   /* Allow EL0 access to CTR_EL0 */ | \
				 SCTLR_EL2_NTWI  /* Don't trap WFI from EL0 to EL2 */ | \
				 SCTLR_EL2_NTWE  /* Don't trap WFE from EL0 to EL2 */ | \
				 SCTLR_EL2_WXN   /* W implies XN */ | \
				 SCTLR_EL2_TSCXT /* Trap EL0 accesss to SCXTNUM_EL0 */ \
				/* SCTLR_EL2_EIS EL2 exception is context
				 * synchronizing
				 * TODO: ARMv8.5-CSEH, otherwise RES1 */ \
				 /* SCTLR_EL2_SPAN = 0 (Set PSTATE.PAN = 1 on
				 * exceptions to EL2)) */ | \
				 SCTLR_EL2_UCI /* Allow cache maintenance
						* instructions at EL0 */ | \
				 SCTLR_EL2_NTLSMD /* A32/T32 only */ | \
				 SCTLR_EL2_LSMAOE /* A32/T32 only */ | \
				 SECURE_SCTLR_EL2_RES1)

#define TCR_EL2			(TCR_EL2_T0SZ_36	/* 28 (VA range 36 bits, for now) */ | \
				 /* TTBR0_EL2 enabled */ \
				 TCR_EL2_IRGN0_WBWA	/* 0b01 (Normal, Inner WB RA WA Cacheable) */ | \
				 TCR_EL2_ORGN0_WBWA	/* 0b01 (Normal, Outer WB RA WA Cacheable) */ | \
				 TCR_EL2_SH0_IS		/* 0b11 (IS) */ | \
				/* TCR_EL2_TG0 = 0b00 (4K pages) */ \
				 TCR_EL2_T1SZ_36	/* 36 bit VA at S-EL2 upper range */ | \
				/* TCR_EL2_A1: 0 (TTBR0_EL2.ASID defines the ASID) */ \
				/* TCR_RL2_EPD1: 0 (TTBR1_EL2 not disabled) */ \
				 TCR_EL2_IRGN1_WBWA	/* 0b01 (Normal, Inner WB RA WA Cacheable) */ | \
				 TCR_EL2_ORGN1_WBWA	/* 0b01 (Normal, Outer WB RA WA Cacheable) */ | \
				 TCR_EL2_SH1_IS		/* 0b11 (inner shareable page table walks) */ | \
				 TCR_EL2_TG1_4K		/* 0b10 (4K pages) */ | \
				 TCR_EL2_IPS		/* 0b001 (36 bits for now) */ | \
				 TCR_EL2_AS		/* 1 (16 bits ASID) */ | \
				/* TCR_EL2_TBI0: 0 (top byte used for address calculation) */ \
				/* TCR_EL2_TBI1: 0 (top byte used for address calculation) */ \
				/* TCR_EL2_HA: 0 (hardware access flag update disabled) */ \
				/* TCR_EL2_HD: 0 (hardware managed of dirty flag disabled) */ \
				 TCR_EL2_HPD0 /* 1 (hierarchical permissions
					       * are disabled - simpler to reason about) */ | \
				 TCR_EL2_HPD1 /* 1 (hierarchical permissions
					       * are disabled - simpler to reason about)*/ \
				/* TCR_EL2_HWU059: 0 (hardware cannot use bit 59 in page tables) */ \
				/* TCR_EL2_HWU060: 0 (hardware cannot use bit 60 in page tables) */ \
				/* TCR_EL2_HWU061: 0 (hardware cannot use bit 61 in page tables) */ \
				/* TCR_EL2_HWU062: 0 (hardware cannot use bit 62 in page tables) */ \
				/* TCR_EL2_HWU159: 0 (hardware cannot use bit 59 in page tables) */ \
				/* TCR_EL2_HWU160: 0 (hardware cannot use bit 60 in page tables) */ \
				/* TCR_EL2_HWU161: 0 (hardware cannot use bit 61 in page tables) */ \
				/* TCR_EL2_HWU162: 0 (hardware cannot use bit 62 in page tables) */ \
				/* TCR_EL2_TBID0: 0 (pointer auth not yet enabled) */ \
				/* TCR_EL2_TBID1: 0 (pointer auth not yet enabled) */ \
				/* TCR_EL2_NFD0: 0 (SVE not enabled, do not disable
				 * stage 1 translation walks using TTBR0_EL2) */ \
				/* TCR_EL2_NFD1: 0 (SVE not enabled, do not disable
				 * stage 1 translation walks using TTBR1_EL2) */ \
				/* TCR_EL2_E0PD0: 0 (EL0 access to anything
				 * translated by TTBR0_EL2 will not generate a fault) */ \
				/* RES0 (TODO: ARMv8.5-E0PD, otherwise RES0
				 * TCR_EL2_E0PD1=1 (EL0 access to anything translated by
				 * TTBR1_EL2 will generate a fault) */ \
				/* TCR_EL2_TCCMA0: 0 (memory tagging not yet enabled) */ \
				/* TCR_EL2_TCCMA1: 0 (memory tagging not yet enabled) */)


#define MAIR_EL2	((MAIR_DEV_NGNRNE << (MAIR_DEV_NGNRNE_IDX*8)) | \
			 (MAIR_NIOWBNTRW << (MAIR_NIOWBNTRW_IDX*8)))


/* x0: cpuid */
ENTRY(setup_mmu):
	str	lr, [sp, #-16]!

	/*
	 * setup pagetables only on primary core. For secondary cores
	 * use the translation base populated by primary core
	 */
	cbnz	x0, 1f

	/* create upper half pagetables for RMMs use */
	bl		pagetable_create

	/* create lower half pagetable entry for MMU enable code */
	va_to_pa	x0, tt_l2_mmu
	va_to_pa	x1, setup_mmu
	bic		x1, x1, #SZ_2M - 1
	mov		x2, x1
	mov		x3, #SZ_2M
	ldr		x4, =CODE_ATTRS
	bl		create_pte_l2
	cbnz		x0, 2f

1:	/* switch on the mmu */
	/*
	 *  Get the value of lr before we switch on the mmu
	 * as it will be inaccessible after that.
	 */
	ldr		lr, [sp], #16

	ldr		x1, =TCR_EL2
	msr		tcr_el2, x1

	ldr		x1, =MAIR_EL2
	msr		mair_el2, x1

	va_to_pa	x0, tt_l1_upper
	msr		ttbr1_el2, x0

	va_to_pa	x0, tt_l1_lower
	msr		ttbr0_el2, x0

	ldr		x1, =SCTLR_EL2
	msr		sctlr_el2, x1

	dsb		sy
	isb

	/* computer the offset for phys to virt calculation */
	ldr		x0, =RMM_PHYS
	ldr		x1, =RMM_VIRT
	sub		x1, x1, x0

	/* fix lr and sp from pa to va */
	add		lr, lr, x1
	add		sp, sp, x1
	ret

2:	/* pagetable creation error */
	b	.
ENDPROC(setup_mmu)

ENTRY(pagetable_create):
	str		lr, [sp, #-16]!

	/* populate rmm code and ro pt entries */
	map_section	tt_l3_rmm, rmm_text_start, rmm_text_end, CODE_ATTRS, 3

	/* populate rmm ro pt entries */
	map_section	tt_l3_rmm, rmm_ro_start, rmm_ro_end, RO_ATTRS, 3

	/* populate rmm rw pt entries */
	map_section	tt_l2_rmm, rmm_rw_start, rmm_rw_end, RW_ATTRS, 2

	/* populate IO entries */
	map_io		tt_l3_io, UART0_VIRT, UART0_PHYS, SZ_4K, IO_ATTRS, 3

	ldr		lr, [sp], #16
	ret

pt_error:
	b	.
ENDPROC(pagetable_create)

.section ".data"
.align 12
	/* construct a level 1 page and level 2 entry for mmu enable code */
	tt_l1_lower:
		table_1G	tt_l2_mmu, 0	/* block 0 for RMM code, data */
		fill_entry	tt_l1_lower

	/* allocate level 2 page each entry is of size 2M */
	tt_l2_mmu:
		fill_entry	tt_l2_mmu


	/* construct a level 1 page with 1G sized entries for rmm */
	tt_l1_upper:
		table_1G	tt_l2_rmm, 0	/* block 0 for RMM code, data */
		inv_entry 	61		/* invalid entries x61 */
		table_1G	pgtable_l2, 0	/* block 62 for buffer mapping */
		table_1G	tt_l2_io, 0	/* block 63 for IO */
		fill_entry	tt_l1_upper

	/* allocate level 2 page each entry is of size 2M */
	tt_l2_rmm:
		table_2M	tt_l3_rmm
		fill_entry	tt_l2_rmm

	tt_l2_io:
		inv_entry 	511		/* invalid entries  */
		table_2M	tt_l3_io

	tt_l3_rmm:
		fill_entry	tt_l3_rmm

	tt_l3_io:
		fill_entry	tt_l3_io

