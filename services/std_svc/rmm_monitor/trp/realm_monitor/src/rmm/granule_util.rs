// Granule.c
pub const SZ_2G: usize = (2)<<30;
pub const SZ_4K: usize = (4) << 10;
pub const SZ_512M: usize = (1)<<29;
// FIXME: resize the stack size
// pub const MEM0_SIZE: usize = SZ_2G;
// For Linux
pub const MEM0_SIZE: usize = SZ_512M*4/2;
// For TF-A-TEST
// pub const MEM0_SIZE: usize = SZ_512M*2;
pub const GRANULE_SIZE: usize = SZ_4K;
pub const GRANULE_SHIFT: usize = 12;
pub const NR_GRANULES: usize = MEM0_SIZE/GRANULE_SIZE;
// For Linux
pub const MEM_PHYS_BASE:usize = 0x000000080000000;
pub const MEM1_PHYS:usize = 0x0000008c0000000;
// pub const MEM0_PHYS:usize = 0x0000000880000000;
pub const MEM0_PHYS:usize = 0x000000080000000;
// For TF-A-TEST
// pub const MEM0_PHYS:usize = 0x000000080000000;
pub const ERROR_MAX:usize = 0x1000;

// Auxiliary functions
pub fn ALIGNED(_size: usize, _alignment: usize) -> bool {
    return (_size%_alignment) == 0
}

// For TF-A-TEST
// pub fn addr_to_idx(addr: usize) -> usize {
//     return (addr - MEM0_PHYS) / GRANULE_SIZE;
// }

// For Linux-fvp
// pub fn addr_to_idx(addr: usize) -> usize {
//     if (addr>MEM1_PHYS) && (addr<(MEM1_PHYS+MEM0_SIZE/2)){
//         crate::println!("DEBUG: addr_to_idx1 {:x}, idx {:x}", addr, ((addr - MEM1_PHYS) / GRANULE_SIZE) + NR_GRANULES/2);
//         return ((addr - MEM1_PHYS) / GRANULE_SIZE) + NR_GRANULES/2;
//     }
//     else if (addr>MEM0_PHYS) && (addr<(MEM0_PHYS+MEM0_SIZE/2)) {
//         crate::println!("DEBUG: addr_to_idx2 {:x}, idx {:x}", addr, (addr - MEM0_PHYS) / GRANULE_SIZE);
//         return (addr - MEM0_PHYS) / GRANULE_SIZE;
//     }
//     else {
//         crate::println!("ERROR: addr_to_idx fails {:x}", addr);
//         return NR_GRANULES;
//     }
// }

// For Linux-qemu
pub fn addr_to_idx(addr: usize) -> usize {
    if (addr>MEM0_PHYS) && (addr<(MEM0_PHYS+MEM0_SIZE)) {
        crate::dprintln!("DEBUG: addr_to_idx2 {:x}, idx {:x}", addr, (addr - MEM0_PHYS) / GRANULE_SIZE);
        return (addr - MEM0_PHYS) / GRANULE_SIZE;
    }
    else {
        crate::dprintln!("ERROR: addr_to_idx fails {:x}", addr);
        return NR_GRANULES;
    }
}


use alloc::{
    vec::Vec,
};

use crate::rmm::realm_util::{
    Rd,
};

use crate::io::Write;

// use spinning_top::Spinlock;

#[repr(C)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum GranuleState {
    GranuleStateNs,
    GranuleStateDelegated,
    GranuleStateRd,
    GranuleStateRec,
    GranuleStateData,
    GranuleStateTable,
    GranuleStateRecList,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum ErrorStatus{
	StatusSuccess = 0,

	/*
	 * A parameter is invalid, and the reason cannot be represented using any of
	 * the other error codes.
	 */
	StatusErrorParameter = 1,

	/*
	 * An object cannot be released because it is in use.
	 */
    StatusErrorInUse = 2,

	/*
	 * An address is misaligned.
	 */
    StatusErrorAddrAlign = 3,

	/*
	 * An address is out of bounds.
	 */
	StatusErrorAddrBounds = 4,

	/*
	 * An attribute of a Granule does not match the expected value.
	 */
    StatusErrorGranuleAttr = 5,

	/*
	 * An attribute of a Realm does not match the expected value.
	 */
    StatusErrorRealmAttr = 6,

	/*
	 * An attribute of a REC does not match the expected value.
	 */
	StatusErrorRECAttr = 7,

	/*
	 * An attribute of a Table does not match the expected value.
	 */
	StatusErrorTableAttr = 8,

	/*
	 * A table walk failed to reach the target entry.
	 */
	StatusErrorTableWalk = 9,

	/*
	 * A Table entry does not match the expected value.
	 */
	StatusErrorTableEntry = 10,
}

#[repr(C)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum BufferSlot {
	/*
	 * NS or Realm-private.
	 */
	SLOT_NS,
	SLOT_INPUT,
	SLOT_OUTPUT,

	/*
	 * RMM-private.
	 */
	SLOT_DELEGATED,
	SLOT_RD,
	SLOT_REC,
	SLOT_REC_TARGET,	/* Target REC for interrupts */
	SLOT_RTT,
	SLOT_RTT2,
	SLOT_REC_LIST,
	NR_CPU_SLOTS
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Granule {
    /* The granule should take this lock whenever modifying
     * any field in this structure
     */
    /* We use a global mutex to protect Granule */
    is_lock: bool,
    // spin: Spinlock<u32>,
    /* Granule type */
    pub state: GranuleState,
    /* refcount represents how many rmm or realm reference this granule*/
    refcount: u32,
    /* rd_id represent the realm descriptor associated with this granule*/
    rd_id: u32,
    /* id represent the id in the granule vector */
    pub id: u32,
    /* the RBA this granule associated with */
    map_addr: u64,
}

impl Granule {
    // construct a initialized granule structure
    pub fn new(_id: usize) -> Self {
        Granule{
            // spin: Spinlock::new(0),
            is_lock: false,
            state: GranuleState::GranuleStateNs,
            refcount: 0,
            rd_id: NR_GRANULES as u32,
            map_addr: 0,
            id: _id as u32
        }
    }
}

unsafe impl Sync for Granule {}
unsafe impl Send for Granule {}

use spinning_top::Spinlock as Mutex;
use lazy_static::lazy_static;

// lazy_static! {
//     pub static ref GRANULE_LOCK: Mutex<Vec<Granule>> = {
//         crate::println!("GRANULE_LOCK new()");
//         let mut GRANULE_LIST:Vec<Granule> =Vec::new();
//         for _i in 0..NR_GRANULES {
//             // crate::println!("initialize the granule id {}", _i);
//             GRANULE_LIST.push(Granule::new(_i));
//         }
//         crate::println!("initialize the granule is finished");
//         Mutex::new(GRANULE_LIST)
//     };
// }

static mut GRANULE_LIST: [Granule; NR_GRANULES] = [Granule{is_lock: false,
    state: GranuleState::GranuleStateNs,
    refcount: 0,
    rd_id: NR_GRANULES as u32,
    map_addr: 0,
    id: 0}; NR_GRANULES];

static GRANULE_LOCK: Mutex<u32> = Mutex::new(0);

#[repr(C)]
pub struct GranuleUtil {
    // The global list of all granules
    // global_granule: Vec<Granule>,
}

impl GranuleUtil {
    pub fn init_granule() {
        for _i in 0..NR_GRANULES {
            unsafe {GRANULE_LIST[_i].id = _i as u32;}
        }
    }

    /* granule_from_idx */
    pub fn granule_from_index(id: usize) -> Granule {
        // let global_granule = GRANULE_LOCK.lock();
        // let tmp_granule = (global_granule[id]);
        // drop(global_granule);
        // return tmp_granule;
        let tmp_granule = unsafe {GRANULE_LIST[id]};
        return tmp_granule;
    }

    /* Find the granule from the physical address */
    pub fn find_granule(addr: usize) -> Result<Granule,ErrorStatus> {
        if !ALIGNED(addr, GRANULE_SIZE) {
            return Err(ErrorStatus::StatusErrorAddrAlign);
        }

        let idx = addr_to_idx(addr);

        if idx>NR_GRANULES {
            crate::println!("ERROR: idx: {:x}", idx);
            crate::println!("ERROR: NR_GRANULES: {:x}", NR_GRANULES);
            return Err(ErrorStatus::StatusErrorAddrBounds);
        }

        return Ok(GranuleUtil::granule_from_index(idx));
    }

    // Lock the granule after acquiring GRANULE_LOCK 
    pub fn lock_granule(idx: u32) -> bool {
        // let mut global_granule = GRANULE_LOCK.lock();
        // if (global_granule[idx as usize]).is_lock == true {
        //     drop(global_granule);
        //     crate::println!("lock_granule is failed");
        //     return false;
        // }
        // (global_granule[idx as usize]).is_lock = true;
        // drop(global_granule);
        // return true;

        //FIXME: atomic set for granule lock
        let mut granule_lock = GRANULE_LOCK.lock();
        let mut is_lock = unsafe {GRANULE_LIST[idx as usize].is_lock};
        while is_lock == true {
            crate::println!("try to lock granule {:x}", idx);
            drop(granule_lock);
            granule_lock = GRANULE_LOCK.lock();
            is_lock = unsafe {GRANULE_LIST[idx as usize].is_lock};
        }
        unsafe {GRANULE_LIST[idx as usize].is_lock = true};
        drop(granule_lock);
        return true;
    }

    // Unock the granule after acquiring GRANULE_LOCK 
    pub fn unlock_granule(idx: u32) -> bool {
        // let mut global_granule = GRANULE_LOCK.lock();
        // if (global_granule[idx as usize]).is_lock == false {
        //     drop(global_granule);
        //     crate::println!("unlock_granule is failed");
        //     return false;
        // }
        // (global_granule[idx as usize]).is_lock = false;
        // drop(global_granule);
        // return true;

        //FIXME: atomic set for granule lock
        let granule_lock = GRANULE_LOCK.lock();
        let mut is_lock = unsafe {GRANULE_LIST[idx as usize].is_lock};
        if is_lock == false {
            crate::println!("WARNING: unlock_granule is failed idx {:x}", idx);
            drop(granule_lock);
            return false;
        }
        unsafe {GRANULE_LIST[idx as usize].is_lock = false};
        drop(granule_lock);
        return true;
    }

    /* Lock and return the granule according to the physical address */
    pub fn find_lock_granule(addr: usize, expected_state: GranuleState) -> Result<Granule,ErrorStatus> {
        let g = GranuleUtil::find_granule(addr);

        match g {
            Ok(granule) => {
                if GranuleUtil::lock_granule(granule.id) {
                    // crate::println!("Debug: lock idx: {:x}", granule.id);
                    return Ok(granule);
                } 
                else {
                    return Err(ErrorStatus::StatusErrorGranuleAttr);
                }
            }
            Err(err) => {
                return Err(err);
            }
        }        
    }

    pub fn granule_refcount_read_acquire(idx: u32) -> bool {
        // let global_granule = GRANULE_LOCK.lock();
        // if global_granule[idx as usize].refcount > 0 {drop(global_granule); return true;}
        // else {drop(global_granule); return false}
        let refcount = unsafe {GRANULE_LIST[idx as usize].refcount};
        if refcount > 0 {return true;}
        else {return false}
    }

    pub fn acquire_granule_refcount(idx: u32) -> u32 {
        // let global_granule = GRANULE_LOCK.lock();
        // let refcount = global_granule[idx as usize].refcount;
        // drop(global_granule); 
        // return refcount;
        let refcount = unsafe {GRANULE_LIST[idx as usize].refcount};
        return refcount;
    }

    pub fn granule_refcount_inc(idx: u32, val: usize) {
        // let mut global_granule = GRANULE_LOCK.lock();
        // global_granule[idx as usize].refcount += val as u32;
        // drop(global_granule);
        unsafe {
            GRANULE_LIST[idx as usize].refcount += val as u32;
        }
    }

    pub fn granule_refcount_dec(idx: u32, val: usize) {
        // let mut global_granule = GRANULE_LOCK.lock();
        // global_granule[idx as usize].refcount += val as u32;
        // drop(global_granule);
        unsafe {
            if GRANULE_LIST[idx as usize].refcount < val as u32 {crate::println!("ERROR: granule_refcount_dec the reference is less than value");}
            GRANULE_LIST[idx as usize].refcount -= val as u32;
        }
    }

    pub fn get_granule(idx: u32) {
        // let mut global_granule = GRANULE_LOCK.lock();
        // global_granule[idx as usize].refcount += 1;
        // drop(global_granule); 
        unsafe {
            GRANULE_LIST[idx as usize].refcount += 1;
        }
    }

    pub fn put_granule(idx: u32) {
        unsafe {
            if GRANULE_LIST[idx as usize].refcount == 0 {crate::println!("ERROR: put granule the reference is zero");}
            GRANULE_LIST[idx as usize].refcount -= 1;
        }
    }

    pub fn acquire_granule_state(idx: u32) -> GranuleState {
        // let global_granule = GRANULE_LOCK.lock();
        // let state = global_granule[idx as usize].state;
        // drop(global_granule);
        // return state;
        let state = unsafe {GRANULE_LIST[idx as usize].state};
        return state;
    }

    pub fn granule_set_state(idx: u32, state: GranuleState)  {
        // let mut global_granule = GRANULE_LOCK.lock();
        // global_granule[idx as usize].state = state;
        // drop(global_granule);
        unsafe {GRANULE_LIST[idx as usize].state = state;}
    }

    pub fn granule_set_rd(idx: u32, rd_id: u32)  {
        // let mut global_granule = GRANULE_LOCK.lock();
        // global_granule[idx as usize].rd_id = rd_id;
        // drop(global_granule);
        unsafe {GRANULE_LIST[idx as usize].rd_id = rd_id};
    }

    pub fn granule_get_rd(idx: u32) -> u32 {
        return unsafe {GRANULE_LIST[idx as usize].rd_id};
    }


    /* 
     * \brief: auxiliary function for realm_create_granule
     */
    pub fn realm_create_granule_ops1(g_rd_id: u32, g_table_id: u32, g_rec_list_id: u32) {
        // let mut global_granule = GRANULE_LOCK.lock();
        // global_granule[g_table_id].refcount += 1;
        // global_granule[g_table_id].rd_id = g_rd_id as u32; // We cannot transfer a &mut out of the scope of global_granule
        // global_granule[g_table_id].state = GranuleState::GranuleStateTable;
        // GranuleUtil::unlock_granule(g_table_id as u32);

        // global_granule[g_rec_list_id].state = GranuleState::GranuleStateRecList;
        // GranuleUtil::unlock_granule(g_rec_list_id as u32);
        // drop(global_granule);
        unsafe {
            GRANULE_LIST[g_table_id as usize].refcount += 1;
            GRANULE_LIST[g_table_id as usize].rd_id = g_rd_id; // We cannot transfer a &mut out of the scope of global_granule
            GRANULE_LIST[g_table_id as usize].state = GranuleState::GranuleStateTable;
        }
        GranuleUtil::unlock_granule(g_table_id);

        unsafe {
            GRANULE_LIST[g_rec_list_id as usize].state = GranuleState::GranuleStateRecList;
        }
        GranuleUtil::unlock_granule(g_rec_list_id);
    }

    /* 
     * \brief: auxiliary function for realm_create_granule
     */
    pub fn realm_create_granule_ops2(rd: &mut Rd, g_table_id: u32, g_rec_list_id: u32,
        // realm_par_base: usize, realm_par_size: usize) {
        // let global_granule = GRANULE_LOCK.lock();

        // rd.state = 0; // REALM_STATE_NEW
        // rd.par_base = realm_par_base;
        // rd.par_end = realm_par_base + realm_par_size;
        // rd.g_table = global_granule[g_table_id];
        // rd.g_rec_list = global_granule[g_rec_list_id];

        // drop(global_granule);
        realm_par_base: usize, realm_par_size: usize) {
    
        rd.state = 0; // REALM_STATE_NEW
        rd.par_base = realm_par_base;
        rd.par_end = realm_par_base + realm_par_size;
        unsafe {
            rd.g_table = GRANULE_LIST[g_table_id as usize];
            rd.g_rec_list = GRANULE_LIST[g_rec_list_id as usize];
        }
    
    }

}
