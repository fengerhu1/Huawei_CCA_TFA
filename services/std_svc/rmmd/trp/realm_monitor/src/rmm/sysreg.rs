#![feature(asm)]
pub const MPIDR_EL1_AFF_MASK: usize =	0xFF;
pub const MPIDR_EL1_AFF0_SHIFT: usize =	0;
pub const MPIDR_EL1_AFF1_SHIFT: usize =	8;
pub const MPIDR_EL1_AFF2_SHIFT: usize =	16;
pub const MPIDR_EL1_AFF3_SHIFT: usize =	32;

pub const MPIDR_EL1_AFF0: usize =		crate::INPLACE!(MPIDR_EL1_AFF0_SHIFT, MPIDR_EL1_AFF_MASK);
pub const MPIDR_EL1_AFF1: usize =		crate::INPLACE!(MPIDR_EL1_AFF1_SHIFT, MPIDR_EL1_AFF_MASK);
pub const MPIDR_EL1_AFF2: usize =		crate::INPLACE!(MPIDR_EL1_AFF2_SHIFT, MPIDR_EL1_AFF_MASK);
pub const MPIDR_EL1_AFF3: usize =		crate::INPLACE!(MPIDR_EL1_AFF3_SHIFT, MPIDR_EL1_AFF_MASK);

/* SPSR definitions */
pub const SPSR_EL2_MODE_SHIFT:usize = 		0;
pub const SPSR_EL2_MODE_WIDTH:usize = 		4;
pub const SPSR_EL2_MODE_EL0t:usize = 		crate::INPLACE!(SPSR_EL2_MODE_SHIFT, 0);
pub const SPSR_EL2_MODE_EL1h:usize = 		crate::INPLACE!(SPSR_EL2_MODE_SHIFT, 5);

pub const SPSR_EL2_nRW_SHIFT:usize = 		4;
pub const SPSR_EL2_nRW_WIDTH:usize =		1;
pub const SPSR_EL2_nRW_AARCH64:usize =		crate::INPLACE!(SPSR_EL2_nRW_SHIFT, 0);
pub const SPSR_EL2_nRW_AARCH32:usize =		crate::INPLACE!(SPSR_EL2_nRW_SHIFT, 1);

pub const SPSR_EL2_F_SHIFT:usize =		6;
pub const SPSR_EL2_F_WIDTH:usize =		1;
pub const SPSR_EL2_F_BIT:usize =			crate::INPLACE!(SPSR_EL2_F_SHIFT, 1);

pub const SPSR_EL2_I_SHIFT:usize =		7;
pub const SPSR_EL2_I_WIDTH:usize =		1;
pub const SPSR_EL2_I_BIT:usize =			crate::INPLACE!(SPSR_EL2_I_SHIFT, 1);

pub const SPSR_EL2_A_SHIFT:usize =		8;
pub const SPSR_EL2_A_WIDTH:usize =		1;
pub const SPSR_EL2_A_BIT:usize =			crate::INPLACE!(SPSR_EL2_A_SHIFT, 1);

pub const SPSR_EL2_D_SHIFT:usize =		9;
pub const SPSR_EL2_D_WIDTH:usize =		1;
pub const SPSR_EL2_D_BIT:usize =			crate::INPLACE!(SPSR_EL2_D_SHIFT, 1);

pub const PSTATE_INIT:usize = SPSR_EL2_MODE_EL1h |	SPSR_EL2_nRW_AARCH64 |	SPSR_EL2_F_BIT |
                SPSR_EL2_I_BIT | SPSR_EL2_A_BIT | SPSR_EL2_D_BIT;

/* PMCR_EL0 Definitions */
pub const PMCR_EL0_LC_SHIFT:usize =		6;
pub const PMCR_EL0_LC_WIDTH:usize =		1;
pub const PMCR_EL0_LC_BIT:usize =			crate::INPLACE!(PMCR_EL0_LC_SHIFT, 1);

pub const PMCR_EL0_RES1:usize =			PMCR_EL0_LC_BIT;

/* MDSCR_EL1 Definitions */
pub const MDSCR_EL1_TDCC_SHIFT:usize =		12;
pub const MDSCR_EL1_TDCC_WIDTH:usize =		1;
pub const MDSCR_EL1_TDCC_BIT:usize =		crate::INPLACE!(MDSCR_EL1_TDCC_SHIFT, 1);

/* SCTLR definitions */
pub const SCTLR_EL1_EE:usize =		1<< 25;
pub const SCTLR_EL1_SPAN:usize =		1<< 23;
pub const SCTLR_EL1_EIS:usize =		1<< 22;
pub const SCTLR_EL1_nTWE:usize =		1<< 18;
pub const SCTLR_EL1_nTWI:usize =		1<< 16;
pub const SCTLR_EL1_EOS:usize =		1<< 11;
pub const SCTLR_EL1_nAA:usize =		1<< 6;
pub const SCTLR_EL1_CP15BEN:usize =	1<< 5;
pub const SCTLR_EL1_SA0:usize =		1<< 4;
pub const SCTLR_EL1_SA:usize =		1<< 3;

pub const SCTLR_EL1_FLAGS:usize = (SCTLR_EL1_SPAN | SCTLR_EL1_EIS | SCTLR_EL1_nTWE | 
	SCTLR_EL1_nTWI | SCTLR_EL1_EOS | SCTLR_EL1_nAA | SCTLR_EL1_CP15BEN | 
	SCTLR_EL1_SA0 | SCTLR_EL1_SA);

/* HCR definitions */
pub const HCR_FWB:usize =		1<< 46; // Forced write-back
pub const HCR_TEA:usize =		1<< 37;
pub const HCR_API:usize =		1<< 41;
pub const HCR_APK:usize =		1<< 40;
pub const HCR_TERR:usize =	1<< 36;
pub const HCR_TLOR:usize =	1<< 35;
pub const HCR_E2H:usize =		1<< 34;
pub const HCR_RW:usize =		1<< 31;
pub const HCR_TGE:usize =		1<< 27;
pub const HCR_TSW:usize =		1<< 22;
pub const HCR_TACR:usize =	1<< 21;
pub const HCR_TIDCP:usize =	1<< 20;
pub const HCR_TSC:usize =		1<< 19; //Trap SMC instructions 
pub const HCR_TWE:usize =		1<< 14;
pub const HCR_TWI:usize =		1<< 13;
pub const HCR_TID3:usize =	1<< 18;

pub const HCR_BSU_SHIFT:usize =	10;
pub const HCR_BSU_WIDTH:usize =	2;
pub const HCR_BSU_IS:usize =	crate::INPLACE!(HCR_BSU_SHIFT, 1); /* Barriers are promoted to IS */

pub const HCR_FB:usize =		1<< 9;
pub const HCR_VI:usize =		1<< 7;
pub const HCR_AMO:usize =		1<< 5; //Physical SError interrupt routing
pub const HCR_IMO:usize =		1<< 4; //Physical IRQ Routing
pub const HCR_FMO:usize =		1<< 3; //Physical FIQ Routing
pub const HCR_PTW:usize =		1<< 2; //Protected Table Walk
pub const HCR_SWIO:usize =	1<< 1;  //Set/Way Invalidation Override
pub const HCR_VM:usize =		1<< 0; //Virtualization enable

/* TODO verify that all the traps are enabled */
pub const HCR_FLAGS:usize = (HCR_FWB | HCR_E2H | HCR_RW | HCR_TSC | HCR_AMO |
	HCR_BSU_IS | HCR_IMO | HCR_FMO | HCR_PTW | HCR_SWIO | HCR_VM |
	HCR_TID3);

/* VTCR definitions */
pub const VTCR_T0SZ_SHIFT:usize =		0;
pub const VTCR_T0SZ_WIDTH:usize =		6;
pub const VTCR_T0SZ_48:usize =		crate::INPLACE!(VTCR_T0SZ_SHIFT, 16);

pub const VTCR_SL0_SHIFT:usize =		6;
pub const VTCR_SL0_WIDTH:usize =		2;
pub const VTCR_SL0_4:usize =		crate::INPLACE!(VTCR_SL0_SHIFT, 2);

pub const VTCR_IRGN0_SHIFT:usize =	8;
pub const VTCR_IRGN0_WIDTH:usize =	2;
pub const VTCR_IRGN0_WBRAWA:usize =	crate::INPLACE!(VTCR_IRGN0_SHIFT, 1);

pub const VTCR_ORGN0_SHIFT:usize =	10;
pub const VTCR_ORGN0_WIDTH:usize =	2;
pub const VTCR_ORGN0_WBRAWA:usize =	crate::INPLACE!(VTCR_ORGN0_SHIFT, 1);

pub const VTCR_SH0_SHIFT:usize =		12;
pub const VTCR_SH0_WIDTH:usize =		2;
pub const VTCR_SH0_IS:usize =		crate::INPLACE!(VTCR_SH0_SHIFT, 3);

pub const VTCR_TG0_SHIFT:usize =		14;
pub const VTCR_TG0_WIDTH:usize =		2;
pub const VTCR_TG0_4K:usize =		crate::INPLACE!(VTCR_TG0_SHIFT, 0);

pub const VTCR_PS_SHIFT:usize =		16;
pub const VTCR_PS_WIDTH:usize =		3;
pub const VTCR_PS_40:usize =		crate::INPLACE!(VTCR_PS_SHIFT, 2);

pub const VTCR_NSA:usize =		1 << 30;
pub const VTCR_RES1:usize =		1 << 31;

pub const VTCR_FLAGS:usize = 
	VTCR_T0SZ_48      | /* size(non-secure IPA) = 48 */ 
	VTCR_SL0_4        | /* 4 levels in non-secure s2 PT */ 
	VTCR_IRGN0_WBRAWA | /* PTW inner cache attr. is WB RAWA*/ 
	VTCR_ORGN0_WBRAWA | /* PTW outer cache attr. is WB RAWA*/ 
	VTCR_SH0_IS       | /* PTW shareability attr. is Outer Sharable*/
	VTCR_TG0_4K       | /* 4K granule size in non-secure PT*/ 
	VTCR_PS_40        | /* size(PA) = 40 */   
	/* VS = 0              size(VMID) = 8 */ 
	/* NSW = 0             non-secure s2 is made of secure pages*/ 
	VTCR_NSA           | /* non-secure IPA maps to non-secure PA */ 
	VTCR_RES1;

// CNTHCTL
pub const CNTHCTL_EL2_EL1PCTEN:usize =	1 << 10;
pub const CNTHCTL_EL2_EL1PTEN:usize =	1 << 11;
pub const NOT_CNTHCTL_EL2_EL1PTEN:usize =	!CNTHCTL_EL2_EL1PTEN;
pub const CNTHCTL_EL2_EL1TVT:usize =	1 << 13;
pub const NOT_CNTHCTL_EL2_EL1TVT:usize =	(!CNTHCTL_EL2_EL1TVT);
pub const CNTHCTL_EL2_EL1TVCT:usize =	1 << 14;

pub const CNTHCTL_EL2_NO_TRAPS:usize =	CNTHCTL_EL2_EL1PCTEN | 
                    CNTHCTL_EL2_EL1PTEN;

// TTBRx
pub const TTBRx_EL2_CnP_SHIFT:usize =	0;
pub const TTBRx_EL2_CnP_WIDTH:usize =	1;

pub const TTBRx_EL2_BADDR_SHIFT:usize =	1;
pub const TTBRx_EL2_BADDR_WIDTH:usize =	47;

pub const TTBRx_EL2_ASID_SHIFT:usize =	48;
pub const TTBRx_EL2_ASID_WIDTH:usize =	16;

//ESR EL2 register
pub const ESR_EL2_EC_SHIFT:usize =	26;
pub const ESR_EL2_EC_WIDTH:usize =	6;
pub const ESR_EL2_EC_MASK:usize =		crate::MASK!(ESR_EL2_EC_WIDTH, ESR_EL2_EC_SHIFT);

pub const ESR_EL2_IL_SHIFT:usize =	25;
pub const ESR_EL2_IL_WIDTH:usize =	1;
pub const ESR_EL2_IL_MASK:usize =		crate::MASK!(ESR_EL2_EC_WIDTH, ESR_EL2_EC_SHIFT);

pub const ESR_EL2_ISS_SHIFT:usize =	0;
pub const ESR_EL2_ISS_WIDTH:usize =	25;
pub const ESR_EL2_ISS_MASK:usize =	crate::MASK!(ESR_EL2_ISS_WIDTH, ESR_EL2_ISS_SHIFT);

pub const ESR_EL2_EC_WFX:usize =		crate::INPLACE!(ESR_EL2_EC_SHIFT, 1);
pub const ESR_EL2_EC_SVC:usize =		crate::INPLACE!(ESR_EL2_EC_SHIFT, 21);
pub const ESR_EL2_EC_HVC:usize =		crate::INPLACE!(ESR_EL2_EC_SHIFT, 22);
pub const ESR_EL2_EC_SMC:usize =		crate::INPLACE!(ESR_EL2_EC_SHIFT, 23);
pub const ESR_EL2_EC_SYSREG:usize =	crate::INPLACE!(ESR_EL2_EC_SHIFT, 24);
pub const ESR_EL2_EC_INST_ABORT:usize =	crate::INPLACE!(ESR_EL2_EC_SHIFT, 32);
pub const ESR_EL2_EC_DATA_ABORT:usize =	crate::INPLACE!(ESR_EL2_EC_SHIFT, 36);
pub const ESR_EL2_EC_FP:usize =	crate::INPLACE!(ESR_EL2_EC_SHIFT, 7);

/* Data/Instruction Abort ESR fields */
pub const ESR_EL2_ABORT_ISV_BIT:usize =		1 << 24;
pub const NOT_ESR_EL2_ABORT_ISV_BIT:usize =	!ESR_EL2_ABORT_ISV_BIT;

pub const ESR_EL2_ABORT_SAS_SHIFT:usize =		22;
pub const ESR_EL2_ABORT_SAS_WIDTH:usize =		2;
pub const ESR_EL2_ABORT_SAS_MASK:usize =		crate::MASK!(ESR_EL2_ABORT_SAS_WIDTH, ESR_EL2_ABORT_SAS_SHIFT);

pub const ESR_EL2_ABORT_SAS_BYTE_VAL:usize =	0;
pub const ESR_EL2_ABORT_SAS_HWORD_VAL:usize =	1;
pub const ESR_EL2_ABORT_SAS_WORD_VAL:usize =	2;
pub const ESR_EL2_ABORT_SAS_DWORD_VAL:usize =	3;

pub const ESR_EL2_ABORT_SSE_BIT:usize =		1 << 21;

pub const ESR_EL2_ABORT_SRT_SHIFT:usize =		16;
pub const ESR_EL2_ABORT_SRT_WIDTH:usize =		5;
pub const ESR_EL2_ABORT_SRT_MASK:usize =		crate::MASK!(ESR_EL2_ABORT_SRT_WIDTH, ESR_EL2_ABORT_SRT_SHIFT);

pub const ESR_EL2_ABORT_SF_BIT:usize =		1 << 15;
pub const ESR_EL2_ABORT_FNV_BIT:usize =		1 << 10;
pub const ESR_EL2_ABORT_WNR_BIT:usize =		1 << 6;
pub const ESR_EL2_ABORT_FSC_SHIFT:usize =		0;
pub const ESR_EL2_ABORT_FSC_WIDTH:usize =		6;
pub const ESR_EL2_ABORT_FSC_MASK:usize =		crate::MASK!(ESR_EL2_ABORT_FSC_WIDTH, ESR_EL2_ABORT_FSC_SHIFT);

pub const ESR_EL2_ABORT_FSC_TRANSLATION_FAULT:usize =	0x04;
pub const ESR_EL2_ABORT_FSC_PERMISSION_FAULT:usize =	0x0c;
pub const ESR_EL2_ABORT_FSC_LEVEL_SHIFT:usize =		0;
pub const ESR_EL2_ABORT_FSC_LEVEL_WIDTH:usize =		2;
pub const ESR_EL2_ABORT_FSC_LEVEL_MASK:usize =		crate::MASK!(ESR_EL2_ABORT_FSC_LEVEL_WIDTH, ESR_EL2_ABORT_FSC_LEVEL_SHIFT);

/* WFx ESR fields */
pub const ESR_EL2_WFx_TI_BIT:usize =		1 << 0;

/* xVC ESR fields */
pub const ESR_EL2_xVC_IMM_SHIFT:usize =		0;
pub const ESR_EL2_xVC_IMM_WIDTH:usize =		16;
pub const ESR_EL2_xVC_IMM_MASK:usize =		crate::MASK!(ESR_EL2_xVC_IMM_WIDTH, ESR_EL2_xVC_IMM_SHIFT);

pub const ICC_SRE_EL2_ENABLE:usize =		1 << 3;
pub const NOT_ICC_SRE_EL2_ENABLE:usize =		!ICC_SRE_EL2_ENABLE;

pub const ICC_HPPIR1_EL1_INTID_SHIFT:usize =	0;
pub const ICC_HPPIR1_EL1_INTID_WIDTH:usize =	24;
pub const ICC_HPPIR1_EL1_INTID:usize =		crate::MASK!(ICC_HPPIR1_EL1_INTID_WIDTH, ICC_HPPIR1_EL1_INTID_SHIFT);

pub const RVIC_INFO_KEY_NR_TRUSTED_INTERRUPTS: usize =	0;
pub const RVIC_INFO_KEY_NR_UNTRUSTED_INTERRUPTS: usize =	1;

pub const RVIC_STATUS_SUCCESS: usize =		0;
pub const RVIC_STATUS_ERROR_PARAMETER: usize =		1;
pub const RVIC_STATUS_INVALID_CPU: usize =			2;
pub const RVIC_STATUS_DISABLED: usize =			3;
pub const RVIC_STATUS_NO_INTERRUPT: usize =		4;

pub const NR_TRUSTED_INTERRUPTS: usize =			32;
pub const NR_UNTRUSTED_INTERRUPTS: usize =			480;
pub const NR_INTERRUPTS: usize =				(NR_TRUSTED_INTERRUPTS + NR_UNTRUSTED_INTERRUPTS);

pub const INTID_TRUSTED_BASE: usize =			0;
pub const INTID_TRUSTED_MAX: usize =			(NR_TRUSTED_INTERRUPTS - 1);

pub const INTID_UNTRUSTED_BASE: usize =			NR_TRUSTED_INTERRUPTS;
pub const INTID_UNTRUSTED_MAX: usize =			(NR_UNTRUSTED_INTERRUPTS + NR_TRUSTED_INTERRUPTS - 1);

pub const INTID_SPURIOUS: usize =				usize::MAX;

pub const INTID_VTIMER_EL1: usize =			27;
pub const INTID_PTIMER_EL1: usize =			30;

pub const RVIC_BITMAP_BYTES: usize =	(NR_INTERRUPTS / 8);
pub const RVIC_BITMAP_ULS: usize =		(RVIC_BITMAP_BYTES / 8);
pub const BITS_PER_UL: usize =		(8 * 8);

// SYSCALLS
#[repr(i32)]   
pub enum SyscallNumber {
    __NR_invalid = 1000,
    __NR_ioctl = 29,
    __NR_openat = 56,
    __NR_close = 57,
    __NR_lseek = 62,
    __NR_readv = 65,
    __NR_writev = 66,
    __NR_brk = 214,
    __NR_mmap = 222,
    // Add  other syscall number if needed
    // ...
}

// Transform i32 to SyscallNumber type
impl From<i32> for SyscallNumber {
    fn from(num: i32) -> Self {
        match num {
            29 => SyscallNumber::__NR_ioctl,
            56 => SyscallNumber::__NR_openat,
            57 => SyscallNumber::__NR_close,
            62 => SyscallNumber::__NR_lseek,
            65 => SyscallNumber::__NR_readv,
            66 => SyscallNumber::__NR_writev,
            214 => SyscallNumber::__NR_brk,
            222 => SyscallNumber::__NR_mmap,
            _ => SyscallNumber::__NR_invalid,
        }
    }
}



#[macro_export]
macro_rules! read_sysreg {
    ($r: ident) => {
        {
            let mut val: usize;
            unsafe{core::arch::asm!(concat!("mrs {0}, ", stringify!($r)), out(reg) val);}
            val
        }
        
    };
}

#[macro_export]
macro_rules! read_sysreg_sp {
    () => {
        {
            let mut val: usize = 0;
            unsafe{core::arch::asm!("mrs {0}, sp_el0", out(reg) val);}
            val
        }
        
    };
}

#[macro_export]
macro_rules! write_sysreg {
    ($r: ident, $id: expr) => {
        {
            unsafe{core::arch::asm!(concat!("msr ", stringify!($r), ", {0}"), in(reg) $id);}
        }
        
    };
}

#[macro_export]
macro_rules! write_sysreg_vttbr {
    ($id: expr) => {
        {
            unsafe{core::arch::asm!("msr vttbr_el2, {0}", in(reg) $id);}
        }
        
    };
}

#[macro_export]
macro_rules! INPLACE {
    ($shift: expr, $mask: expr) => {
        $mask << $shift
    };
}

#[macro_export]
macro_rules! MASK {
    ($width: expr, $shift: expr) => {
        ((!(0 as usize)) >> (64 - $width)) << $shift
    };
}

#[macro_export]
macro_rules! cpuid {
    () => {
        {
            use crate::io::Write;

            let mut mpidr: usize;
            let val: usize;
            // val = crate::read_sysreg!(MPIDR_EL1);
            unsafe{core::arch::asm!("mrs {0}, MPIDR_EL1", out(reg) mpidr);}
            val = ((mpidr>>16) & 0xff) *4 + ((mpidr>>8) & 0xff);
            // $crate::println!("Debug: Rust CPUID {:x}", val);
            val
        }
    } 
}

pub const CNTx_CTL_ENABLE: usize = 	1 << 0;
pub const CNTx_CTL_IMASK: usize = 	1 << 1;
pub const CNTx_CTL_ISTATUS: usize = 1 << 2;