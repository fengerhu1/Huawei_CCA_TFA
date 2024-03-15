#![feature(asm)]
use crate::println;
use crate::io::Write;

extern "C" {
    pub fn asm_isb();
}

pub fn enable_timer() {
    crate::println!("Enabling timer");

    let mut pmcrfilter_el0 = crate::read_sysreg!(PMCCFILTR_EL0);
    crate::write_sysreg!(PMCCFILTR_EL0, pmcrfilter_el0 | (1 << 27));

    let pmcntenset = crate::read_sysreg!(PMCNTENSET_EL0);
    crate::write_sysreg!(PMCNTENSET_EL0, pmcntenset | (1 << 31));

    let mut pmcr = crate::read_sysreg!(PMCR_EL0);
    crate::write_sysreg!(PMCR_EL0, pmcr | 0b1);

    unsafe{
        asm_isb();
    }

}

pub fn read_timer(cycle: &mut usize) {
    crate::println!("Reading timer");
    *cycle = crate::read_sysreg!(PMCCNTR_EL0);
    crate::println!("Cycle: {:x}", *cycle);
}