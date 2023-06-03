
use super::vm::TitaniumVM;
use alloc::boxed::Box;

const TITANIUM_VMEXIT_SYNC: usize = 0;
const TITANIUM_VMEXIT_IRQ: usize = 1;
const TITANIUM_VMEXIT_FIQ: usize = 2;
const TITANIUM_VMEXIT_ERR: usize = 3;
const TITANIUM_HYP_SYNC: usize = 4;
const TITANIUM_HYP_IRQ: usize = 5;
const TITANIUM_HYP_FIQ: usize = 6;
const TITANIUM_HYP_ERR: usize = 7;

const ESR_ELx_EC_UNKNOWN: usize = (0x00);
const ESR_ELx_EC_WFx: usize = (0x01);
/* Unallocated EC: 0x02 */
const ESR_ELx_EC_CP15_32: usize = (0x03);
const ESR_ELx_EC_CP15_64: usize = (0x04);
const ESR_ELx_EC_CP14_MR: usize = (0x05);
const ESR_ELx_EC_CP14_LS: usize = (0x06);
const ESR_ELx_EC_FP_ASIMD: usize = (0x07);
const ESR_ELx_EC_CP10_ID: usize = (0x08); /* EL2 only */
const ESR_ELx_EC_PAC: usize = (0x09); /* EL2 and above */
/* Unallocated EC: 0x0A - 0x0B */
const ESR_ELx_EC_CP14_64: usize = (0x0C);
/* Unallocated EC: 0x0d */
const ESR_ELx_EC_ILL: usize = (0x0E);
/* Unallocated EC: 0x0F - 0x10 */
const ESR_ELx_EC_SVC32: usize = (0x11);
const ESR_ELx_EC_HVC32: usize = (0x12); /* EL2 only */
const ESR_ELx_EC_SMC32: usize = (0x13); /* EL2 and above */
/* Unallocated EC: 0x14 */
const ESR_ELx_EC_SVC64: usize = (0x15);
const ESR_ELx_EC_HVC64: usize = (0x16); /* EL2 and above */
const ESR_ELx_EC_SMC64: usize = (0x17); /* EL2 and above */
const ESR_ELx_EC_SYS64: usize = (0x18);
const ESR_ELx_EC_SVE: usize = (0x19);
/* Unallocated EC: 0x1A - 0x1E */
const ESR_ELx_EC_IMP_DEF: usize = (0x1f); /* EL3 only */
const ESR_ELx_EC_IABT_LOW: usize = (0x20);
const ESR_ELx_EC_IABT_CUR: usize = (0x21);
const ESR_ELx_EC_PC_ALIGN: usize = (0x22);
/* Unallocated EC: 0x23 */
const ESR_ELx_EC_DABT_LOW: usize = (0x24);
const ESR_ELx_EC_DABT_CUR: usize = (0x25);
const ESR_ELx_EC_SP_ALIGN: usize = (0x26);
/* Unallocated EC: 0x27 */
const ESR_ELx_EC_FP_EXC32: usize = (0x28);
/* Unallocated EC: 0x29 - 0x2B */
const ESR_ELx_EC_FP_EXC64: usize = (0x2C);
/* Unallocated EC: 0x2D - 0x2E */
const ESR_ELx_EC_SERROR: usize = (0x2F);
const ESR_ELx_EC_BREAKPT_LOW: usize = (0x30);
const ESR_ELx_EC_BREAKPT_CUR: usize = (0x31);
const ESR_ELx_EC_SOFTSTP_LOW: usize = (0x32);
const ESR_ELx_EC_SOFTSTP_CUR: usize = (0x33);
const ESR_ELx_EC_WATCHPT_LOW: usize = (0x34);
const ESR_ELx_EC_WATCHPT_CUR: usize = (0x35);
/* Unallocated EC: 0x36 - 0x37 */
const ESR_ELx_EC_BKPT32: usize = (0x38);
/* Unallocated EC: 0x39 */
const ESR_ELx_EC_VECTOR32: usize = (0x3A); /* EL2 only */
/* Unallocted EC: 0x3B */
const ESR_ELx_EC_BRK64: usize = (0x3C);
/* Unallocated EC: 0x3D - 0x3F */

const ESR_ELx_EC_MAX: usize = (0x3F);

const ESR_EL_EC_SHIFT: usize = (26);
const ESR_EL_EC_MASK: usize = ((0x3F) << ESR_EL_EC_SHIFT);
//TODO: translate to macro_rule!
// #define ESR_EL_EC(esr)		(((esr) & ESR_EL_EC_MASK) >> ESR_EL_EC_SHIFT)
macro_rules! ESR_EL_EC {
    ($esr: ident) => {
        ((($esr) & ESR_EL_EC_MASK) >> ESR_EL_EC_SHIFT)
    };
}

/* Shared ISS field definitions for Data/Instruction aborts */
const ESR_ELx_SET_SHIFT: usize = (11);
const ESR_ELx_SET_MASK: usize = ((3) << ESR_ELx_SET_SHIFT);
const ESR_ELx_FnV_SHIFT: usize = (10);
const ESR_ELx_FnV: usize = ((1) << ESR_ELx_FnV_SHIFT);
const ESR_ELx_EA_SHIFT: usize = (9);
const ESR_ELx_EA: usize = ((1) << ESR_ELx_EA_SHIFT);
const ESR_ELx_S1PTW_SHIFT: usize = (7);
const ESR_ELx_S1PTW: usize = ((1) << ESR_ELx_S1PTW_SHIFT);

/* Shared ISS fault status code(IFSC/DFSC) for Data/Instruction aborts */
const ESR_ELx_FSC: usize = (0x3F);
const ESR_ELx_FSC_TYPE: usize = (0x3C);
const ESR_ELx_FSC_EXTABT: usize = (0x10);
const ESR_ELx_FSC_SERROR: usize = (0x11);
const ESR_ELx_FSC_ACCESS: usize = (0x08);
const ESR_ELx_FSC_FAULT: usize = (0x04);
const ESR_ELx_FSC_PERM: usize = (0x0C);

/* ISS field definitions for Data Aborts */
const ESR_ELx_ISV_SHIFT: usize = (24);
const ESR_ELx_ISV: usize = ((1) << ESR_ELx_ISV_SHIFT);
const ESR_ELx_SAS_SHIFT: usize = (22);
const ESR_ELx_SAS: usize = ((3) << ESR_ELx_SAS_SHIFT);
const ESR_ELx_SSE_SHIFT: usize = (21);
const ESR_ELx_SSE: usize = ((1) << ESR_ELx_SSE_SHIFT);
const ESR_ELx_SRT_SHIFT: usize = (16);
const ESR_ELx_SRT_MASK: usize = ((0x1F) << ESR_ELx_SRT_SHIFT);
const ESR_ELx_SF_SHIFT: usize = (15);
const ESR_ELx_SF: usize = ((1) << ESR_ELx_SF_SHIFT);
const ESR_ELx_AR_SHIFT: usize = (14);
const ESR_ELx_AR: usize = ((1) << ESR_ELx_AR_SHIFT);
const ESR_ELx_VNCR_SHIFT: usize = (13);
const ESR_ELx_VNCR: usize = ((1) << ESR_ELx_VNCR_SHIFT);
const ESR_ELx_CM_SHIFT: usize = (8);
const ESR_ELx_CM: usize = ((1) << ESR_ELx_CM_SHIFT);
const ESR_ELx_WNR_SHIFT: usize = (6);
const ESR_ELx_WNR: usize = ((1) << ESR_ELx_WNR_SHIFT);

/* ISS field definitions for exceptions taken in to Hyp */
const ESR_ELx_CV: usize = ((1) << 24);
const ESR_ELx_COND_SHIFT: usize = (20);
const ESR_ELx_COND_MASK: usize = ((0xF) << ESR_ELx_COND_SHIFT);
const ESR_ELx_WFx_ISS_TI: usize = ((1) << 0);
const ESR_ELx_WFx_ISS_WFI: usize = ((0) << 0);
const ESR_ELx_WFx_ISS_WFE: usize = ((1) << 0);
const ESR_ELx_xVC_IMM_MASK: usize = ((1 << 16) - 1);

/// Struct definition below

#[repr(C)]
pub enum VcpuState {
    VcpuInit = 0,
    VcpuReady,
    VcpuTrapped,
    VcpuRunning,
    VcpuDestroy,
}

#[repr(C)]
pub struct GpRegs {
    x: [usize; 30],
    lr: usize,
    pc: usize,
}

#[repr(C)]
pub struct SysRegs {
    spsr: usize,
    elr: usize,
    sctlr: usize,
    sp: usize,
    sp_el0: usize,
    esr: usize,
    vbar: usize,
    ttbr0: usize,
    ttbr1: usize,
    mair: usize,
    amair: usize,
    tcr: usize,
    tpidr: usize,
}

#[repr(C)]
pub struct VcpuCtx {
    pub gp_regs: GpRegs,
    sys_regs: SysRegs,
}

#[repr(C)]
pub struct TitaniumHostRegs {
    gp_regs: GpRegs,
    sys_regs: SysRegs,
    far_el2: usize,
    hpfar_el2: usize,
    vmpidr_el2: usize,
}

#[repr(C)]
pub struct ThreadVectorTable {
    std_smc_entry: usize,
    fast_smc_entry: usize,
    cpu_on_entry: usize,
    cpu_off_entry: usize,
    cpu_resume_entry: usize,
    cpu_suspend_entry: usize,
    fiq_entry: usize,
    system_off_entry: usize,
    system_reset_entry: usize,
}

#[repr(C)]
pub struct TitaniumState {
    host_state: TitaniumHostRegs,
    current_vcpu_ctx: *mut VcpuCtx,
    current_vm: *mut TitaniumVM,
    current_vcpu_id: i32,
    ret_lr: usize,
    tmp_gp_regs: GpRegs,
}

#[repr(C)]
pub struct TitaniumVCPU {
    pub vm: *mut TitaniumVM,
    pub vcpu_id: isize,
    pub vcpu_ctx: *mut VcpuCtx,
    pub vcpu_state: VcpuState,
    pub entrypoint: usize,
    pub entry_spsr: usize,
}

#[repr(C)]
pub struct TitaniumVMOps {
    smc_handler: extern "C" fn(*mut TitaniumState) -> isize,
}

/*
 * Resolve the IPA the hard way using the guest VA.
 *
 * Stage-1 translation already validated the memory access
 * rights. As such, we can use the EL1 translation regime, and
 * don't have to distinguish between EL0 and EL1 access.
 *
 * We do need to save/restore PAR_EL1 though, as we haven't
 * saved the guest context yet, and we may return early...
 */
// pub fn translate_far_to_hpfar(far: usize) -> usize {
//     let par = read_sysreg!(par_el1);
//     asm!("at s1e1r, {0}", in(reg) far);

// }

use crate::util::list::ListHead;

//TODO: use c-version for tmp use
pub const PHYSICAL_CORE_NUM: usize = 16;
extern "C" {
    pub fn translate_far_to_hpfar(far: usize) -> usize;
    pub fn get_core_id() -> usize;
    pub fn titanium_handle_exit(exit_reason: usize, state: *mut TitaniumState) -> isize;

    static mut global_titanium_states: [TitaniumState; PHYSICAL_CORE_NUM];
    pub fn enter_guest() -> usize;
    pub static mut titanium_vm_list: ListHead;
    pub fn context_switch_to_vcpu(next_vm: *mut TitaniumVM, vcpu_id: usize, core_id: usize);
    pub fn run_vcpu();
}



pub fn just_test() -> isize {
    1
}

const HVC_FUNC_MAX: usize = 56;
const HVC_SECURE_FUNC_MAX: usize = 56;

//TODO: recover from here: the function pointer array

// static fn_array:[fn() -> isize;1] = {

// };

const ARM_SMCCC_STD_CALL: usize = 0;
const ARM_SMCCC_FAST_CALL: usize = 1;
const ARM_PSCI_CPU_ON: usize = 2;
const ARM_PSCI_CPU_OFF: usize = 3;
const ARM_PSCI_CPU_RESUME: usize = 4;
const ARM_PSCI_CPU_SUSPEND: usize = 5;
const ARM_PSCI_FIQ: usize = 6;
const ARM_PSCI_SYSTEM_OFF: usize = 7;
const ARM_PSCI_SYSTEM_RESET: usize = 8;

pub const OPTEE_VM_ID:usize =  0;
pub const TSP_VM_ID:usize = 1;

use crate::virt::vm::rust_get_vm_by_id;

impl TitaniumState {
    /**                                                                                           │
     * (From Linux)                                                                               │
     * The HPFAR can be invalid if the stage 2 fault did not                                      │
     * happen during a stage 1 page table walk (the ESR_EL2.S1PTW                                 │
     * bit is clear) and one of the two following cases are true:                                 │
     *   1. The fault was due to a permission fault                                               │
     *   2. The processor carries errata 834220                                                   │
     *                                                                                            │
     * We do not consider errata 834220 here.                                                     │
     * Therefore, for all non S1PTW faults where we have a                                        │
     * permission fault, we resolve the IPA using the AT instruction.                             │
     **/
    pub fn titanium_get_vcpu_fault_ipa(&mut self) -> usize {
        let guest_vcpu_esr = self.host_state.sys_regs.esr;
        let far = self.host_state.far_el2;
        let hpfar: usize;

        if (guest_vcpu_esr & ESR_ELx_S1PTW == 0)
            && (guest_vcpu_esr & ESR_ELx_FSC_TYPE == ESR_ELx_FSC_PERM)
        {
            hpfar = unsafe { translate_far_to_hpfar(far) };
        } else {
            hpfar = self.host_state.hpfar_el2;
        }
        (hpfar << 8) + (far & super::s2mmu::PAGE_MASK)
    }

    pub fn titanium_get_vcpu_fault_va(&self) -> usize {
        self.host_state.far_el2
    }

    pub fn titanium_get_vcpu_fault_reason(&self) -> usize {
        let guest_vcpu_esr = self.host_state.sys_regs.esr;
        ESR_EL_EC!(guest_vcpu_esr)
    }

    pub fn titanium_is_instruction_abort(&self) -> bool {
        let guest_vcpu_esr = self.host_state.sys_regs.esr;
        ESR_EL_EC!(guest_vcpu_esr) == ESR_ELx_EC_IABT_LOW
    }

    pub fn titanium_is_write_abort(&self) -> isize {
        let guest_vcpu_esr = self.host_state.sys_regs.esr;
        if ESR_EL_EC!(guest_vcpu_esr) == ESR_ELx_EC_DABT_LOW {
            return ((guest_vcpu_esr & ESR_ELx_WNR) >> ESR_ELx_WNR_SHIFT) as isize;
        } else {
            0
        }
    }

    pub fn handle_guest_stage2_abort(&mut self) -> isize {
        let fault_ipa = self.titanium_get_vcpu_fault_ipa();
        let fault_va = self.titanium_get_vcpu_fault_va();
        let fault_reason = self.titanium_get_vcpu_fault_reason();
        let is_instruction_abort = self.titanium_is_instruction_abort();
        let is_write_abort: usize = match is_instruction_abort {
            true => 0,
            false => self.titanium_is_write_abort() as usize,
        };

        let vm_ptr: *mut TitaniumVM = self.current_vm;
        let s2mmu_ptr = unsafe { (*vm_ptr).s2mmu };
        let mut s2mmu = unsafe { Box::from_raw(s2mmu_ptr) };
        let ret = s2mmu.as_mut().handle_guest_stage2_page_fault(
            fault_reason,
            is_instruction_abort as usize,
            is_write_abort,
            fault_ipa,
            fault_va,
        );
        let _useless = Box::into_raw(s2mmu);
        if ret == 0 {
            return 1;
        } else {
            panic!("Failed to handle stage 2 page fault");
        }
    }

    //TODO: not done yet
    pub fn titanium_handle_exit(&mut self, exit_reason: usize) -> isize {
        let guest_vcpu_esr = self.host_state.sys_regs.esr;
        let mut vmresume = 0;
        // match exit_reason {
        //     TITANIUM_VMEXIT_SYNC => {
        //         vmresume =
        //     },
        // }
        //TODO: we need to impl titanium_exit_handler first;
        let func_ptr: fn() -> isize = just_test;
        let array: [fn() -> isize; 2] = [func_ptr, func_ptr];
        0
    }

    //TODO: not done yet
    pub fn forward_smc_to_vm(&mut self, smc_type: usize) {
        let core_id = unsafe{get_core_id()};
        let vm_id = self.tmp_gp_regs.x[1];
        let a0 = self.tmp_gp_regs.x[0];
        let a1 = self.tmp_gp_regs.x[2];
        let a2 = self.tmp_gp_regs.x[3];
        let a3 = self.tmp_gp_regs.x[4];
        let a4 = self.tmp_gp_regs.x[5];
        let a5 = self.tmp_gp_regs.x[6];

        let vm_ptr: *mut TitaniumVM = rust_get_vm_by_id(vm_id);
        unsafe{
            context_switch_to_vcpu(vm_ptr, core_id, core_id);
        }
        
        if vm_id == TSP_VM_ID || vm_id == OPTEE_VM_ID {
            unsafe{
                (*self.current_vcpu_ctx).gp_regs.x[0] = a0;
                (*self.current_vcpu_ctx).gp_regs.x[1] = a1;
                (*self.current_vcpu_ctx).gp_regs.x[2] = a2;
                (*self.current_vcpu_ctx).gp_regs.x[3] = a3;
                (*self.current_vcpu_ctx).gp_regs.x[4] = a4;
                (*self.current_vcpu_ctx).gp_regs.x[5] = a5;
            }
        }

        match smc_type {
            ARM_SMCCC_STD_CALL => unsafe{
                self.host_state.sys_regs.elr = (*(*(self.current_vm)).thread_vector_table).std_smc_entry
            },
            ARM_SMCCC_FAST_CALL => unsafe{
                self.host_state.sys_regs.elr = (*(*(self.current_vm)).thread_vector_table).fast_smc_entry
            },
            _ => panic!("Unsupported Type")
        }

        unsafe{
            run_vcpu();
            (*((*self.current_vm).vcpus[core_id])).vcpu_state = VcpuState::VcpuReady;
        }
        

    }

    pub fn install_vcpu_eret_address(&mut self, command: usize) {
        let thread_vector_table: &ThreadVectorTable = unsafe{
            &(*(*self.current_vm).thread_vector_table)
        };
        match command {
            ARM_PSCI_CPU_ON => self.host_state.sys_regs.elr = thread_vector_table.cpu_on_entry,
            ARM_PSCI_CPU_OFF => self.host_state.sys_regs.elr = thread_vector_table.cpu_off_entry,
            ARM_PSCI_CPU_RESUME => self.host_state.sys_regs.elr = thread_vector_table.cpu_resume_entry,
            ARM_PSCI_CPU_SUSPEND => self.host_state.sys_regs.elr = thread_vector_table.cpu_suspend_entry,
            ARM_PSCI_FIQ => self.host_state.sys_regs.elr = thread_vector_table.fiq_entry,
            ARM_PSCI_SYSTEM_OFF => self.host_state.sys_regs.elr = thread_vector_table.system_off_entry,
            ARM_PSCI_SYSTEM_RESET => self.host_state.sys_regs.elr = thread_vector_table.system_reset_entry,
            _ => panic!("Unsupported SMC Type"),
        }
    }
}



impl VcpuCtx {
    pub fn init_vcpu_ctx(&mut self) {
        // let spsr_el1: usize = crate::read_sysreg!(spsr_el1);
        // let elr_el1: usize = crate::read_sysreg!(elr_el1);
        // let sctlr_el1: usize = crate::read_sysreg!(sctlr_el1);
        // let sp_el1: usize = crate::read_sysreg!(sp_el1);
        // let esr_el1: usize = crate::read_sysreg!(esr_el1);
        // let vbar_el1: usize = crate::read_sysreg!(vbar_el1);
        // let ttbr0_el1: usize = crate::read_sysreg!(ttbr0_el1);
        // let ttbr1_el1: usize = crate::read_sysreg!(ttbr1_el1);
        // let mair_el1: usize = crate::read_sysreg!(mair_el1);
        // let amair_el1: usize = crate::read_sysreg!(amair_el1);
        // let tcr_el1: usize = crate::read_sysreg!(tcr_el1);
        // let tpidr_el1: usize = crate::read_sysreg!(tpidr_el1);

        // /* Init guest sys regs */
        // self.sys_regs.spsr = spsr_el1;
        // self.sys_regs.elr = elr_el1;
        // self.sys_regs.sctlr = sctlr_el1 & 0xFFFFFFFFFFFFFFFE;
        // self.sys_regs.sp = sp_el1;
        // self.sys_regs.esr = esr_el1;
        // self.sys_regs.vbar = vbar_el1;
        // self.sys_regs.ttbr0 = ttbr0_el1;
        // self.sys_regs.ttbr1 = ttbr1_el1;
        // self.sys_regs.mair = mair_el1;
        // self.sys_regs.amair = amair_el1;
        // self.sys_regs.tcr = tcr_el1;
        // self.sys_regs.tpidr = tpidr_el1;
    }
}
//TODO: not done yet, unknown bug occurs, maybe due to frequent call-in and call-out
pub fn rust_run_vcpu() -> isize {
    let mut exit_reason: usize = 0;
    let core_id: usize = unsafe { get_core_id() };
    unsafe {
        // crate::printf(crate::to_c_str("core_id = %d\n"), core_id);
        // crate::printf(crate::to_c_str("global_titanium_states = %p\n"), global_titanium_states);
        // crate::printf(crate::to_c_str("global_titanium_states = %p\n"), &global_titanium_states);
    }
    let mut vmresume = 1;
    while vmresume == 1 {
        exit_reason = unsafe { enter_guest() };
        vmresume = unsafe {
            let target_state = &mut global_titanium_states[core_id];
            // crate::printf(crate::to_c_str("target_state = %p\n"), &target_state);
            titanium_handle_exit(exit_reason, target_state)
        };
    }
    0
}
