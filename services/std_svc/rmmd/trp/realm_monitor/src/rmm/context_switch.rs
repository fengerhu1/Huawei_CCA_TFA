use crate::rmm::rec_util::{
    NovSysregState,
	Rec,
};

use crate::rmm::abs_util::{
    VPERCPU_LOCK,
};

use crate::io::Write;

extern "C"{
	pub fn c_save_ns_state_sysreg_state(regs: *mut usize);
	pub fn c_restore_ns_state_sysreg_state(regs: *mut usize);
	pub fn c_restore_sysreg_state(regs: *mut usize);
	pub fn c_save_sysreg_state(regs: *mut usize);
}

pub fn save_ns_state_sysreg_state() {
    let mut v_percpu_list = VPERCPU_LOCK.lock();
    let mut sysregs = &mut v_percpu_list[crate::cpuid!()].ns_state.sysregs;
	unsafe {c_save_ns_state_sysreg_state(&mut sysregs[0] as *mut usize);}
	// sysregs[NovSysregState::sp_el0 as usize] = crate::read_sysreg!(sp_el0);
	// sysregs[NovSysregState::sp_el1 as usize] = crate::read_sysreg!(sp_el1);
	// sysregs[NovSysregState::elr_el1 as usize] = crate::read_sysreg!(elr_el12);
	// sysregs[NovSysregState::spsr_el1 as usize] = crate::read_sysreg!(spsr_el12);
	// sysregs[NovSysregState::pmcr_el0 as usize] = crate::read_sysreg!(pmcr_el0);
	// sysregs[NovSysregState::pmuserenr_el0 as usize] = crate::read_sysreg!(pmuserenr_el0);
	// sysregs[NovSysregState::tpidrro_el0 as usize] = crate::read_sysreg!(tpidrro_el0);
	// sysregs[NovSysregState::tpidr_el0 as usize] = crate::read_sysreg!(tpidr_el0);
	// sysregs[NovSysregState::csselr_el1 as usize] = crate::read_sysreg!(csselr_el1);
	// sysregs[NovSysregState::sctlr_el1 as usize] = crate::read_sysreg!(sctlr_el12);
	// sysregs[NovSysregState::actlr_el1 as usize] = crate::read_sysreg!(actlr_el1);
	// sysregs[NovSysregState::cpacr_el1 as usize] = crate::read_sysreg!(cpacr_el12);
	// // SVE(	sysregs[NovSysregState::zcr_el1 as usize] = crate::read_sysreg!(zcr_el1);)
	// sysregs[NovSysregState::ttbr0_el1 as usize] = crate::read_sysreg!(ttbr0_el12);
	// sysregs[NovSysregState::ttbr1_el1 as usize] = crate::read_sysreg!(ttbr1_el12);
	// sysregs[NovSysregState::tcr_el1 as usize] = crate::read_sysreg!(tcr_el12);
	// sysregs[NovSysregState::esr_el1 as usize] = crate::read_sysreg!(esr_el12);
	// sysregs[NovSysregState::afsr0_el1 as usize] = crate::read_sysreg!(afsr0_el12);
	// sysregs[NovSysregState::afsr1_el1 as usize] = crate::read_sysreg!(afsr1_el12);
	// sysregs[NovSysregState::far_el1 as usize] = crate::read_sysreg!(far_el12);
	// sysregs[NovSysregState::mair_el1 as usize] = crate::read_sysreg!(mair_el12);
	// sysregs[NovSysregState::vbar_el1 as usize] = crate::read_sysreg!(vbar_el12);

	// sysregs[NovSysregState::contextidr_el1 as usize] = crate::read_sysreg!(contextidr_el12);
	// sysregs[NovSysregState::tpidr_el1 as usize] = crate::read_sysreg!(tpidr_el1);
	// sysregs[NovSysregState::amair_el1 as usize] = crate::read_sysreg!(amair_el12);
	// sysregs[NovSysregState::cntkctl_el1 as usize] = crate::read_sysreg!(cntkctl_el12);
	// sysregs[NovSysregState::par_el1 as usize] = crate::read_sysreg!(par_el1);
	// sysregs[NovSysregState::mdscr_el1 as usize] = crate::read_sysreg!(mdscr_el1);
	// sysregs[NovSysregState::mdccint_el1 as usize] = crate::read_sysreg!(mdccint_el1);
	// sysregs[NovSysregState::disr_el1 as usize] = crate::read_sysreg!(disr_el1);
	// // MPAM(	sysregs[NovSysregState::mpam0_el1 as usize] = crate::read_sysreg!(MPAM0_EL1);	)
	// sysregs[NovSysregState::cnthctl_el2 as usize] = crate::read_sysreg!(cnthctl_el2);
	// sysregs[NovSysregState::cntvoff_el2 as usize] = crate::read_sysreg!(cntvoff_el2);
	// sysregs[NovSysregState::cntp_ctl_el0 as usize] = crate::read_sysreg!(cntp_ctl_el02);
	// sysregs[NovSysregState::cntp_cval_el0 as usize] = crate::read_sysreg!(cntp_cval_el02);
	// sysregs[NovSysregState::cntv_ctl_el0 as usize] = crate::read_sysreg!(cntv_ctl_el02);
	// sysregs[NovSysregState::cntv_cval_el0 as usize] = crate::read_sysreg!(cntv_cval_el02);
	drop(v_percpu_list);
}

pub fn restore_ns_state_sysreg_state()
{
	let mut v_percpu_list = VPERCPU_LOCK.lock();
    let mut sysregs = &mut v_percpu_list[crate::cpuid!()].ns_state.sysregs;
	unsafe {c_restore_ns_state_sysreg_state(&mut sysregs[0] as *mut usize);}
	// crate::write_sysreg!(sp_el0, sysregs[NovSysregState::sp_el0 as usize]);
	// crate::write_sysreg!(sp_el1, sysregs[NovSysregState::sp_el1 as usize]);
	// crate::write_sysreg!(elr_el12, sysregs[NovSysregState::elr_el1 as usize]);
	// crate::write_sysreg!(spsr_el12, sysregs[NovSysregState::spsr_el1 as usize]);
	// crate::write_sysreg!(pmcr_el0, sysregs[NovSysregState::pmcr_el0 as usize]);
	// crate::write_sysreg!(pmuserenr_el0, sysregs[NovSysregState::pmuserenr_el0 as usize]);
	// crate::write_sysreg!(tpidrro_el0, sysregs[NovSysregState::tpidrro_el0 as usize]);
	// crate::write_sysreg!(tpidr_el0, sysregs[NovSysregState::tpidr_el0 as usize]);
	// crate::write_sysreg!(csselr_el1, sysregs[NovSysregState::csselr_el1 as usize]);
	// crate::write_sysreg!(sctlr_el12, sysregs[NovSysregState::sctlr_el1 as usize]);
	// crate::write_sysreg!(actlr_el1, sysregs[NovSysregState::actlr_el1 as usize]);
	// crate::write_sysreg!(cpacr_el12, sysregs[NovSysregState::cpacr_el1 as usize]);
	// // SVE(	crate::write_sysreg!(zcr_el1, sysregs[NovSysregState::zcr_el1 as usize]);)
	// crate::write_sysreg!(ttbr0_el12, sysregs[NovSysregState::ttbr0_el1 as usize]);
	// crate::write_sysreg!(ttbr1_el12, sysregs[NovSysregState::ttbr1_el1 as usize]);
	// crate::write_sysreg!(tcr_el12, sysregs[NovSysregState::tcr_el1 as usize]);
	// crate::write_sysreg!(esr_el12, sysregs[NovSysregState::esr_el1 as usize]);
	// crate::write_sysreg!(afsr0_el12, sysregs[NovSysregState::afsr0_el1 as usize]);
	// crate::write_sysreg!(afsr1_el12, sysregs[NovSysregState::afsr1_el1 as usize]);
	// crate::write_sysreg!(far_el12, sysregs[NovSysregState::far_el1 as usize]);
	// crate::write_sysreg!(mair_el12, sysregs[NovSysregState::mair_el1 as usize]);
	// crate::write_sysreg!(vbar_el12, sysregs[NovSysregState::vbar_el1 as usize]);
	// crate::write_sysreg!(contextidr_el12, sysregs[NovSysregState::contextidr_el1 as usize]);
	// crate::write_sysreg!(tpidr_el1, sysregs[NovSysregState::tpidr_el1 as usize]);
	// crate::write_sysreg!(amair_el12, sysregs[NovSysregState::amair_el1 as usize]);
	// crate::write_sysreg!(cntkctl_el12, sysregs[NovSysregState::cntkctl_el1 as usize]);
	// crate::write_sysreg!(par_el1, sysregs[NovSysregState::par_el1 as usize]);
	// crate::write_sysreg!(mdscr_el1, sysregs[NovSysregState::mdscr_el1 as usize]);
	// crate::write_sysreg!(mdccint_el1, sysregs[NovSysregState::mdccint_el1 as usize]);
	// crate::write_sysreg!(disr_el1, sysregs[NovSysregState::disr_el1 as usize]);
	// // MPAM(	crate::write_sysreg!(MPAM0_EL1, sysregs[NovSysregState::mpam0_el1 as usize]);	)
	// crate::write_sysreg!(vmpidr_el2, sysregs[NovSysregState::vmpidr_el2 as usize]);
	// crate::write_sysreg!(cnthctl_el2, sysregs[NovSysregState::cnthctl_el2 as usize]);
	// crate::write_sysreg!(cntvoff_el2, sysregs[NovSysregState::cntvoff_el2 as usize]);
	// crate::write_sysreg!(cntp_ctl_el02, sysregs[NovSysregState::cntp_ctl_el0 as usize]);
	// crate::write_sysreg!(cntp_cval_el02, sysregs[NovSysregState::cntp_cval_el0 as usize]);
	// crate::write_sysreg!(cntv_ctl_el02, sysregs[NovSysregState::cntv_ctl_el0 as usize]);
	// crate::write_sysreg!(cntv_cval_el02, sysregs[NovSysregState::cntv_cval_el0 as usize]);
	drop(v_percpu_list);
}

#[inline] 
pub fn get_rec_sysregs(rec: &mut Rec, sysregs: NovSysregState) -> usize
{
	return rec.sysregs[sysregs as usize];
}

pub fn restore_sysreg_state(rec: &mut Rec)
{
	unsafe {c_restore_sysreg_state(&mut (rec.sysregs[0]) as *mut usize);}
	// crate::write_sysreg!(sp_el0, get_rec_sysregs(rec, NovSysregState::sp_el0));
	// crate::write_sysreg!(sp_el1, get_rec_sysregs(rec, NovSysregState::sp_el1));
	// crate::write_sysreg!(elr_el12, get_rec_sysregs(rec, NovSysregState::elr_el1));
	// crate::write_sysreg!(spsr_el12, get_rec_sysregs(rec, NovSysregState::spsr_el1));
	// crate::write_sysreg!(pmcr_el0, get_rec_sysregs(rec, NovSysregState::pmcr_el0));
	// crate::write_sysreg!(pmuserenr_el0, get_rec_sysregs(rec, NovSysregState::pmuserenr_el0));
	// crate::write_sysreg!(tpidrro_el0, get_rec_sysregs(rec, NovSysregState::tpidrro_el0));
	// crate::write_sysreg!(tpidr_el0, get_rec_sysregs(rec, NovSysregState::tpidr_el0));
	// crate::write_sysreg!(csselr_el1, get_rec_sysregs(rec, NovSysregState::csselr_el1));
	// crate::write_sysreg!(sctlr_el12, get_rec_sysregs(rec, NovSysregState::sctlr_el1));
	// crate::write_sysreg!(actlr_el1, get_rec_sysregs(rec, NovSysregState::actlr_el1));
	// crate::write_sysreg!(cpacr_el12, get_rec_sysregs(rec, NovSysregState::cpacr_el1));
	// // SVE(	crate::write_sysreg!(zcr_el1, get_rec_sysregs(rec, NovSysregState::zcr_el1));)
	// crate::write_sysreg!(ttbr0_el12, get_rec_sysregs(rec, NovSysregState::ttbr0_el1));
	// crate::write_sysreg!(ttbr1_el12, get_rec_sysregs(rec, NovSysregState::ttbr1_el1));
	// crate::write_sysreg!(tcr_el12, get_rec_sysregs(rec, NovSysregState::tcr_el1));
	// crate::write_sysreg!(esr_el12, get_rec_sysregs(rec, NovSysregState::esr_el1));
	// crate::write_sysreg!(afsr0_el12, get_rec_sysregs(rec, NovSysregState::afsr0_el1));
	// crate::write_sysreg!(afsr1_el12, get_rec_sysregs(rec, NovSysregState::afsr1_el1));
	// crate::write_sysreg!(far_el12, get_rec_sysregs(rec, NovSysregState::far_el1));
	// crate::write_sysreg!(mair_el12, get_rec_sysregs(rec, NovSysregState::mair_el1));
	// crate::write_sysreg!(vbar_el12, get_rec_sysregs(rec, NovSysregState::vbar_el1));
	// crate::write_sysreg!(contextidr_el12, get_rec_sysregs(rec, NovSysregState::contextidr_el1));
	// crate::write_sysreg!(tpidr_el1, get_rec_sysregs(rec, NovSysregState::tpidr_el1));
	// crate::write_sysreg!(amair_el12, get_rec_sysregs(rec, NovSysregState::amair_el1));
	// crate::write_sysreg!(cntkctl_el12, get_rec_sysregs(rec, NovSysregState::cntkctl_el1));
	// crate::write_sysreg!(par_el1, get_rec_sysregs(rec, NovSysregState::par_el1));
	// crate::write_sysreg!(mdscr_el1, get_rec_sysregs(rec, NovSysregState::mdscr_el1));
	// crate::write_sysreg!(mdccint_el1, get_rec_sysregs(rec, NovSysregState::mdccint_el1));
	// crate::write_sysreg!(disr_el1, get_rec_sysregs(rec, NovSysregState::disr_el1));
	// // MPAM(	crate::write_sysreg!(MPAM0_EL1, get_rec_sysregs(rec, NovSysregState::mpam0_el1));	)
	// crate::write_sysreg!(vmpidr_el2, get_rec_sysregs(rec, NovSysregState::vmpidr_el2));
	// crate::write_sysreg!(cnthctl_el2, get_rec_sysregs(rec, NovSysregState::cnthctl_el2));
	// crate::write_sysreg!(cntvoff_el2, get_rec_sysregs(rec, NovSysregState::cntvoff_el2));
	// crate::write_sysreg!(cntp_ctl_el02, get_rec_sysregs(rec, NovSysregState::cntp_ctl_el0));
	// crate::write_sysreg!(cntp_cval_el02, get_rec_sysregs(rec, NovSysregState::cntp_cval_el0));
	// crate::write_sysreg!(cntv_ctl_el02, get_rec_sysregs(rec, NovSysregState::cntv_ctl_el0));
	// crate::write_sysreg!(cntv_cval_el02, get_rec_sysregs(rec, NovSysregState::cntv_cval_el0));
}

#[inline] 
pub fn set_rec_sysregs(rec: &mut Rec, sysregs: NovSysregState, val: usize)
{
	rec.sysregs[sysregs as usize] = val;
}

pub fn save_sysreg_state(rec: &mut Rec)
{
	unsafe {c_save_sysreg_state(&mut (rec.sysregs[0]) as *mut usize);}
	// set_rec_sysregs(rec, NovSysregState::sp_el0, crate::read_sysreg!(sp_el0));
	// set_rec_sysregs(rec, NovSysregState::sp_el1, crate::read_sysreg!(sp_el1));
	// set_rec_sysregs(rec, NovSysregState::elr_el1, crate::read_sysreg!(elr_el12));
	// set_rec_sysregs(rec, NovSysregState::spsr_el1, crate::read_sysreg!(spsr_el12));
	// set_rec_sysregs(rec, NovSysregState::pmcr_el0, crate::read_sysreg!(pmcr_el0));
	// set_rec_sysregs(rec, NovSysregState::pmuserenr_el0, crate::read_sysreg!(pmuserenr_el0));
	// set_rec_sysregs(rec, NovSysregState::tpidrro_el0, crate::read_sysreg!(tpidrro_el0));
	// set_rec_sysregs(rec, NovSysregState::tpidr_el0, crate::read_sysreg!(tpidr_el0));
	// set_rec_sysregs(rec, NovSysregState::csselr_el1, crate::read_sysreg!(csselr_el1));
	// set_rec_sysregs(rec, NovSysregState::sctlr_el1, crate::read_sysreg!(sctlr_el12));
	// set_rec_sysregs(rec, NovSysregState::actlr_el1, crate::read_sysreg!(actlr_el1));
	// set_rec_sysregs(rec, NovSysregState::cpacr_el1, crate::read_sysreg!(cpacr_el12));
	// // SVE(	set_rec_sysregs(rec, NovSysregState::zcr_el1, crate::read_sysreg!(zcr_el1));)
	// set_rec_sysregs(rec, NovSysregState::ttbr0_el1, crate::read_sysreg!(ttbr0_el12));
	// set_rec_sysregs(rec, NovSysregState::ttbr1_el1, crate::read_sysreg!(ttbr1_el12));
	// set_rec_sysregs(rec, NovSysregState::tcr_el1, crate::read_sysreg!(tcr_el12));
	// set_rec_sysregs(rec, NovSysregState::esr_el1, crate::read_sysreg!(esr_el12));
	// set_rec_sysregs(rec, NovSysregState::afsr0_el1, crate::read_sysreg!(afsr0_el12));
	// set_rec_sysregs(rec, NovSysregState::afsr1_el1, crate::read_sysreg!(afsr1_el12));
	// set_rec_sysregs(rec, NovSysregState::far_el1, crate::read_sysreg!(far_el12));
	// set_rec_sysregs(rec, NovSysregState::mair_el1, crate::read_sysreg!(mair_el12));
	// set_rec_sysregs(rec, NovSysregState::vbar_el1, crate::read_sysreg!(vbar_el12));

	// set_rec_sysregs(rec, NovSysregState::contextidr_el1, crate::read_sysreg!(contextidr_el12));
	// set_rec_sysregs(rec, NovSysregState::tpidr_el1, crate::read_sysreg!(tpidr_el1));
	// set_rec_sysregs(rec, NovSysregState::amair_el1, crate::read_sysreg!(amair_el12));
	// set_rec_sysregs(rec, NovSysregState::cntkctl_el1, crate::read_sysreg!(cntkctl_el12));
	// set_rec_sysregs(rec, NovSysregState::par_el1, crate::read_sysreg!(par_el1));
	// set_rec_sysregs(rec, NovSysregState::mdscr_el1, crate::read_sysreg!(mdscr_el1));
	// set_rec_sysregs(rec, NovSysregState::mdccint_el1, crate::read_sysreg!(mdccint_el1));
	// set_rec_sysregs(rec, NovSysregState::disr_el1, crate::read_sysreg!(disr_el1));
	// // MPAM(	set_rec_sysregs(rec, NovSysregState::mpam0_el1, crate::read_sysreg!(MPAM0_EL1));)

	// set_rec_sysregs(rec, NovSysregState::cnthctl_el2, crate::read_sysreg!(cnthctl_el2));
	// set_rec_sysregs(rec, NovSysregState::cntvoff_el2, crate::read_sysreg!(cntvoff_el2));
	// set_rec_sysregs(rec, NovSysregState::cntp_ctl_el0, crate::read_sysreg!(cntp_ctl_el02));
	// set_rec_sysregs(rec, NovSysregState::cntp_cval_el0, crate::read_sysreg!(cntp_cval_el02));
	// set_rec_sysregs(rec, NovSysregState::cntv_ctl_el0, crate::read_sysreg!(cntv_ctl_el02));
	// set_rec_sysregs(rec, NovSysregState::cntv_cval_el0, crate::read_sysreg!(cntv_cval_el02));
}