#![no_std]
#![feature(asm)]
#![feature(const_mut_refs)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(const_option)]
#![feature(core_intrinsics)]

use core::panic::PanicInfo;
use core::fmt;
use crate::io::Write;

#[panic_handler]
fn panic(_info: &PanicInfo)-> !{
    loop{}
}

extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use alloc::{
    vec::Vec,
    boxed::Box,
};
// mod slab;
// mod util;
mod rmm;
pub mod io;
pub mod driver;
pub mod error;
pub mod allocator;
pub mod logger;
pub mod r#macro;

#[macro_use]
extern crate log;
// mod virt;
// use crate::slab::SlabAllocator;

/************************************************
 * Implement the stdout for Rust-RMM
 * Redirect Rust stdout to the C print string
 ************************************************/
struct Stdout;
impl fmt::Write for Stdout {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            print_string(to_c_str(s));
        }
        Ok(())
    }
}

#[inline]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    Stdout.write_fmt(args).unwrap();
}

// extern "C"{
//     pub fn print_info(format: *const u8, ...);
// }

extern "C"{
    pub fn print_string(string: *const u8);
}

pub fn to_c_str(s: &str)-> *const u8
{
    s as *const str as *const u8
}

use crate::rmm::rmm_smc::RmmSMC;
use crate::rmm::granule_util::GranuleUtil;
// use crate::rmm::rmm_smc::CALL_LOCK;
#[no_mangle]
pub extern "C" fn smc_realm_create(rd_addr: usize, rlm_para_addr: usize) -> usize {
    let ret = RmmSMC::smc_realm_create(rd_addr, rlm_para_addr);
    crate::dprintln!("smc_realm_create is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_realm_destroy(rd_addr: usize) -> usize {
    let ret = RmmSMC::smc_realm_destroy(rd_addr);
    crate::dprintln!("smc_realm_destroy is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_realm_activate(rd_addr: usize) -> usize {
    let ret = RmmSMC::smc_realm_activate(rd_addr);
    crate::dprintln!("smc_realm_activate is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_table_create(rtt_addr: usize, rd_addr: usize, map_addr: usize,
    level: usize) -> usize {
    let ret = RmmSMC::smc_table_create(rtt_addr, rd_addr, map_addr, level);
    crate::dprintln!("smc_table_create is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_table_create_compact(rd_addr: usize, args_addr: usize,
    size: usize) -> usize {
    let ret = RmmSMC::smc_table_create_compact(rd_addr, args_addr, size);
    crate::dprintln!("smc_table_create is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_table_destroy(rtt_addr: usize, rd_addr: usize, map_addr: usize,
    level: usize) -> usize {
    let ret = RmmSMC::smc_table_destroy(rtt_addr, rd_addr, map_addr, level);
    crate::dprintln!("smc_table_destroy is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_map(rd_addr: usize, map_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_map(rd_addr, map_addr);
    crate::dprintln!("smc_data_map is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_unmap(rd_addr: usize, map_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_unmap(rd_addr, map_addr);
    crate::dprintln!("smc_data_unmap is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_create(data_addr: usize, rd_addr: usize,
                                  src_addr: usize, map_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_create(data_addr, rd_addr, src_addr, map_addr);
    crate::dprintln!("smc_data_create is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_create_compact(rd_addr: usize, map_addr: usize,
                                          src_seg_cnt: usize, src_args_addr: usize,
                                          dst_seg_cnt: usize, dst_args_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_create_compact(rd_addr, map_addr,
                                              src_seg_cnt, src_args_addr,
                                              dst_seg_cnt, dst_args_addr);
    crate::dprintln!("smc_data_create_compact is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_destroy(map_addr: usize, rd_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_destroy(map_addr, rd_addr);
    crate::dprintln!("smc_data_destroy is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_destroy_compact(rd_addr: usize,
                                           map_addr: usize,
                                           segment_cnt: usize,
                                           args_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_destroy_compact(rd_addr, map_addr, segment_cnt, args_addr);
    crate::dprintln!("smc_data_destroy is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_destroy_shared_compact(rd_addr: usize,
                                                  map_addr: usize,
                                                  size: usize) -> usize {
    let ret = RmmSMC::smc_data_destroy_shared_compact(rd_addr, map_addr, size);
    crate::dprintln!("smc_data_destroy is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_create_unknown(data_addr: usize,
                                          rd_addr: usize,
                                          map_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_create_unknown(data_addr, rd_addr, map_addr);
    crate::dprintln!("smc_data_create_unknown is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_create_unknown_compact(rd_addr: usize, map_addr: usize,
                                                  size: usize, args_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_create_unknown_compact(rd_addr, map_addr, size, args_addr);
    crate::dprintln!("smc_data_create_unknown_compacted is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_create_shared_compact(rd_addr: usize,
                                                 map_addr: usize,
                                                 size: usize,
                                                 args_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_create_shared_compact(rd_addr, map_addr, size, args_addr);
    crate::dprintln!("smc_data_create_shared_compact is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_dispose(rd_addr: usize, rec_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_dispose(rd_addr, rec_addr);
    crate::dprintln!("smc_data_dispose is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_rec_create(rec_addr: usize, rd_addr: usize,
                                 mpidr: usize, rec_params_addr: usize) -> usize {
    let ret = RmmSMC::smc_rec_create(rec_addr, rd_addr, mpidr, rec_params_addr);
    crate::dprintln!("smc_rec_create is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_rec_destroy(rec_addr: usize) -> usize {
    let ret = RmmSMC::smc_rec_destroy(rec_addr);
    crate::dprintln!("smc_rec_destroy is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_rec_run(rec_addr: usize, rec_run_addr: usize) -> usize {
    let ret = RmmSMC::smc_rec_run(rec_addr, rec_run_addr);
    crate::dprintln!("smc_rec_run is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_granule_delegate(addr: usize) -> usize {
    let ret = RmmSMC::smc_granule_delegate(addr);
    crate::dprintln!("smc_granule_delegate addr: {:x}", addr);
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_granule_delegate_compact(args_addr: usize, segment_cnt: usize) -> usize {
    let ret = RmmSMC::smc_granule_delegate_compact(args_addr, segment_cnt);
    crate::dprintln!("smc_granule_delegate_compact finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_granule_undelegate(addr: usize) -> usize {
    let ret = RmmSMC::smc_granule_undelegate(addr);
    crate::dprintln!("smc_granule_undelegate addr: {:x}", addr);
    return ret;
}

#[no_mangle]
pub extern "C" fn smc_data_create_shared(data_addr: usize,
                                         rd_addr: usize,
                                         map_addr: usize) -> usize {
    let ret = RmmSMC::smc_data_create_shared(data_addr, rd_addr, map_addr);
    crate::dprintln!("smc_data_create_shared is finished");
    return ret;
}

#[no_mangle]
pub extern "C" fn rust_test_alloc() {
    let mut test:Vec<usize> =Vec::new();
    test.push(1);
}

#[no_mangle]
pub extern "C" fn allocator_init() {
    unsafe {
        crate::allocator::init();
    }
}

#[no_mangle]
pub extern "C" fn rust_printf() {
    crate::rmm_println!("rust_printf {}", 1);
}

use log::LevelFilter;
use io::stdout;

#[no_mangle]
pub extern "C" fn init_console() {
    let _ = stdout().attach(crate::driver::uart::pl011::device());
    // logger::register_global_logger(LevelFilter::Info); // Control log level
    // crate::rmm_println!("Initialized the console!");
    crate::println!("Initialized the console!");
    // let mut test:Vec<usize> =Vec::new();
    // test.push(1);
    // crate::dprintln!("test address {:p}", &test[0]);
}

#[no_mangle]
pub extern "C" fn init_granule() {
    GranuleUtil::init_granule();
}


#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> !{
    // crate::dprintln!("allocation error {:?}", layout);
    panic!("allocation error {:?}", layout);
}