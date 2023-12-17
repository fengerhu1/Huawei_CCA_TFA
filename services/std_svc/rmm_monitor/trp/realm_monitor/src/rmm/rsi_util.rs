use crate::rmm::rvic_util::{
    RecRvicState,
	INTID_VTIMER_EL1,
	INTID_PTIMER_EL1,
};

use core::arch::asm;
use crate::io::Write;

use crate::rmm::granule_util::{
    Granule,
    ErrorStatus,
    BufferSlot,
    NR_GRANULES,
    GRANULE_SHIFT,
    GRANULE_SIZE,
    GranuleUtil,
    GranuleState,
	MEM0_PHYS,
};

use crate::rmm::smc_rmi::{
	RecParams,
	RecRun,
	REC_RUN_HVC_NR_GPRS,
    REC_RUN_SMC_NR_GPRS
};

use crate::rmm::rec_util::{
    Rec,
    set_rec_run_exit_reason,
    EXIT_REASON_RSI_MMAP,
    EXIT_REASON_RSI_UNMAP,
    EXIT_REASON_RSI_SYSCALL,
};

use crate::rmm::rmm_smc::{
	pack_return_code,
};

use crate::rmm::realm_util::{
    Rd,
};

use crate::rmm::context_switch::{
	save_ns_state_sysreg_state,
	restore_ns_state_sysreg_state,
	restore_sysreg_state,
	save_sysreg_state,
};

use crate::rmm::sysreg::{
    ESR_EL2_EC_FP,
    ESR_EL2_EC_MASK,
    SyscallNumber,
};

use crate::rmm::rmm_util::{
    granule_map_with_id,
    granule_map_with_id_state,
    buffer_unmap,
    granule_map_zero,
    RmmUtil,
};

use crate::rmm::table_util::{
    table_walk_lock_unlock,
    entry_to_phys,
    RTT_PAGE_LEVEL,
    Page,
};

use crate::rmm::abs_util::{
    VPERCPU_LOCK,
};

use crate::rmm::smc_rmi::{
	REC_CREATE_NR_GPRS,
};

pub const SMC_RSI_BASE: usize =		0xC8000000;
pub const SMC_RSI_CALL_MASK: usize =	0xFFFF;

/* The version number of the RSI implementation.  Increase this whenever the
 * binary format or semantics of the SMC calls change */
pub const RSI_ABI_VERSION: usize =			1;

pub const SMC_RSI_ABI_VERSION: usize =			(SMC_RSI_BASE + 0);
//FIXME: Realm exit flag is temporary
pub const SMC_RSI_ABI_EXIT: usize = (SMC_RSI_BASE + 100);
pub const SMC_RSI_ABI_OCALL: usize = (SMC_RSI_BASE + 101);
pub const SMC_RSI_ABI_SYSCALL: usize = (SMC_RSI_BASE + 102);

/*
 * arg0 == IPA address
 */
pub const SMC_RSI_IPA_DISPOSE: usize =		(SMC_RSI_BASE + 1);

pub const SMC_RSI_REALM_GET_MEASUREMENT: usize =	(SMC_RSI_BASE + 2);

pub const OCALL_SYS_MMAP: usize =   1;
pub const OCALL_SYS_UNMAP: usize =   2;
pub const OCALL_SYS_WRITE: usize =   3;
pub const OCALL_SYS_ABORT: usize =   4;

extern "C"{
    pub fn realm_printf(output: *mut usize);
    pub fn read_cpacr_el12() ->usize;
    pub fn set_cpacr_el12(val: usize);
}

pub fn handle_rsi_realm_getmeasurement(rec: &mut Rec) -> usize{
    let rd_ptr = granule_map_with_id(rec.realm_info.g_rd as usize, BufferSlot::SLOT_RD) as *mut Rd;
	let rd = unsafe {&mut (*rd_ptr)};
    let usize_measurement_ptr = &(rd.measurement) as *const [u8; 32] as *const [usize; 4];
    rec.regs[1] = unsafe {(*usize_measurement_ptr)[0]};
    rec.regs[2] = unsafe {(*usize_measurement_ptr)[1]};
    rec.regs[3] = unsafe {(*usize_measurement_ptr)[2]};
    rec.regs[4] = unsafe {(*usize_measurement_ptr)[3]};

    unsafe {buffer_unmap(rd_ptr as usize);}
    return 0; //return value when realm resume
}

pub fn handle_realm_rsi(rec: &mut Rec) -> bool {
    // FIXME: handle the rsi
    let arg1 = rec.regs[1];
	let function_id = rec.regs[0];
	let arg2 = rec.regs[2];
	let arg3 = rec.regs[3];

    match function_id {
        SMC_RSI_ABI_VERSION => {
            rec.regs[1] = RSI_ABI_VERSION;
            rec.regs[0] = 0; // SUCCESS
            return true;
        }
        SMC_RSI_ABI_EXIT => {
            crate::println!("DEBUG: handle_realm_rsi: exit realm");
            return false;
        }
        SMC_RSI_REALM_GET_MEASUREMENT => {
            crate::println!("DEBUG: return the measurement");
            rec.regs[0] = handle_rsi_realm_getmeasurement(rec);
            return true;
        }
        SMC_RSI_ABI_SYSCALL => {
            crate::println!("DEBUG: syscall num {:}\n", rec.regs[8]);
            
            match SyscallNumber::from(rec.regs[8] as i32){
                SyscallNumber::__NR_brk | SyscallNumber::__NR_lseek | SyscallNumber::__NR_mmap 
                | SyscallNumber::__NR_openat | SyscallNumber::__NR_readv | SyscallNumber::__NR_writev
                | SyscallNumber::__NR_ioctl | SyscallNumber::__NR_close
                | SyscallNumber::__NR_clock_gettime | SyscallNumber::__NR_gettimeofday => {
                    let mut v_percpu_list = VPERCPU_LOCK.lock();
                    let mut i = 0;
                    while i < REC_RUN_SMC_NR_GPRS {
                        v_percpu_list[crate::cpuid!()].run.gprs[i] = rec.regs[i];
                        i+=1;
                    }
                    drop(v_percpu_list);
                    set_rec_run_exit_reason(EXIT_REASON_RSI_SYSCALL);
                    return false;
                }
                _ => {
                    for i in 0..9 {
                        crate::println!("ERROR: syscall reg {:} {:} {:x}", i, rec.regs[i], rec.regs[i]);
                    }

                    crate::println!("ERROR: unhandled syscall for {:?}", rec.regs[8]);
                    return false;
                }
            }
        }
        SMC_RSI_ABI_OCALL => {
            // let pc = crate::read_sysreg!(elr_el2);
            // crate::println!("DEBUG: trap pc is {:?}, OCALL id is {:?}", pc-4, arg1);

            match arg1 {
                OCALL_SYS_MMAP => {
                    // crate::println!("DEBUG: handle_realm_rsi: realm_mmap vaddr {:x}, size {:x}", arg2, arg3);
                    let mut i = 0;

                    let mut v_percpu_list = VPERCPU_LOCK.lock();

                    while i < REC_RUN_HVC_NR_GPRS {
                        v_percpu_list[crate::cpuid!()].run.gprs[i] = rec.regs[i];
                        i+=1;
                    }
                    drop(v_percpu_list);
                    set_rec_run_exit_reason(EXIT_REASON_RSI_MMAP);
                    return false;
                }
                OCALL_SYS_UNMAP => {
                    // crate::println!("DEBUG: handle_realm_rsi: realm_unmap vaddr {:x}, size {:x}", arg2, arg3);
                    let mut i = 0;
                    let mut v_percpu_list = VPERCPU_LOCK.lock();

                    while i < REC_RUN_HVC_NR_GPRS {
                        v_percpu_list[crate::cpuid!()].run.gprs[i] = rec.regs[i];
                        i+=1;
                    }
                    drop(v_percpu_list);

                    set_rec_run_exit_reason(EXIT_REASON_RSI_UNMAP);
                    return false;
                }
                OCALL_SYS_WRITE => { //Realm print
                    let mut ret = true;
                    crate::dprintln!("DEBUG: handle_realm_rsi: handle the realm print");
                    table_walk_lock_unlock(rec.realm_info.g_rd, arg2, RTT_PAGE_LEVEL);
                    let v_percpu_list = VPERCPU_LOCK.lock();
                    let g_llt_id = v_percpu_list[crate::cpuid!()].wi.g_llt_id;
                    let index = v_percpu_list[crate::cpuid!()].wi.index;
                    drop(v_percpu_list);

                    if g_llt_id == NR_GRANULES as u32 {
                        crate::println!("ERROR: handle_realm_rsi: realm print, g_llt_id is invalid");
                        GranuleUtil::unlock_granule(g_llt_id);
                        return false;
                    }
                    else {
                        let table_ptr = granule_map_with_id(g_llt_id as usize, BufferSlot::SLOT_RTT) as *mut Page;
                        let table = unsafe {&mut (*table_ptr)};
                        let entry = table.entry[index];

                        let g = GranuleUtil::find_granule(entry_to_phys(entry, 3));
                        let output_id; // the granule id of the output page 
                        match g {
                            Ok(granule) => {
                                output_id = granule.id;
                                let output_ptr = granule_map_with_id(output_id as usize, BufferSlot::SLOT_OUTPUT) as *mut usize;
                                unsafe {
                                    realm_printf(output_ptr);
                                }

                                unsafe {buffer_unmap(output_ptr as usize);}
                            }
                            Err(_) => {
                                ret = false;
                            }
                        }
            
                        unsafe {buffer_unmap(table_ptr as usize);}
                        GranuleUtil::unlock_granule(g_llt_id);
                    }
                    return ret;
                }
                OCALL_SYS_ABORT => { //ABORT handler
                    crate::println!("ERROR: handle_realm_rsi: realm abort, the error address is {:x}, esr is {:x} \n", arg2, arg3);
                    let ec = arg3 & ESR_EL2_EC_MASK;
                    if ec == ESR_EL2_EC_FP {
                        crate::println!("ERROR: handle_realm_rsi: realm abort, access FP or SIMD instruction and trap\n");   
                    }
                    return false;
                }
                other => {
                }
            }
            crate::dprintln!("DEBUG: handle_realm_rsi: realm ocall id {:x}", arg1);
            return false;
        } 
        _ => {
            crate::println!("ERROR: unhandled RSI call, function_id {:x}", function_id);
            return true;
        }
    }
    return true;
}