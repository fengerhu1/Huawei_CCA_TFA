use crate::rmm::rmm_util::{
    RmmUtil,
    granule_map_with_id,
};

use crate::rmm::granule_util::{
    Granule, 
    GranuleState,
    BufferSlot,
    ErrorStatus,
    GranuleUtil,
    NR_GRANULES
};

use crate::rmm::abs_util::{
    get_realm_params_rtt_addr,
    get_realm_params_rec_list_addr,
};

use crate::rmm::table_util::{
    RTT_PAGE_LEVEL,
    TableUtil,
    NR_TABLE_LEVELS,
};

use crate::rmm::rec_util::{
    RecUtil,
};

use crate::io::Write;

use alloc::vec::Vec;

const PAGE_SIZE: usize = 4096;
const RMM_GTSI_DELEGATE: usize = 0xc40001b0;
const RMM_GTSI_UNDELEGATE: usize = 0xc40001b1;
const RMM_GTSI_DELEGATE_COMPACT: usize = 0xc40001b4;
const RMM_GTSI_UNDELEGATE_COMPACT: usize = 0xc40001b5;

// use lazy_static::lazy_static;
// use spinning_top::Spinlock as Mutex;


// lazy_static! {
//     static ref SMCC: RmmSMC = RmmSMC::new();
// }

// lazy_static! {
//     pub static ref CALL_LOCK: Mutex<RmmSMC> = {
//            let mut smcc:RmmSMC =RmmSMC::new();

//             Mutex::new(smcc)
//            };
//    }


pub fn pack_return_code(error_state: ErrorStatus, index: u32) -> usize {
    let tmp  = (error_state as u32);
    return ((index <<8) | tmp) as usize;
}

#[repr(C)]
pub struct RmmSMC {

}

extern "C"{
    // map the physical address into a tmp va per cpu core
    pub fn buffer_map(slot: BufferSlot, addr: usize, ns: bool) -> usize;
    // unmap the tmp va
    pub fn buffer_unmap(buf: usize);
    pub fn set_smc_args(arg0: usize, arg1: usize, arg2: usize, arg3: usize,
			            arg4: usize, arg5: usize, arg6: usize, arg7: usize) -> usize;
    pub fn trp_smc(trp_args_t: usize) -> usize;
    pub fn ns_buffer_read_data(slot: BufferSlot, data: usize) -> bool;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct addr_segment {
    base: usize,
    size: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct triple {
    args1: usize,
    args2: usize,
    args3: usize,
}

impl RmmSMC {
    pub fn smc_granule_delegate(addr: usize) -> usize {
        // crate::println!("Debug: smc_granule_delegate addr: {:x}", addr);
        let g = GranuleUtil::find_lock_granule(addr, GranuleState::GranuleStateNs);
        //crate::println!("smc_granule_delegate returned");
        match g {
            Ok(granule) => {
                RmmUtil::granule_delegate_ops(granule.id, addr);
                return ErrorStatus::StatusSuccess as usize;
            }
            Err(err) => {
                crate::println!("Error: smc_granule_delegate is failed: {:?}", err);
                return pack_return_code(err, 0);
            }
        }
    }

    pub fn smc_granule_delegate_without_clear(addr: usize) -> usize {
        // crate::println!("Debug: smc_granule_delegate_without clear addr: {:x}", addr);
        let g = GranuleUtil::find_lock_granule(addr, GranuleState::GranuleStateNs);
        //crate::println!("smc_granule_delegate returned");
        match g {
            Ok(granule) => {
                RmmUtil::granule_delegate_without_clear_ops(granule.id, addr);
                return ErrorStatus::StatusSuccess as usize;
            }
            Err(err) => {
                crate::println!("Error: smc_granule_delegate is failed: {:?}", err);
                return pack_return_code(err, 0);
            }
        }
    }

    pub fn smc_granule_undelegate(addr: usize) -> usize {
        // crate::println!("Debug: smc_granule_undelegate addr: {:x}", addr);
        let g = GranuleUtil::find_lock_granule(addr, GranuleState::GranuleStateDelegated);
        match g {
            Ok(granule) => {
                RmmUtil::granule_undelegate_ops(granule.id, addr);
                return ErrorStatus::StatusSuccess as usize;
            }
            Err(err) => {
                crate::println!("Error: smc_granule_delegate is failed: {:?}", err);
                return pack_return_code(err, 0);
            }
        }
    }

    /**
     * \brief: Create the realm from a smc call.
     *
     * rd_addr: The address of the rd.
     * rlm_para_addr: The address of the realm parameters
     */
    pub fn smc_realm_create(rd_addr: usize, rlm_para_addr: usize) -> usize {
        crate::println!("Debug: smc_realm_create rd_addr: {:x} rlm_para_addr: {:x}", rd_addr, rlm_para_addr);
        if RmmUtil::rmm_para_parser(rlm_para_addr) == ErrorStatus::StatusSuccess {
            if RmmUtil::validate_realm_params() == 0 {
                let rtt_addr = get_realm_params_rtt_addr();
                let rec_list_addr = get_realm_params_rec_list_addr();
                let ret_val = RmmUtil::find_lock_three_delegated_granules(rd_addr, rtt_addr, rec_list_addr);

                match ret_val {
                    ErrorStatus::StatusSuccess=> {
                        RmmUtil::realm_create_ops();
                        return 0;
                    }
                    _ => {
                        crate::println!("ERROR: smc_realm_create is failed: REALM_CREATE_IS_FAILED");
                        return 1;
                    }
                } 
            }
            crate::println!("ERROR: smc_realm_create is failed: ERROR PARAMETERS");
            return 1;
        }
        else {
            crate::println!("ERROR: smc_realm_create is failed: ERROR PARAMETERS ADDRESS");
            return 1;
        }

    }

    pub fn smc_realm_destroy(rd_addr: usize) -> usize{
        
        let g_rd = RmmUtil::find_lock_unused_granule(rd_addr, GranuleState::GranuleStateRd);
        match g_rd {
            Ok(rd_id) => {
                let ret = RmmUtil::prepare_and_destroy_realm(rd_id);
                match ret {
                    Ok(_) => {return 0;}
                    Err(err) => {
                        crate::println!("ERROR: smc_realm_destroy is failed: {:?}", err); 
                        return pack_return_code(ErrorStatus::StatusErrorInUse, 1);}
                }
            }
            Err(err) => {
                crate::println!("ERROR: smc_realm_destroy is failed: {:?}", err);
                return pack_return_code(err, 0);
            }
        }
    }

    pub fn smc_realm_activate(rd_addr: usize) -> usize {
        let mut ret:usize;
        let g_rd = RmmUtil::find_lock_granule(rd_addr, GranuleState::GranuleStateRd);
        match g_rd {
            Ok(rd_id) => {
                match RmmUtil::activate_realm(rd_id) {
                    ErrorStatus::StatusSuccess => {
                        ret = 0;
                    }
                    other => {
                        ret = pack_return_code(other, 0);
                    }
                }
            }
            Err(err) => {
                ret = pack_return_code(err,0);
            }
        }
        return ret;
    }

    pub fn smc_table_create(rtt_addr: usize, rd_addr: usize, map_addr: usize,
        level: usize) -> usize {
        let mut ret = TableUtil::validate_table_commands(map_addr, level, 1, RTT_PAGE_LEVEL);

        if ret == 0 {
            let g_rtt = RmmUtil::find_lock_unused_granule(rtt_addr, GranuleState::GranuleStateDelegated);
            match g_rtt {
                Ok(rtt_id) => {
                    let rd_id = RmmUtil::find_lock_granule(rd_addr, GranuleState::GranuleStateRd).unwrap_or_else(|err| {
                        GranuleUtil::unlock_granule(rtt_id);
                        ret =  pack_return_code(err, 1) as usize;
                        0 // rd_id is out of the granule range
                    });

                    if ret != 0 { 
                        crate::println!("ERROR: smc_table_create is failed: {:?}", ret);
                        return ret;
                    }

                    ret = TableUtil::table_create(rd_id, map_addr, level, rtt_id, rtt_addr).unwrap_or_else(|err| {
                        crate::println!("ERROR: smc_table_create is failed: {:?}", err);
                        pack_return_code(err, 0)
                    });

                    GranuleUtil::unlock_granule(rd_id);
                    GranuleUtil::unlock_granule(rtt_id);
                }
                Err(err) => {
                    crate::println!("ERROR: smc_table_create is failed: {:?}", err);
                    ret = pack_return_code(err, 2);
                }
            }
        }
        return ret;
    }

    pub fn smc_table_destroy(rtt_addr: usize, rd_addr: usize, map_addr: usize, level: usize) -> usize {
        let mut ret = TableUtil::validate_table_commands(map_addr, level, 1, RTT_PAGE_LEVEL);
        crate::dprintln!("DEBUG: smc_table_destroy: rtt_addr {:x}, map_addr {:x}, level {:x}", rtt_addr, map_addr, level);
        if ret == 0 {
            let g_rd = RmmUtil::find_lock_granule(rd_addr, GranuleState::GranuleStateRd);
            match g_rd {
                Ok(rd_id) => {
                    ret = TableUtil::table_destroy(rd_id, map_addr, rtt_addr, level).unwrap_or_else(|err| {
                        crate::println!("ERROR: smc_table_destroy is failed: {:?}", err);
                        pack_return_code(err, 0)
                    });
                    GranuleUtil::unlock_granule(rd_id);
                }
                Err(err) => {
                    ret = pack_return_code(err, 1);
                }
            }
        }

        return ret;
    }

    /* 
     * \brief smc_data_map only work for the last level page table and change the IPA state
     */
    pub fn smc_data_map(rd_addr: usize, map_addr: usize) -> usize {
        let level = NR_TABLE_LEVELS - 1;
        let mut ret = TableUtil::validate_table_commands(map_addr, level, 1, RTT_PAGE_LEVEL);

        if ret == 0 {
            let g_rd = RmmUtil::find_lock_granule(rd_addr, GranuleState::GranuleStateRd);
            match g_rd {
                Ok(rd_id) => {
                    ret = TableUtil::table_map(rd_id, map_addr, level).unwrap_or_else(|err| {
                        crate::println!("ERROR: smc_data_map is failed: {:?}", err);
                        pack_return_code(err, 0)
                    });
                    
                    GranuleUtil::unlock_granule(rd_id);
                }
                Err(err) => {
                    ret = pack_return_code(err, 0);
                }
            }
        }
        return ret;
    }

    /* 
     * \brief smc_data_unmap only work for the last level page table and change the IPA state
     */
    pub fn smc_data_unmap(rd_addr: usize, map_addr: usize) -> usize {
        let level = NR_TABLE_LEVELS - 1;
        let mut ret = TableUtil::validate_table_commands(map_addr, level, 1, RTT_PAGE_LEVEL);

        if ret == 0 {
            let g_rd = RmmUtil::find_lock_granule(rd_addr, GranuleState::GranuleStateRd);
            match g_rd {
                Ok(rd_id) => {
                    ret = TableUtil::table_unmap(rd_id, map_addr, level).unwrap_or_else(|err| {
                        crate::println!("ERROR: smc_data_unmap is failed: {:?}", err);
                        pack_return_code(err, 0)
                    });
                    
                    GranuleUtil::unlock_granule(rd_id);
                }
                Err(err) => {
                    ret = pack_return_code(err, 0);
                }
            }
        }
        return ret;
    }

    pub fn smc_data_create(data_addr: usize, rd_addr: usize, src_addr: usize, map_addr: usize) -> usize{
        let mut ret:usize = 0;
        let g_src = GranuleUtil::find_granule(src_addr);

        match g_src {
            Ok(src_granule) => {
                // find an unused granule for src_data
                let data_id = RmmUtil::find_lock_unused_granule(data_addr, GranuleState::GranuleStateDelegated).unwrap_or_else(|err| {
                    crate::println!("ERROR: smc_data_create is failed: {:?}", err);
                    ret = pack_return_code(err, 1);
                    0
                });

                if ret != 0 {return ret;}

                // find the corresponding realm_rd
                let g_rd = RmmUtil::find_lock_granule(rd_addr, GranuleState::GranuleStateRd);
                match g_rd {
                    Ok(rd_id) => {
                        ret = TableUtil::data_create(rd_id, data_addr, map_addr, data_id, src_granule.id).unwrap_or_else(|err| {
                            crate::println!("ERROR: smc_data_create is failed: {:?}", err);
                            pack_return_code(err, 1)
                        });
                    }
                    Err(err) => {
                        crate::println!("ERROR: smc_data_create: find_lock_granule rd granule is failed: {:?}", err);
                        ret = pack_return_code(err, 2);
                    }
                }
                GranuleUtil::unlock_granule(data_id);
            }
            Err(err) => {
                crate::println!("ERROR: smc_data_create: find src granule is failed: {:?}", err);
                ret = pack_return_code(ErrorStatus::StatusErrorParameter, 4);
            }
        }

        return ret;
    }

    pub fn smc_data_destroy(map_addr: usize, rd_addr: usize) -> usize {
        let mut ret:usize = 0;
        let g_rd = RmmUtil::find_lock_granule(rd_addr, GranuleState::GranuleStateRd);
        match g_rd {
            Ok(rd_id) => {
                ret = TableUtil::data_destroy(rd_id, map_addr).unwrap_or_else(|err| {
                    crate::println!("ERROR: smc_data_destroy is failed: {:?}", err);
                    pack_return_code(err, 1)
                });
                // Unlock the rd granule
                GranuleUtil::unlock_granule(rd_id);
            }
            Err(err) => {
                ret = pack_return_code(err, 0);
            }
        }
        return ret;
    }

    pub fn smc_data_create_unknown(data_addr: usize, rd_addr: usize, map_addr: usize) -> usize{
        let mut ret:usize = 0;

        // find an unused granule for src_data
        let data_id = RmmUtil::find_lock_unused_granule(data_addr, GranuleState::GranuleStateDelegated).unwrap_or_else(|err| {
            crate::println!("ERROR: smc_data_create_unknown is failed: {:?}", err);
            ret = pack_return_code(err, 0);
            0
        });

        if ret != 0 {return ret;}

        // find the corresponding realm_rd
        let g_rd = RmmUtil::find_lock_granule(rd_addr, GranuleState::GranuleStateRd);
        match g_rd {
            Ok(rd_id) => {
                ret = TableUtil::data_create_unknown(rd_id, data_addr, map_addr, data_id).unwrap_or_else(|err| {
                    crate::println!("ERROR: smc_data_create_unknow is failed: {:?}", err);
                    pack_return_code(err, 0)
                });                
            }
            Err(err) => {
                crate::println!("ERROR: smc_data_create: find_lock_granule rd granule is failed: {:?}", err);
                ret = pack_return_code(err, 1);
            }
        }
        GranuleUtil::unlock_granule(data_id);

        return ret;
    }

    pub fn smc_data_create_shared(data_addr: usize, rd_addr: usize, map_addr: usize) -> usize{
        let mut ret:usize = 0;
        // crate::println!("smc_data_create_shared data_addr: {:x}, rd_addr: {:x}, map_addr {:x}", data_addr, rd_addr, map_addr);
        // find an unused granule for src_data
        let data_id = RmmUtil::find_lock_unused_granule(data_addr, GranuleState::GranuleStateDelegated).unwrap_or_else(|err| {
            crate::println!("ERROR: smc_data_create_shared is failed: {:?}", err);
            ret = pack_return_code(err, 0);
            0
        });

        if ret != 0 {return ret;}

        // find the corresponding realm_rd
        let g_rd = RmmUtil::find_lock_granule(rd_addr, GranuleState::GranuleStateRd);
        match g_rd {
            Ok(rd_id) => {
                ret = TableUtil::data_create_shared(rd_id, data_addr, map_addr, data_id).unwrap_or_else(|err| {
                    crate::println!("ERROR: smc_data_create_shared is failed: {:?}", err);
                    pack_return_code(err, 0)
                });                
            }
            Err(err) => {
                crate::println!("ERROR: smc_data_create_shared: find_lock_granule rd granule is failed: {:?}", err);
                ret = pack_return_code(err, 1);
            }
        }
        GranuleUtil::unlock_granule(data_id);

        return ret;
    }

    pub fn smc_data_dispose(rd_addr: usize, rec_addr: usize) -> usize{
        let mut ret:usize = 0;

        return ret;
    }

    pub fn smc_rec_create(rec_addr: usize, rd_addr: usize, mpidr: usize, rec_params_addr: usize) -> usize {
        let mut ret:usize = 0;
        let g_rec_params = GranuleUtil::find_granule(rec_params_addr);

        match g_rec_params {
            Ok(rec_params) => {
                // find an unused granule for src_data
                let rec_id = RmmUtil::find_lock_unused_granule(rec_addr, GranuleState::GranuleStateDelegated).unwrap_or_else(|err| {
                    crate::println!("ERROR: smc_rec_create is failed: {:?}", err);
                    ret = pack_return_code(err, 1);
                    0
                });

                if ret != 0 {return ret;}

                let g_rd = RmmUtil::find_lock_granule(rd_addr, GranuleState::GranuleStateRd);
                match g_rd {
                    Ok(rd_id) => {
                        // unlock the rd in rec_create
                        ret = RecUtil::rec_create(rd_id, rec_id, mpidr, rec_params.id);
                    }
                    Err(err) => {
                        ret = pack_return_code(err, 2);
                    }
                }

                GranuleUtil::unlock_granule(rec_id);
            }
            Err(err) => {
                ret = pack_return_code(err, 0);
            }
        }

        return ret;
    }

    pub fn smc_rec_destroy(rec_addr: usize) -> usize {
        let mut ret:usize = 0;
        // We delete the rec granule before
        let rec_id = RmmUtil::find_lock_unused_granule(rec_addr, GranuleState::GranuleStateRec).unwrap_or_else(|err| {
            crate::println!("ERROR: smc_rec_destroy is failed: {:?}", err);
            ret = pack_return_code(err, 0);
            0
        });

        if ret != 0 {return ret;}
        RecUtil::rec_destroy(rec_id);

        return ret;
    }

    pub fn smc_rec_run(rec_addr: usize, rec_run_addr: usize) -> usize {
        let mut ret:usize = 0;
        let g_rec_run = GranuleUtil::find_granule(rec_run_addr);
        
        match g_rec_run {
            Ok(rec_run) => {
                let rec_id = RmmUtil::find_lock_unused_granule(rec_addr, GranuleState::GranuleStateDelegated).unwrap_or_else(|err| {
                    crate::println!("ERROR: smc_rec_run is failed: {:?}", err);
                    ret = pack_return_code(err, 1);
                    0
                });

                if ret != 0 {return ret;}
                // need to unlock rec here
                ret = RecUtil::rec_run(rec_id, rec_run.id);

            }
            Err(err) => {
                ret = pack_return_code(err, 0);
            }
        }
        return ret;
    }

    pub fn smc_data_create_unknown_compact(rd_addr: usize, map_addr: usize,
                                           segment_cnt: usize, args_addr: usize) -> usize {
        crate::println!("smc_data_create_unknown_compact called");
        let mut ret: usize = 0;
        let mut cur_map_addr = map_addr;
        let g_args = GranuleUtil::find_lock_granule(args_addr, GranuleState::GranuleStateNs);
        
        match g_args {
            Ok(args) => {
                let args_buf = granule_map_with_id(args.id as usize, BufferSlot::SLOT_NS);
                (0..segment_cnt).for_each(|idx| {
                    let mut delegate_seg;
                    unsafe {
                        delegate_seg = *(args_buf as *const addr_segment).offset(idx as isize);
                    }    
                    // crate::println!("current target_addr_seg:[{:x},{:x}]", delegate_seg.base, delegate_seg.base + (delegate_seg.size as usize - 1) * PAGE_SIZE);
                    let mut delegate_addr = delegate_seg.base;
                    unsafe {
                        ret = trp_smc(set_smc_args(RMM_GTSI_DELEGATE_COMPACT,
                                                   delegate_addr,
                                                   delegate_seg.size as usize,
                                                   0, 0, 0, 0, 0));
                        if ret != 0 { return; }
                    }
                    (0..delegate_seg.size).for_each(|_| {
                        ret = Self::smc_granule_delegate_without_clear(delegate_addr);
                        if ret != 0 { return; }
                        ret = Self::smc_data_create_unknown(delegate_addr, rd_addr, cur_map_addr);
                        if ret != 0 { return; }
                        ret = Self::smc_data_map(rd_addr, cur_map_addr);
                        if ret != 0 { return; }
                        cur_map_addr = cur_map_addr + PAGE_SIZE;
                        delegate_addr = delegate_addr + PAGE_SIZE;
                    });
                    if ret != 0 { return; }
                });
                unsafe {
                    buffer_unmap(args_buf);
                }
                GranuleUtil::unlock_granule(args.id);
            }
            Err(err) => {
                ret = pack_return_code(err, 0);
            }
        }
        return ret;
    }

    pub fn smc_data_create_shared_compact(rd_addr: usize, map_addr: usize,
                                          segment_cnt: usize, args_addr: usize) -> usize {
        crate::println!("smc_data_create_shared_compact called");
        let mut ret: usize = 0;
        let mut cur_map_addr = map_addr;
        let g_args = GranuleUtil::find_lock_granule(args_addr, GranuleState::GranuleStateNs);
        
        match g_args {
            Ok(args) => {
                let args_buf = granule_map_with_id(args.id as usize, BufferSlot::SLOT_NS);
                (0..segment_cnt).for_each(|idx| {
                    let mut ns_seg;
                    unsafe {
                        ns_seg = *(args_buf as *const addr_segment).offset(idx as isize);
                    }    
                    // crate::println!("current target_addr_seg:[{:x},{:x}] ns_seg.size:{:x}", ns_seg.base, ns_seg.base + (ns_seg.size as usize - 1) * PAGE_SIZE, ns_seg.size);
                    let mut ns_addr = ns_seg.base;
                    (0..ns_seg.size).for_each(|_| {
                        ret = Self::smc_data_create_shared(ns_addr, rd_addr, cur_map_addr);
                        if ret != 0 { return; }
                        ret = Self::smc_data_map(rd_addr, cur_map_addr);
                        if ret != 0 { return; }
                        cur_map_addr = cur_map_addr + PAGE_SIZE;
                        ns_addr = ns_addr + PAGE_SIZE;
                    });
                    if ret != 0 { return; }
                });
                unsafe {
                    buffer_unmap(args_buf);
                }
                GranuleUtil::unlock_granule(args.id);
            }
            Err(err) => {
                ret = pack_return_code(err, 0);
            }
        }
        return ret;
    }

    pub fn smc_data_create_compact(rd_addr: usize, map_addr: usize,
                                   src_seg_cnt: usize, src_args_addr: usize,
                                   dst_seg_cnt: usize, dst_args_addr: usize) -> usize {
        crate::println!("smc_data_create_compact called");
        let mut ret: usize = 0;
        let mut cur_map_addr = map_addr;
        let g_src_args = GranuleUtil::find_lock_granule(src_args_addr, GranuleState::GranuleStateNs);
        let g_dst_args = GranuleUtil::find_lock_granule(dst_args_addr, GranuleState::GranuleStateNs);

        match (g_src_args, g_dst_args) {
            (Ok(src_args), Ok(dst_args)) => {
                let src_args_temp_buf = granule_map_with_id(src_args.id as usize, BufferSlot::SLOT_NS);
                let src_args_buf: Vec<u8> = Vec::with_capacity(PAGE_SIZE);
                unsafe {
                    let read_ret = ns_buffer_read_data(BufferSlot::SLOT_NS, src_args_buf.as_ptr() as usize);
                    buffer_unmap(src_args_temp_buf);
                }
                let dst_args_temp_buf = granule_map_with_id(dst_args.id as usize, BufferSlot::SLOT_NS);
                let dst_args_buf: Vec<u8> = Vec::with_capacity(PAGE_SIZE);
                unsafe {
                    let read_ret = ns_buffer_read_data(BufferSlot::SLOT_NS, dst_args_buf.as_ptr() as usize);
                    buffer_unmap(dst_args_temp_buf);
                }
                let mut src_seg;
                let mut dst_seg;
                let mut src_seg_ptr: isize = 0;
                let mut dst_seg_ptr: isize = 0;
                let mut src_seg_pos: usize = 0;
                let mut dst_seg_pos: usize = 0;
                unsafe {
                    src_seg = *(src_args_buf.as_ptr() as *const addr_segment);
                    dst_seg = *(dst_args_buf.as_ptr() as *const addr_segment); 
                }
                // crate::println!("current src_seg:[{:x}, {:x}]", src_seg.base, src_seg.base + (src_seg.size as usize - 1) * PAGE_SIZE);
                // crate::println!("current dst_seg:[{:x}, {:x}]", dst_seg.base, dst_seg.base + (dst_seg.size as usize - 1) * PAGE_SIZE);
                let mut dst_addr = dst_seg.base;
                let mut src_addr = src_seg.base;
                unsafe {
                    ret = trp_smc(set_smc_args(RMM_GTSI_DELEGATE_COMPACT,
                                               dst_addr,
                                               dst_seg.size as usize,
                                               0, 0, 0, 0, 0));
                }
                if ret != 0 { return ret; }
                while true {
                    ret = Self::smc_granule_delegate_without_clear(dst_addr);
                    if ret != 0 { return ret; }
                    ret = Self::smc_data_create(dst_addr, rd_addr, src_addr, cur_map_addr);
                    if ret != 0 { return ret; }
                    ret = Self::smc_data_map(rd_addr, cur_map_addr);
                    if ret != 0 { return ret; }

                    dst_addr += PAGE_SIZE;
                    src_addr += PAGE_SIZE;
                    cur_map_addr = cur_map_addr + PAGE_SIZE;

                    src_seg_pos += 1;
                    if src_seg_pos == src_seg.size as usize {
                        src_seg_ptr += 1;
                        if src_seg_cnt == src_seg_ptr as usize {
                            break;
                        }
                        unsafe {
                            src_seg = *(src_args_buf.as_ptr() as *const addr_segment).offset(src_seg_ptr);
                        }
                        src_seg_pos = 0;
                        src_addr = src_seg.base;
                    }
                    dst_seg_pos += 1;
                    if dst_seg_pos == dst_seg.size as usize {
                        dst_seg_ptr += 1;
                        if dst_seg_cnt == dst_seg_ptr as usize {
                            break;
                        }
                        unsafe {
                            dst_seg = *(dst_args_buf.as_ptr() as *const addr_segment).offset(dst_seg_ptr);
                        }
                        dst_seg_pos = 0;
                        dst_addr = dst_seg.base;
                        unsafe {
                            ret = trp_smc(set_smc_args(RMM_GTSI_DELEGATE_COMPACT,
                                                       dst_addr,
                                                       dst_seg.size as usize,
                                                       0, 0, 0, 0, 0));
                        }
                        if ret != 0 { return ret; }
                    }
                }
                GranuleUtil::unlock_granule(src_args.id);
                GranuleUtil::unlock_granule(dst_args.id);
            }
            _ => {
                ret = 1;
            }
        }
        return ret;
    }
    pub fn smc_data_destroy_compact(rd_addr: usize, map_addr: usize,
                                           segment_cnt: usize, args_addr: usize) -> usize {
        crate::println!("smc_data_destroy_compact called");
        let mut ret: usize = 0;
        let mut cur_map_addr = map_addr;
        let g_args = GranuleUtil::find_lock_granule(args_addr, GranuleState::GranuleStateNs);
        
        match g_args {
            Ok(args) => {
                let args_buf = granule_map_with_id(args.id as usize, BufferSlot::SLOT_NS);
                (0..segment_cnt).for_each(|idx| {
                    let mut delegate_seg;
                    unsafe {
                        delegate_seg = *(args_buf as *const addr_segment).offset(idx as isize);
                    }    
                    // crate::println!("current target_addr_seg:[{:x},{:x}]", delegate_seg.base, delegate_seg.base + (delegate_seg.size as usize - 1) * PAGE_SIZE);
                    let mut delegate_addr = delegate_seg.base;
                    (0..delegate_seg.size).for_each(|_| {
                        ret = Self::smc_data_unmap(rd_addr, cur_map_addr);
                        if ret != 0 { return; }
                        ret = Self::smc_data_destroy(cur_map_addr, rd_addr);
                        if ret != 0 { return; }
                        ret = Self::smc_granule_undelegate(delegate_addr);
                        if ret != 0 { return; }
                        cur_map_addr = cur_map_addr + PAGE_SIZE;
                        delegate_addr = delegate_addr + PAGE_SIZE;
                    });
                    unsafe {
                        ret = trp_smc(set_smc_args(RMM_GTSI_UNDELEGATE_COMPACT,
                                                   delegate_seg.base,
                                                   delegate_seg.size as usize,
                                                   0, 0, 0, 0, 0));
                    }
                    if ret != 0 { return; }
                });
                unsafe {
                    buffer_unmap(args_buf);
                }
                GranuleUtil::unlock_granule(args.id);
            }
            Err(err) => {
                ret = pack_return_code(err, 0);
            }
        }
        return ret;
    }

    pub fn smc_data_destroy_shared_compact(rd_addr: usize, map_addr: usize, size: usize) -> usize {
        crate::println!("smc_data_destroy_shared_compact called");
        let mut ret: usize = 0;
        let mut cur_map_addr = map_addr;
        (0..size).for_each(|_| {
            ret = Self::smc_data_unmap(rd_addr, cur_map_addr);
            if ret != 0 { return; }
            ret = Self::smc_data_destroy(cur_map_addr, rd_addr);
            if ret != 0 { return; }
            cur_map_addr = cur_map_addr + PAGE_SIZE;
        });

        return ret;
    }

    pub fn smc_table_create_compact(rd_addr: usize, args_addr: usize,
                                    size: usize) -> usize {
        crate::println!("smc_table_create_compact called");
        let mut ret: usize = 0;
        let g_args = GranuleUtil::find_lock_granule(args_addr, GranuleState::GranuleStateNs);
        
        match g_args {
            Ok(args) => {
                let args_buf = granule_map_with_id(args.id as usize, BufferSlot::SLOT_NS);
                (0..size).for_each(|idx| {
                    let mut args_triple;
                    unsafe {
                        args_triple = *(args_buf as *const triple).offset(idx as isize);
                    }
                    unsafe {
                        ret = trp_smc(set_smc_args(RMM_GTSI_DELEGATE, args_triple.args1, 0, 0, 0, 0, 0, 0));
                    }
                    if ret != 0 { return; }
                    ret = Self::smc_granule_delegate(args_triple.args1);
                    if ret != 0 { return; }
                    ret = Self::smc_table_create(args_triple.args1, rd_addr, args_triple.args2, args_triple.args3);
                    if ret != 0 { return; }
                });
                unsafe {
                    buffer_unmap(args_buf);
                }
                GranuleUtil::unlock_granule(args.id);
            }
            Err(err) => {
                ret = pack_return_code(err, 0);
            }
        }
        return ret;
    }

    // without memset to zero!
    pub fn smc_granule_delegate_compact(args_addr: usize, segment_cnt: usize) -> usize {
        crate::println!("smc_granule_delegate_compact called");
        let mut ret: usize = 0;
        let g_args = GranuleUtil::find_lock_granule(args_addr, GranuleState::GranuleStateNs);
        
        match g_args {
            Ok(args) => {
                let args_buf = granule_map_with_id(args.id as usize, BufferSlot::SLOT_NS);
                (0..segment_cnt).for_each(|idx| {
                    let mut delegate_seg;
                    unsafe {
                        delegate_seg = *(args_buf as *const addr_segment).offset(idx as isize);
                    }    
                    let mut delegate_addr = delegate_seg.base;
                    unsafe {
                        ret = trp_smc(set_smc_args(RMM_GTSI_DELEGATE_COMPACT,
                                                   delegate_addr,
                                                   delegate_seg.size as usize,
                                                   0, 0, 0, 0, 0));
                        if ret != 0 { return; }
                    }
                    (0..delegate_seg.size).for_each(|_| {
                        ret = Self::smc_granule_delegate_without_clear(delegate_addr);
                        if ret != 0 { return; }
                        delegate_addr = delegate_addr + PAGE_SIZE;
                    });
                    
                    if ret != 0 { return; }
                });
                unsafe {
                    buffer_unmap(args_buf);
                }
                GranuleUtil::unlock_granule(args.id);
            }
            Err(err) => {
                ret = pack_return_code(err, 0);
            }
        }
        return ret;
    }

}

unsafe impl Sync for RmmSMC {}
unsafe impl Send for RmmSMC {}