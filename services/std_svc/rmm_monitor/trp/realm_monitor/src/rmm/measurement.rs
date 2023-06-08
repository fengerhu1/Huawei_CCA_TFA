use crate::rmm::realm_util::{
    Rd,
};

use crate::rmm::granule_util::{
    Granule, 
    GranuleUtil,

    BufferSlot,
    GranuleState,
    ErrorStatus,
};

use crate::rmm::rmm_util::{
    granule_map_with_id_state,
};

use crate::io::Write;
use sha2::{Sha256, Sha512, Digest};

pub const SHA256: i32 = 0;

extern "C"{
    // map the physical address into a tmp va per cpu core
    pub fn buffer_map(slot: BufferSlot, addr: usize, ns: bool) -> usize;
    // unmap the tmp va
    pub fn buffer_unmap(buf: usize);
    // set the memory range to zero
    pub fn memzero(buf: usize, size: usize);
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum MeasurementAlgo {
    MeasurementAlgoZero = 0,
	MeasurementAlgoSha256,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct mbedtls_sha256_context {
    private_total: [u32; 2],         
    private_state: [u32; 8],      
    private_buffer: [u8; 64],  
    private_is224: i32,
}

#[derive(Clone)]
#[repr(C)]
pub struct measurement_ctx {
	pub c: Sha256,
	pub measurement_algo: MeasurementAlgo,
}

impl measurement_ctx {
    pub fn new() -> measurement_ctx {
        measurement_ctx {
            c: Sha256::new(),
            measurement_algo: MeasurementAlgo::MeasurementAlgoZero,
        }
    }
}

extern "C"{
    pub fn mbedtls_sha256_init(c: *mut mbedtls_sha256_context);
    pub fn mbedtls_sha256_starts_ret(c: *mut mbedtls_sha256_context, is224: i32) -> i32;
}

pub fn measurement_start(rd: &mut Rd) {
    let ctx = &mut (rd.ctx);
    match ctx.measurement_algo {
        MeasurementAlgo::MeasurementAlgoZero => return,
        MeasurementAlgo::MeasurementAlgoSha256 => {
            ctx.c = Sha256::new();
            // ctx.c.update(b"hello world");
            return;
        }
    };
}

pub fn measurement_finish(rd: &mut Rd) {
    let ctx = &(rd.ctx);
    match ctx.measurement_algo {
        MeasurementAlgo::MeasurementAlgoZero => {
            return;
        },
        MeasurementAlgo::MeasurementAlgoSha256 => {
            rd.measurement = rd.clone().ctx.c.finalize().as_slice().try_into().expect("Wrong length");
            crate::println!("DEBUG: measurement digest {:x?}", rd.measurement);
            return;
        }
    };
}

pub fn measurement_extend_data(rd: &mut Rd, data: &mut [u8]) {
    let ctx = &mut (rd.ctx);
    match ctx.measurement_algo {
        MeasurementAlgo::MeasurementAlgoZero => {
            return;
        },
        MeasurementAlgo::MeasurementAlgoSha256 => {
            // crate::println!("DEBUG: measurement update {:x?}", data);
            ctx.c.update(data);
            return;
        }
    };
}
