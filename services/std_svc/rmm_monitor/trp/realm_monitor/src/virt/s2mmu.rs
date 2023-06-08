use core::arch::asm;
/* x most significant bits should be all 0 or 1. */
/* aka translation input size */
macro_rules! TCR_T0SZ {
    ($x: ident) => {
        ((64 - ($x)) << 0)
    };
}
macro_rules! TCR_T1SZ {
    ($x: ident) => {
        ((64 - ($x)) << 16)
    };
}
macro_rules! TCR_TxSZ {
    ($x: ident) => {
        ((TCR_T0SZ!($x)) | (TCR_T1SZ!($x)))
    };
}

macro_rules! ROUND_DOWN {
    ($x: ident, $n: ident) => {
        ($x) & !(($n) - 1)
    };
}

/* 00=4KB, 01=16KB, 11=64KB */
/* Translation granularity, or the page size */
/**
 *  * FIXME: WARNING: DOC-CONFLICT:
 *   * In doc `DDI0487E_a_armv8_arm.pdf`, the values for TG0 are:
 *	* 10=16KB, 00=4KB, 01=64KB
 *	 * The following values are from doc `DEN0024A_v8_architecture_PG.pdf`.
 *	  */
const TCR_TG0_4K: usize = 0x0 << 14;
const TCR_TG0_16K: usize = 0x1 << 14;
const TCR_TG0_64K: usize = 0x3 << 14;
/**
 *  * FIXME: WARNING: DOC-CONFLICT:
 *   * In doc `DDI0487E_a_armv8_arm.pdf`, the values for TG1 are:
 *	* 01=16KB, 10=4KB, 11=64KB
 *	 * The following values are from doc `DEN0024A_v8_architecture_PG.pdf`.
 *	  */
const TCR_TG1_4K: usize = 0x0 << 30;
const TCR_TG1_16K: usize = 0x1 << 30;
const TCR_TG1_64K: usize = 0x3 << 30;
/* TCR.{
 * I}PS PA SIZE, aka translation output size */
/* 000=32bits 4GB */
/* 001=36bits 64GB */
/* 010=40bits 1TB */
/* 011=42bits 4TB */
/* 100=44bits 16TB */
/* 101=48bits 256TB */
/* 110=52bits 4PB */
const TCR_PS_4G: usize = (0x0usize) << 32;
const TCR_PS_64G: usize = (0x1usize) << 32;
const TCR_PS_1T: usize = (0x2usize) << 32;
const TCR_PS_4T: usize = (0x3usize) << 32;
const TCR_PS_16T: usize = (0x4usize) << 32;
const TCR_PS_256T: usize = (0x5usize) << 32;
const TCR_PS_4P: usize = (0x6usize) << 32;

/**
 * TTBR1's shareability:
 * NS: non-shareable
 * OS: outer sharable
 * IS: inner sharable
 **/
const TCR_SH1_NS: usize = (0x0 << 28);
const TCR_SH1_OS: usize = (0x2 << 28);
const TCR_SH1_IS: usize = (0x3 << 28);
/* TTBR0's shareability */
const TCR_SH0_NS: usize = (0x0 << 12);
const TCR_SH0_OS: usize = (0x2 << 12);
const TCR_SH0_IS: usize = (0x3 << 12);

/**
* Outer cacheability for TBBR1:
* NC: non-cacheable
* WBWA: write-back read-allocate write-allocate cacheable
* WTnWA: write-back read-allocate no write-allocate cacheable
* WBnWA: write-back read-allocate no write-allocate cacheable
**/
const TCR_ORGN1_NC: usize = (0x0 << 26);
const TCR_ORGN1_WBWA: usize = (0x1 << 26);
const TCR_ORGN1_WTnWA: usize = (0x2 << 26);
const TCR_ORGN1_WBnWA: usize = (0x3 << 26);
/* Inner shareability for TBBR1 */
const TCR_IRGN1_NC: usize = (0x0 << 24);
const TCR_IRGN1_WBWA: usize = (0x1 << 24);
const TCR_IRGN1_WTnWA: usize = (0x2 << 24);
const TCR_IRGN1_WBnWA: usize = (0x3 << 24);
/* Outer shareability for TBBR0 */
const TCR_ORGN0_NC: usize = (0x0 << 10);
const TCR_ORGN0_WBWA: usize = (0x1 << 10);
const TCR_ORGN0_WTnWA: usize = (0x2 << 10);
const TCR_ORGN0_WBnWA: usize = (0x3 << 10);
/* Inner shareability for TBBR0 */
const TCR_IRGN0_NC: usize = (0x0 << 8);
const TCR_IRGN0_WBWA: usize = (0x1 << 8);
const TCR_IRGN0_WTnWA: usize = (0x2 << 8);
const TCR_IRGN0_WBnWA: usize = (0x3 << 8);

const INNER_SHAREABLE: u64 = 0x3;
const NORMAL_MEMORY: u64 = 0xF;

/**
 * Whether a translation table walk is performed on a TLB miss, for an
 * address that is translated using TTBR1_EL1/TTBR0_EL1.
 **/
const TCR_EPD1_WALK: usize = (0x0 << 23);
const TCR_EPD1_FAULT: usize = (0x1 << 23);
const TCR_EPD0_WALK: usize = (0x0 << 7);
const TCR_EPD0_FAULT: usize = (0x1 << 7);

/* Who defines the ASID */
const TCR_A1_TTBR0: usize = (0x0 << 22);
const TCR_A1_TTBR1: usize = (0x1 << 22);

/* TCR_EL1 */

/**
 * Four-level page table for 4KB pages
 *  - p0d_t is the address of the 4K page
 *  - each p1d_t contains 512 p1e_t that points to one p0d_t
 *  - each p2d_t contains 512 p2e_t that points to one p1d_t
 *  - each p3d_t contains 512 p3e_t that points to one p2d_t
 *  - each p4d_t contains 512 p4e_t that points to one p3d_t
 *
 * Relations to the ARM document terminalogies:
 * p1d_t: level 3 table
 * P2d_t: level 2 table
 * p3d_t: level 1 table
 * p4d_t: level 0 table
 **/

/* Table attributes */
const ARM64_MMU_ATTR_TBL_AP_TABLE_NOEFFECT: usize = (0);
const ARM64_MMU_ATTR_TBL_AP_TABLE_NOEL0: usize = (1);
const ARM64_MMU_ATTR_TBL_AP_TABLE_NOWRITE: usize = (2);
const ARM64_MMU_ATTR_TBL_AP_TABLE_NOACCESS: usize = (3);

/* Block/Page access permission */
const ARM64_MMU_ATTR_STAGE2_PAGE_AP_NONE: u64 = (0);
const ARM64_MMU_ATTR_STAGE2_PAGE_AP_RO: u64 = (1);
const ARM64_MMU_ATTR_STAGE2_PAGE_AP_WO: u64 = (2);
const ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW: u64 = (3);

/* Block/Page execution permission */
const ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL: u64 = 0;
const ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_EL0: u64 = 1;
const ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE: u64 = 2;
const ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_EL1: u64 = 3;

const ARM64_MMU_ATTR_PAGE_AF_NONE: usize = 0;
const ARM64_MMU_ATTR_PAGE_AF_ACCESSED: usize = 1;

const ARM64_MMU_PTE_INVALID_MASK: u64 = 1 << 0;
const ARM64_MMU_PTE_TABLE_MASK: u64 = 1 << 1;

//  const IS_PTE_INVALID(pte) (!((pte) & ARM64_MMU_PTE_INVALID_MASK))
macro_rules! IS_PTE_INVALID {
    ($pte: ident) => {
        (!((pte) & ARM64_MMU_PTE_INVALID_MASK))
    };
}
//  const IS_PTE_TABLE(pte) (!!((pte) & ARM64_MMU_PTE_TABLE_MASK))
macro_rules! IS_PTE_TABLE {
    ($pte: ident) => {
        (!!((pte) & ARM64_MMU_PTE_TABLE_MASK))
    };
}

// const MMU_ATTR_PAGE_RO: usize = 1;
// const MMU_ATTR_PAGE_WO: usize = 2;
// const MMU_ATTR_PAGE_RW: usize = 3;
// const MMU_ATTR_PAGE_EO: usize = 4;
// const MMU_ATTR_PAGE_RWE: usize = 5;
// const GET_L0_INDEX(addr) ((addr >> L0_INDEX_SHIFT) & PTP_INDEX_MASK)
macro_rules! GET_L0_INDEX {
    ($addr: ident) => {
        ((addr >> L0_INDEX_SHIFT) & PTP_INDEX_MASK)
    };
}
// const GET_L1_INDEX(addr) ((addr >> L1_INDEX_SHIFT) & PTP_INDEX_MASK)
macro_rules! GET_L1_INDEX {
    ($addr: ident) => {
        ((addr >> L1_INDEX_SHIFT) & PTP_INDEX_MASK)
    };
}
// const GET_L2_INDEX(addr) ((addr >> L2_INDEX_SHIFT) & PTP_INDEX_MASK)
macro_rules! GET_L2_INDEX {
    ($addr: ident) => {
        ((addr >> L2_INDEX_SHIFT) & PTP_INDEX_MASK)
    };
}
// const GET_L3_INDEX(addr) ((addr >> L3_INDEX_SHIFT) & PTP_INDEX_MASK)
macro_rules! GET_L3_INDEX {
    ($addr: ident) => {
        ((addr >> L3_INDEX_SHIFT) & PTP_INDEX_MASK)
    };
}

const PAGE_SHIFT: usize = (12);
const PAGE_SIZE: usize = (1 << (PAGE_SHIFT));
pub const PAGE_MASK: usize = (PAGE_SIZE - 1);
const PAGE_ORDER: usize = (9);

const PTP_ENTRIES: usize = (1 << PAGE_ORDER);
const L3: usize = (3);
const L2: usize = (2);
const L1: usize = (1);
const L0: usize = (0);

const PTP_INDEX_MASK: usize = ((1 << (PAGE_ORDER)) - 1);
const L0_INDEX_SHIFT: usize = ((3 * PAGE_ORDER) + PAGE_SHIFT);
const L1_INDEX_SHIFT: usize = ((2 * PAGE_ORDER) + PAGE_SHIFT);
const L2_INDEX_SHIFT: usize = ((1 * PAGE_ORDER) + PAGE_SHIFT);
const L3_INDEX_SHIFT: usize = ((0 * PAGE_ORDER) + PAGE_SHIFT);

/* PAGE TABLE PAGE TYPE */
// const TABLE_TYPE: usize = 1;
// const BLOCK_TYPE: usize = 2;

const PGTBL_4K_BITS: usize = (9);
const PGTBL_4K_ENTRIES: usize = (1 << (PGTBL_4K_BITS));
const PGTBL_4K_MAX_INDEX: usize = ((PGTBL_4K_ENTRIES) - 1);

const ARM64_MMU_L1_BLOCK_ORDER: usize = (18);
const ARM64_MMU_L2_BLOCK_ORDER: usize = (9);
const ARM64_MMU_L3_PAGE_ORDER: usize = (0);

const ARM64_MMU_L0_BLOCK_PAGES: usize = (PTP_ENTRIES * ARM64_MMU_L1_BLOCK_PAGES);
const ARM64_MMU_L1_BLOCK_PAGES: usize = (1usize << ARM64_MMU_L1_BLOCK_ORDER);
const ARM64_MMU_L2_BLOCK_PAGES: usize = (1usize << ARM64_MMU_L2_BLOCK_ORDER);
const ARM64_MMU_L3_PAGE_PAGES: usize = (1usize << ARM64_MMU_L3_PAGE_ORDER);

const L0_PER_ENTRY_PAGES: usize = (ARM64_MMU_L0_BLOCK_PAGES);
const L1_PER_ENTRY_PAGES: usize = (ARM64_MMU_L1_BLOCK_PAGES);
const L2_PER_ENTRY_PAGES: usize = (ARM64_MMU_L2_BLOCK_PAGES);
const L3_PER_ENTRY_PAGES: usize = (ARM64_MMU_L3_PAGE_PAGES);

const ARM64_MMU_L1_BLOCK_SIZE: usize = (ARM64_MMU_L1_BLOCK_PAGES << PAGE_SHIFT);
const ARM64_MMU_L2_BLOCK_SIZE: usize = (ARM64_MMU_L2_BLOCK_PAGES << PAGE_SHIFT);
const ARM64_MMU_L3_PAGE_SIZE: usize = (ARM64_MMU_L3_PAGE_PAGES << PAGE_SHIFT);

const ARM64_MMU_L1_BLOCK_MASK: usize = (ARM64_MMU_L1_BLOCK_SIZE - 1);
const ARM64_MMU_L2_BLOCK_MASK: usize = (ARM64_MMU_L2_BLOCK_SIZE - 1);
const ARM64_MMU_L3_PAGE_MASK: usize = (ARM64_MMU_L3_PAGE_SIZE - 1);

// const GET_VA_OFFSET_L1(va)	  (va & ARM64_MMU_L1_BLOCK_MASK)
macro_rules! GET_VA_OFFSET_L1 {
    ($va: ident) => {
        (va & ARM64_MMU_L1_BLOCK_MASK)
    };
}
// const GET_VA_OFFSET_L2(va)	  (va & ARM64_MMU_L2_BLOCK_MASK)
macro_rules! GET_VA_OFFSET_L2 {
    ($va: ident) => {
        (va & ARM64_MMU_L2_BLOCK_MASK)
    };
}
// const GET_VA_OFFSET_L3(va)	  (va & ARM64_MMU_L3_PAGE_MASK)
macro_rules! GET_VA_OFFSET_L3 {
    ($va: ident) => {
        (va & ARM64_MMU_L3_PAGE_MASK)
    };
}

const PTE_DESCRIPTOR_INVALID: u64 = (0);
const PTE_DESCRIPTOR_BLOCK: u64 = (1);
const PTE_DESCRIPTOR_TABLE: u64 = (3);
const PTE_DESCRIPTOR_MASK: u64 = (3);

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct Table {
    data: u64,
}

use bit_field::BitField;
use core::ops::RangeInclusive;
const TABLE_IS_VALID_RANGE: RangeInclusive<usize> = 0..=0;
const TABLE_IS_TABLE_RANGE: RangeInclusive<usize> = 1..=1;
const TABLE_IGNORED1_RANGE: RangeInclusive<usize> = 2..=11;
const TABLE_NEXT_TABLE_ADDR_RANGE: RangeInclusive<usize> = 12..=47;
const TABLE_RESERVED1_RANGE: RangeInclusive<usize> = 48..=51;
const TABLE_IGNORED2_RANGE: RangeInclusive<usize> = 52..=58;
const TABLE_RESERVED2_RANGE: RangeInclusive<usize> = 59..=63;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct L1Block {
    data: u64,
}

const L1BLOCK_IS_VALID_RANGE: RangeInclusive<usize> = 0..=0;
const L1BLOCK_IS_TABLE_RANGE: RangeInclusive<usize> = 1..=1;
const L1BLOCK_MEM_ATTR_RANGE: RangeInclusive<usize> = 2..=5;
const L1BLOCK_S2AP_RANGE: RangeInclusive<usize> = 6..=7;
const L1BLOCK_SH_RANGE: RangeInclusive<usize> = 8..=9;
const L1BLOCK_AF_RANGE: RangeInclusive<usize> = 10..=10;
const L1BLOCK_ZERO_RANGE: RangeInclusive<usize> = 11..=11;
const L1BLOCK_RESERVED1_RANGE: RangeInclusive<usize> = 12..=15;
const L1BLOCK_NT_RANGE: RangeInclusive<usize> = 16..=16;
const L1BLOCK_RESERVED2_RANGE: RangeInclusive<usize> = 17..=29;
const L1BLOCK_PFN_RANGE: RangeInclusive<usize> = 30..=47;
const L1BLOCK_RESERVED3_RANGE: RangeInclusive<usize> = 48..=50;
const L1BLOCK_DBM_RANGE: RangeInclusive<usize> = 51..=51;
const L1BLOCK_CONTINUOUS_RANGE: RangeInclusive<usize> = 52..=52;
const L1BLOCK_XN_RANGE: RangeInclusive<usize> = 53..=54;
const L1BLOCK_SOFT_RESERVED_RANGE: RangeInclusive<usize> = 55..=58;
const L1BLOCK_PBHA_RANGE: RangeInclusive<usize> = 59..=62;
const L1BLOCK_RESERVED4_RANGE: RangeInclusive<usize> = 63..=63;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct L2Block {
    data: u64,
}

const L2BLOCK_IS_VALID_RANGE: RangeInclusive<usize> = 0..=0;
const L2BLOCK_IS_TABLE_RANGE: RangeInclusive<usize> = 1..=1;
const L2BLOCK_MEM_ATTR_RANGE: RangeInclusive<usize> = 2..=5;
const L2BLOCK_S2AP_RANGE: RangeInclusive<usize> = 6..=7;
const L2BLOCK_SH_RANGE: RangeInclusive<usize> = 8..=9;
const L2BLOCK_AF_RANGE: RangeInclusive<usize> = 10..=10;
const L2BLOCK_ZERO_RANGE: RangeInclusive<usize> = 11..=11;
const L2BLOCK_RESERVED1_RANGE: RangeInclusive<usize> = 12..=15;
const L2BLOCK_NT_RANGE: RangeInclusive<usize> = 16..=16;
const L2BLOCK_RESERVED2_RANGE: RangeInclusive<usize> = 17..=20;
const L2BLOCK_PFN_RANGE: RangeInclusive<usize> = 21..=47;
const L2BLOCK_RESERVED3_RANGE: RangeInclusive<usize> = 48..=50;
const L2BLOCK_DBM_RANGE: RangeInclusive<usize> = 51..=51;
const L2BLOCK_CONTINUOUS_RANGE: RangeInclusive<usize> = 52..=52;
const L2BLOCK_XN_RANGE: RangeInclusive<usize> = 53..=54;
const L2BLOCK_SOFT_RESERVED_RANGE: RangeInclusive<usize> = 55..=58;
const L2BLOCK_PBHA_RANGE: RangeInclusive<usize> = 59..=62;
const L2BLOCK_RESERVED4_RANGE: RangeInclusive<usize> = 63..=63;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct L3Page {
    data: u64,
}

const L3PAGE_IS_VALID_RANGE: RangeInclusive<usize> = 0..=0;
const L3PAGE_IS_PAGE_RANGE: RangeInclusive<usize> = 1..=1;
const L3PAGE_MEM_ATTR_RANGE: RangeInclusive<usize> = 2..=5;
const L3PAGE_S2AP_RANGE: RangeInclusive<usize> = 6..=7;
const L3PAGE_SH_RANGE: RangeInclusive<usize> = 8..=9;
const L3PAGE_AF_RANGE: RangeInclusive<usize> = 10..=10;
const L3PAGE_ZERO_RANGE: RangeInclusive<usize> = 11..=11;
const L3PAGE_PFN_RANGE: RangeInclusive<usize> = 12..=47;
const L3PAGE_RESERVED1_RANGE: RangeInclusive<usize> = 48..=50;
const L3PAGE_DBM_RANGE: RangeInclusive<usize> = 51..=51;
const L3PAGE_CONTINUOUS_RANGE: RangeInclusive<usize> = 52..=52;
const L3PAGE_XN_RANGE: RangeInclusive<usize> = 53..=54;
const L3PAGE_SOFT_RESERVED_RANGE: RangeInclusive<usize> = 55..=58;
const L3PAGE_PBHA_RANGE: RangeInclusive<usize> = 59..=62;
const L3PAGE_RESERVED2_RANGE: RangeInclusive<usize> = 63..=63;

#[repr(C)]
#[derive(Copy, Clone)]
pub union Pte {
    table: Table,
    l1_block: L1Block,
    l2_block: L2Block,
    l3_page: L3Page,
    pte: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum VmFlags {
    MMU_ATTR_PAGE_RO = 1,
    MMU_ATTR_PAGE_WO = 2,
    MMU_ATTR_PAGE_RW = 3,
    MMU_ATTR_PAGE_EO = 4,
    MMU_ATTR_PAGE_RWE = 5,
    Unknown = 6,
}

impl From<usize> for VmFlags {
    fn from(origin: usize) -> Self {
        match origin {
            1 => VmFlags::MMU_ATTR_PAGE_RO,
            2 => VmFlags::MMU_ATTR_PAGE_WO,
            3 => VmFlags::MMU_ATTR_PAGE_RW,
            4 => VmFlags::MMU_ATTR_PAGE_EO,
            5 => VmFlags::MMU_ATTR_PAGE_RWE,
            _ => VmFlags::Unknown,
        }
    }
}

#[repr(C)]
pub enum PteLevel {
    L1 = 1,
    L2 = 2,
    L3 = 3,
}

impl From<usize> for PteLevel {
    fn from(origin: usize) -> Self {
        match origin {
            1 => PteLevel::L1,
            2 => PteLevel::L2,
            3 => PteLevel::L3,
            /* Dangerous: change it*/
            _ => PteLevel::L1,
        }
    }
}

impl Pte {
    /// Thinking about if there is better way to express it without so much match/if/switch
    pub fn set_pte_flags(&mut self, level: PteLevel, flags: VmFlags) -> isize {
        unsafe {
            match flags {
                VmFlags::MMU_ATTR_PAGE_RO => match level {
                    PteLevel::L1 => {
                        self.l1_block
                            .data
                            .set_bits(L1BLOCK_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_RO);
                        self.l1_block
                            .data
                            .set_bits(L1BLOCK_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE);
                    }
                    PteLevel::L2 => {
                        self.l2_block
                            .data
                            .set_bits(L2BLOCK_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_RO);
                        self.l2_block
                            .data
                            .set_bits(L2BLOCK_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE);
                    }
                    PteLevel::L3 => {
                        self.l3_page
                            .data
                            .set_bits(L3PAGE_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_RO);
                        self.l3_page
                            .data
                            .set_bits(L3PAGE_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE);
                    }
                },
                VmFlags::MMU_ATTR_PAGE_WO => match level {
                    PteLevel::L1 => {
                        self.l1_block
                            .data
                            .set_bits(L1BLOCK_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_WO);
                        self.l1_block
                            .data
                            .set_bits(L1BLOCK_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE);
                    }
                    PteLevel::L2 => {
                        self.l2_block
                            .data
                            .set_bits(L2BLOCK_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_WO);
                        self.l2_block
                            .data
                            .set_bits(L2BLOCK_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE);
                    }
                    PteLevel::L3 => {
                        self.l3_page
                            .data
                            .set_bits(L3PAGE_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_WO);
                        self.l3_page
                            .data
                            .set_bits(L3PAGE_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE);
                    }
                },
                VmFlags::MMU_ATTR_PAGE_RW => match level {
                    PteLevel::L1 => {
                        self.l1_block
                            .data
                            .set_bits(L1BLOCK_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW);
                        self.l1_block
                            .data
                            .set_bits(L1BLOCK_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE);
                    }
                    PteLevel::L2 => {
                        self.l2_block
                            .data
                            .set_bits(L2BLOCK_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW);
                        self.l2_block
                            .data
                            .set_bits(L2BLOCK_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE);
                    }
                    PteLevel::L3 => {
                        self.l3_page
                            .data
                            .set_bits(L3PAGE_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW);
                        self.l3_page
                            .data
                            .set_bits(L3PAGE_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE);
                    }
                },
                VmFlags::MMU_ATTR_PAGE_EO => match level {
                    PteLevel::L1 => {
                        self.l1_block
                            .data
                            .set_bits(L1BLOCK_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_NONE);
                        self.l1_block
                            .data
                            .set_bits(L1BLOCK_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL);
                    }
                    PteLevel::L2 => {
                        self.l2_block
                            .data
                            .set_bits(L2BLOCK_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_NONE);
                        self.l2_block
                            .data
                            .set_bits(L2BLOCK_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL);
                    }
                    PteLevel::L3 => {
                        self.l3_page
                            .data
                            .set_bits(L3PAGE_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_NONE);
                        self.l3_page
                            .data
                            .set_bits(L3PAGE_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL);
                    }
                },
                VmFlags::MMU_ATTR_PAGE_RWE => match level {
                    PteLevel::L1 => {
                        self.l1_block
                            .data
                            .set_bits(L1BLOCK_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW);
                        self.l1_block
                            .data
                            .set_bits(L1BLOCK_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL);
                    }
                    PteLevel::L2 => {
                        self.l2_block
                            .data
                            .set_bits(L2BLOCK_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW);
                        self.l2_block
                            .data
                            .set_bits(L2BLOCK_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL);
                    }
                    PteLevel::L3 => {
                        self.l3_page
                            .data
                            .set_bits(L3PAGE_S2AP_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW);
                        self.l3_page
                            .data
                            .set_bits(L3PAGE_XN_RANGE, ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL);
                    }
                },
                VmFlags::Unknown => {
                    // printf(to_c_str("level Unknown\n"));
                    // self.l3_page.set_SAP(1);
                }
            }

            /*Set other attributes*/
            match level {
                PteLevel::L1 => {
                    self.l1_block
                        .data
                        .set_bits(L1BLOCK_SH_RANGE, INNER_SHAREABLE);
                    self.l1_block.data.set_bits(L1BLOCK_AF_RANGE, 1);
                    self.l1_block
                        .data
                        .set_bits(L1BLOCK_MEM_ATTR_RANGE, NORMAL_MEMORY);
                }
                PteLevel::L2 => {
                    self.l2_block
                        .data
                        .set_bits(L2BLOCK_SH_RANGE, INNER_SHAREABLE);
                    self.l2_block.data.set_bits(L2BLOCK_AF_RANGE, 1);
                    self.l2_block
                        .data
                        .set_bits(L2BLOCK_MEM_ATTR_RANGE, NORMAL_MEMORY);
                }
                PteLevel::L3 => {
                    self.l3_page.data.set_bits(L3PAGE_SH_RANGE, INNER_SHAREABLE);
                    self.l3_page.data.set_bits(L3PAGE_AF_RANGE, 1);
                    self.l3_page
                        .data
                        .set_bits(L3PAGE_MEM_ATTR_RANGE, NORMAL_MEMORY);
                }
            }
            /* end of unsafe */
        }
        /* return 0 */
        0
    }

    pub fn new() -> Self {
        Pte { pte: 0 }
    }
}

#[repr(C)]
#[derive(PartialEq)]
pub enum PtpType {
    TableType = 1,
    BlockType = 2,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ptp {
    ent: [Pte; 1 << PGTBL_4K_BITS],
}

pub fn phys_to_virt(phys: u64) -> u64 {
    phys
}

pub fn virt_to_phys(virt: u64) -> u64 {
    virt
}

const EINVAL: isize = 5;
const ENOMEM: isize = 12;
const ENOMMAPING: isize = 14;
use alloc::boxed::Box;
impl Ptp {
    fn is_clear(&self) -> bool {
        for i in 0..PTP_ENTRIES {
            let entry = &self.ent[i];

            if unsafe { entry.pte } & ARM64_MMU_PTE_INVALID_MASK != 0 {
                return false;
            }
        }
        true
    }

    /// Get next level ptp
    pub fn get_next_level_ptp(
        &mut self,
        current_level: usize,
        current_index: usize,
    ) -> Result<(Box<Ptp>, PtpType), isize> {
        /* Sanity Check */
        if current_level < 0 || 2 < current_level {
            return Err(-EINVAL);
        }
        if current_index < 0 || PTP_ENTRIES <= current_index {
            return Err(-EINVAL);
        }

        let entry: &mut Pte = &mut self.ent[current_index];

        match unsafe { entry.pte } & PTE_DESCRIPTOR_MASK {
            PTE_DESCRIPTOR_INVALID => {
                // TODO: remind the alignment issue
                // TODO: What if out of memory
                let next_ptp_virt;
                match unsafe { super::super::ALLOCATOR.bd_alloc(PAGE_SIZE, 12) } {
                    Some(addr) => next_ptp_virt = addr,
                    None => panic!(),
                }
                unsafe {
                    for i in 0..PAGE_SIZE {
                        let addr = next_ptp_virt + i;
                        let ptr = addr as *mut u8;
                        *ptr = 0;
                    }
                }
                let next_ptp = unsafe { Box::from_raw(next_ptp_virt as *mut Ptp) };
                let next_ptp_phys: u64 = virt_to_phys(next_ptp.as_ref() as *const Ptp as u64);
                unsafe {
                    // access to union data is unsafe
                    entry.table.data.set_bits(TABLE_IS_VALID_RANGE, 1);
                    entry.table.data.set_bits(TABLE_IS_TABLE_RANGE, 1);
                    entry
                        .table
                        .data
                        .set_bits(TABLE_NEXT_TABLE_ADDR_RANGE, next_ptp_phys >> PAGE_SHIFT);
                }
                return Ok((next_ptp, PtpType::TableType));
            }
            PTE_DESCRIPTOR_TABLE => {
                let next_ptp_phys: u64 =
                    unsafe { entry.table.data }.get_bits(TABLE_NEXT_TABLE_ADDR_RANGE) << PAGE_SHIFT;
                let next_ptp_virt: u64 = phys_to_virt(next_ptp_phys);
                let next_ptp = unsafe { Box::from_raw(next_ptp_virt as *mut Ptp) };
                return Ok((next_ptp, PtpType::TableType));
            }
            PTE_DESCRIPTOR_BLOCK => {
                let next_ptp_phys: u64 =
                    unsafe { entry.table.data }.get_bits(TABLE_NEXT_TABLE_ADDR_RANGE) << PAGE_SHIFT;
                let next_ptp_virt: u64 = phys_to_virt(next_ptp_phys);
                let next_ptp = unsafe { Box::from_raw(next_ptp_virt as *mut Ptp) };
                return Ok((next_ptp, PtpType::BlockType));
            }
            _ => {
                panic!("Unsupported Pgtbl Type.");
            }
        }
    }

    /// Map page
    pub fn map_page(
        &mut self,
        order: usize,
        level: usize,
        ipa: usize,
        pa: usize,
        flags: VmFlags,
    ) -> isize {
        let shift = (3 - level) * PAGE_ORDER + PAGE_SHIFT;
        let index = (ipa >> shift) & ((1 << PAGE_ORDER) - 1);
        let entry = &mut self.ent[index as usize];

        if level == 3 {
            unsafe {
                // Access to union is unsafe
                entry.l3_page.data.set_bits(L3PAGE_IS_VALID_RANGE, 1);
                entry.l3_page.data.set_bits(L3PAGE_IS_PAGE_RANGE, 1);
                entry
                    .l3_page
                    .data
                    .set_bits(L3PAGE_PFN_RANGE, (pa >> PAGE_SHIFT) as u64);
            }
            entry.set_pte_flags(PteLevel::L3, flags);
            //TODO: How to add a BUG_ON
            return 0;
        } else {
            if (level != 0) && ((3 - level) * 9 == order) {
                if level == 1 {
                    unsafe {
                        // Access to uion is unsafe
                        entry.l1_block.data.set_bits(L1BLOCK_IS_VALID_RANGE, 1);
                        entry.l1_block.data.set_bits(L1BLOCK_IS_TABLE_RANGE, 0);
                        entry
                            .l1_block
                            .data
                            .set_bits(L1BLOCK_PFN_RANGE, (pa >> (order + PAGE_SHIFT)) as u64);
                    }
                } else {
                    unsafe {
                        entry.l2_block.data.set_bits(L2BLOCK_IS_VALID_RANGE, 1);
                        entry.l2_block.data.set_bits(L2BLOCK_IS_TABLE_RANGE, 0);
                        entry
                            .l2_block
                            .data
                            .set_bits(L2BLOCK_PFN_RANGE, (pa >> (order + PAGE_SHIFT)) as u64);
                    }
                }
                entry.set_pte_flags(PteLevel::from(level as usize), flags);
                return 0;
            } else {
                let res = self.get_next_level_ptp(level as usize, index as usize);
                match res {
                    Ok((mut res_ptp, res_ptp_type)) => {
                        if res_ptp_type == PtpType::TableType {
                            let ret = res_ptp.map_page(order, level + 1, ipa, pa, flags);
                            return ret;
                        } else {
                            return -ENOMEM;
                        }
                    }
                    Err(_err_code) => {
                        return -ENOMEM;
                    }
                }
            }
        }
    }

    /// Unmap range
    pub fn unmap_range(&mut self, level: usize, ipa: usize, size: usize) -> isize {
        let mut block_size = 0;
        let mut total_unmapped_size = size;
        let mut current_unmapped_size = 0;
        let mut unmapped_ipa = ipa;
        let mut next_ptp_phys = 0;
        let mut entry: &mut Pte;

        let shift = (3 - level) * PAGE_ORDER * PAGE_SHIFT;
        let index = (unmapped_ipa >> shift) & ((1 << PAGE_ORDER) - 1);

        while total_unmapped_size > 0 {
            block_size = 1 << shift;
            current_unmapped_size = match block_size < total_unmapped_size {
                true => block_size,
                false => total_unmapped_size,
            };
            entry = &mut self.ent[index];

            if level <= 2 && ((unsafe { entry.pte } & PTE_DESCRIPTOR_MASK) == PTE_DESCRIPTOR_TABLE)
            {
                unsafe {
                    next_ptp_phys =
                        entry.table.data.get_bits(TABLE_NEXT_TABLE_ADDR_RANGE) << PAGE_SHIFT;
                }

                let next_ptp_virt = phys_to_virt(next_ptp_phys);
                let mut next_ptp = unsafe { Box::from_raw(next_ptp_virt as *mut Ptp) };
                next_ptp
                    .as_mut()
                    .unmap_range(level + 1, unmapped_ipa, current_unmapped_size);

                if current_unmapped_size == block_size || next_ptp.as_mut().is_clear() {
                    self.ent[index].pte = PTE_DESCRIPTOR_INVALID;
                    unsafe {
                        super::super::ALLOCATOR.bd_free(Box::into_raw(next_ptp) as usize);
                    }
                }
            } else {
                self.ent[index].pte = PTE_DESCRIPTOR_INVALID;
            }

            total_unmapped_size -= current_unmapped_size;
            unmapped_ipa += current_unmapped_size;
        }
        0
    }
}

/// Stage 2 MMU
use crate::util::list::ListHead;
use crate::virt::ipa_region::IrType;
use crate::IpaRegion;
#[repr(C)]
pub struct S2mmu {
    ipa_region_list: ListHead,
    pgtbl: *mut Ptp, // TODO: Is there better way to express it?
}

extern "C" {
    pub fn flush_tlb();
}

impl S2mmu {
    pub fn create_s2mmu() -> *mut S2mmu {
        let new_s2mmu: *mut S2mmu = unsafe {
            super::super::ALLOCATOR
                .slab_alloc(core::mem::size_of::<S2mmu>(), 0)
                .unwrap() as *mut S2mmu
        };

        /* Do memset */
        let new_s2mmu_virt = new_s2mmu as usize;
        for i in 0..core::mem::size_of::<S2mmu>() {
            unsafe {
                *((new_s2mmu_virt + i) as *mut u8) = 0;
            }
        }

        unsafe {
            (*new_s2mmu).ipa_region_list.init();
            (*new_s2mmu).pgtbl =
                super::super::ALLOCATOR.bd_alloc(PAGE_SIZE, 13).unwrap() as *mut Ptp;
        }

        /* Do memset */
        for i in 0..PAGE_SIZE {
            unsafe {
                let addr = i + (*new_s2mmu).pgtbl as usize;
                let ptr = addr as *mut u8;
                (*ptr) = 0;
            }
        }

        /* Return Value */
        new_s2mmu
    }

    pub fn find_page_or_block_pte(
        &self,
        start_level: usize,
        ipa: usize,
    ) -> Result<(Box<Pte>, usize), isize> {
        if (ipa & PAGE_MASK) != 0 {
            return Err(-EINVAL);
        }
        if self.pgtbl.is_null() {
            return Err(-EINVAL);
        }

        let mut level = start_level;
        let mut shift = (3 - level) * PAGE_ORDER + PAGE_SHIFT;
        let mut index = (ipa >> shift) & ((1 << PAGE_ORDER) - 1);
        let current_ptp_ptr: *mut Ptp = self.pgtbl;
        let mut current_ptp: Box<Ptp> = unsafe { Box::from_raw(current_ptp_ptr) };
        let mut ret_pte: Box<Pte> = unsafe { Box::from_raw(0 as *mut Pte) };
        let mut ret_level: usize = 0;

        while level <= 3 {
            let entry: &mut Pte = &mut current_ptp.as_mut().ent[index];

            if (unsafe { entry.pte } & ARM64_MMU_PTE_INVALID_MASK) == 0 {
                return Err(-ENOMMAPING);
            }
            if level < 3 && (unsafe { entry.pte } & PTE_DESCRIPTOR_MASK) == PTE_DESCRIPTOR_TABLE {
                let res = current_ptp.as_mut().get_next_level_ptp(level, index);
                match res {
                    Ok((res_ptp, res_ptp_type)) => {
                        if res_ptp_type != PtpType::TableType {
                            //TODO: should be BUG_ON!
                            return Err(-ENOMMAPING);
                        }

                        // Avoid box to be freed
                        let _useless = Box::into_raw(current_ptp);
                        current_ptp = res_ptp;
                    }
                    Err(err_code) => return Err(err_code),
                }
            } else {
                let ret_pte_ref: &mut Pte = &mut current_ptp.ent[index];
                let ret_pte_box = unsafe { Box::from_raw(ret_pte_ref) };
                ret_pte = ret_pte_box;
                ret_level = level;
            }
            level += 1;
            shift = (3 * level) * PAGE_ORDER + PAGE_SHIFT;
            index = (ipa >> shift) & ((1 << PAGE_ORDER) - 1);
        }

        // temp remember to change
        Ok((ret_pte, ret_level))
    }

    /// S2mmu map page
    pub fn map_page(&mut self, ipa: usize, pa: usize, start_level: usize, flags: VmFlags) -> isize {
        if ((ipa & PAGE_MASK) != 0) || ((pa & PAGE_MASK) != 0) {
            return -EINVAL;
        }
        if self.pgtbl.is_null() {
            return -EINVAL;
        }

        let mut pgtbl = unsafe { Box::from_raw(self.pgtbl) };
        let res = pgtbl.as_mut().map_page(0, start_level, ipa, pa, flags);
        // let res = unsafe{*self.pgtbl}
        //         .map_page(0, start_level, ipa, pa, flags);
        let _useless = Box::into_raw(pgtbl);
        res
    }

    pub fn map_range(
        &mut self,
        ipa: usize,
        pa: usize,
        start_level: usize,
        len: usize,
        enable_huge_page: isize,
        flags: VmFlags,
    ) -> isize {
        if len <= 0 {
            return -EINVAL;
        }
        if 3 <= start_level {
            return -EINVAL;
        }
        if ((ipa & PAGE_MASK) != 0) || ((pa & PAGE_MASK) != 0) {
            return -EINVAL;
        }
        if self.pgtbl.is_null() {
            return -EINVAL;
        }

        let mut to_mapped_ipa = ipa;
        let mut to_mapped_pa = pa;
        let mut order = 0;
        let mut total_page_count = len / PAGE_SIZE + if len % PAGE_SIZE > 0 { 1 } else { 0 };

        while total_page_count > 0 {
            // TODO: enable_huge_page should be a bool not a int
            if enable_huge_page == 1 {
                if (total_page_count >= (1 << 18))
                    && (to_mapped_ipa & ((1 << 18) - 1) == 0)
                    && (to_mapped_pa & ((1 << 18) - 1) == 0)
                {
                    order = 18;
                } else if (total_page_count >= (1 << 9))
                    && (to_mapped_ipa & ((1 << 9) - 1) == 0)
                    && (to_mapped_pa & ((1 << 9) - 1) == 0)
                {
                    order = 9;
                } else {
                    order = 0;
                }
            } else {
                order = 0; /* Do not use huge page */
            }

            let mut pgtbl = unsafe { Box::from_raw(self.pgtbl) };
            let ret =
                pgtbl
                    .as_mut()
                    .map_page(order, start_level, to_mapped_ipa, to_mapped_pa, flags);
            let _useless = Box::into_raw(pgtbl);
            if ret != 0 {
                return ret;
            }

            total_page_count -= 1 << order;
            to_mapped_ipa += 1 << (order + PAGE_SHIFT);
            to_mapped_pa += 1 << (order + PAGE_SHIFT);
        }
        0
    }

    /// Stage-2 MMU unmap range
    pub fn s2mmu_unmap_range(&mut self, ipa: usize, len: usize) -> isize {
        if ipa & PAGE_MASK != 0 {
            return -EINVAL;
        }
        if self.pgtbl.is_null() {
            return -EINVAL;
        }

        let mut pgtbl = unsafe { Box::from_raw(self.pgtbl) };
        let ret = pgtbl.as_mut().unmap_range(1, ipa, len);
        let _useless = Box::into_raw(pgtbl);

        ret
    }

    pub fn s2mmu_unmap_page(&mut self, ipa: usize) -> isize {
        if ipa & PAGE_MASK != 0 {
            return -EINVAL;
        }
        if self.pgtbl.is_null() {
            return -EINVAL;
        }
        /* The initial lookup level FVP is 1 */
        let mut pgtbl = unsafe { Box::from_raw(self.pgtbl) };
        let ret = pgtbl.as_mut().unmap_range(1, ipa, PAGE_SIZE);
        ret
    }

    pub fn s2mmu_protect(&mut self, ipa: usize, len: usize, flags: VmFlags) -> isize {
        if ipa & PAGE_MASK != 0 {
            return -EINVAL;
        }
        if self.pgtbl.is_null() {
            return -EINVAL;
        }

        let mut left_len = len;
        let mut next_ipa = ipa;
        let mut order = 0;

        while left_len > 0 {
            let res = self.find_page_or_block_pte(1, next_ipa);
            match res {
                Err(_) => return -ENOMMAPING,
                Ok((mut res_pte, res_level)) => {
                    let level = PteLevel::from(res_level);
                    res_pte.as_mut().set_pte_flags(level, flags);

                    order = (3 - res_level) * PAGE_ORDER;
                    let tmp_len = 1 << (order + PAGE_SHIFT);
                    left_len -= tmp_len;
                    next_ipa += tmp_len;
                }
            }
        }

        0
    }

    pub fn s2mmu_query(&self, ipa: usize) -> Result<(usize, PteLevel, VmFlags), isize> {
        if ipa & PAGE_MASK != 0 {
            return Err(-EINVAL);
        }
        if self.pgtbl.is_null() {
            return Err(-EINVAL);
        }

        let res = self.find_page_or_block_pte(1, ipa);
        match res {
            Err(_) => return Err(-ENOMMAPING),
            Ok((res_pte, res_level)) => {
                // TODO: remember to the lifetime of res_pte, which is a box type
                match res_level {
                    1 => {
                        let pfn: usize = unsafe { res_pte.as_ref().l1_block.data }
                            .get_bits(L1BLOCK_PFN_RANGE)
                            as usize;
                        let pa = pfn << (18 + PAGE_SHIFT);
                        let s2ap =
                            unsafe { res_pte.as_ref().l1_block.data }.get_bits(L1BLOCK_S2AP_RANGE);
                        let xn =
                            unsafe { res_pte.as_ref().l1_block.data }.get_bits(L1BLOCK_XN_RANGE);

                        if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_RO
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_RO));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_WO
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_WO));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_RW));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_NONE
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_EO));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_RWE));
                        }
                    }
                    2 => {
                        let pfn =
                            unsafe { res_pte.as_ref().l2_block.data }.get_bits(L2BLOCK_PFN_RANGE);
                        let pa: usize = (pfn << (9 + PAGE_SHIFT)) as usize;
                        let s2ap =
                            unsafe { res_pte.as_ref().l2_block.data }.get_bits(L1BLOCK_S2AP_RANGE);
                        let xn =
                            unsafe { res_pte.as_ref().l2_block.data }.get_bits(L1BLOCK_XN_RANGE);

                        if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_RO
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_RO));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_WO
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_WO));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_RW));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_NONE
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_EO));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_RWE));
                        }
                    }
                    3 => {
                        let pfn =
                            unsafe { res_pte.as_ref().l3_page.data }.get_bits(L3PAGE_PFN_RANGE);
                        let pa: usize = (pfn << PAGE_SHIFT) as usize;
                        let s2ap =
                            unsafe { res_pte.as_ref().l3_page.data }.get_bits(L3PAGE_S2AP_RANGE);
                        let xn = unsafe { res_pte.as_ref().l3_page.data }.get_bits(L3PAGE_XN_RANGE);

                        if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_RO
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_RO));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_WO
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_WO));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_NONE
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_RWE));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_NONE
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_EO));
                        } else if s2ap == ARM64_MMU_ATTR_STAGE2_PAGE_AP_RW
                            && xn == ARM64_MMU_ATTR_STAGE2_PAGE_XN_X_ALL
                        {
                            return Ok((pa, PteLevel::from(res_level), VmFlags::MMU_ATTR_PAGE_RWE));
                        }
                    }
                    _ => {
                        // Should be a BUG_ON here
                        // BUG("Invalid page table level !");
                        return Err(-1);
                    }
                }

                // TODO avoid res_pte being freed
                let _useless = Box::into_raw(res_pte);
            }
        }

        Err(-1)
    }

    /// Install Stage-2 Page table
    /// TODO: But the asm inline is buggy!
    pub fn install_stage2_pt(&mut self) {
        // asm volatile ("msr vsttbr_el2, %0" : : "r" (to_install_pgtbl));
        // asm volatile ("msr vttbr_el2, %0" : : "r" (to_install_pgtbl));
        let addr = self.pgtbl as usize;
        unsafe {
            /* This line causes error, don't know why */
            // asm!("msr vsttbr_el2, {0}", in(reg) addr);
            asm!("msr vttbr_el2, {0}", in(reg) addr );
        }
    }

    pub fn handle_guest_stage2_page_fault(
        &mut self,
        fault_reasion: usize,
        is_instruction_abort: usize,
        is_write_abort: usize,
        fault_ipa: usize,
        fault_va: usize,
    ) -> isize {
        let fault_ipa_page = ROUND_DOWN!(fault_ipa, PAGE_SIZE);

        let res = self.s2mmu_query(fault_ipa_page);
        match res {
            Ok((res_pa, res_level, res_flags)) => {
                let ipa = fault_ipa_page;
                let len = 1 << ((3 - res_level as usize) * PAGE_ORDER + PAGE_SHIFT);
                let flags = VmFlags::MMU_ATTR_PAGE_RWE;
                let ret = self.s2mmu_protect(fault_ipa_page, len, flags);
                if ret != 0 {
                    // TODO: should be a BUG() here
                    // BUG("change permission failed.\n");
                    panic!("change permission failed.");
                }
            }
            Err(err_code) => {
                if err_code != -ENOMMAPING {
                    panic!("Query Failed.");
                }

                let mut mapped_pa: usize;

                if fault_ipa_page > 0x8000000 {
                    mapped_pa = fault_ipa_page;
                } else {
                    // [tmp comment]: we need to impl find_ipa_region_by_ipa first
                    let ir: *mut IpaRegion = self.find_ipa_region_by_ipa(fault_ipa_page);
                    if ir.is_null() {
                        panic!("Illegal memory access.");
                    }
                    if unsafe { (*ir).region_type == IrType::IrEagerMapping as u32 } {
                        panic!("EAGER mapping");
                    }
                    mapped_pa = unsafe { super::super::ALLOCATOR.bd_alloc(PAGE_SIZE, 12).unwrap() };
                }

                let ret = self.map_page(fault_ipa_page, mapped_pa, 1, VmFlags::MMU_ATTR_PAGE_RWE);
                if ret != 0 {
                    panic!("map failed");
                }
            }
        }

        unsafe { flush_tlb() };
        0
    }

    /// TODO: Check if newly added ipa_region overlapped
    /// with existing ipa_regon in s2mmu list
    /// (Probably buggy! Check it carefully)
    pub fn region_overlap(&mut self, checked_region: *mut IpaRegion) -> bool {
        let checked_region_virt = checked_region as usize;
        let checked_ipa_start = unsafe { (*checked_region).ipa_start };
        let checked_ipa_end = checked_ipa_start + unsafe { (*checked_region).size };

        let list_head_virt = &mut self.ipa_region_list as *mut ListHead as usize;
        let mut current_region_virt: usize =
            unsafe { (*(list_head_virt as *mut ListHead)).next as usize };
        while current_region_virt != list_head_virt {
            let current_region_ptr = current_region_virt as *mut IpaRegion;
            unsafe {
                let start = (*current_region_ptr).ipa_start;
                let end = start + (*current_region_ptr).size;
                if (start <= checked_ipa_start && checked_ipa_start < end)
                    || (start < checked_ipa_end && checked_ipa_end <= end)
                {
                    return true;
                }
            }
            // get next
            current_region_virt =
                unsafe { (*(current_region_virt as *mut ListHead)).next as usize };
        }
        false
    }

    pub fn add_ipa_region(&mut self, ipa_region: *mut IpaRegion) -> isize {
        if ipa_region.is_null() {
            return -EINVAL;
        }
        if self.region_overlap(ipa_region) {
            return -EINVAL;
        }

        let list_head_ptr: *mut ListHead = &mut self.ipa_region_list as *mut ListHead;
        unsafe {
            (*list_head_ptr).push(ipa_region as usize as *mut ListHead);
        }
        0
    }

    pub fn delete_ipa_region(&mut self, ipa_region: *mut IpaRegion) -> isize {
        /* Sanity Check */
        if ipa_region.is_null() {
            return -EINVAL;
        }
        /* Check if existing, return error if not */
        if self.region_overlap(ipa_region) == false {
            return -EINVAL;
        }
        /* Do the deletion */
        //TODO: the c-version of this function uses list_pop here, which might be wrong
        // should use list_remove instead
        let list_head_ptr: *mut ListHead = ipa_region as *mut ListHead;
        unsafe {
            (*list_head_ptr).remove_self();
        }
        0
    }

    /// Find ipa_region by ipa
    pub fn find_ipa_region_by_ipa(&mut self, ipa: usize) -> *mut IpaRegion {
        let list_head_virt = &mut self.ipa_region_list as *mut ListHead as usize;
        let mut current_region_virt: usize = unsafe {
            let list_head = list_head_virt as *mut ListHead;
            (*list_head).next as usize
        };
        while current_region_virt != list_head_virt {
            let current_region_ptr: *mut IpaRegion = current_region_virt as *mut IpaRegion;
            if unsafe { (*current_region_ptr).contains_ipa(ipa as u64) } {
                return current_region_ptr;
            }
            current_region_virt =
                unsafe { (*(current_region_virt as *mut ListHead)).next } as usize;
        }
        core::ptr::null_mut()
    }

    pub fn sync_ipa_regions_to_page_table(&mut self) -> isize {
        let list_head_virt: usize = &mut self.ipa_region_list as *mut ListHead as usize;
        let mut current_region_virt: usize = unsafe {
            let list_head_ptr = list_head_virt as *mut ListHead;
            (*list_head_ptr).next as usize
        };
        while current_region_virt != list_head_virt {
            let current_region_ptr = current_region_virt as *mut IpaRegion;
            if unsafe { (*current_region_ptr).region_type == IrType::IrEagerMapping as u32 } {
                let ipa_start: usize = unsafe { (*current_region_ptr).ipa_start } as usize;
                let pa_start: usize = unsafe { (*current_region_ptr).pa_start } as usize;
                let size: usize = unsafe { (*current_region_ptr).size } as usize;
                let attr = unsafe { (*current_region_ptr).region_attr };
                self.map_range(
                    ipa_start,
                    pa_start,
                    1,
                    size,
                    1, /* enable Huge Page */
                    VmFlags::from(attr as usize),
                );
            }
            current_region_virt =
                unsafe { (*(current_region_virt as *mut ListHead)).next as usize };
        }
        0
    }
}
