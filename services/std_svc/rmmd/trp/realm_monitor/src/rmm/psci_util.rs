use crate::rmm::smc_rmi::{
    SmcResult
};

#[derive(Clone, Copy)]
#[repr(C)]
struct HvcForward {
    forward_psci_call: bool,
    x1: usize,
    x2: usize,
    x3: usize,
}

impl HvcForward {
    pub fn new() -> Self {
        HvcForward { 
            forward_psci_call: false,
            x1: 0,
            x2: 0,
            x3: 0,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PsciResult {
    hvc_forward: HvcForward,
    smc_result: SmcResult,
}

impl PsciResult {
    pub fn new() -> Self {
        PsciResult { 
            hvc_forward: HvcForward::new(),
            smc_result: SmcResult::new(),
        }
    }
}