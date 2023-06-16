use crate::rmm::rmm_util::RmmUtil;

use crate::rmm::granule_util::{
    Granule, 
    GranuleState,
    ErrorStatus,
    GranuleUtil,
    NR_GRANULES};

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

impl RmmSMC {
    pub fn smc_granule_delegate(addr: usize) -> usize {
        crate::dprintln!("Debug: smc_granule_delegate addr: {:x}", addr);
        let g = GranuleUtil::find_lock_granule(addr, GranuleState::GranuleStateNs);
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
        crate::dprintln!("Debug: smc_granule_delegate rd_addr: {:x}", rd_addr);
        if RmmUtil::rmm_para_parser(rlm_para_addr) == ErrorStatus::StatusSuccess {
            if RmmUtil::validate_realm_params() == 0 {
                let rtt_addr =  get_realm_params_rtt_addr();
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
        crate::println!("smc_data_create_shared data_addr: {:x}, rd_addr: {:x}, map_addr {:x}", data_addr, rd_addr, map_addr);
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

}

unsafe impl Sync for RmmSMC {}
unsafe impl Send for RmmSMC {}