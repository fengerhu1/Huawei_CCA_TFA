use crate::rmm::smc_rmi::{
    RealmParams,
    RecParams,
    RecRun,
};

use crate::rmm::granule_util::{
    Granule,
};

use crate::rmm::psci_util::{
    PsciResult,
};

use crate::rmm::rvic_util::{
    RvicResult,
};

use crate::rmm::rec_util::{
    Rec,
};

use crate::rmm::table_util::{
    TblWalk,
};

use crate::rmm::rmm_util::{
    NsState,
};

use crate::rmm::platform::{
    MAX_CPUS,
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VPercpu
{
	pub p: RealmParams, // User given param for realm create
    pub r: RecParams, // User given param for rec create
    pub locked_granules: [Granule; 3],
	pub run: RecRun, // Store the Rec running state for normal
	psci_result: PsciResult,
	rvic_result: RvicResult,
	target_rec: Option<*mut Rec>,
	pub wi: TblWalk, // Used by stage 2 page table walker
	pub ns_state: NsState, // Store the ns state for context switch
}

impl VPercpu {
    pub fn new() -> Self {
        VPercpu { 
            p: RealmParams::new(),
            r: RecParams::new(),
            locked_granules: [Granule::new(usize::MAX); 3],
            run: RecRun::new(),
            psci_result: PsciResult::new(),
            rvic_result: RvicResult::new(),
            target_rec: None,
            wi: TblWalk::new(),
            ns_state: NsState::new(),
        }
    }
}

unsafe impl Sync for VPercpu {}
unsafe impl Send for VPercpu {}

use spinning_top::Spinlock as Mutex;
use lazy_static::lazy_static;
use alloc::{
    vec::Vec,
};

lazy_static! {
    pub static ref VPERCPU_LOCK: Mutex<Vec<VPercpu>> = {
        let mut vpercpu_list:Vec<VPercpu> =Vec::new();
        for _i in 0..MAX_CPUS {
            vpercpu_list.push(VPercpu::new());
        }

        Mutex::new(vpercpu_list)
    };
}

#[inline]
pub fn get_realm_params_rtt_addr() -> usize {
    let v_percpu_list = VPERCPU_LOCK.lock();                
                
    // dereference a raw pointer
    // copy the realm_parameter into v_percpu
    let rtt_addr = v_percpu_list[crate::cpuid!()].p.table_addr;
    drop(v_percpu_list);
    return rtt_addr;
}

#[inline]
pub fn get_realm_params_rec_list_addr() -> usize {
    let v_percpu_list = VPERCPU_LOCK.lock();                
                
    // dereference a raw pointer
    // copy the realm_parameter into v_percpu
    let rtt_addr = v_percpu_list[crate::cpuid!()].p.rec_list_addr;
    drop(v_percpu_list);
    return rtt_addr;
}