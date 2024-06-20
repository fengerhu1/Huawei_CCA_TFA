use crate::rmm::rvic_util::{
    RecRvicState,
	INTID_VTIMER_EL1,
	INTID_PTIMER_EL1,
	INTID_ARCHTIMER_EL1,
	INTID_UNKNOWN_EL1,
};

use crate::io::Write;

use crate::rmm::rsi_util::{
	handle_realm_rsi,
};

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
	MEM1_PHYS,
	MEM_PHYS_BASE,
};

use crate::rmm::smc_rmi::{
	RecParams,
	RecRun,
	REC_RUN_HVC_NR_GPRS,
	REC_RUN_SMC_NR_GPRS
};

use crate::rmm::rmm_smc::{
	pack_return_code,
};

use crate::rmm::realm_util::{
    Rd,
};

use crate::rmm::rvic_util::{
	Rvic,
};

use crate::rmm::context_switch::{
	save_ns_state_sysreg_state,
	restore_ns_state_sysreg_state,
	restore_sysreg_state,
	save_sysreg_state,
	get_rec_sysregs,
	set_rec_sysregs,
};

use crate::rmm::sysreg::{
	MPIDR_EL1_AFF0,
	MPIDR_EL1_AFF1,
	MPIDR_EL1_AFF2,
	MPIDR_EL1_AFF3,
	MPIDR_EL1_AFF0_SHIFT,
	MPIDR_EL1_AFF1_SHIFT,
	MPIDR_EL1_AFF2_SHIFT,
	MPIDR_EL1_AFF3_SHIFT,
	PSTATE_INIT,
	PMCR_EL0_RES1,
	SCTLR_EL1_FLAGS,
	MDSCR_EL1_TDCC_BIT,
	CNTHCTL_EL2_NO_TRAPS,
	TTBRx_EL2_BADDR_SHIFT,
	TTBRx_EL2_BADDR_WIDTH,
	HCR_FLAGS,
	VTCR_FLAGS,
	ESR_EL2_EC_MASK,
	ESR_EL2_EC_HVC,
	NOT_ICC_SRE_EL2_ENABLE,
	ICC_HPPIR1_EL1_INTID,
	ESR_EL2_EC_WFX,
	ESR_EL2_WFx_TI_BIT,
	ESR_EL2_xVC_IMM_MASK,
	ESR_EL2_EC_SMC,
	ESR_EL2_EC_SYSREG,
	ESR_EL2_EC_INST_ABORT,
	ESR_EL2_EC_DATA_ABORT,
	ESR_EL2_ABORT_SRT_MASK,
	ESR_EL2_ABORT_SRT_SHIFT,
	ESR_EL2_ABORT_ISV_BIT,
	ESR_EL2_ABORT_SAS_MASK,
	ESR_EL2_ABORT_SAS_SHIFT,
	ESR_EL2_ABORT_SAS_BYTE_VAL,
	ESR_EL2_ABORT_SAS_HWORD_VAL,
	ESR_EL2_ABORT_SAS_WORD_VAL,
	ESR_EL2_ABORT_SAS_DWORD_VAL,
	ESR_EL2_ABORT_SSE_BIT,
	ESR_EL2_ABORT_SF_BIT,
	ESR_EL2_ABORT_WNR_BIT,
	CNTx_CTL_IMASK,
	CNTx_CTL_ENABLE,
	CNTx_CTL_ISTATUS,
	CNTHCTL_EL2_EL1PCTEN,
	CNTHCTL_EL2_EL1PTEN,
	NOT_CNTHCTL_EL2_EL1PTEN,
	CNTHCTL_EL2_EL1TVT,
	NOT_CNTHCTL_EL2_EL1TVT,
	CNTHCTL_EL2_EL1TVCT,
	RVIC_BITMAP_ULS,
	INTID_SPURIOUS,
	HCR_VI,
	BITS_PER_UL,
};

use crate::rmm::rmm_util::{
    granule_map_with_id,
    granule_map_with_id_state,
    buffer_unmap,
    granule_map_zero,
    RmmUtil,
};

use crate::rmm::abs_util::{
    VPERCPU_LOCK,
};

use crate::rmm::smc_rmi::{
	REC_CREATE_NR_GPRS,
};

use crate::rmm::measurement::{
	measurement_extend_data,
};

const NR_SYSREG_STATE: usize = 39;
pub const NR_NS_STATE: usize = 40;
const MAX_NUM_RECS: usize = 512;

const ARM_EXCEPTION_SYNC_LEL:u32 =	0;
const ARM_EXCEPTION_IRQ_LEL:u32 =	1;
const ARM_EXCEPTION_FIQ_LEL:u32 =	2;

const EXIT_REASON_SYNC: usize =			0;
const EXIT_REASON_IRQ: usize =				1;
const EXIT_REASON_FIQ: usize =				2;
const EXIT_REASON_PSCI: usize =			3;
const EXIT_REASON_REC_INTERRUPT_PENDING: usize =	4;
const EXIT_REASON_RSI_DISPOSE: usize =			5;
pub const EXIT_REASON_RSI_MMAP: usize =			6;
pub const EXIT_REASON_RSI_UNMAP: usize =			7;
pub const EXIT_REASON_RSI_SYSCALL: usize =			8;
pub const EXIT_REASON_RSI_MODEL_REQUEST: usize =	9;



// #[repr(C)]
// #[derive(Clone, Copy)]
// #[derive(PartialEq)]
// pub enum VCommonSysregState {
// 	v_vttbr_el2 = 0,
// 	v_vtcr_el2,
// 	v_hcr_el2,
// }

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum NovSysregState {
	sp_el0 = 0,
	sp_el1,
	elr_el1,
	spsr_el1,
	pmcr_el0,
	pmuserenr_el0,
	tpidrro_el0,
	tpidr_el0,
	csselr_el1,
	sctlr_el1,
	actlr_el1,
	cpacr_el1,
	zcr_el1,
	ttbr0_el1,
	ttbr1_el1,
	tcr_el1,
	esr_el1,
	afsr0_el1,
	afsr1_el1,
	far_el1,
	mair_el1,
	vbar_el1,
	contextidr_el1,
	tpidr_el1,
	amair_el1,
	cntkctl_el1,
	par_el1,
	mdscr_el1,
	mdccint_el1,
	disr_el1,
	mpam0_el1,

	cnthctl_el2,
	cntvoff_el2,
	cntpoff_el2,
	cntp_ctl_el0,
	cntp_cval_el0,
	cntv_ctl_el0,
	cntv_cval_el0,

	vmpidr_el2,
	icc_sre_el2,
}

extern "C"{
    // fflush the data cache
    pub fn barrier();
    pub fn invalidate_block(map_addr: usize);
    pub fn invalidate_page(map_addr: usize);
    pub fn invalidate_pages_in_block(map_addr: usize);
    // set the memory range to zero
    // Granule_memzero_mapped
    pub fn memzero(buf: usize, size: usize);
    pub fn ns_buffer_read_data(slot: BufferSlot, data: *mut usize) -> bool;
	pub fn ns_buffer_read_rec_params(slot: BufferSlot, data: *mut usize) -> bool;
	pub fn ns_buffer_read_rec_run(slot: BufferSlot, data: *mut usize) -> bool;
	pub fn ns_buffer_write_rec_run(slot: BufferSlot, data: *mut usize) -> bool;
    pub fn ns_buffer_unmap(slot: BufferSlot);
	pub fn clean_realm_stage2();
	pub fn run_realm(regs: *mut usize) -> u32;
	pub fn set_vttbr_el2(val: usize);
	pub fn read_vttbr_el2() -> usize;
	pub fn read_cntv_ctl_el02() -> usize;
	pub fn set_cntv_ctl_el02(val: usize);
	pub fn find_next_set_bit(bitmap: usize, start: usize) -> usize;
	pub fn read_sctlr_el12() -> usize;
	pub fn set_sctlr_el12(val: usize);
	pub fn read_cpacr_el12() ->usize;
    pub fn  set_cpacr_el12(val: usize);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisposeInfo {
    dispose_pending: bool,
	dispose_addr: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RealmInfo {
    par_base: usize,
    par_end: usize,
    pub g_rd: u32,
    g_rec_list: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LastRunInfo{
    /*
     * The contents of the esr system register that is returned to
     * the host the last time the Realm was running.
     * The values of:
     *   (1) "esr.ec = data abort" &
     *   (2) "esr.iss.isv = 1"
     * ... indicate that the last execution of the realm resulted
     * in a stage 2 data abort that was:
     *   (1) outside PAR and that
     *   (2)the realm execution state was AArch64.
     * ... and that the REC can be subjected to
     * mmio emulation by the host.
     */
    esr: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EmulatedTimerState {
	masked: bool,
	asserted: bool,
}

impl EmulatedTimerState {
    pub fn new() -> Self {
        EmulatedTimerState { 
            masked: false,
            asserted: false,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommonSysregState {
	vttbr_el2: usize,
	vtcr_el2: usize,
	hcr_el2: usize,
}

impl CommonSysregState {
    pub fn new() -> Self {
        CommonSysregState { 
            vttbr_el2: 0,
            vtcr_el2: 0,
			hcr_el2: 0,
        }
    }
}

// except for AFF0 and first bit in the AFF1, other bits need to be zero
#[inline]
pub fn mpidr_is_valid(mpidr: usize) -> bool {
	// return true;
	return (mpidr & !(MPIDR_EL1_AFF0 | crate::INPLACE!(MPIDR_EL1_AFF1_SHIFT, 1))) == 0;
}

#[inline]
pub fn mpidr_to_rec_idx(mpidr: usize) -> usize {
	return mpidr & (MPIDR_EL1_AFF0 | MPIDR_EL1_AFF1);
}

// #[inline]
// pub fn realm_get_rec_entry(rec_idx: usize, rec_list: &mut RecList) -> u32 {
// 	let granule_idx = rec_list.g_recs[rec_idx] as usize;
// 	if granule_idx >= NR_GRANULES {return NR_GRANULES as u32;}
// 	return GranuleUtil::granule_from_index(rec_list.g_recs[rec_idx] as usize).id;
// }

#[inline]
pub fn is_rec_valid(rec_idx: usize, rec_list: &mut RecList) -> bool {
	return rec_idx < MAX_NUM_RECS;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rec {
	g_rec_id: u32, /* the granule in which this rec lives, not the rd_id */
	rec_idx: usize, /* Which rec is this */
	runnable: bool,

	vtimer: EmulatedTimerState,
	ptimer: EmulatedTimerState,

	pub regs: [usize; 96], // 32 x registers, 32 v registers
	pub pc: usize,
	pstate: usize,

	//struct sysreg_state sysregs;
	pub sysregs: [usize; NR_SYSREG_STATE],
	pub common_sysregs: CommonSysregState,

	dispose_info: DisposeInfo,

	/*
	 * Common values across all RECs in a Realm.
	 */
    pub realm_info: RealmInfo,

	last_run_info: LastRunInfo,

	/*
	 * The bitmaps in the rvic structure can be evaluated and modified
	 * using atomic accessors defined in rvic.h without holding a lock
	 * on the REC granule.  Use only the accessors defined in rvic.h
	 * when accessing the bitmaps in this structure.
	 */
	rvic: RecRvicState,
}

impl Rec {
    pub fn new() -> Self {
        Rec { 
            g_rec_id: NR_GRANULES as u32,
			rec_idx: 0,
			runnable: false,
			vtimer: EmulatedTimerState::new(),
			ptimer: EmulatedTimerState::new(),
            regs: [0; 96],
			pc: 0,
			pstate: 0,

			//struct sysreg_state sysregs;
			sysregs: [0; NR_SYSREG_STATE],
			common_sysregs: CommonSysregState::new(),

			dispose_info: DisposeInfo {
				dispose_pending: false,
				dispose_addr: 0,
			},
			realm_info: RealmInfo {
				par_base: 0,
				par_end: 0,
				g_rd: NR_GRANULES as u32,
				g_rec_list: NR_GRANULES as u32,
			},

			last_run_info: LastRunInfo {
				esr: 0,
			},

			rvic: RecRvicState::new(),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RecList {
	g_recs: [u32; MAX_NUM_RECS],
}

/*
 *\brief: set the virtualization related register
 */
pub fn init_common_sysregs(rec: &mut Rec, rd: &mut Rd) {
	rec.common_sysregs.hcr_el2 = HCR_FLAGS;
	//TODO:RMIAux.c 55
	rec.common_sysregs.vtcr_el2 = VTCR_FLAGS;
	//find the physical address of the g_table
	let addr;
	if cfg!(feature = "platform_qemu") {
		addr =  MEM0_PHYS + (rd.g_table.id as usize * GRANULE_SIZE);
	}
	else if cfg!(feature = "platform_fvp") {
		if (rd.g_table.id as usize) > (NR_GRANULES/2) {
			addr = MEM1_PHYS + ((rd.g_table.id as usize)-(NR_GRANULES/2))*GRANULE_SIZE;
		}
		else {// For tf-a-test
			addr =  MEM0_PHYS + ((rd.g_table.id as usize * GRANULE_SIZE) );
		}
	} else {
        addr = 0;
        crate::println!("ERROR: init_common_sysregs is failed: invalied platform");
    }
	//ignore the last bit of address and retrieve [47:1] bits
	rec.common_sysregs.vttbr_el2 = addr & crate::MASK!(TTBRx_EL2_BADDR_WIDTH, TTBRx_EL2_BADDR_SHIFT);
}

/*
 *\brief: set the system register
 */
pub fn init_rec_sysregs(rec: &mut Rec, mpidr: usize) {
	rec.sysregs[NovSysregState::pmcr_el0 as usize] = PMCR_EL0_RES1;
	rec.sysregs[NovSysregState::sctlr_el1 as usize] = SCTLR_EL1_FLAGS;
	rec.sysregs[NovSysregState::mdscr_el1 as usize] = MDSCR_EL1_TDCC_BIT;
	rec.sysregs[NovSysregState::vmpidr_el2 as usize] = mpidr;
	rec.sysregs[NovSysregState::cnthctl_el2 as usize] = CNTHCTL_EL2_NO_TRAPS;
}

/* 
 * \brief: initialize the gprs, system register and virtualization-related register for rec
 */
pub fn init_rec_regs(rec: &mut Rec, mpidr: usize, rd: &mut Rd) {
	let mut i = 0;
	let mut v_percpu_list = VPERCPU_LOCK.lock();

	// init the gprs
	while i<REC_CREATE_NR_GPRS {
		rec.regs[i] = v_percpu_list[crate::cpuid!()].r.gprs[i];
		i += 1;
	}
	// init pc and pstate
	rec.pc = v_percpu_list[crate::cpuid!()].r.pc;
	rec.pstate = PSTATE_INIT;

	// init system register
	init_rec_sysregs(rec, mpidr);
	// init virtualization-related register
	init_common_sysregs(rec, rd);

	drop(v_percpu_list);
}

pub fn init_rec_rvic_state(rvic: &mut RecRvicState) {
	let mut i = 0;
	while i<RVIC_BITMAP_ULS {
		rvic.mask_bits[i] = 0;
		i+=1;
	}
}

/* 
 * \brief: Set the rec with the initialized value and add the rec into the rec_list 
 */
pub fn rec_create_ops(rd_id: u32, rec_id: u32, rd: &mut Rd, rec_list: &mut RecList, rec: &mut Rec, mpidr: usize, rec_idx: usize) {
	GranuleUtil::granule_set_state(rec_id, GranuleState::GranuleStateRec);
	rec_list.g_recs[rec_idx] = rec_id;
	
	// set the rec
	rec.g_rec_id = rec_id;
	rec.rec_idx = rec_idx;

	// set the initial reg for rec
	init_rec_regs(rec, mpidr, rd);
	init_rec_rvic_state(&mut rec.rvic);

	rec.realm_info.par_base = rd.par_base;
	rec.realm_info.par_end = rd.par_end;

	// set the rd in rec, not rec granule
	rec.realm_info.g_rd = rd_id;
	rec.realm_info.g_rec_list = rd.g_rec_list.id;

	//FIXME: ignore the measurement here
	// set rd in rec granule
	GranuleUtil::granule_set_rd(rec_id, rd_id);
	//FIXME: atomic get
	GranuleUtil::get_granule(rd_id);
	let v_percpu_list = VPERCPU_LOCK.lock();
	rec.runnable = (v_percpu_list[crate::cpuid!()].r.flags & 1) == 1;
	drop(v_percpu_list);
}

// restore the gprs into the rec and reset the esr
pub fn complete_hvc_exit(rec: &mut Rec) {
	let esr = rec.last_run_info.esr;
	let mut i = 0;

	// hvc exit reason
	if (esr & ESR_EL2_EC_MASK) == ESR_EL2_EC_HVC {
		// read the initial gprs from the run_params
		let mut v_percpu_list = VPERCPU_LOCK.lock();
        let run = &mut v_percpu_list[crate::cpuid!()].run;
		while i < REC_RUN_HVC_NR_GPRS {
			rec.regs[i] = run.gprs[i];
			i += 1;
		}
		drop(v_percpu_list);
		rec.last_run_info.esr = 0;
	}
}

// restor the syscall instruction for the redirect syscall
pub fn complete_rsi_exit(rec: &mut Rec) {
	let mut v_percpu_list = VPERCPU_LOCK.lock();
    let run = &mut v_percpu_list[crate::cpuid!()].run;
	let mut i = 0;

	// hvc exit reason
	if (run.exit_reason == EXIT_REASON_RSI_MMAP) 
		|| (run.exit_reason == EXIT_REASON_RSI_UNMAP) 
		|| (run.exit_reason == EXIT_REASON_RSI_SYSCALL)
	{
		// read the initial gprs from the run_params
		while i < REC_RUN_SMC_NR_GPRS {
			rec.regs[i] = run.gprs[i];
			i += 1;
		}
	}
	drop(v_percpu_list);
}

pub fn save_ns_state() {
	save_ns_state_sysreg_state();
	let mut v_percpu_list = VPERCPU_LOCK.lock();
    let mut sysregs = &mut v_percpu_list[crate::cpuid!()].ns_state.sysregs;
	sysregs[NovSysregState::icc_sre_el2 as usize] = crate::read_sysreg!(ICC_SRE_EL2);
	drop(v_percpu_list);
}

pub fn restore_ns_state() {
	restore_ns_state_sysreg_state();
	let mut v_percpu_list = VPERCPU_LOCK.lock();
    let mut sysregs = &mut v_percpu_list[crate::cpuid!()].ns_state.sysregs;
	crate::write_sysreg!(ICC_SRE_EL2, sysregs[NovSysregState::icc_sre_el2 as usize]);
	drop(v_percpu_list);
}

/* 
 * \brief: restore el1 system regs, pc and pstate of realm
 */
pub fn restore_realm_state(rec: &mut Rec)
{
	//restore the el1 and el2 register, and entry pc of realm
	restore_sysreg_state(rec);
	let pc = rec.pc;
	crate::write_sysreg!(elr_el2, pc);
	let pstate = rec.pstate;
	crate::write_sysreg!(spsr_el2, pstate);

	let icc_sre_el2:usize = crate::read_sysreg!(ICC_SRE_EL2);
	let new_icc_sre_el2 = icc_sre_el2& NOT_ICC_SRE_EL2_ENABLE;
	crate::write_sysreg!(ICC_SRE_EL2, new_icc_sre_el2);
}

pub fn  save_realm_state(rec: &mut Rec)
{
	save_sysreg_state(rec);
	rec.pc = crate::read_sysreg!(elr_el2);
	rec.pstate = crate::read_sysreg!(spsr_el2);
}

/* 
 * \brief: set virtualization-related regs
 */
pub fn configure_realm_stage2(rec: &mut Rec)
{
	crate::write_sysreg!(vtcr_el2, rec.common_sysregs.vtcr_el2);
	// currently, rust-asm cannot suppott vttbr register
	unsafe {set_vttbr_el2(rec.common_sysregs.vttbr_el2);}

	let cptr = crate::read_sysreg!(cptr_el2);
	let cpacr_el12 = unsafe {read_cpacr_el12()};
	// crate::println!("DEBUG: rec_run_loop: cptr {:x}, cpacr_el12 {:x} \n", cptr, cpacr_el12);
	unsafe {set_cpacr_el12(cpacr_el12 | (0b11<<20))}
	crate::write_sysreg!(cptr_el2, cptr| (0b11<<20));
}
#[inline]
pub fn set_rec_run_exit_reason(exit_reason: usize)
{
	let mut v_percpu_list = VPERCPU_LOCK.lock();
    v_percpu_list[crate::cpuid!()].run.exit_reason = exit_reason;
	drop(v_percpu_list);
}

#[inline]
pub fn timer_is_masked(cntx_ctl: usize) -> bool
{
	return (cntx_ctl & CNTx_CTL_IMASK) != 0;
}

#[inline]
pub fn timer_condition_met(cntx_ctl: usize) -> bool
{
	return ((cntx_ctl & CNTx_CTL_ENABLE) != 0)&& 
			((cntx_ctl & CNTx_CTL_ISTATUS)!= 0);
}

/* Handle timer check */
pub fn check_timer_became_asserted(timer: &mut EmulatedTimerState,
					cntx_ctl: usize) -> bool
{
	if timer.asserted {
		return false;
	}

	/*
	 * If trapping wasn't enabled since we last ran the Realm (timer was not
	 * asserted) then we need to read back the mask state from the CTL
	 * register because the guest could have modified this without us
	 * knowing.
	 */
	timer.masked = timer_is_masked(cntx_ctl);

	/*
	 * Check if the emulated timer interrupt signal just became asserted
	 * and start trapping if so.
	 */
	return !timer.masked && timer_condition_met(cntx_ctl);
}


pub fn check_pending_vtimers(rec: &mut Rec)
{
	let g_rec_id = rec.g_rec_id;
	
	/* 
	 * Timer condition register
	 * ISTATUS: The status of the timer. This bit indicates whether the timer condition is met:
	 * IMASK: Timer interrupt mask bit.
	 * ENABLE: Enables the timer.
	 */
	let mut cntv_ctl = unsafe {read_cntv_ctl_el02()};

	// Read the state in the cntv_ctl and set to the vtimer
	if check_timer_became_asserted(&mut rec.vtimer, cntv_ctl) {
		GranuleUtil::lock_granule(g_rec_id);
		cntv_ctl |= CNTx_CTL_IMASK;
		unsafe {
			set_cntv_ctl_el02(cntv_ctl);
		}

		/* 
		 * Counter-timer Hypervisor Control register
		 * Controls the generation of an event stream from the physical counter, and access from EL1 to the physical counter and the EL1 physical timer.
		 * Traps EL0 and EL1 accesses to the EL1 virtual timer registers to EL2, when EL2 is enabled for the current Security state.
		 */
		let tmp_cnthctl_el2 = get_rec_sysregs(rec, NovSysregState::cnthctl_el2) | CNTHCTL_EL2_EL1TVT;
		set_rec_sysregs(rec, NovSysregState::cnthctl_el2, tmp_cnthctl_el2);
		crate::write_sysreg!(cnthctl_el2, get_rec_sysregs(rec, NovSysregState::cnthctl_el2));

		rec.vtimer.asserted = true;
		
		Rvic::rvic_set_pending(&mut rec.rvic, INTID_VTIMER_EL1);
		GranuleUtil::unlock_granule(g_rec_id);
	}
}

pub fn check_pending_ptimers(rec: &mut Rec)
{
	let g_rec_id = rec.g_rec_id;
	
	// timer condition register
	let mut cntp_ctl = unsafe {read_cntv_ctl_el02()};

	if check_timer_became_asserted(&mut rec.ptimer, cntp_ctl) {
		//timer is not asserted && cntv_ctl is not masked
		GranuleUtil::lock_granule(g_rec_id);
		//Mask the cntp_ctl, other ptimer will be ignored
		cntp_ctl |= CNTx_CTL_IMASK;
		unsafe {
			set_cntv_ctl_el02(cntp_ctl);
		}

		// Counter-timer Hypervisor Control register
		// Trap to the El2 when accessing the ptime in the EL0 and EL1 mode
		let tmp_cnthctl_el2 = get_rec_sysregs(rec, NovSysregState::cnthctl_el2) | NOT_CNTHCTL_EL2_EL1PTEN;
		set_rec_sysregs(rec, NovSysregState::cnthctl_el2, tmp_cnthctl_el2);
		crate::write_sysreg!(cnthctl_el2, get_rec_sysregs(rec, NovSysregState::cnthctl_el2));

		rec.ptimer.asserted = true;
		
		// set the pending bit in the rvic bitmap
		Rvic::rvic_set_pending(&mut rec.rvic, INTID_PTIMER_EL1);
		GranuleUtil::unlock_granule(g_rec_id);
	}
}

pub fn check_pending_timers(rec: &mut Rec) {
	check_pending_vtimers(rec);
	check_pending_ptimers(rec);
}

/* Handle interrupt check */
pub fn rvic_find_pending_unmasked(rvic: & RecRvicState) -> usize
{
	// Pending bit is set but mask bit is not set
	for i in 0..RVIC_BITMAP_ULS {
		let bitmap = rvic.pending_bits[i];
		let mut bit = unsafe{find_next_set_bit(bitmap, 0)};
		while bit < BITS_PER_UL {
			let intid = i * BITS_PER_UL + bit;
			if !Rvic::rvic_is_masked(rvic, intid) {
				return intid;
			}
			bit = unsafe{find_next_set_bit(bitmap, bit+1)};
		}
	}

	return INTID_SPURIOUS;
}

// Inject the virtual IRQ exception if needed.
pub fn check_pending_interrupts(rec: &mut Rec)
{
	//set the interrupt id to invalid
	let mut intid: usize = INTID_SPURIOUS;

	GranuleUtil::lock_granule(rec.g_rec_id);
	if rec.rvic.rvic_enabled {
		intid = rvic_find_pending_unmasked(&rec.rvic);
	}
	GranuleUtil::unlock_granule(rec.g_rec_id);

	if intid == INTID_SPURIOUS{
		rec.common_sysregs.hcr_el2 &= !HCR_VI;
	}
	else {
		crate::println!("Debug: inject a virtual interrupt");
		rec.common_sysregs.hcr_el2 |= HCR_VI;
	}
}

pub fn handle_exception_sync(rec: &mut Rec) -> bool
{
	let esr = crate::read_sysreg!(esr_el2);
	let ec = esr & ESR_EL2_EC_MASK;

	if ec ==  ESR_EL2_EC_WFX {
		let mut v_percpu_list = VPERCPU_LOCK.lock();
		v_percpu_list[crate::cpuid!()].run.esr = esr & (ESR_EL2_EC_MASK | ESR_EL2_WFx_TI_BIT);
		drop(v_percpu_list);
		crate::println!("Debug: handle_exception_sync: ESR_EL2_EC_WFX");
		return false;
	} else if ec == ESR_EL2_EC_HVC {
		let mut i = 0;
		let info = esr &
				     (ESR_EL2_EC_MASK | ESR_EL2_xVC_IMM_MASK);

		rec.last_run_info.esr = info;
		let mut v_percpu_list = VPERCPU_LOCK.lock();
		v_percpu_list[crate::cpuid!()].run.esr = info;

		while i < REC_RUN_HVC_NR_GPRS {
			v_percpu_list[crate::cpuid!()].run.gprs[i] = rec.regs[i];
			i+=1;
		}
		drop(v_percpu_list);

		crate::println!("Debug: handle_exception_sync: ESR_EL2_EC_HVC");
		return false;
	} else if ec == ESR_EL2_EC_SMC {
		let pc = crate::read_sysreg!(elr_el2);

		crate::write_sysreg!(elr_el2, pc + 4);
		// crate::println!("Debug: handle_exception_sync: ESR_EL2_EC_SMC: x0: {:x}, x1: {:x}, elr_el2 {:x}", rec.regs[0], rec.regs[1], pc);
		// let mut i = 0;
		// for gprs in rec.regs {
		// 	crate::println!("x{:?}: {:x}", i, gprs);
		// 	i = i+1;
		// }
		return handle_realm_rsi(rec);
	} else if ec == ESR_EL2_EC_SYSREG {
		// FIXME: handle_exception_sync
		// let pc;

		// handle_sysreg_access_trap(rec, esr);

		// pc = sysreg_read(elr_el2);
		// sysreg_write(elr_el2, pc + 4U);
		crate::println!("Debug: handle_exception_sync: ESR_EL2_EC_SYSREG");
		return true;
	} else if (ec == ESR_EL2_EC_INST_ABORT) {
		// if (handle_instruction_abort(rec, esr) == 1U)
		// 	return 0U;
		// else {
		// 	set_rec_run_esr(0);
		// 	set_rec_run_far(0);
		// 	set_rec_run_hpfar(0);
		// 	return 0U;
		// }
		let hpfar_el2 = crate::read_sysreg!(hpfar_el2);
		let far_el2 = crate::read_sysreg!(far_el2);
		let vtcr_el2 = crate::read_sysreg!(vtcr_el2);
		let tcr_el2 = crate::read_sysreg!(tcr_el2);
		unsafe {
			let sctlr_el1 = read_sctlr_el12();
			crate::println!("Debug: handle_exception_sync: ESR_EL2_EC_INST_ABORT esr {:x}, hpfar_el2 {:x}, far_el2 {:x}, sctlr_el1: {:x}, vttbr_el2: {:x}, vtcr_el2 {:x}, tcr_el2 {:x}", esr, hpfar_el2, far_el2, sctlr_el1, read_vttbr_el2(), vtcr_el2, tcr_el2);
		}
		return false
	} else if ec == ESR_EL2_EC_DATA_ABORT {
		// handle_data_abort(rec, esr);
		let hpfar_el2 = crate::read_sysreg!(hpfar_el2);
		let far_el2 = crate::read_sysreg!(far_el2);
		let vtcr_el2 = crate::read_sysreg!(vtcr_el2);
		let tcr_el2 = crate::read_sysreg!(tcr_el2);
		let elr_el2 = crate::read_sysreg!(elr_el2);
		unsafe {
			let sctlr_el1 = read_sctlr_el12();
			crate::println!("Debug: handle_exception_sync: ESR_EL2_EC_DATA_ABORT esr {:x}, hpfar_el2 {:x}, far_el2 {:x}, sctlr_el1: {:x}, vttbr_el2: {:x}, vtcr_el2 {:x}, tcr_el2 {:x} elr_el2 {:x}", esr, hpfar_el2, far_el2, sctlr_el1, read_vttbr_el2(), vtcr_el2, tcr_el2, elr_el2);
		}
		return false;
	} else {
		let mut v_percpu_list = VPERCPU_LOCK.lock();
		v_percpu_list[crate::cpuid!()].run.esr = 0;
		v_percpu_list[crate::cpuid!()].run.far = 0;
		v_percpu_list[crate::cpuid!()].run.hpfar = 0;
		drop(v_percpu_list);

		crate::println!("Debug: handle_exception_sync: UNKNOWN ERROR");
		return false;
	}
}

/*
 * \brief: ignore the VTIMER and P timer interrupt
 */
pub fn handle_exception_irq_lel(rec: &mut Rec) -> bool {
	let icc_hppir1_el1 = crate::read_sysreg!(ICC_HPPIR1_EL1);
	let intid = icc_hppir1_el1 & ICC_HPPIR1_EL1_INTID;
	if intid == INTID_VTIMER_EL1  || intid == INTID_PTIMER_EL1 {
		return true;}
	// else if intid == INTID_UNKNOWN_EL1{
	// 	crate::println!("WARNING: handle_excpetion_irq_lel: unknown interrupt {:x}", intid);
	// 	return true;
	// }
	else {
		if intid != 0x1a {
			crate::println!("Debug: handle_exception_irq_lel: cannot hanle such irq request {:x}", intid);
		}
		set_rec_run_exit_reason(EXIT_REASON_IRQ);
		return false;
	}
}

pub fn handle_realm_exit(rec: &mut Rec, realm_exception_code: u32, first_loop: bool) -> bool {
	if first_loop {
		// crate::println!("Debug: handle_realm_exit: first loop");
		return true;
	}
	else {
		if realm_exception_code == ARM_EXCEPTION_SYNC_LEL {
			set_rec_run_exit_reason(EXIT_REASON_SYNC);
			// crate::println!("Debug: handle_realm_exit: ARM_EXCEPTION_SYNC_LEL");
			return handle_exception_sync(rec);
		} else if realm_exception_code == ARM_EXCEPTION_IRQ_LEL {
			// crate::println!("Debug: handle_realm_exit: ARM_EXCEPTION_IRQ_LEL");
			return handle_exception_irq_lel(rec);}
		else if realm_exception_code == ARM_EXCEPTION_FIQ_LEL {
			crate::println!("Debug: handle_realm_exit: ARM_EXCEPTION_FIQ_LEL");
			set_rec_run_exit_reason(EXIT_REASON_FIQ);
			return false;
		} else {
			crate::println!("Debug: handle_realm_exit: Error");
			return false;
		}
	}
}

pub fn rec_run_loop(rec: &mut Rec) {
	let mut realm_exception_code: u32 = 0;

	save_ns_state();
	restore_realm_state(rec);

	// set vtcr and vttbr of realm
	// access and not trap for FP and SIMD instructions
	configure_realm_stage2(rec);
	
	// disable arch-timer
	let mut cnthp_ctl = crate::read_sysreg!(cnthp_ctl_el2);
	cnthp_ctl &= !CNTx_CTL_ENABLE;
	crate::write_sysreg!(cnthp_ctl_el2, cnthp_ctl);

	// crate::println!("vttbr_el2 {:x}", rec.common_sysregs.vttbr_el2);
	// crate::println!("vtcr_el2 {:x}", rec.common_sysregs.vtcr_el2);
	// crate::println!("hcr_el2 {:x}", rec.common_sysregs.hcr_el2);

	let mut first_loop = true;

	while handle_realm_exit(rec, realm_exception_code, first_loop) {
		first_loop = false;
		check_pending_timers(rec);
		check_pending_interrupts(rec);
		// ser hcr register		
		crate::write_sysreg!(hcr_el2, rec.common_sysregs.hcr_el2);
		realm_exception_code = unsafe {run_realm((&mut rec.regs[0]) as *mut usize)};
	}

	unsafe {clean_realm_stage2();}

	save_realm_state(rec);
	restore_ns_state();
}

#[inline] 
pub fn esr_sas(esr: usize) -> usize {
	// Syndrome Access Size. Indicates the size of the access attempted by the faulting operation
	return (esr & ESR_EL2_ABORT_SAS_MASK) >> ESR_EL2_ABORT_SAS_SHIFT;
}

#[inline] 
pub fn access_len(esr: usize) -> u32 {
	return 1 << esr_sas(esr);
}

#[inline] 
pub fn access_mask(esr: usize) -> usize {
	match esr_sas(esr) {
		ESR_EL2_ABORT_SAS_BYTE_VAL => {
			return 0xff;
		}
		ESR_EL2_ABORT_SAS_HWORD_VAL=> {
			return 0xffff;
		}
		ESR_EL2_ABORT_SAS_WORD_VAL=> {
			return 0xffffffff;
		}
		ESR_EL2_ABORT_SAS_DWORD_VAL=> {
			return !(0 as usize);
		}
		_ => {
			return 0;
		}
	}
}

// Set the value into the rt value
pub fn emulate_mmio_read(esr: usize, rt: usize, rec: &mut Rec) {
	let mut v_percpu_list = VPERCPU_LOCK.lock();
	let mut val  = v_percpu_list[crate::cpuid!()].run.emulated_read_val & access_mask(esr);
	drop(v_percpu_list);

	// Data item must be sign-extended is SSE is equal to 1
	if (esr & ESR_EL2_ABORT_SSE_BIT) != 0 {
		let bit_count =  access_len(esr) * 8;

		let mask = 1 << (bit_count - 1);

		val = (val ^ mask) - mask;
		if (esr & ESR_EL2_ABORT_SF_BIT)  == 0 {
			val &= (1 << 32) - 1;
		}
	}
	// set the mmio value
	rec.regs[rt] = val;
}

/* 
 * \brief: return true if mmio emulation is completed
 */
pub fn complete_mmio_emulation(rec: &mut Rec) -> bool {
	let mut v_percpu_list = VPERCPU_LOCK.lock();
	if  v_percpu_list[crate::cpuid!()].run.is_emulated_mmio == 0 {
		return true;
	}
	drop(v_percpu_list);

	let esr = rec.last_run_info.esr;
	// The Arm register number of the Wt/Xt/Rt operand of the faulting instruction
	let rt = (esr & ESR_EL2_ABORT_SRT_MASK) >> ESR_EL2_ABORT_SRT_SHIFT;

	if ((esr & ESR_EL2_EC_MASK) != ESR_EL2_EC_DATA_ABORT) || ((esr & ESR_EL2_ABORT_ISV_BIT) == 0) {
		return false;
	}
	else {
		if ((esr & ESR_EL2_ABORT_WNR_BIT) == 0) && (rt != 31) {
			emulate_mmio_read(esr, rt, rec);
		}
		rec.pc = rec.pc + 4;
		return true;
	}
}

pub fn rec_granule_measure(rd: &mut Rd, rec: &mut Rec) {
	let mut hash_rec_header: [u64; 3] = [0,0,0];
	hash_rec_header[0] = GranuleState::GranuleStateRec as u64;
	// update with the header of rec granule
	let u8_header_ptr = hash_rec_header.as_mut_ptr() as *mut u8 as *mut[u8; 24];
	let u8_header = unsafe {&mut (*u8_header_ptr)};
	measurement_extend_data(rd, u8_header);
	// update with general registers for realm
	let u8_regs_ptr = rec.regs.as_mut_ptr() as *mut u8 as *mut[u8; 32*8];
	let u8_regs = unsafe {&mut (*u8_regs_ptr)};
	measurement_extend_data(rd, u8_regs);
	// update with pstate for realm
	let u8_pstate_ptr = &mut (rec.pstate) as *mut usize as *mut[u8; 8];
	let u8_pstate = unsafe {&mut (*u8_pstate_ptr)};
	measurement_extend_data(rd, u8_pstate);
	// update with sysregs for realm
	// update with general registers for realm
	let u8_sysregs_ptr = rec.sysregs.as_mut_ptr() as *mut u8 as *mut[u8; NR_SYSREG_STATE*8];
	let u8_sysregs = unsafe {&mut (*u8_sysregs_ptr)};
	measurement_extend_data(rd, u8_sysregs);
}

#[repr(C)]
pub struct RecUtil {
    // The global list of all granules
    // global_granule: Vec<Granule>,
}

impl RecUtil {
	pub fn rec_create(rd_id: u32, rec_id: u32, mpidr: usize, rec_params_id: u32) -> usize {
		let mut ret: usize = 0;
		// Copy the rec_params into the v_percpu_list
		// ns_granule_map
		granule_map_with_id_state(rec_params_id as usize, GranuleState::GranuleStateNs, BufferSlot::SLOT_NS);

		let ns_access_ok;
		let mut v_percpu_list = VPERCPU_LOCK.lock();
        let r = &mut v_percpu_list[crate::cpuid!()].r;

		// read the gprs from the RecParams resided in the normal page
		unsafe {
			ns_access_ok = ns_buffer_read_rec_params(BufferSlot::SLOT_NS, r as *mut RecParams as *mut usize);
			ns_buffer_unmap(BufferSlot::SLOT_NS);
		}

		drop(v_percpu_list);

		if ns_access_ok {
			// Map the rec, rd, rec_list granule
			let rec_ptr = granule_map_with_id(rec_id as usize, BufferSlot::SLOT_REC) as *mut Rec;
			let rec = unsafe {&mut (*rec_ptr)};
			let rd_ptr = granule_map_with_id(rd_id as usize, BufferSlot::SLOT_RD) as *mut Rd;
			let rd = unsafe {&mut (*rd_ptr)};
			// rec_list is set when realm create
			let rec_list_ptr = granule_map_with_id(rd.g_rec_list.id as usize, BufferSlot::SLOT_REC_LIST) as *mut RecList;
			let rec_list = unsafe {&mut (*rec_list_ptr)};

			if rd.state == 0 {
				if mpidr_is_valid(mpidr) == true {
					let rec_idx = mpidr_to_rec_idx(mpidr);
					// rec_idx should less then MAX_NUM_RECS
					if is_rec_valid(rec_idx, rec_list) {
						rec_create_ops(rd_id, rec_id, rd, rec_list, rec, mpidr, rec_idx);
						// Calculate the measurement of realm context and rec granule
						rec_granule_measure(rd, rec);
					}
					else {
						ret = pack_return_code(ErrorStatus::StatusErrorParameter, 1);
					}
				}
				else {
					ret = pack_return_code(ErrorStatus::StatusErrorParameter, 0);
				}
			}
			else {
				ret = pack_return_code(ErrorStatus::StatusErrorRealmAttr, 0);
			}

			unsafe {buffer_unmap(rec_ptr as usize);}
			unsafe {buffer_unmap(rd_ptr as usize);}
			unsafe {buffer_unmap(rec_list_ptr as usize);}
		}
		else {
			ret = pack_return_code(ErrorStatus::StatusErrorParameter, 0);
		}

		GranuleUtil::unlock_granule(rd_id);
		return ret;
	}

	pub fn rec_destroy(rec_id: u32) {
		let rd_id = GranuleUtil::granule_get_rd(rec_id);
		let rec_ptr = granule_map_with_id(rec_id as usize, BufferSlot::SLOT_REC) as *mut Rec;
		let rec = unsafe {&mut (*rec_ptr)};
		let rec_list_ptr = granule_map_with_id(rec.realm_info.g_rec_list as usize, BufferSlot::SLOT_REC_LIST) as *mut RecList;
		let rec_list = unsafe {&mut (*rec_list_ptr)};

		let rec_idx = rec.rec_idx;
		rec_list.g_recs[rec_idx] = NR_GRANULES as u32;

		unsafe {buffer_unmap(rec_ptr as usize);}
		unsafe {buffer_unmap(rec_list_ptr as usize);}

		GranuleUtil::granule_set_rd(rec_id, NR_GRANULES as u32);
		granule_map_zero(rec_id, BufferSlot::SLOT_REC);
		GranuleUtil::granule_set_state(rec_id, GranuleState::GranuleStateDelegated);
		GranuleUtil::unlock_granule(rec_id);
		//FIXME need atomic ops
		GranuleUtil::put_granule(rd_id);
	}

	pub fn rec_run(rec_id: u32, rec_run_id: u32) -> usize {
		let mut ret: usize = 0;
		//FIXME: atomic get
		GranuleUtil::get_granule(rec_id);
		GranuleUtil::unlock_granule(rec_id);

		// ns_granule_map
		granule_map_with_id_state(rec_run_id as usize,  GranuleState::GranuleStateNs, BufferSlot::SLOT_NS);
		let mut v_percpu_list = VPERCPU_LOCK.lock();
        let run = &mut v_percpu_list[crate::cpuid!()].run;
		let ns_access_ok;

		// read the gprs from the RecParams resided in the normal page
		unsafe {
			ns_access_ok = ns_buffer_read_rec_run(BufferSlot::SLOT_NS, run as *mut RecRun as *mut usize);
		}
		drop(v_percpu_list);

		if 	ns_access_ok == false {
			unsafe {ns_buffer_unmap(BufferSlot::SLOT_NS);}
			// FIXME: atomic put
			GranuleUtil::put_granule(rec_id);
			ret = pack_return_code(ErrorStatus::StatusErrorRECAttr, 1);
		}
		else {
			let rec_ptr = granule_map_with_id(rec_id as usize, BufferSlot::SLOT_REC) as *mut Rec;
			let rec = unsafe {&mut (*rec_ptr)};

			if rec.runnable == false {
				unsafe {buffer_unmap(rec_ptr as usize);}
				unsafe {ns_buffer_unmap(BufferSlot::SLOT_NS);}
				// FIXME: atomic put
				GranuleUtil::put_granule(rec_id);
				ret = pack_return_code(ErrorStatus::StatusErrorRECAttr, 0);
			} else {
				if complete_mmio_emulation(rec) == false {
					unsafe {buffer_unmap(rec_ptr as usize);}
					unsafe {ns_buffer_unmap(BufferSlot::SLOT_NS);}
					// FIXME: atomic put
					GranuleUtil::put_granule(rec_id);
					ret = pack_return_code(ErrorStatus::StatusErrorRECAttr, 0);
				} 
				else {
					complete_hvc_exit(rec);
					complete_rsi_exit(rec);

					rec.last_run_info.esr = 0;
					rec.dispose_info.dispose_pending = false;

					rec_run_loop(rec);

					unsafe {buffer_unmap(rec_ptr as usize);}
					// write run param back to user's normal granule
					let mut v_percpu_list = VPERCPU_LOCK.lock();
					let run = &mut v_percpu_list[crate::cpuid!()].run;
					unsafe {
						if !ns_buffer_write_rec_run(BufferSlot::SLOT_NS, run as *mut RecRun as *mut usize) {
							ret = pack_return_code(ErrorStatus::StatusErrorRECAttr, 0);
						}
					}
					drop(v_percpu_list);

					unsafe {ns_buffer_unmap(BufferSlot::SLOT_NS);}
					// FIXME: atomic put
					GranuleUtil::put_granule(rec_id);
				}
			}
		}
		
		return ret;
	}

}