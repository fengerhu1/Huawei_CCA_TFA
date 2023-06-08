use super::vcpu::*;
use crate::util::list::ListHead;
use crate::util::mmops::ti_memset;
use crate::virt::s2mmu::S2mmu;

const MAX_VCPU_NUM: usize = 16;

#[repr(C)]
pub enum VmState{
    VsCreate = 0,
    VsInit,
    VsReady,
    VsRunning,
    VsDestroy,
}

#[repr(C)]
pub struct TitaniumVM {
    vm_id: isize,
    vm_name: *mut u8,
    pub vm_list_node: ListHead,
    pub vcpus: [*mut TitaniumVCPU; MAX_VCPU_NUM],
    pub thread_vector_table: *mut ThreadVectorTable,
    vm_ops: *mut TitaniumVMOps,
    pub s2mmu: *mut S2mmu,
    vm_state: VmState,
}

extern "C"{
    pub fn init_vcpu_ctx(ctx: *mut VcpuCtx);
    pub static mut titanium_vm_list: ListHead;
    pub fn run_vcpu();
    static mut global_titanium_states: [TitaniumState; PHYSICAL_CORE_NUM];
    static mut tsp_vm_ops: TitaniumVMOps;
    static mut optee_vm_ops: TitaniumVMOps;
}

impl TitaniumVM {
    pub fn init_vm(
        &mut self,
        vm_id: isize,
        vm_name: *mut u8,
        entrypoint: usize,
        spsr: usize,
        vcpu_num: isize,
        ops: *mut TitaniumVMOps,
    ) -> isize {
        /* Do memset */
        ti_memset(
            self as *mut TitaniumVM as usize as *mut u8,
            0,
            core::mem::size_of::<TitaniumVM>(),
        );

        self.vm_id = vm_id;
        self.vm_name = vm_name;
        self.vm_ops = ops;

        /* Init a stage2 page table for this VM */
        self.s2mmu = S2mmu::create_s2mmu();

        for i in 0..vcpu_num {
            let vcpu: *mut TitaniumVCPU = unsafe {
                super::super::ALLOCATOR
                    .slab_alloc(core::mem::size_of::<TitaniumVCPU>(), 0)
                    .unwrap()
            } as *mut TitaniumVCPU;
            ti_memset(vcpu as *mut u8, 0, core::mem::size_of::<TitaniumVCPU>());
            unsafe{
                (*vcpu).vm = self;
                (*vcpu).vcpu_id = i;
            }

            let ctx: *mut VcpuCtx = unsafe{
                super::super::ALLOCATOR
                    .slab_alloc(core::mem::size_of::<VcpuCtx>(), 0)
                    .unwrap()
            } as *mut VcpuCtx;
            
            ti_memset(ctx as *mut u8, 0, core::mem::size_of::<VcpuCtx>());
            unsafe{init_vcpu_ctx(ctx)};
            unsafe{
                (*vcpu).vcpu_ctx = ctx;
                (*vcpu).vcpu_state = VcpuState::VcpuInit;
                (*vcpu).entrypoint = entrypoint;
                (*vcpu).entry_spsr = spsr;
            }

            self.vcpus[i as usize] = vcpu;
        }

        self.vm_state = VmState::VsInit;
        0
    }

    pub fn titanium_vm_enqueue(&mut self) {
        unsafe{
            titanium_vm_list.push(&mut self.vm_list_node as *mut ListHead);
        }
    }

}


pub fn rust_get_vm_by_id(vm_id: usize) -> *mut TitaniumVM {
    let offset:usize = unsafe{
        &mut ((*(0 as *mut TitaniumVM)).vm_list_node) as *mut ListHead as usize
    };
    
    let list_head_virt = unsafe{&mut titanium_vm_list} as *mut ListHead as usize;
    let mut current_node_virt = unsafe{(*(list_head_virt as *mut ListHead)).next} as usize;
    while current_node_virt != list_head_virt {
        let current_vm = (current_node_virt - offset) as *mut TitaniumVM;
        if unsafe{(*current_vm).vm_id} == vm_id as isize {
            return current_vm;
        }
        //to next
        current_node_virt = unsafe{
            ((*current_vm).vm_list_node).next as usize
        };
    }
    core::ptr::null_mut()
}

use alloc::boxed::Box;
use crate::virt::ipa_region::{IpaRegion, IrType};
use crate::virt::s2mmu::VmFlags;
pub unsafe fn rust_init_vms() {
    /* Init vm list which holds all VMs in titanium */
    titanium_vm_list.init();
    let ptr: *mut u8 = &mut global_titanium_states[0] as *mut TitaniumState as *mut u8;
    ti_memset(ptr, 0, core::mem::size_of::<TitaniumState>() * PHYSICAL_CORE_NUM);

    /* initialize OPTEE vm struct */
    let optee_vm_ptr: *mut TitaniumVM = 
        super::super::ALLOCATOR.slab_alloc(core::mem::size_of::<TitaniumVM>(), 0).unwrap() as *mut TitaniumVM;
    
    (*optee_vm_ptr).init_vm(OPTEE_VM_ID as isize, 
        "optee" as *const str as *mut u8, 0x6000000, 0x3c4, MAX_VCPU_NUM as isize, &mut optee_vm_ops);

    /* create ipa regions for OPTEE vm */
    let optee_ipa_region_1: *mut IpaRegion = 
        crate::rust_create_ipa_region(0, 0, 0x6000000, VmFlags::MMU_ATTR_PAGE_RWE as u32, IrType::IrLazyMapping as u32);
    let optee_ipa_region_2: *mut IpaRegion = 
        crate::rust_create_ipa_region(0x6000000, 0x6000000, 0x200000, VmFlags::MMU_ATTR_PAGE_RWE as u32, IrType::IrEagerMapping as u32);
    let optee_ipa_region_3: *mut IpaRegion = 
        crate::rust_create_ipa_region(0x6200000, 0, 0x1E00000, VmFlags::MMU_ATTR_PAGE_RWE as u32, IrType::IrLazyMapping as u32);

    /* Add ipa regions to s2mmu */
    (*(*optee_vm_ptr).s2mmu).add_ipa_region(optee_ipa_region_1);
    (*(*optee_vm_ptr).s2mmu).add_ipa_region(optee_ipa_region_2);
    (*(*optee_vm_ptr).s2mmu).add_ipa_region(optee_ipa_region_3);

    /* flush ipa_region to the hardware stage2 page table */
    crate::rust_sync_ipa_regions_to_page_table((*optee_vm_ptr).s2mmu);

    /* Add optee vm into titanium vm list */
    crate::rust_titanium_vm_enqueue(optee_vm_ptr);

    /* initialize TSP vm struct */
    let tsp_vm_ptr: *mut TitaniumVM = 
        super::super::ALLOCATOR.slab_alloc(core::mem::size_of::<TitaniumVM>(), 0).unwrap() as *mut TitaniumVM;
    crate::rust_init_vm(tsp_vm_ptr, TSP_VM_ID as isize, 
        "tsp" as *const str as *mut u8, 0x6600000, 0x3c4, MAX_VCPU_NUM as isize, &mut tsp_vm_ops);

    /* Create ipa region */
    let tsp_ipa_region_1: *mut IpaRegion = 
        crate::rust_create_ipa_region(0x6A00000, 0, 0x1600000, VmFlags::MMU_ATTR_PAGE_RWE as u32, IrType::IrLazyMapping as u32);
    let tsp_ipa_region_2: *mut IpaRegion = 
        crate::rust_create_ipa_region(0x6600000, 0x6600000, 0x200000, VmFlags::MMU_ATTR_PAGE_RWE as u32, IrType::IrEagerMapping as u32);
    let tsp_ipa_region_3: *mut IpaRegion = 
        crate::rust_create_ipa_region(0x6800000, 0x6800000, 0x200000, VmFlags::MMU_ATTR_PAGE_RWE as u32, IrType::IrEagerMapping as u32);
    let tsp_ipa_region_4: *mut IpaRegion = 
        crate::rust_create_ipa_region(0x6A00000, 0, 0x1600000, VmFlags::MMU_ATTR_PAGE_RWE as u32, IrType::IrLazyMapping as u32);
    
    /* Add ipa regions to s2mmu */
    /* Add ipa regions to s2mmu */
    (*(*tsp_vm_ptr).s2mmu).add_ipa_region(tsp_ipa_region_1);
    (*(*tsp_vm_ptr).s2mmu).add_ipa_region(tsp_ipa_region_2);
    (*(*tsp_vm_ptr).s2mmu).add_ipa_region(tsp_ipa_region_3);
    crate::rust_sync_ipa_regions_to_page_table((*tsp_vm_ptr).s2mmu);

    /* Add tsp vm into titanium vm list */
    crate::rust_titanium_vm_enqueue(tsp_vm_ptr);

}