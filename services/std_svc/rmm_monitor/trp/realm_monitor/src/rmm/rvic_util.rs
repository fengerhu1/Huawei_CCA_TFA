use crate::rmm::smc_rmi::{
    SmcResult
};

use crate::rmm::sysreg::{
    RVIC_BITMAP_ULS,
    BITS_PER_UL,
};

pub const INTID_ARCHTIMER_EL1:usize =	    26;
pub const INTID_VTIMER_EL1:usize =			27;
pub const INTID_PTIMER_EL1:usize =			30;
pub const INTID_UNKNOWN_EL1:usize =         0x1a;

extern "C"{
    pub fn c_atomic_bit_set_release_64(bitmap: *mut usize, idx: usize, bit: usize);
    pub fn c_test_bit_acquire_64(bitmap: *const usize, idx: usize, bit: usize) -> bool;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RvicResult {
	ns_notify: bool,
	target: usize,
	smc_result: SmcResult,
}

impl RvicResult {
    pub fn new() -> Self {
        RvicResult { 
            ns_notify: false,
			target: 0,
			smc_result: SmcResult::new(),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RecRvicState {
	pub rvic_enabled: bool,
	pub mask_bits: [usize; RVIC_BITMAP_ULS],
	pub pending_bits: [usize; RVIC_BITMAP_ULS],
}

impl RecRvicState {
    pub fn new() -> Self {
        RecRvicState { 
            rvic_enabled: false,
            mask_bits: [0; RVIC_BITMAP_ULS],
			pending_bits: [0; RVIC_BITMAP_ULS],
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rvic {
}

impl Rvic {
    pub fn rvic_set_flag(intid: usize, bitmap: *mut usize) {
        let idx: usize = intid / BITS_PER_UL;
        let bit: usize = intid % BITS_PER_UL;

        unsafe {
            c_atomic_bit_set_release_64(bitmap , idx, bit)
        }
    }

    pub fn rvic_test_flag(intid: usize, bitmap: *const usize)-> bool
    {
        let idx: usize = intid / BITS_PER_UL;
        let bit: usize = intid % BITS_PER_UL;
        let ret = unsafe {c_test_bit_acquire_64(bitmap , idx, bit)};

        return ret;
    }

    pub fn rvic_set_pending(rvic: &mut RecRvicState, intid: usize) {
        Rvic::rvic_set_flag(intid, &mut (rvic.pending_bits[0]) as &mut usize as *mut usize);
    }

    pub fn rvic_is_masked(rvic: & RecRvicState, intid: usize) -> bool
    {
        let bits = rvic.mask_bits;
        let ret = Rvic::rvic_test_flag(intid, &(bits[0]) as &usize as *const usize);

        return ret;
    }
}
