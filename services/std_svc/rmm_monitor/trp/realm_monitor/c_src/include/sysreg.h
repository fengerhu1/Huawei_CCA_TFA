#ifndef __SYSREG_H_
#define __SYSREG_H_

#include <const.h>

#define MPAM0_EL1		S3_0_C10_C5_1
#define ICC_HPPIR1_EL1		S3_0_C12_C12_2
#define ICC_SRE_EL2		S3_4_C12_C9_5

#define __str(reg)		#reg

#define sysreg_read(reg)					\
({								\
	unsigned long __val;					\
	asm volatile("mrs %0, " __str(reg) "\n" : "=r" (__val));	\
	__val;						\
})

#define sysreg_write(reg, val)					\
do {								\
	unsigned long __val = (val);				\
	asm volatile("msr " __str(reg) ", %0\n" : : "r" (__val));	\
} while (0)

#define cpuid() \
	({	\
		unsigned long mpidr = sysreg_read(MPIDR_EL1); \
		((mpidr>>16) & 0xff) *4 + ((mpidr>>8) & 0xff); \
	})

/*
 * System register field definitions.
 *
 * For any register field we define:
 * - <register>_<field>_SHIFT
 *   The bit offset of the LSB of the field.
 * - <register>_<field>_WIDTH
 *   The width of the field in bits.
 *
 * For single bit fields, we define:
 * - <register>_<field>_BIT
 *   The in-place value of the field with the bit set.
 *
 * For multi-bit fields, we define:
 * - <register>_<field>_<enum>
 *   The in-place value of the field set to the value corresponding to the
 *   enumeration name.
 *
 * For any register field, we define:
 * - INPLACE(<register>_<field>, val)
 *   The in-place value of the field set to val, handling any necessary type
 *   promotion to avoid truncation of val.
 * - MASK(<register>_<field)
 *   An in-place bitmask covering all bits of the field.
 */

#define INPLACE(regfield, val) \
	(((val) + UL(0)) << regfield##_SHIFT)

#define MASK(regfield) \
	((~0UL >> (64 - regfield##_WIDTH)) << regfield##_SHIFT)

#define TCR_EL2_T0SZ_SHIFT	0
#define TCR_EL2_T0SZ_WIDTH	6
#define TCR_TCR_EL2_T0SZ_48	INPLACE(TCR_EL2_T0SZ, 16)

#define TCR_EL2_EPD0_SHIFT	7
#define TCR_EL2_EPD0_WIDTH	1
#define TCR_EL2_EPD0_BIT	INPLACE(TCR_EL2_EPD0, 1)

#define TCR_EL2_IRGN0_SHIFT	8
#define TCR_EL2_IRGN0_WIDTH	2
#define TCR_EL2_IRGN0_WBWA	INPLACE(TCR_EL2_IRGN0, 1)

#define TCR_EL2_ORGN0_SHIFT	10
#define TCR_EL2_ORGN0_WIDTH	2
#define TCR_EL2_ORGN0_WBWA	INPLACE(TCR_EL2_ORGN0, 1)

#define TCR_EL2_SH0_SHIFT	12
#define TCR_EL2_SH0_WIDTH	2
#define TCR_EL2_SH0_IS		INPLACE(TCR_EL2_SH0, 3)

#define TCR_EL2_TG0_SHIFT	14
#define TCR_EL2_TG0_WIDTH	2
#define TCR_EL2_TG0_4K		INPLACE(TCR_EL2_TG0, 0)

#define MAIR_ELx_ATTR0_SHIFT	0
#define MAIR_ELx_ATTR0_WIDTH	8
#define MAIR_ELx_ATTR0_MASK	MASK(MAIR_ELx_ATTR0)

#define MAIR_DEV_NGNRNE		0x0 /* Device nGnRnE */
#define MAIR_DEV_NGNRNE_IDX	0x1

#define MAIR_NIOWBNTRW		0xff
#define MAIR_NIOWBNTRW_IDX	0x0

#define TTBRx_EL2_CnP_SHIFT	0
#define TTBRx_EL2_CnP_WIDTH	1

#define TTBRx_EL2_BADDR_SHIFT	1
#define TTBRx_EL2_BADDR_WIDTH	47

#define TTBRx_EL2_ASID_SHIFT	48
#define TTBRx_EL2_ASID_WIDTH	16


#define ESR_EL2_EC_SHIFT	26
#define ESR_EL2_EC_WIDTH	6
#define ESR_EL2_EC_MASK		MASK(ESR_EL2_EC)

#define ESR_EL2_IL_SHIFT	25
#define ESR_EL2_IL_WIDTH	1
#define ESR_EL2_IL_MASK		MASK(ESR_EL2_EC)

#define ESR_EL2_ISS_SHIFT	0
#define ESR_EL2_ISS_WIDTH	25
#define ESR_EL2_ISS_MASK	MASK(ESR_EL2_ISS)

#define ESR_EL2_EC_WFX		INPLACE(ESR_EL2_EC, 1)
#define ESR_EL2_EC_SVC		INPLACE(ESR_EL2_EC, 21)
#define ESR_EL2_EC_HVC		INPLACE(ESR_EL2_EC, 22)
#define ESR_EL2_EC_SMC		INPLACE(ESR_EL2_EC, 23)
#define ESR_EL2_EC_SYSREG	INPLACE(ESR_EL2_EC, 24)
#define ESR_EL2_EC_INST_ABORT	INPLACE(ESR_EL2_EC, 32)
#define ESR_EL2_EC_DATA_ABORT	INPLACE(ESR_EL2_EC, 36)


/* Data/Instruction Abort ESR fields */
#define ESR_EL2_ABORT_ISV_BIT		(1UL << 24)
#define NOT_ESR_EL2_ABORT_ISV_BIT	(~ESR_EL2_ABORT_ISV_BIT)

#define ESR_EL2_ABORT_SAS_SHIFT		22
#define ESR_EL2_ABORT_SAS_WIDTH		2
#define ESR_EL2_ABORT_SAS_MASK		MASK(ESR_EL2_ABORT_SAS)

#define ESR_EL2_ABORT_SAS_BYTE_VAL	0
#define ESR_EL2_ABORT_SAS_HWORD_VAL	1
#define ESR_EL2_ABORT_SAS_WORD_VAL	2
#define ESR_EL2_ABORT_SAS_DWORD_VAL	3

#define ESR_EL2_ABORT_SSE_BIT		(1UL << 21)

#define ESR_EL2_ABORT_SRT_SHIFT		16
#define ESR_EL2_ABORT_SRT_WIDTH		5
#define ESR_EL2_ABORT_SRT_MASK		MASK(ESR_EL2_ABORT_SRT)

#define ESR_EL2_ABORT_SF_BIT		(1UL << 15)
#define ESR_EL2_ABORT_FNV_BIT		(1UL << 10)
#define ESR_EL2_ABORT_WNR_BIT		(1UL << 6)
#define ESR_EL2_ABORT_FSC_SHIFT		0
#define ESR_EL2_ABORT_FSC_WIDTH		6
#define ESR_EL2_ABORT_FSC_MASK		MASK(ESR_EL2_ABORT_FSC)

#define ESR_EL2_ABORT_FSC_TRANSLATION_FAULT	0x04
#define ESR_EL2_ABORT_FSC_PERMISSION_FAULT	0x0c
#define ESR_EL2_ABORT_FSC_LEVEL_SHIFT		0
#define ESR_EL2_ABORT_FSC_LEVEL_WIDTH		2
#define ESR_EL2_ABORT_FSC_LEVEL_MASK		MASK(ESR_EL2_ABORT_FSC_LEVEL)

#define ESR_EL2_SYSREG_TRAP_OP0_SHIFT	20
#define ESR_EL2_SYSREG_TRAP_OP0_WIDTH	2
#define ESR_EL2_SYSREG_TRAP_OP0_MASK	MASK(ESR_EL2_SYSREG_TRAP_OP0)

#define ESR_EL2_SYSREG_TRAP_OP2_SHIFT	17
#define ESR_EL2_SYSREG_TRAP_OP2_WIDTH	3
#define ESR_EL2_SYSREG_TRAP_OP2_MASK	MASK(ESR_EL2_SYSREG_TRAP_OP2)

#define ESR_EL2_SYSREG_TRAP_OP1_SHIFT	14
#define ESR_EL2_SYSREG_TRAP_OP1_WIDTH	3
#define ESR_EL2_SYSREG_TRAP_OP1_MASK	MASK(ESR_EL2_SYSREG_TRAP_OP1)

#define ESR_EL2_SYSREG_TRAP_CRN_SHIFT	10
#define ESR_EL2_SYSREG_TRAP_CRN_WIDTH	4
#define ESR_EL2_SYSREG_TRAP_CRN_MASK	MASK(ESR_EL2_SYSREG_TRAP_CRN)

#define ESR_EL2_SYSREG_TRAP_RT_SHIFT	5
#define ESR_EL2_SYSREG_TRAP_RT_WIDTH	5
#define ESR_EL2_SYSREG_TRAP_RT_MASK	MASK(ESR_EL2_SYSREG_TRAP_RT)

#define ESR_EL2_SYSREG_TRAP_CRM_SHIFT	1
#define ESR_EL2_SYSREG_TRAP_CRM_WIDTH	4
#define ESR_EL2_SYSREG_TRAP_CRM_MASK	MASK(ESR_EL2_SYSREG_TRAP_CRM)

/* WFx ESR fields */
#define ESR_EL2_WFx_TI_BIT		(1UL << 0)

/* xVC ESR fields */
#define ESR_EL2_xVC_IMM_SHIFT		0
#define ESR_EL2_xVC_IMM_WIDTH		16
#define ESR_EL2_xVC_IMM_MASK		MASK(ESR_EL2_xVC_IMM)

/* HPFAR_EL2 definitions */
#define HPFAR_EL2_FIPA_SHIFT		4
#define HPFAR_EL2_FIPA_WIDTH		40
#define HPFAR_EL2_FIPA_MASK		MASK(HPFAR_EL2_FIPA)
#define HPFAR_EL2_FIPA_OFFSET		8

/* SPSR definitions */
#define SPSR_EL2_MODE_SHIFT		0
#define SPSR_EL2_MODE_WIDTH		4
#define SPSR_EL2_MODE_EL0t		INPLACE(SPSR_EL2_MODE, 0)

#define SPSR_EL2_MODE_SHIFT		0
#define SPSR_EL2_MODE_WIDTH		4
#define SPSR_EL2_MODE_EL1h		INPLACE(SPSR_EL2_MODE, 5)


#define SPSR_EL2_nRW_SHIFT		4
#define SPSR_EL2_nRW_WIDTH		1
#define SPSR_EL2_nRW_AARCH64		INPLACE(SPSR_EL2_nRW, 0)
#define SPSR_EL2_nRW_AARCH32		INPLACE(SPSR_EL2_nRW, 1)

#define SPSR_EL2_F_SHIFT		6
#define SPSR_EL2_F_WIDTH		1
#define SPSR_EL2_F_BIT			INPLACE(SPSR_EL2_F, 1)

#define SPSR_EL2_I_SHIFT		7
#define SPSR_EL2_I_WIDTH		1
#define SPSR_EL2_I_BIT			INPLACE(SPSR_EL2_I, 1)

#define SPSR_EL2_A_SHIFT		8
#define SPSR_EL2_A_WIDTH		1
#define SPSR_EL2_A_BIT			INPLACE(SPSR_EL2_A, 1)

#define SPSR_EL2_D_SHIFT		9
#define SPSR_EL2_D_WIDTH		1
#define SPSR_EL2_D_BIT			INPLACE(SPSR_EL2_D, 1)

#define SPSR_EL2_SSBS_SHIFT		12
#define SPSR_EL2_SSBS_WIDTH		1
#define SPSR_EL2_SSBS_BIT		INPLACE(SPSR_EL2_SSBS, 1)

#define SPSR_EL2_IL_SHIFT		20
#define SPSR_EL2_IL_WIDTH		1
#define SPSR_EL2_IL_BIT			INPLACE(SPSR_EL2_IL, 1)

#define SPSR_EL2_SS_SHIFT		21
#define SPSR_EL2_SS_WIDTH		1
#define SPSR_EL2_SS_BIT			INPLACE(SPSR_EL2_SS, 1)

#define SPSR_EL2_PAN_SHIFT		22
#define SPSR_EL2_PAN_WIDTH		1
#define SPSR_EL2_PAN_BIT		INPLACE(SPSR_EL2_PAN, 1)

#define SPSR_EL2_UAO_SHIFT		23
#define SPSR_EL2_UAO_WIDTH		1
#define SPSR_EL2_UAO_BIT		INPLACE(SPSR_EL2_UAO, 1)

#define SPSR_EL2_V_SHIFT		28
#define SPSR_EL2_V_WIDTH		1
#define SPSR_EL2_V_BIT			INPLACE(SPSR_EL2_V, 1)

#define SPSR_EL2_C_SHIFT		29
#define SPSR_EL2_C_WIDTH		1
#define SPSR_EL2_C_BIT			INPLACE(SPSR_EL2_C, 1)

#define SPSR_EL2_Z_SHIFT		30
#define SPSR_EL2_Z_WIDTH		1
#define SPSR_EL2_Z_BIT			INPLACE(SPSR_EL2_Z, 1)

#define SPSR_EL2_N_SHIFT		31
#define SPSR_EL2_N_WIDTH		1
#define SPSR_EL2_N_BIT			INPLACE(SPSR_EL2_N, 1)


/* HCR definitions */
#define HCR_FWB		(UL(1) << 46)
#define HCR_TEA		(UL(1) << 37)
#define HCR_API		(UL(1) << 41)
#define HCR_APK		(UL(1) << 40)
#define HCR_TERR	(UL(1) << 36)
#define HCR_TLOR	(UL(1) << 35)
#define HCR_E2H		(UL(1) << 34)
#define HCR_RW		(UL(1) << 31)
#define HCR_TGE		(UL(1) << 27)
#define HCR_TSW		(UL(1) << 22)
#define HCR_TACR	(UL(1) << 21)
#define HCR_TIDCP	(UL(1) << 20)
#define HCR_TSC		(UL(1) << 19)
#define HCR_TWE		(UL(1) << 14)
#define HCR_TWI		(UL(1) << 13)
#define HCR_TID3	(UL(1) << 18)

#define HCR_BSU_SHIFT	10
#define HCR_BSU_WIDTH	2
#define HCR_BSU_IS	INPLACE(HCR_BSU, 1) /* Barriers are promoted to IS */

#define HCR_FB		(UL(1) << 9)
#define HCR_VI		(UL(1) << 7)
#define HCR_AMO		(UL(1) << 5)
#define HCR_IMO		(UL(1) << 4)
#define HCR_FMO		(UL(1) << 3)
#define HCR_PTW		(UL(1) << 2)
#define HCR_SWIO	(UL(1) << 1)
#define HCR_VM		(UL(1) << 0)

/* TODO verify that all the traps are enabled */
#define HCR_FLAGS (HCR_FWB | HCR_E2H | HCR_RW | HCR_TSC | HCR_AMO | \
	HCR_BSU_IS | HCR_IMO | HCR_FMO | HCR_PTW | HCR_SWIO | HCR_VM | \
	HCR_TID3)

/* VTCR definitions */
#define VTCR_T0SZ_SHIFT		0
#define VTCR_T0SZ_WIDTH		6
#define VTCR_T0SZ_48		INPLACE(VTCR_T0SZ, 16)

#define VTCR_SL0_SHIFT		6
#define VTCR_SL0_WIDTH		2
#define VTCR_SL0_4		INPLACE(VTCR_SL0, 2)

#define VTCR_IRGN0_SHIFT	8
#define VTCR_IRGN0_WIDTH	2
#define VTCR_IRGN0_WBRAWA	INPLACE(VTCR_IRGN0, 1)

#define VTCR_ORGN0_SHIFT	10
#define VTCR_ORGN0_WIDTH	2
#define VTCR_ORGN0_WBRAWA	INPLACE(VTCR_ORGN0, 1)

#define VTCR_SH0_SHIFT		12
#define VTCR_SH0_WIDTH		2
#define VTCR_SH0_IS		INPLACE(VTCR_SH0, 3)

#define VTCR_TG0_SHIFT		14
#define VTCR_TG0_WIDTH		2
#define VTCR_TG0_4K		INPLACE(VTCR_TG0, 0)

#define VTCR_PS_SHIFT		6
#define VTCR_PS_WIDTH		3
#define VTCR_PS_40		INPLACE(VTCR_PS, 2)

#define VTCR_NSA		(UL(1) << 30)
#define VTCR_RES1		(UL(1) << 31)

#define VTCR_FLAGS ( \
	VTCR_T0SZ_48      | /* size(non-secure IPA) = 48 */ \
	VTCR_SL0_4        | /* 4 levels in non-secure s2 PT */ \
	VTCR_IRGN0_WBRAWA | /* PTW inner cache attr. is WB RAWA*/ \
	VTCR_ORGN0_WBRAWA | /* PTW outer cache attr. is WB RAWA*/ \
	VTCR_SH0_IS       | /* PTW shareability attr. is Outer Sharable*/\
	VTCR_TG0_4K       | /* 4K granule size in non-secure PT*/ \
	VTCR_PS_40        | /* size(PA) = 40 */   \
	/* VS = 0              size(VMID) = 8 */ \
	/* NSW = 0             non-secure s2 is made of secure pages*/ \
	VTCR_NSA           | /* non-secure IPA maps to non-secure PA */ \
	VTCR_RES1 \
	)


/* SCTLR definitions */
#define SCTLR_EL1_EE		(UL(1) << 25)
#define SCTLR_EL1_SPAN		(UL(1) << 23)
#define SCTLR_EL1_EIS		(UL(1) << 22)
#define SCTLR_EL1_nTWE		(UL(1) << 18)
#define SCTLR_EL1_nTWI		(UL(1) << 16)
#define SCTLR_EL1_EOS		(UL(1) << 11)
#define SCTLR_EL1_nAA		(UL(1) << 6)
#define SCTLR_EL1_CP15BEN	(UL(1) << 5)
#define SCTLR_EL1_SA0		(UL(1) << 4)
#define SCTLR_EL1_SA		(UL(1) << 3)

#define SCTLR_EL1_FLAGS (SCTLR_EL1_SPAN | SCTLR_EL1_EIS | SCTLR_EL1_nTWE | \
	SCTLR_EL1_nTWI | SCTLR_EL1_EOS | SCTLR_EL1_nAA | SCTLR_EL1_CP15BEN | \
	SCTLR_EL1_SA0 | SCTLR_EL1_SA)

/* PMCR_EL0 Definitions */
#define PMCR_EL0_LC_SHIFT		6
#define PMCR_EL0_LC_WIDTH		1
#define PMCR_EL0_LC_BIT			INPLACE(PMCR_EL0_LC, 1)

#define PMCR_EL0_RES1			PMCR_EL0_LC_BIT


/* MDSCR_EL1 Definitions */
#define MDSCR_EL1_TDCC_SHIFT		12
#define MDSCR_EL1_TDCC_WIDTH		1
#define MDSCR_EL1_TDCC_BIT		INPLACE(MDSCR_EL1_TDCC, 1)

/* SCTLR register definitions */
#define SCTLR_EL2_RES1		((1 << 22) /* TODO: ARMv8.5-CSEH, otherwise RES1 */ | \
				 (1 << 11) /* TODO: ARMv8.5-CSEH, otherwise RES1 */)

#define SCTLR_EL2_M		(1 << 0)
#define SCTLR_EL2_C		(1 << 2)
#define SCTLR_EL2_SA		(1 << 3)
#define SCTLR_EL2_SA0		(1 << 4)
#define SCTLR_EL2_SED		(1 << 8)
//#define SCTLR_EL2_EOS		(1 << 11)	/* TODO: ARMv8.5-CSEH, otherwise RES1 */
#define SCTLR_EL2_I		(1 << 12)
#define SCTLR_EL2_DZE		(1 << 14)
#define SCTLR_EL2_UCT		(1 << 15)
#define SCTLR_EL2_NTWI		(1 << 16)
#define SCTLR_EL2_NTWE		(1 << 18)
#define SCTLR_EL2_WXN		(1 << 19)
#define SCTLR_EL2_TSCXT		(1 << 20)
//#define SCTLR_EL2_EIS		(1 << 22)	/* TODO: ARMv8.5-CSEH, otherwise RES1 */
#define SCTLR_EL2_SPAN		(1 << 23)
#define SCTLR_EL2_UCI		(1 << 26)
#define SCTLR_EL2_NTLSMD	(1 << 28)
#define SCTLR_EL2_LSMAOE	(1 << 29)
/* HCR_EL2.E2H == 0b1 and HCR_EL2.TGE == 0b1 */
#define SECURE_SCTLR_EL2_RES1	((1 << 22) /* TODO: ARMv8.5-CSEH, otherwise RES1 */ | \
				 (1 << 11) /* TODO: ARMv8.5-CSEH, otherwise RES1 */)


#define TCR_EL2_RES1		((1 << 31) | (1 << 23))
#define TCR_EL2_T0SZ_48		(0b010000 << 0)
#define TCR_EL2_T0SZ_36		(0b011100 << 0)
#define TCR_EL2_T1SZ_36		(0b011100 << 16)
#define TCR_EL2_EPD1		(0b01 << 23)
#define TCR_EL2_IRGN1_WBWA	(0b01 << 24)
#define TCR_EL2_ORGN1_WBWA	(0b01 << 26)
#define TCR_EL2_SH1_IS		(0b11 << 28)
#define TCR_EL2_TG1_4K		(0b10 << 30)
#define TCR_EL2_IPS		(0b001 << 32)
#define TCR_EL2_AS		(1 << 36)
#define TCR_EL2_HPD0		(1 << 41)
#define TCR_EL2_HPD1		(1 << 42)
#define TCR_EL2_E0PD1		(1 << 56)	/* TODO: ARMv8.5-E0PD, otherwise RES0 */

#define MPIDR_EL1_AFF_MASK	0xFF
#define MPIDR_EL1_AFF0_SHIFT	0
#define MPIDR_EL1_AFF1_SHIFT	8
#define MPIDR_EL1_AFF2_SHIFT	16
#define MPIDR_EL1_AFF3_SHIFT	32

#define MPIDR_EL1_AFF0		INPLACE(MPIDR_EL1_AFF0, MPIDR_EL1_AFF_MASK)
#define MPIDR_EL1_AFF1		INPLACE(MPIDR_EL1_AFF1, MPIDR_EL1_AFF_MASK)
#define MPIDR_EL1_AFF2		INPLACE(MPIDR_EL1_AFF2, MPIDR_EL1_AFF_MASK)
#define MPIDR_EL1_AFF3		INPLACE(MPIDR_EL1_AFF3, MPIDR_EL1_AFF_MASK)

#define SYSREG_ESR(op0, op1, crn, crm, op2) \
		(((op0) << ESR_EL2_SYSREG_TRAP_OP0_SHIFT) | \
		 ((op1) << ESR_EL2_SYSREG_TRAP_OP1_SHIFT) | \
		 ((crn) << ESR_EL2_SYSREG_TRAP_CRN_SHIFT) | \
		 ((crm) << ESR_EL2_SYSREG_TRAP_CRM_SHIFT) | \
		 ((op2) << ESR_EL2_SYSREG_TRAP_OP2_SHIFT))

#define ESR_EL2_SYSREG_MASK		SYSREG_ESR(3, 7, 15, 15, 7)

#define ESR_EL2_SYSREG_ID_MASK		SYSREG_ESR(3, 7, 15, 0, 0)
#define ESR_EL2_SYSREG_ID		SYSREG_ESR(3, 0, 0, 0, 0)

#define ESR_EL2_SYSREG_ID_AA64PFR0_EL1	SYSREG_ESR(3, 0, 0, 4, 0)
#define ESR_EL2_SYSREG_ID_AA64PFR1_EL1	SYSREG_ESR(3, 0, 0, 4, 1)
#define ESR_EL2_SYSREG_ID_AA64ZFR0_EL1	SYSREG_ESR(3, 0, 0, 4, 4)

#define ESR_EL2_SYSREG_ID_AA64DFR0_EL1	SYSREG_ESR(3, 0, 0, 5, 0)
#define ESR_EL2_SYSREG_ID_AA64DFR1_EL1	SYSREG_ESR(3, 0, 0, 5, 1)

#define ESR_EL2_SYSREG_ID_AA64AFR0_EL1	SYSREG_ESR(3, 0, 0, 5, 4)
#define ESR_EL2_SYSREG_ID_AA64AFR1_EL1	SYSREG_ESR(3, 0, 0, 5, 5)

#define ESR_EL2_SYSREG_ID_AA64ISAR0_EL1	SYSREG_ESR(3, 0, 0, 6, 0)
#define ESR_EL2_SYSREG_ID_AA64ISAR1_EL1	SYSREG_ESR(3, 0, 0, 6, 1)

#define ESR_EL2_SYSREG_ID_AA64MMFR0_EL1	SYSREG_ESR(3, 0, 0, 7, 0)
#define ESR_EL2_SYSREG_ID_AA64MMFR1_EL1	SYSREG_ESR(3, 0, 0, 7, 1)
#define ESR_EL2_SYSREG_ID_AA64MMFR2_EL1	SYSREG_ESR(3, 0, 0, 7, 2)

#define ESR_EL2_SYSREG_ID_AA64ISAR1_GPI_SHIFT	28
#define ESR_EL2_SYSREG_ID_AA64ISAR1_GPA_SHIFT	24
#define ESR_EL2_SYSREG_ID_AA64ISAR1_API_SHIFT	8
#define ESR_EL2_SYSREG_ID_AA64ISAR1_APA_SHIFT	4

#define ESR_EL2_SYSREG_TIMERS_MASK		SYSREG_ESR(3, 3, 15, 12, 0)
#define ESR_EL2_SYSREG_TIMERS			SYSREG_ESR(3, 3, 14, 0, 0)

#define ESR_EL2_SYSREG_TIMER_CNTP_TVAL_EL0	SYSREG_ESR(3, 3, 14, 2, 0)
#define ESR_EL2_SYSREG_TIMER_CNTP_CTL_EL0	SYSREG_ESR(3, 3, 14, 2, 1)
#define ESR_EL2_SYSREG_TIMER_CNTP_CVAL_EL0	SYSREG_ESR(3, 3, 14, 2, 2)
#define ESR_EL2_SYSREG_TIMER_CNTV_TVAL_EL0	SYSREG_ESR(3, 3, 14, 3, 0)
#define ESR_EL2_SYSREG_TIMER_CNTV_CTL_EL0	SYSREG_ESR(3, 3, 14, 3, 1)
#define ESR_EL2_SYSREG_TIMER_CNTV_CVAL_EL0	SYSREG_ESR(3, 3, 14, 3, 2)

#define ESR_EL2_SYSREG_ICC_EL1_MASK		SYSREG_ESR(3, 3, 15, 8, 0)
#define ESR_EL2_SYSREG_ICC_EL1			SYSREG_ESR(3, 0, 12, 8, 0)

#define ESR_EL2_SYSREG_DIRECTION	(1 << 0)
#define ESR_EL2_SYSREG_IS_WRITE(esr)	(!((esr) & ESR_EL2_SYSREG_DIRECTION))

#define ESR_IL(esr)	(!!((esr) & ESR_EL2_IL_MASK))
#define ESR_ISS(esr)	((esr) & ESR_EL2_ISS_MASK)

#define ESR_EL2_SYSREG_ISS_RT(esr) \
	((ESR_ISS(esr) & ESR_EL2_SYSREG_TRAP_RT_MASK) >> ESR_EL2_SYSREG_TRAP_RT_SHIFT)

#define ICC_SRE_EL2_ENABLE		(1 << 3)
#define NOT_ICC_SRE_EL2_ENABLE		(~ICC_SRE_EL2_ENABLE)

#define ICC_HPPIR1_EL1_INTID_SHIFT	0
#define ICC_HPPIR1_EL1_INTID_WIDTH	24
#define ICC_HPPIR1_EL1_INTID		MASK(ICC_HPPIR1_EL1_INTID)

#define CNTHCTL_EL2_EL1PCTEN	(1 << 10)
#define CNTHCTL_EL2_EL1PTEN	(1 << 11)
#define NOT_CNTHCTL_EL2_EL1PTEN	(~CNTHCTL_EL2_EL1PTEN)
#define CNTHCTL_EL2_EL1TVT	(1 << 13)
#define NOT_CNTHCTL_EL2_EL1TVT	(~CNTHCTL_EL2_EL1TVT)
#define CNTHCTL_EL2_EL1TVCT	(1 << 14)

#define CNTHCTL_EL2_NO_TRAPS	(CNTHCTL_EL2_EL1PCTEN | \
				 CNTHCTL_EL2_EL1PTEN)

#define CNTx_CTL_ENABLE		(1 << 0)
#define CNTx_CTL_IMASK		(1 << 1)
#define CNTx_CTL_ISTATUS	(1 << 2)

#endif /* __SYSREG_H_ */
