//RmiAux
use crate::rmm::granule_util::{
    Granule, 
    GranuleUtil,
    ALIGNED,
    addr_to_idx,
    GRANULE_SIZE,
    NR_GRANULES,
    ERROR_MAX,
    MEM0_PHYS,
    MEM1_PHYS,
    MEM_PHYS_BASE,
    BufferSlot,
    GranuleState,
    ErrorStatus};

use crate::rmm::smc_rmi::{
    RealmParams,
};

use crate::rmm::rec_util::{
    NR_NS_STATE,
};

use crate::rmm::abs_util::{
    VPERCPU_LOCK,
};

use crate::rmm::realm_util::{
    Rd,
};

use crate::rmm::measurement::{
    MeasurementAlgo,
    measurement_start,
    measurement_finish,
};

use crate::io::Write;

use sha2::{Sha256, Sha512, Digest};


#[derive(Clone, Copy)]
#[repr(C)]
pub struct NsState {
	pub sysregs: [usize; NR_NS_STATE],
}

impl NsState {
    pub fn new() -> Self {
        NsState { 
            sysregs: [0; NR_NS_STATE],
        }
    }
}

#[repr(C)]
pub struct RmmUtil {
    // RmmUtil is placeholder structure for better organization
    // rmm_granule: GranuleUtil
}

extern "C"{
    // map the physical address into a tmp va per cpu core
    pub fn buffer_map(slot: BufferSlot, addr: usize, ns: bool) -> usize;
    // unmap the tmp va
    pub fn buffer_unmap(buf: usize);
    // set the memory range to zero
    pub fn memzero(buf: usize, size: usize);
}


/**
 * \brief Map the granule and return the corresponding va
 */
pub fn granule_map(rlm_para_addr: usize, realm_granule: Granule, slot: BufferSlot) -> usize {
    let idx = addr_to_idx(rlm_para_addr);
    let addr;
    if cfg!(feature = "platform_qemu") {
        addr =  MEM0_PHYS + (idx * GRANULE_SIZE);
    } else if cfg!(feature = "platform_fvp") {
        if idx > (NR_GRANULES/2) {
            addr = MEM1_PHYS + (idx-(NR_GRANULES/2))*GRANULE_SIZE;
        }
        else {// For tf-a-test
            addr =  MEM0_PHYS + (idx * GRANULE_SIZE);
        }
    } else {
        addr = 0;
        crate::println!("ERROR: granule_map is failed: invalied platform");
    }
    let ns=(realm_granule.state == GranuleState::GranuleStateNs);
    let ret: usize;

    unsafe {
        ret = buffer_map(slot, addr, ns);
    }

    return ret;
}

/**
 * \brief Map the granule with the granule id, and return the corresponding va
 */
pub fn granule_map_with_id(idx: usize, slot: BufferSlot) -> usize {
    if idx > NR_GRANULES {
        crate::println!("ERROR: granule_map_with_id is failed: invalied idx");
    }
    // cal the physical address of the mapped granule
    let addr;
    if cfg!(feature = "platform_qemu") {
        addr = MEM0_PHYS + (idx * GRANULE_SIZE);
    } else if cfg!(feature = "platform_fvp") {
        if idx > (NR_GRANULES/2) {
            addr = MEM1_PHYS + (idx-(NR_GRANULES/2))*GRANULE_SIZE;
        }
        else {// For tf-a-test
            addr = MEM0_PHYS + (idx * GRANULE_SIZE);
        }
    } else {
        addr = 0;
        crate::println!("ERROR: granule_map_with_id is failed: invalied platform");
    }
    let state = GranuleUtil::acquire_granule_state(idx as u32);
    let ns=(state == GranuleState::GranuleStateNs);
    let ret: usize;

    unsafe {
        ret = buffer_map(slot, addr, ns);
    }

    return ret;
}

/**
 * \brief Map the granule with the granule id and state, and return the corresponding va
 */
pub fn granule_map_with_id_state(idx: usize, state: GranuleState, slot: BufferSlot) -> usize {
    if idx > NR_GRANULES {
        crate::println!("ERROR: granule_map_with_id_state is failed: invalied idx");
    }
    // cal the physical address of the mapped granule
    let addr;
    if cfg!(feature = "platform_qemu") {
        addr =  MEM0_PHYS + (idx * GRANULE_SIZE);
    } else if cfg!(feature = "platform_fvp") {
        if idx > (NR_GRANULES/2) {
            addr = MEM1_PHYS + (idx-(NR_GRANULES/2))*GRANULE_SIZE;
        }
        else {// For tf-a-test
            addr =  MEM0_PHYS + (idx * GRANULE_SIZE);
        }
    } else {
        addr = 0;
        crate::println!("ERROR: granule_map_with_id_state is failed: invalied platform");
    }
    let ns=(state == GranuleState::GranuleStateNs);
    let ret: usize;

    unsafe {
        ret = buffer_map(slot, addr, ns);
    }

    return ret;
}

/* granule_memzero */
pub fn granule_map_zero(idx: u32, slot: BufferSlot) {
    let buf = granule_map_with_id(idx as usize, slot) as *mut usize;
	unsafe {
        memzero(buf as usize, GRANULE_SIZE);
        buffer_unmap(buf as usize);
    }
}

impl RmmUtil {
    /* get_RealmParams
     * 
     * \brief: Get the realm parameters and set to the global v_percpu  
     */
    pub fn rmm_para_parser(rlm_para_addr: usize) -> ErrorStatus {
        let realm_granule = GranuleUtil::find_granule(rlm_para_addr);

        match realm_granule {
            Err(error) => {return(error)}
            Ok(granule) =>{
                let debug_para = ((rlm_para_addr as usize) + 0x200000000) as *const RealmParams; 
                let tmp_RealmParams = granule_map(rlm_para_addr, granule, BufferSlot::SLOT_INPUT) as *mut RealmParams;
                // acquire the global v_percpu_list
                let mut v_percpu_list = VPERCPU_LOCK.lock();                
                
                // dereference a raw pointer
                // copy the realm_parameter into v_percpu
                v_percpu_list[crate::cpuid!()].p = unsafe{ (*tmp_RealmParams).clone()};

                drop(v_percpu_list);

                unsafe {
                    buffer_unmap(tmp_RealmParams as usize);
                }
            }
        }
        
        return ErrorStatus::StatusSuccess; //STATUS_SUCCESS
    }

    pub fn validate_realm_params() -> usize {
        let v_percpu_list = VPERCPU_LOCK.lock();                
                
        // dereference a raw pointer
        // copy the realm_parameter into v_percpu
        let base = v_percpu_list[crate::cpuid!()].p.par_base;
        let size = v_percpu_list[crate::cpuid!()].p.par_size;
        drop(v_percpu_list);

        if ALIGNED(base, GRANULE_SIZE) && ALIGNED(size, GRANULE_SIZE) {
            return 0;
        } else {
            crate::println!("ERROR: base {:x} size {:x}", base, size);
            return 1;
        }
    }

    pub fn set_lock_granule(addr: usize, recv_granule: &mut Granule) -> ErrorStatus {
        // Retrieve three granules and set them in the current CPU structure.
        let tmp_granule = GranuleUtil::find_lock_granule(addr, GranuleState::GranuleStateDelegated);
        match tmp_granule {
            Ok(granule) => {
                *recv_granule = granule;
                return ErrorStatus::StatusSuccess;
            }
            Err(err) => {
                return err;
            }
        }
    }

    pub fn find_lock_three_delegated_granules(addr1: usize, addr2: usize, addr3: usize) -> ErrorStatus {
        if (addr1 == addr2) || (addr1 == addr3)|| (addr2 == addr3) {
            return ErrorStatus::StatusErrorParameter;
        }

        let mut v_percpu_list = VPERCPU_LOCK.lock();                

        let ret1 = RmmUtil::set_lock_granule(addr1, &mut (v_percpu_list[crate::cpuid!()].locked_granules[0]));

        let ret2 = RmmUtil::set_lock_granule(addr2, &mut (v_percpu_list[crate::cpuid!()].locked_granules[1]));

        let ret3 = RmmUtil::set_lock_granule(addr3, &mut (v_percpu_list[crate::cpuid!()].locked_granules[2]));

        drop(v_percpu_list);

        if  (ret1 == ErrorStatus::StatusSuccess) && 
            (ret2 == ErrorStatus::StatusSuccess) &&
            (ret3 == ErrorStatus::StatusSuccess) {
            return ErrorStatus::StatusSuccess
        }
        else {
            return ret1;
        }
    }

    pub fn find_lock_unused_granule(addr: usize, expected_state: GranuleState) -> Result<u32, ErrorStatus> {
        let g = GranuleUtil::find_lock_granule(addr, expected_state);

        match g {
            Ok(granule) => {
                if (GranuleUtil::granule_refcount_read_acquire(granule.id)) {
                    GranuleUtil::unlock_granule(granule.id);
                    return Err(ErrorStatus::StatusErrorInUse);
                }
                return Ok(granule.id)
            }
            Err(err) => {
                crate::println!("ERROR: find_lock_unused_granule {:?}", err);
                return Err(err);
            }
        }

    }

    pub fn find_lock_granule(addr: usize, expexted_state: GranuleState) -> Result<u32, ErrorStatus> {
        let g = GranuleUtil::find_granule(addr);
        match g {
            Ok(granule) => {
                if GranuleUtil::lock_granule(granule.id) == false {
                    return Err(ErrorStatus::StatusErrorGranuleAttr);
                }
                else {
                    return Ok(granule.id);
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub fn prepare_and_destroy_realm(rd_id: u32) -> Result<usize, ErrorStatus> {
        let rd_ptr = granule_map_with_id(rd_id as usize,  BufferSlot::SLOT_RD) as *mut Rd;
        let rd = unsafe {&mut (*rd_ptr)};
        let g_table = rd.g_table;
        let g_rec_list = rd.g_rec_list;

        //FIXME: measurement_destroy is ignored 
        
        unsafe {
            buffer_unmap(rd_ptr as usize);
        }
        
        GranuleUtil::lock_granule(g_table.id);
        if GranuleUtil::acquire_granule_refcount(g_table.id) != 1 {
            GranuleUtil::unlock_granule(g_table.id);
            GranuleUtil::unlock_granule(rd_id);
            return Err(ErrorStatus::StatusErrorInUse);
        }
        else {
            //zeorlize the all granule pages
            granule_map_zero(g_table.id, BufferSlot::SLOT_RTT);
            granule_map_zero(g_rec_list.id, BufferSlot::SLOT_REC_LIST);
            granule_map_zero(rd_id, BufferSlot::SLOT_RD);

            GranuleUtil::put_granule(g_table.id as u32);
            GranuleUtil::granule_set_rd(g_table.id as u32, NR_GRANULES as u32);
            GranuleUtil::granule_set_state(g_table.id as u32, GranuleState::GranuleStateDelegated);
            GranuleUtil::unlock_granule(g_table.id as u32);

            GranuleUtil::granule_set_state(g_rec_list.id as u32, GranuleState::GranuleStateDelegated);

            GranuleUtil::granule_set_state(rd_id as u32, GranuleState::GranuleStateDelegated);
            GranuleUtil::unlock_granule(rd_id as u32);

            return Ok(0);
        }

    }

    pub fn activate_realm(rd_id: u32) -> ErrorStatus {
        let mut ret:ErrorStatus;
        let rd_ptr = granule_map_with_id(rd_id as usize,  BufferSlot::SLOT_RD) as *mut Rd;
        let rd = unsafe {&mut (*rd_ptr)};

        if rd.state == 0 {
           RmmUtil::realm_activate_ops(rd);
            ret = ErrorStatus::StatusSuccess;
        }
        else {
            ret = ErrorStatus::StatusErrorRealmAttr;
        }

        unsafe {
            buffer_unmap(rd_ptr as usize);
        }
        GranuleUtil::unlock_granule(rd_id);

        return ret;
    }
    
    pub fn realm_create_ops() {
        crate::dprintln!("Debug: realm_create_ops");
        let v_percpu_list = VPERCPU_LOCK.lock();

        let g_rd_id = v_percpu_list[crate::cpuid!()].locked_granules[0].id;
        let g_table_id = v_percpu_list[crate::cpuid!()].locked_granules[1].id;
        let g_rec_list_id = v_percpu_list[crate::cpuid!()].locked_granules[2].id;
        let g_rd_state = v_percpu_list[crate::cpuid!()].locked_granules[0].state;
        let realm_par_base = v_percpu_list[crate::cpuid!()].p.par_base;
        let realm_par_size = v_percpu_list[crate::cpuid!()].p.par_size;

        drop(v_percpu_list);

        GranuleUtil::realm_create_granule_ops1(g_rd_id, g_table_id, g_rec_list_id);
        crate::dprintln!("Debug: realm_create_ops 2");
        let rd_ptr = granule_map_with_id_state(g_rd_id as usize, g_rd_state, BufferSlot::SLOT_RD) as *mut Rd;
        let rd = unsafe {&mut (*rd_ptr)};

        GranuleUtil::realm_create_granule_ops2(rd, g_table_id, g_rec_list_id, realm_par_base, realm_par_size);

        // FIXME: We ignore the measurement here for realm_create here
        let v_percpu_list = VPERCPU_LOCK.lock();
        match v_percpu_list[crate::cpuid!()].p.measurement_algo {
            0 => {
                rd.ctx.measurement_algo = MeasurementAlgo::MeasurementAlgoZero;
            }
            1 => {
                rd.ctx.measurement_algo = MeasurementAlgo::MeasurementAlgoSha256;
            }
            _ => {
                // crate::println!("ERROR: realm_create_ops is failed: error measurement_algo")
            }
        }
        drop(v_percpu_list);

        measurement_start(rd);

        unsafe {
            buffer_unmap(rd_ptr as usize);
        }

        GranuleUtil::granule_set_state(g_rd_id as u32, GranuleState::GranuleStateRd);
        GranuleUtil::unlock_granule(g_rd_id as u32);
        
    }

    pub fn realm_activate_ops(rd: &mut Rd) {
        // FIXME: Ignore the measurement here
        measurement_finish(rd);
        rd.state = 1;
    }

    pub fn granule_delegate_ops(idx: u32, addr: usize) {
        GranuleUtil::granule_set_state(idx, GranuleState::GranuleStateDelegated);
        granule_map_zero(idx, BufferSlot::SLOT_DELEGATED);
        // We need to call smc call to el3 monitor to delegate the granule
        // We lock the delegate granule before
        GranuleUtil::unlock_granule(idx);
    }

    pub fn granule_undelegate_ops(idx: u32, addr: usize) {
        GranuleUtil::granule_set_state(idx, GranuleState::GranuleStateNs);
        // We need to call smc call to el3 monitor to delegate the granule
        // We lock the delegate granule before
        GranuleUtil::unlock_granule(idx);
    }
}
