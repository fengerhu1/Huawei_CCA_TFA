#include <sysreg.h>
extern void print_info(const char *fmt, ...);


#define HAS_MPAM 0

#if HAS_MPAM
#define MPAM(_x...) _x
#else
#define MPAM(_x...)
#endif

#define HAS_SVE 0

#if HAS_SVE
#define SVE(_x...) _x
#else
#define SVE(_x...)
#endif

enum nov_sysreg_state {
	sp_el0,
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

	NR_SYSREG_STATE
};

void set_vttbr_el2(unsigned long val) {
	sysreg_write(vttbr_el2, val);
}

unsigned long read_vttbr_el2() {
	return sysreg_read(vttbr_el2);
}

void set_cntv_ctl_el02(unsigned long val) {
	sysreg_write(cntv_ctl_el02, val);
}

unsigned long read_cntv_ctl_el02() {
	return sysreg_read(cntv_ctl_el02);
}

unsigned long read_cpacr_el12() {
	return sysreg_read(cpacr_el12);
}

void set_cpacr_el12(unsigned long val) {
	sysreg_write(cpacr_el12, val);
}

void set_sctlr_el12(unsigned long val) {
	sysreg_write(sctlr_el12, val);
}
unsigned long read_sctlr_el12() {
	return sysreg_read(sctlr_el12);
}

inline static void set_ns_state(unsigned long *sysregs_ptr, enum nov_sysreg_state sysregs,
				unsigned long val)
{
	sysregs_ptr[sysregs] = val;
}

void c_save_ns_state_sysreg_state(unsigned long *sysregs)
{
	set_ns_state(sysregs, sp_el0, sysreg_read(sp_el0));
	set_ns_state(sysregs, sp_el1, sysreg_read(sp_el1));
	set_ns_state(sysregs, elr_el1, sysreg_read(elr_el12));
	set_ns_state(sysregs, spsr_el1, sysreg_read(spsr_el12));
	set_ns_state(sysregs, pmcr_el0, sysreg_read(pmcr_el0));
	set_ns_state(sysregs, pmuserenr_el0, sysreg_read(pmuserenr_el0));
	set_ns_state(sysregs, tpidrro_el0, sysreg_read(tpidrro_el0));
	set_ns_state(sysregs, tpidr_el0, sysreg_read(tpidr_el0));
	set_ns_state(sysregs, csselr_el1, sysreg_read(csselr_el1));
	set_ns_state(sysregs, sctlr_el1, sysreg_read(sctlr_el12));
	set_ns_state(sysregs, actlr_el1, sysreg_read(actlr_el1));
	set_ns_state(sysregs, cpacr_el1, sysreg_read(cpacr_el12));
SVE(	set_ns_state(sysregs, zcr_el1, sysreg_read(zcr_el1));)
	set_ns_state(sysregs, ttbr0_el1, sysreg_read(ttbr0_el12));
	set_ns_state(sysregs, ttbr1_el1, sysreg_read(ttbr1_el12));
	set_ns_state(sysregs, tcr_el1, sysreg_read(tcr_el12));
	set_ns_state(sysregs, esr_el1, sysreg_read(esr_el12));
	set_ns_state(sysregs, afsr0_el1, sysreg_read(afsr0_el12));
	set_ns_state(sysregs, afsr1_el1, sysreg_read(afsr1_el12));
	set_ns_state(sysregs, far_el1, sysreg_read(far_el12));
	set_ns_state(sysregs, mair_el1, sysreg_read(mair_el12));
	set_ns_state(sysregs, vbar_el1, sysreg_read(vbar_el12));

	set_ns_state(sysregs, contextidr_el1, sysreg_read(contextidr_el12));
	set_ns_state(sysregs, tpidr_el1, sysreg_read(tpidr_el1));
	set_ns_state(sysregs, amair_el1, sysreg_read(amair_el12));
	set_ns_state(sysregs, cntkctl_el1, sysreg_read(cntkctl_el12));
	set_ns_state(sysregs, par_el1, sysreg_read(par_el1));
	set_ns_state(sysregs, mdscr_el1, sysreg_read(mdscr_el1));
	set_ns_state(sysregs, mdccint_el1, sysreg_read(mdccint_el1));
	set_ns_state(sysregs, disr_el1, sysreg_read(disr_el1));
MPAM(	set_ns_state(sysregs, mpam0_el1, sysreg_read(MPAM0_EL1));	)
	set_ns_state(sysregs, cnthctl_el2, sysreg_read(cnthctl_el2));
	set_ns_state(sysregs, cntvoff_el2, sysreg_read(cntvoff_el2));
	set_ns_state(sysregs, cntp_ctl_el0, sysreg_read(cntp_ctl_el02));
	set_ns_state(sysregs, cntp_cval_el0, sysreg_read(cntp_cval_el02));
	set_ns_state(sysregs, cntv_ctl_el0, sysreg_read(cntv_ctl_el02));
	set_ns_state(sysregs, cntv_cval_el0, sysreg_read(cntv_cval_el02));
}

inline static unsigned long get_ns_state(unsigned long *sysregs_ptr, enum nov_sysreg_state sysregs)
{
	return sysregs_ptr[sysregs];
}

void c_restore_ns_state_sysreg_state(unsigned long *sysregs)
{
	sysreg_write(sp_el0, get_ns_state(sysregs, sp_el0));
	sysreg_write(sp_el1, get_ns_state(sysregs, sp_el1));
	sysreg_write(elr_el12, get_ns_state(sysregs, elr_el1));
	sysreg_write(spsr_el12, get_ns_state(sysregs, spsr_el1));
	sysreg_write(pmcr_el0, get_ns_state(sysregs, pmcr_el0));
	sysreg_write(pmuserenr_el0, get_ns_state(sysregs, pmuserenr_el0));
	sysreg_write(tpidrro_el0, get_ns_state(sysregs, tpidrro_el0));
	sysreg_write(tpidr_el0, get_ns_state(sysregs, tpidr_el0));
	sysreg_write(csselr_el1, get_ns_state(sysregs, csselr_el1));
	sysreg_write(sctlr_el12, get_ns_state(sysregs, sctlr_el1));
	sysreg_write(actlr_el1, get_ns_state(sysregs, actlr_el1));
	sysreg_write(cpacr_el12, get_ns_state(sysregs, cpacr_el1));
SVE(	sysreg_write(zcr_el1, get_ns_state(sysregs, zcr_el1));)
	sysreg_write(ttbr0_el12, get_ns_state(sysregs, ttbr0_el1));
	sysreg_write(ttbr1_el12, get_ns_state(sysregs, ttbr1_el1));
	sysreg_write(tcr_el12, get_ns_state(sysregs, tcr_el1));
	sysreg_write(esr_el12, get_ns_state(sysregs, esr_el1));
	sysreg_write(afsr0_el12, get_ns_state(sysregs, afsr0_el1));
	sysreg_write(afsr1_el12, get_ns_state(sysregs, afsr1_el1));
	sysreg_write(far_el12, get_ns_state(sysregs, far_el1));
	sysreg_write(mair_el12, get_ns_state(sysregs, mair_el1));
	sysreg_write(vbar_el12, get_ns_state(sysregs, vbar_el1));
	sysreg_write(contextidr_el12, get_ns_state(sysregs, contextidr_el1));
	sysreg_write(tpidr_el1, get_ns_state(sysregs, tpidr_el1));
	sysreg_write(amair_el12, get_ns_state(sysregs, amair_el1));
	sysreg_write(cntkctl_el12, get_ns_state(sysregs, cntkctl_el1));
	sysreg_write(par_el1, get_ns_state(sysregs, par_el1));
	sysreg_write(mdscr_el1, get_ns_state(sysregs, mdscr_el1));
	sysreg_write(mdccint_el1, get_ns_state(sysregs, mdccint_el1));
	sysreg_write(disr_el1, get_ns_state(sysregs, disr_el1));
MPAM(	sysreg_write(MPAM0_EL1, get_ns_state(sysregs, mpam0_el1));	)
	sysreg_write(vmpidr_el2, get_ns_state(sysregs, vmpidr_el2));
	sysreg_write(cnthctl_el2, get_ns_state(sysregs, cnthctl_el2));
	sysreg_write(cntvoff_el2, get_ns_state(sysregs, cntvoff_el2));
	sysreg_write(cntp_ctl_el02, get_ns_state(sysregs, cntp_ctl_el0));
	sysreg_write(cntp_cval_el02, get_ns_state(sysregs, cntp_cval_el0));
	sysreg_write(cntv_ctl_el02, get_ns_state(sysregs, cntv_ctl_el0));
	sysreg_write(cntv_cval_el02, get_ns_state(sysregs, cntv_cval_el0));
}

inline static void set_rec_sysregs(unsigned long *sysregs_ptr,
				   enum nov_sysreg_state sysregs,
				   unsigned long val)
{
	sysregs_ptr[sysregs] = val;
}

void c_save_sysreg_state(unsigned long *sysregs)
{
	set_rec_sysregs(sysregs, sp_el0, sysreg_read(sp_el0));
	set_rec_sysregs(sysregs, sp_el1, sysreg_read(sp_el1));
	set_rec_sysregs(sysregs, elr_el1, sysreg_read(elr_el12));
	set_rec_sysregs(sysregs, spsr_el1, sysreg_read(spsr_el12));
	set_rec_sysregs(sysregs, pmcr_el0, sysreg_read(pmcr_el0));
	set_rec_sysregs(sysregs, pmuserenr_el0, sysreg_read(pmuserenr_el0));
	set_rec_sysregs(sysregs, tpidrro_el0, sysreg_read(tpidrro_el0));
	set_rec_sysregs(sysregs, tpidr_el0, sysreg_read(tpidr_el0));
	set_rec_sysregs(sysregs, csselr_el1, sysreg_read(csselr_el1));
	set_rec_sysregs(sysregs, sctlr_el1, sysreg_read(sctlr_el12));
	set_rec_sysregs(sysregs, actlr_el1, sysreg_read(actlr_el1));
	set_rec_sysregs(sysregs, cpacr_el1, sysreg_read(cpacr_el12));
SVE(	set_rec_sysregs(sysregs, zcr_el1, sysreg_read(zcr_el1));)
	set_rec_sysregs(sysregs, ttbr0_el1, sysreg_read(ttbr0_el12));
	set_rec_sysregs(sysregs, ttbr1_el1, sysreg_read(ttbr1_el12));
	set_rec_sysregs(sysregs, tcr_el1, sysreg_read(tcr_el12));
	set_rec_sysregs(sysregs, esr_el1, sysreg_read(esr_el12));
	set_rec_sysregs(sysregs, afsr0_el1, sysreg_read(afsr0_el12));
	set_rec_sysregs(sysregs, afsr1_el1, sysreg_read(afsr1_el12));
	set_rec_sysregs(sysregs, far_el1, sysreg_read(far_el12));
	set_rec_sysregs(sysregs, mair_el1, sysreg_read(mair_el12));
	set_rec_sysregs(sysregs, vbar_el1, sysreg_read(vbar_el12));

	set_rec_sysregs(sysregs, contextidr_el1, sysreg_read(contextidr_el12));
	set_rec_sysregs(sysregs, tpidr_el1, sysreg_read(tpidr_el1));
	set_rec_sysregs(sysregs, amair_el1, sysreg_read(amair_el12));
	set_rec_sysregs(sysregs, cntkctl_el1, sysreg_read(cntkctl_el12));
	set_rec_sysregs(sysregs, par_el1, sysreg_read(par_el1));
	set_rec_sysregs(sysregs, mdscr_el1, sysreg_read(mdscr_el1));
	set_rec_sysregs(sysregs, mdccint_el1, sysreg_read(mdccint_el1));
	set_rec_sysregs(sysregs, disr_el1, sysreg_read(disr_el1));
MPAM(	set_rec_sysregs(sysregs, mpam0_el1, sysreg_read(MPAM0_EL1));)

	set_rec_sysregs(sysregs, cnthctl_el2, sysreg_read(cnthctl_el2));
	set_rec_sysregs(sysregs, cntvoff_el2, sysreg_read(cntvoff_el2));
	set_rec_sysregs(sysregs, cntp_ctl_el0, sysreg_read(cntp_ctl_el02));
	set_rec_sysregs(sysregs, cntp_cval_el0, sysreg_read(cntp_cval_el02));
	set_rec_sysregs(sysregs, cntv_ctl_el0, sysreg_read(cntv_ctl_el02));
	set_rec_sysregs(sysregs, cntv_cval_el0, sysreg_read(cntv_cval_el02));
}

inline static unsigned long get_rec_sysregs(unsigned long *sysregs_ptr,
					    enum nov_sysreg_state sysregs)
{
	return sysregs_ptr[sysregs];
}

void c_restore_sysreg_state(unsigned long *sysregs)
{
	sysreg_write(sp_el0, get_rec_sysregs(sysregs, sp_el0));
	sysreg_write(sp_el1, get_rec_sysregs(sysregs, sp_el1));
	sysreg_write(elr_el12, get_rec_sysregs(sysregs, elr_el1));
	sysreg_write(spsr_el12, get_rec_sysregs(sysregs, spsr_el1));
	sysreg_write(pmcr_el0, get_rec_sysregs(sysregs, pmcr_el0));
	sysreg_write(pmuserenr_el0, get_rec_sysregs(sysregs, pmuserenr_el0));
	sysreg_write(tpidrro_el0, get_rec_sysregs(sysregs, tpidrro_el0));
	sysreg_write(tpidr_el0, get_rec_sysregs(sysregs, tpidr_el0));
	sysreg_write(csselr_el1, get_rec_sysregs(sysregs, csselr_el1));
	sysreg_write(sctlr_el12, get_rec_sysregs(sysregs, sctlr_el1));
	sysreg_write(actlr_el1, get_rec_sysregs(sysregs, actlr_el1));
	sysreg_write(cpacr_el12, get_rec_sysregs(sysregs, cpacr_el1));
SVE(	sysreg_write(zcr_el1, get_rec_sysregs(sysregs, zcr_el1));)
	sysreg_write(ttbr0_el12, get_rec_sysregs(sysregs, ttbr0_el1));
	sysreg_write(ttbr1_el12, get_rec_sysregs(sysregs, ttbr1_el1));
	sysreg_write(tcr_el12, get_rec_sysregs(sysregs, tcr_el1));
	sysreg_write(esr_el12, get_rec_sysregs(sysregs, esr_el1));
	sysreg_write(afsr0_el12, get_rec_sysregs(sysregs, afsr0_el1));
	sysreg_write(afsr1_el12, get_rec_sysregs(sysregs, afsr1_el1));
	sysreg_write(far_el12, get_rec_sysregs(sysregs, far_el1));
	sysreg_write(mair_el12, get_rec_sysregs(sysregs, mair_el1));
	sysreg_write(vbar_el12, get_rec_sysregs(sysregs, vbar_el1));
	sysreg_write(contextidr_el12, get_rec_sysregs(sysregs, contextidr_el1));
	sysreg_write(tpidr_el1, get_rec_sysregs(sysregs, tpidr_el1));
	sysreg_write(amair_el12, get_rec_sysregs(sysregs, amair_el1));
	sysreg_write(cntkctl_el12, get_rec_sysregs(sysregs, cntkctl_el1));
	sysreg_write(par_el1, get_rec_sysregs(sysregs, par_el1));
	sysreg_write(mdscr_el1, get_rec_sysregs(sysregs, mdscr_el1));
	sysreg_write(mdccint_el1, get_rec_sysregs(sysregs, mdccint_el1));
	sysreg_write(disr_el1, get_rec_sysregs(sysregs, disr_el1));
MPAM(	sysreg_write(MPAM0_EL1, get_rec_sysregs(sysregs, mpam0_el1));	)
	sysreg_write(vmpidr_el2, get_rec_sysregs(sysregs, vmpidr_el2));
	sysreg_write(cnthctl_el2, get_rec_sysregs(sysregs, cnthctl_el2));
	sysreg_write(cntvoff_el2, get_rec_sysregs(sysregs, cntvoff_el2));
	sysreg_write(cntp_ctl_el02, get_rec_sysregs(sysregs, cntp_ctl_el0));
	sysreg_write(cntp_cval_el02, get_rec_sysregs(sysregs, cntp_cval_el0));
	sysreg_write(cntv_ctl_el02, get_rec_sysregs(sysregs, cntv_ctl_el0));
	sysreg_write(cntv_cval_el02, get_rec_sysregs(sysregs, cntv_cval_el0));
}