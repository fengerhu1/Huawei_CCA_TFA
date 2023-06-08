pub const REC_CREATE_NR_GPRS: usize = 8;
pub const REC_RUN_HVC_NR_GPRS: usize = 7;
pub const REC_RUN_SMC_NR_GPRS: usize = 9;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RealmParams {
	pub par_base: usize,
	pub par_size: usize,
	pub rec_list_addr: usize, /* rec list granule address */
	pub table_addr: usize, /* Translation Table Base Granule address */
	pub measurement_algo: usize, /* Measurement algorithm */
}

impl RealmParams {
    pub fn new() -> Self {
        RealmParams { 
            par_base: 0,
			par_size: 0,
			rec_list_addr: 0, /* rec list granule address */
			table_addr: 0, /* Translation Table Base Granule address */
			measurement_algo: 0, /* Measurement algorithm */
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RecParams {
	pub gprs: [usize; REC_CREATE_NR_GPRS],
	pub pc: usize,
	pub flags: usize, // REC_RUNNABLE_FLAG, RUNNABLE == 1
}

impl RecParams {
    pub fn new() -> Self {
        RecParams { 
            gprs: [0; REC_CREATE_NR_GPRS],
			pc: 0,
			flags: 0,
		}
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RecRun {
	/* output values*/
	pub exit_reason: usize,
	pub esr: usize,
	pub far: usize,
	pub hpfar: usize,
	emulated_write_val: usize,
	pub gprs: [usize; REC_RUN_SMC_NR_GPRS],
	disposed_addr: usize,
	/* input values */
	pub is_emulated_mmio: usize,
	pub emulated_read_val: usize,
	target_rec: usize,
}

impl RecRun {
    pub fn new() -> Self {
        RecRun { 
            exit_reason: 0,
			esr: 0,
			far: 0,
			hpfar: 0,
			emulated_write_val: 0,
			gprs: [0; REC_RUN_SMC_NR_GPRS],
			disposed_addr: 0,
			/* input values */
			is_emulated_mmio: 0,
			emulated_read_val: 0,
			target_rec: 0,
		}
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SmcResult {
	x0: usize,
	x1: usize,
	x2: usize,
	x3: usize,
}

impl SmcResult {
    pub fn new() -> Self {
        SmcResult { 
            x0: 0,
            x1: 0,
            x2: 0,
            x3: 0,
        }
    }
}