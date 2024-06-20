use crate::println;
use crate::rmm::granule_util::{
    Granule,
    ErrorStatus,
    BufferSlot,
    NR_GRANULES,
    GRANULE_SHIFT,
    GRANULE_SIZE,
    GranuleUtil,
    GranuleState,
    idx_to_addr,
};

use crate::rmm::rmm_util::{
    granule_map_with_id,
    granule_map_with_id_state,
    buffer_unmap,
    granule_map_zero,
    RmmUtil,
};

use crate::rmm::realm_util::{
    Rd,
};

use crate::rmm::abs_util::{
    VPERCPU_LOCK,
};

use crate::rmm::measurement::{
    measurement_extend_data,
};

use alloc::{
    slice,
};

use crate::io::Write;

pub const NR_TABLE_LEVELS:usize  = 4;
pub const RTT_PAGE_LEVEL:usize = 3;

pub const Page_Table_Shift: [usize; 4] = [39, 30, 21, 12];
pub const PA_BIT:usize = 48;
pub const PA_MASK: usize = (1<<PA_BIT) -1;

pub const PTE_IPA_STATE_SHIFT: usize =  56;
pub const PTE_IPA_STATE_MASK: usize =  (0x7) << PTE_IPA_STATE_SHIFT;

pub const PGTES_PER_TABLE: usize = 1 << (GRANULE_SHIFT - 3);

pub const PGTE_S2_L012_TABLE: usize = 0x3;
pub const PGTE_S2_L012_BLOCK: usize = 0x1;
pub const PGTE_S2_L3_PAGE: usize = 0x3;
/* We set HCR_EL2.FWB So we set bit[4] to 1 and bits[3:2] to 2 and force
 *  * everyting to be Normal Write-Back */
pub const PGTE_S2_MEMATTR_FWB_NORMAL_WB: usize = ((1 << 4) | (2 << 2));
pub const PGTE_S2_AP_RW: usize = 3 << 6;
pub const PGTE_S2_SH_IS: usize = 3 << 8; /* Inner Shareable */
pub const PGTE_S2_AF: usize = 1 << 10;
pub const PGTE_S2_NS: usize = 1 << 55;
pub const PGTE_S2_XN: usize = 2 << 53;
 
pub const PGTE_S2_ATTRS: usize = PGTE_S2_MEMATTR_FWB_NORMAL_WB | PGTE_S2_AP_RW | PGTE_S2_SH_IS | PGTE_S2_AF;
 
pub const PGTE_S2_TABLE: usize = PGTE_S2_L012_TABLE;
pub const PGTE_S2_BLOCK: usize = PGTE_S2_ATTRS | PGTE_S2_L012_BLOCK;
pub const PGTE_S2_PAGE: usize = PGTE_S2_ATTRS | PGTE_S2_L3_PAGE;
pub const PGTE_INVALID: usize = 0;

pub const ADDRESS_BITS: usize = 48;
pub const ADDRESS_MASK: usize = ((1<<ADDRESS_BITS) - 1) - (0xfff);

pub const SHARED_ADDR_BEGIN:usize = 0xb0000000;
pub const SHARED_ADDR_END:usize = 0xc0000000;

/*
 * The state of IPAs within the PAR that describes the following properties:
 *  - Is there a granule assigned to this IPA?
 *  - Is it mapped?
 *  - Can the host assign a new granule to this IPA?
 *
 * The values are stored in PTEs of Realms' stage 2 page tables:
 *  - When stored at level 3, the value represent the state of
 *    an individual 4KB page.
 *  - When stored level 0, 1 or 2, the value represent the state of
 *    a block of consecutive pages.
 *
 * Name:		Stored at:	Page Count in	Block's
 *					the Block:	alignement:
 * --------------------------------------------------------
 * IPA State		L3 PTE		1		4KB
 * Block 2 IPA State	L2 PTE		2**9		2MB
 * Block 1 IPA State	L1 PTE		2**18		1GB
 * Block 0 IPA State	L0 PTE		2**27		512GB
 *
 *
 * The RMM uses S/W reserved bits 56-58 in Stage 2 PTEs to store the state.
 * Note that, if some of S/W bits 56-58 are required for other features, a more
 * compact encoding for IPA states may be possible.
 */
#[repr(C)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum IpaState {
	/*
	 * The initial state of the address upon the Realm's creation.
	 * No DRAM granule is assigned to the address.
	 * The host may assign a new DATA granule to this address.
	 *
	 * The state can be stored in:
	 * - Invalid PTE at L3 PT, as Vacant.
	 * - Invalid PTE at L2 PT, as Block 2 Vacant.
	 * - Invalid PTE at L1 PT, as Block 1 Vacant.
	 * - Invalid PTE at L0 PT, as Block 0 Vacant.
	 *
	 * The value must be zero. (as a newly created table is zero-filled).
	 */
     IpaStateVacant = 0,
     /*
      * A DATA granule is assigned to the address. The granule is not mapped.
      *
      * The state can be stored in:
      * - Invalid PTE at L3 PT, as Absent.
      * - Invalid PTE at L2 PT, as Block 2 Absent.
      *
      * The PA of the assigned granule is stored in the PTE's bits [47:12]
      * (bits[47:21] for the case Block 2 Absent state).
      */
     IpaStateAbsent = 1,
     /*
      * A DATA granule is assigned and mapped at this address.
      *
      * The state can be stored in:
      * - Page PTE at L3 PT, as Present.
      * - Block PTE at L2 PT, as Block 2 Present.
      *
      * The PA of the assigned granule is stored in the PTE's bits [47:12]
      * (bits[47:21] for the case Block 2 Present state).
      */
      IpaStatePresent = 2,
     /*
      * The previous content of the page has been destroyed and no granule
      * is assigned to this IPA.
      * The host cannot assign a new DATA granule to this address. (To assign
      * a new DATA granule to this address, the address must first transition
      * to IPA_STATE_VACANT, see RMI.Data.Dispose).
      *
      * The state can be stored in:
      * - Invalid PTE at L3 PT, as Destroyed.
      * - Invalid PTE at L2 PT, as Block 2 Destroyed.
      * - Invalid PTE at L1 PT, as Block 1 Destroyed.
      * - Invalid PTE at L0 PT, as Block 0 Destroyed.
      */
      IpaStateDestroyed = 3,
}


#[derive(Clone, Copy)]
#[repr(C)]
pub struct TblWalk {
	pub g_llt_id: u32,
	pub index: usize,
}

impl TblWalk {
    pub fn new() -> Self {
        TblWalk { 
            g_llt_id: NR_GRANULES as u32,
			index: 0,
        }
    }
}

#[repr(C)]
pub struct TableUtil {
    // The global list of all granules
    // global_granule: Vec<Granule>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Page {
    pub entry:[usize; 512], 
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
    pub fn ns_buffer_unmap(slot: BufferSlot);
}

#[inline]
pub fn PTE_TO_IPA_STATE(pte: usize) -> IpaState {
    let ipa_state = (((pte) & PTE_IPA_STATE_MASK) >> PTE_IPA_STATE_SHIFT);
    match ipa_state {
        0 => {return IpaState::IpaStateVacant;}
        1 => {return IpaState::IpaStateAbsent;}
        2 => {return IpaState::IpaStatePresent;}
        3 => {return IpaState::IpaStateDestroyed;}
        _ => {return IpaState::IpaStateVacant;} // never arrived
    }
}

#[inline]
pub fn IPA_STATE_TO_PTE(ipa_state: IpaState) -> usize {
    return (ipa_state as usize) << PTE_IPA_STATE_SHIFT;
}

#[inline]
pub fn addr_is_level_aligned(addr: usize, level: usize) -> bool {
    return (addr & ((1 << Page_Table_Shift[level]) - 1)) == 0;
}

#[inline]
pub fn PTE_TO_PA(pte: usize) -> usize {
    return pte & (0xfffffffff000 as usize);
}

#[inline]
pub fn entry_to_phys(entry: usize, level: usize) -> usize {
    return (entry - (entry & ((1 << Page_Table_Shift[level]) - 1))) & PA_MASK;
}

#[inline]
pub fn entry_is_table(entry: usize) -> bool {
    return (entry & 0x3) == 0x3;
}

#[inline]
pub fn __addr_to_idx(addr: usize, level: usize) -> usize {
    let shift_addr = addr >> Page_Table_Shift[level];
    return shift_addr & ((1 << 9) - 1)
}

#[inline]
pub fn pgte_write(table: &mut Page, index: usize, pte: usize) {
    table.entry[index] = pte;
}

#[inline]
pub fn granule_fill_table(table: &mut Page, pte_val: usize, pte_inc: usize) {
    let mut i = 0;
    let mut entry_val = pte_val;
    while i < PGTES_PER_TABLE {
        table.entry[i] = pte_val;
        entry_val = entry_val + pte_inc;
        i += 1;
    }
    unsafe {
        barrier();
    }
}

pub fn __find_next_level_idx(g_tbl_id: u32, idx: usize) -> u32 {
    let table_ptr = granule_map_with_id(g_tbl_id as usize, BufferSlot::SLOT_RTT) as *mut Page;
    let table = unsafe {&mut (*table_ptr)};
    let entry = table.entry[idx];
    unsafe {buffer_unmap(table_ptr as usize);}
    let next_table_id: u32;

    if (entry_is_table(entry) == false) {
        next_table_id = NR_GRANULES as u32;
    }
    else {
        let g = GranuleUtil::find_granule(entry_to_phys(entry, 3));
        match g {
            Ok(granule) => {
                next_table_id = granule.id;
            }
            Err(_) => {
                next_table_id = NR_GRANULES as u32;
            }
        }
    }
    return next_table_id;
}

pub fn table_walk_lock_unlock(rd_id: u32, map_addr: usize, level: usize) {
    let rd_ptr = granule_map_with_id(rd_id as usize, BufferSlot::SLOT_RD) as *mut Rd;
    let rd = unsafe {&mut (*rd_ptr)};
    let g_root = rd.g_table;
    unsafe {buffer_unmap(rd_ptr as usize)};

    let mut tbl = g_root.id;
    let mut last_tbl = g_root.id;
    let mut l = 0;
    let mut idx = 0;

    GranuleUtil::lock_granule(tbl);
    while l < level {
        if tbl < NR_GRANULES as u32 {
            idx = __addr_to_idx(map_addr, l);
            // find the granule of next page table page 
            tbl = __find_next_level_idx(tbl, idx);
            if tbl != NR_GRANULES as u32 {
                GranuleUtil::lock_granule(tbl);
                GranuleUtil::unlock_granule(last_tbl);
                last_tbl = tbl;
            }
            else {
                crate::println!("ERROR: table_walk_lock_unlock is failed level {:x}, idx {:x}, map_addr {:x}", l, idx, map_addr);
                GranuleUtil::unlock_granule(last_tbl);
            }
        }
        else {
            crate::println!("ERROR: table_walk_lock_unlock is failed, root tbl is invalied");
        }
        l += 1;
    }

    let mut v_percpu_list = VPERCPU_LOCK.lock();
    v_percpu_list[crate::cpuid!()].wi.g_llt_id = tbl;
    v_percpu_list[crate::cpuid!()].wi.index = __addr_to_idx(map_addr, level);
    drop(v_percpu_list);
}

pub fn table_create_init_vacant(ipa_state: IpaState, table: &mut Page, llt_id: u32) {
    granule_fill_table(table, IPA_STATE_TO_PTE(ipa_state) | PGTE_INVALID, 0);
    GranuleUtil::get_granule(llt_id);
}

/*
 * \brief: there is a pa entry in the llt, but the valid bit is not set
 */
pub fn table_create_init_absent(level: usize, llt_pte: usize, table: &mut Page, rtt_id: u32) {
    if level == RTT_PAGE_LEVEL {
        let pa = entry_to_phys(llt_pte, level - 1);
        granule_fill_table(table, IPA_STATE_TO_PTE(IpaState::IpaStateAbsent) |
        pa | PGTE_INVALID, GRANULE_SIZE);
        GranuleUtil::granule_refcount_inc(rtt_id, PGTES_PER_TABLE);
    } else {
        crate::println!("ERROR: table_create_init_absent is failed\n");    
    }
}

/*
 * If last level page table entry has a valid block, 
 * then we need to set rtt page entries to cover the block
 */
pub fn table_create_init_present(level: usize, ll_table: &mut Page, index: usize, map_addr: usize, llt_pte: usize, table: &mut Page, rtt_id: u32) {
    if level == RTT_PAGE_LEVEL {
        pgte_write(ll_table, index, 0);

        unsafe {
		    invalidate_block(map_addr);
        }

		let pa = entry_to_phys(llt_pte, level - 1);
		granule_fill_table(table, IPA_STATE_TO_PTE(IpaState::IpaStatePresent) |
					pa | PGTE_S2_PAGE, GRANULE_SIZE);
		GranuleUtil::granule_refcount_inc(rtt_id, PGTES_PER_TABLE);
    } else {
        crate::println!("ERROR: table_create_init_present is failed\n");
    }
}

/*
 * rd_id: granule id of rtt owner's Rd
 * llt_id: granule id of the last level page table page
 * rtt_id: the granule id of the given page table page
 * llt_pte: the last level page table entry
 * ll_table: pointer of the last level page table
 * level: level of the page table
 * index: index of ipa in the last level page table
 * map_Addr: ipa
 * rtt_addr: the physical address of the given page table
 */
pub fn table_create_aux(rd_id: u32, llt_id: u32, rtt_id:u32, llt_pte: usize, ll_table: &mut Page,
                        level: usize, index: usize, map_addr: usize, rtt_addr: usize) {
    let ipa_state = PTE_TO_IPA_STATE(llt_pte);

    // get the given page table
    let table_ptr = granule_map_with_id(rtt_id as usize, BufferSlot::SLOT_DELEGATED) as *mut Page;    
    let table = unsafe {&mut (*table_ptr)};

    GranuleUtil::granule_set_state(rtt_id, GranuleState::GranuleStateTable);
    if ipa_state == IpaState::IpaStateVacant || ipa_state == IpaState::IpaStateDestroyed {
        table_create_init_vacant(ipa_state, table, llt_id);
    }
    else if (ipa_state == IpaState:: IpaStateAbsent) {
        table_create_init_absent(level, llt_pte, table, rtt_id);
    }
    else if (ipa_state == IpaState:: IpaStatePresent) {
        table_create_init_present(level, ll_table, index, map_addr,
            llt_pte, table, rtt_id);
    }
    
    
    unsafe {buffer_unmap(table_ptr as usize);}
    GranuleUtil::get_granule(rtt_id);
    GranuleUtil::granule_set_rd(rtt_id, rd_id);
    pgte_write(ll_table, index, rtt_addr|PGTE_S2_TABLE);
    
}

pub fn table_has_destroyed(table: &mut Page) -> bool {
    let mut ret:bool = false;
    let mut i: usize = 0;

    while i<PGTES_PER_TABLE {
        let pgte = table.entry[i];
        if PTE_TO_IPA_STATE(pgte) == IpaState::IpaStateDestroyed {
            ret = true;
        }
        i += 1;
    }

    return ret;
}

pub fn table_delete(table: &mut Page, g_llt_id: u32) -> usize {
    let new_pgte: usize;
    let ipa_state: IpaState;

    if table_has_destroyed(table) == true {ipa_state = IpaState::IpaStateDestroyed;}
    else {ipa_state = IpaState::IpaStateVacant;}

    new_pgte = IPA_STATE_TO_PTE(ipa_state) | PGTE_INVALID;
    GranuleUtil::put_granule(g_llt_id);

    return new_pgte;
}

pub fn table_maps_block(table: &mut Page, level: usize, ipa_state: IpaState) -> bool {
    let mut pgte = table.entry[0];
    let base_pa = entry_to_phys(pgte, level);

    if addr_is_level_aligned(base_pa, level - 1) == false {return false}
    else {
        let mut ret = true;
        let mut i = 0;

        while i<PGTES_PER_TABLE {
            let expected_pa = base_pa + i*GRANULE_SIZE;
            pgte = table.entry[i];
            if (PTE_TO_IPA_STATE(pgte) != ipa_state) 
                || (entry_to_phys(pgte, level) != expected_pa) {
                ret = false;
            }
            i += 1;
        }
    
        return ret;
    }
}

pub fn table_fold(table: &mut Page, level: usize, g_tbl_id: u32) -> usize {
    let pgte = table.entry[0];
    let ipa_state = PTE_TO_IPA_STATE(pgte);
    let base_pa = entry_to_phys(pgte, level-1);

    let mut new_pgte: usize = 0;

    // we can merge pages into one block
    if (level != RTT_PAGE_LEVEL) || (table_maps_block(table, level, ipa_state) == false ) {
        crate::println!("ERROR: table fold is error\n");
        return 0;
    }
    else {
        new_pgte = IPA_STATE_TO_PTE(ipa_state) | base_pa;
        if (ipa_state == IpaState::IpaStatePresent) {
            new_pgte |= PGTE_S2_BLOCK;
        }
        else {
            new_pgte |= PGTE_INVALID;
        }

        GranuleUtil::granule_refcount_dec(g_tbl_id, PGTES_PER_TABLE);
    }
    return new_pgte
}

/*
 * g_llt_id: last page tabel id
 * g_tbl_id: the destroy page table id
 * level: the level of the destroy page table
 * index: the pte index in the last page table
 * map_addr: the IPA of the destroy page table
 */
pub fn table_destroy_aux(g_llt_id: u32, g_tbl_id: u32, ll_table: &mut Page, level: usize, index: usize, map_addr: usize) -> Result<usize, ErrorStatus> {
    // get the given page table
    let table_ptr = granule_map_with_id(g_tbl_id as usize, BufferSlot::SLOT_RTT2) as *mut Page;
    let table = unsafe {&mut (*table_ptr)};
    let gcnt = GranuleUtil::acquire_granule_refcount(g_tbl_id);

    let new_pgte: usize;
    let mut ret = Ok(0);

    if gcnt == 1 || gcnt == (PGTES_PER_TABLE + 1) as u32 {
        if gcnt == 1 {
            // Return an invalid PTE
            new_pgte = table_delete(table, g_llt_id);
        }
        else {
            // if table still have all valid pages, we cannot delete this page but merge into one block (huge page)
            new_pgte = table_fold(table, level, g_tbl_id);
        }

        ll_table.entry[index] = new_pgte;
        // If the page table is still valid, we do not destroy this page table, but fold to the huge page
        if (PTE_TO_IPA_STATE(new_pgte) == IpaState::IpaStatePresent) {
            unsafe {invalidate_pages_in_block(map_addr);}
        }
        else {
            unsafe {invalidate_block(map_addr);}
        }
        ll_table.entry[index] = new_pgte;
        
        GranuleUtil::put_granule(g_tbl_id);
        GranuleUtil::granule_set_rd(g_tbl_id, NR_GRANULES as u32);
        unsafe {memzero(table_ptr as usize, GRANULE_SIZE);}
        GranuleUtil::granule_set_state(g_tbl_id, GranuleState::GranuleStateDelegated);    
    }
    else {
        crate::println!("ERROR: table_destroy_aux: gcnt is {:x} table_id {:x}\n", gcnt, g_tbl_id);
        ret = Err(ErrorStatus::StatusErrorTableWalk)
    }

    unsafe {buffer_unmap(table_ptr as usize)};
    return ret;
}

pub fn data_granule_measure(rd_id: u32, data_ptr: *mut Page, map_addr: usize) {
    // Calculate the measurement of data header:
    let rd_ptr = granule_map_with_id(rd_id as usize, BufferSlot::SLOT_RD) as *mut Rd;
    let rd = unsafe {&mut (*rd_ptr)};

    let mut hash_data_header: [u64; 3] = [0,0,0];  // Granule type, IPA, 0
    hash_data_header[0] = GranuleState::GranuleStateData as u64;
    hash_data_header[1] = map_addr as u64;

    // Transfer the u64 array to the u8 array
    let u8_header_ptr = hash_data_header.as_mut_ptr() as *mut u8 as *mut[u8; 24];
    let u8_header = unsafe {&mut (*u8_header_ptr)};
    measurement_extend_data(rd, u8_header);

    // Calculate the measurement of data granule:
    let hash_data = unsafe {&mut *(data_ptr as *mut[u8; 4096]) as &mut[u8; 4096]};
    measurement_extend_data(rd, hash_data);

    unsafe {buffer_unmap(rd_ptr as usize)};
}

impl TableUtil {
    pub fn validate_table_commands(map_addr: usize, level: usize, min_level: usize, max_level: usize) -> usize {
        if level <  min_level || level > max_level || addr_is_level_aligned(map_addr, level) == false {
            return 1;
        }
        return 0;
    }

    pub fn table_create(rd_id: u32, map_addr: usize, level: usize, rtt_id: u32, rtt_addr: usize) -> Result<usize, ErrorStatus> {
        //find the granule of the last valid page table
        table_walk_lock_unlock(rd_id, map_addr, level-1);
        let v_percpu_list = VPERCPU_LOCK.lock();
        let g_llt_id = v_percpu_list[crate::cpuid!()].wi.g_llt_id;
        let index = v_percpu_list[crate::cpuid!()].wi.index;
        drop(v_percpu_list);
        let mut ret: Result<usize, ErrorStatus> = Ok(0);

        if g_llt_id == NR_GRANULES as u32 {
            return Err(ErrorStatus::StatusErrorTableWalk);
        }
        else {
            let table_ptr = granule_map_with_id(g_llt_id as usize, BufferSlot::SLOT_RTT) as *mut Page;
            let table = unsafe {&mut (*table_ptr)};
            let entry = table.entry[index];

            if entry_is_table(entry) == true {
                ret = Err(ErrorStatus::StatusErrorTableEntry);
            }
            else {
                table_create_aux(rd_id, g_llt_id, rtt_id, entry, table, level, index, map_addr, rtt_addr);
            }

            unsafe {buffer_unmap(table_ptr as usize);}
            GranuleUtil::unlock_granule(g_llt_id);
        }

        return ret;
    }

    pub fn table_destroy(rd_id: u32, map_addr: usize, rtt_addr: usize, level: usize) -> Result<usize, ErrorStatus> {
        let mut ret: Result<usize, ErrorStatus> = Ok(0);
        // Find the last page table
        table_walk_lock_unlock(rd_id, map_addr, level-1);
        let v_percpu_list = VPERCPU_LOCK.lock();
        let g_llt_id = v_percpu_list[crate::cpuid!()].wi.g_llt_id;
        let index = v_percpu_list[crate::cpuid!()].wi.index;
        drop(v_percpu_list);

        if g_llt_id == NR_GRANULES as u32 {
            crate::println!("ERROR: table_destroy g_llt_id is error");
            return Err(ErrorStatus::StatusErrorTableWalk);
        }
        else {
            let table_ptr = granule_map_with_id(g_llt_id as usize, BufferSlot::SLOT_RTT) as *mut Page;
            let table = unsafe {&mut (*table_ptr)};
            let llt_pte = table.entry[index]; // The PTE in the last page table

            if (entry_is_table(llt_pte) == false) || (rtt_addr != (llt_pte & ADDRESS_MASK)) {
                crate::println!("ERROR: table_destroy: entry is not a table or is invalid");
                return Err(ErrorStatus::StatusErrorTableWalk);
            }
            else {
                let g_tbl = RmmUtil::find_lock_granule(rtt_addr, GranuleState::GranuleStateTable);
                match g_tbl {
                    Ok(g_tbl_id) => {
                        ret = table_destroy_aux(g_llt_id, g_tbl_id, table, level, index, map_addr);
                        GranuleUtil::unlock_granule(g_tbl_id);
                    }
                    Err(err) => {
                        crate::println!("ERROR: table_destroy: rtt_addr is error");
                        return Err(ErrorStatus::StatusErrorTableWalk);
                    }
                }
            }
            unsafe {buffer_unmap(table_ptr as usize);}
            // Lock pair with table_walk_lock_unlock
            GranuleUtil::unlock_granule(g_llt_id);
        }

        return ret;
    }

    /*
     * \brief table map only change the IpaStateAbsent to IpaStatePresent?
     *  FIXME: why do we need table_map when we set the page Ipa State to Absent?
     */
    pub fn table_map(rd_id: u32, map_addr: usize, level: usize)-> Result<usize, ErrorStatus> {
        let mut ret: Result<usize, ErrorStatus> = Ok(0);
        // level == 3
        table_walk_lock_unlock(rd_id, map_addr, level);
        let v_percpu_list = VPERCPU_LOCK.lock();
        let g_llt_id = v_percpu_list[crate::cpuid!()].wi.g_llt_id;
        let index = v_percpu_list[crate::cpuid!()].wi.index;
        drop(v_percpu_list);

        if g_llt_id == NR_GRANULES as u32 {
            return Err(ErrorStatus::StatusErrorTableWalk);
        }
        else {
            // find the last level page table of the given map_addr
            let table_ptr = granule_map_with_id(g_llt_id as usize, BufferSlot::SLOT_RTT) as *mut Page;
            let table = unsafe {&mut (*table_ptr)};
            let llt_pgte = table.entry[index];
            
            if level < RTT_PAGE_LEVEL && entry_is_table(llt_pgte) == true {
                ret = Err(ErrorStatus::StatusErrorTableWalk); 
            }
            else {
                let data_addr = PTE_TO_PA(llt_pgte);
                let ipa_state = PTE_TO_IPA_STATE(llt_pgte);
                if ipa_state == IpaState::IpaStateAbsent {
                    let mut new_pgte = IPA_STATE_TO_PTE(IpaState::IpaStatePresent) | data_addr;
                    if level == RTT_PAGE_LEVEL {
                        new_pgte |= PGTE_S2_PAGE; 
                    }
                    else {
                        new_pgte |= PGTE_S2_BLOCK;
                    }
                    // FIXME: hard code the shared memory address here
                    if  map_addr >= SHARED_ADDR_BEGIN && map_addr < SHARED_ADDR_END {
                        new_pgte |= PGTE_S2_NS;
                        new_pgte |= PGTE_S2_XN;
                    }
                    table.entry[index] = new_pgte;
                }
            }

            unsafe {buffer_unmap(table_ptr as usize);}
            // Lock pair with table_walk_lock_unlock
            GranuleUtil::unlock_granule(g_llt_id);
        }        

        return ret;
    }

    /*
     * \brief table map change the IpaState from Present to Absent, and invalidate the corresponding page table
     */
    pub fn table_unmap(rd_id: u32, map_addr: usize, level: usize)-> Result<usize, ErrorStatus> {
        let mut ret: Result<usize, ErrorStatus> = Ok(0);
        // level == 3
        table_walk_lock_unlock(rd_id, map_addr, level);
        let v_percpu_list = VPERCPU_LOCK.lock();
        let g_llt_id = v_percpu_list[crate::cpuid!()].wi.g_llt_id;
        let index = v_percpu_list[crate::cpuid!()].wi.index;
        drop(v_percpu_list);

        if g_llt_id == NR_GRANULES as u32 {
            return Err(ErrorStatus::StatusErrorTableWalk);
        }
        else {
            let table_ptr = granule_map_with_id(g_llt_id as usize, BufferSlot::SLOT_RTT) as *mut Page;
            let ll_table = unsafe {&mut (*table_ptr)};
            let llt_pgte = ll_table.entry[index];

            let data_addr = PTE_TO_PA(llt_pgte);
            let ipa_state = PTE_TO_IPA_STATE(llt_pgte);

            if ipa_state == IpaState::IpaStatePresent {
                let new_pgte = IPA_STATE_TO_PTE(IpaState::IpaStateAbsent) | data_addr;
                ll_table.entry[index] = new_pgte;

                // never goto the invalidate block
                if (level == RTT_PAGE_LEVEL) {
					unsafe {invalidate_page(map_addr)};}
				else {
					unsafe {invalidate_block(map_addr)};}
            }
            else {
                return Err(ErrorStatus::StatusErrorTableWalk);
            }

            unsafe {buffer_unmap(table_ptr as usize);}
            // Lock pair with table_walk_lock_unlock
            GranuleUtil::unlock_granule(g_llt_id);
        }

        return ret;
    }

    /*
     * \brief: If the level3 page table state is IpaStateVacent,
     * then we copy the content from the source to the data granule,
     * and set IpaState to absent.
     * Later, we will invoke data_map to change the IpaState to present
     */
    pub fn data_create_op(rd_id: u32, data_addr: usize, map_addr: usize, data_id: u32, src_id: u32) -> Result<usize, ErrorStatus> {
        let mut ret: Result<usize, ErrorStatus> = Ok(0);
        // level == 3
        table_walk_lock_unlock(rd_id, map_addr, NR_TABLE_LEVELS -1);
        let v_percpu_list = VPERCPU_LOCK.lock();
        let g_llt_id = v_percpu_list[crate::cpuid!()].wi.g_llt_id;
        let index = v_percpu_list[crate::cpuid!()].wi.index;
        drop(v_percpu_list);

        if g_llt_id == NR_GRANULES as u32 {
            return Err(ErrorStatus::StatusErrorTableWalk);
        }
        else {
            // Read the last level3 page table content
            let table_ptr = granule_map_with_id(g_llt_id as usize, BufferSlot::SLOT_RTT) as *mut Page;
            let ll_table = unsafe {&mut (*table_ptr)};
            let mut pte_val = ll_table.entry[index];

            if PTE_TO_IPA_STATE(pte_val) != IpaState::IpaStateVacant {
                ret = Err(ErrorStatus::StatusErrorTableWalk);
            }
            else {
                let data_ptr = granule_map_with_id(data_id as usize, BufferSlot::SLOT_DELEGATED) as *mut Page;


                // let data = unsafe {&mut (*data_ptr)};
                let ns_access_ok: bool;

                // ns_granule_map
                granule_map_with_id_state(src_id as usize, GranuleState::GranuleStateNs, BufferSlot::SLOT_NS);
                unsafe {
                    // crate::println!("DEBUG: data_ptr is {:x}", data_ptr as *mut usize as usize);
                    ns_access_ok = ns_buffer_read_data(BufferSlot::SLOT_NS, data_ptr as *mut usize);
                    ns_buffer_unmap(BufferSlot::SLOT_NS);
                }
                
                // Check if we copy the data from the secure to normal
                if !ns_access_ok {
                    unsafe {memzero(data_ptr as usize, GRANULE_SIZE)};
                    return Err(ErrorStatus::StatusErrorTableWalk);
                }
                else {
                    pte_val = IPA_STATE_TO_PTE(IpaState:: IpaStateAbsent) | data_addr;
                    ll_table.entry[index] = pte_val;
                    GranuleUtil::get_granule(g_llt_id);
                }

                //Calculate the measurement of the data granule
                data_granule_measure(rd_id, data_ptr, map_addr);
                unsafe {buffer_unmap(data_ptr as usize);}
            }
            
            unsafe {buffer_unmap(table_ptr as usize);}
            // Lock pair with table_walk_lock_unlock
            GranuleUtil::unlock_granule(g_llt_id);
        }
        return ret;
    }

    // We need first call data create, and then calling data_map, otherwise, data_map will failed
    pub fn data_create(rd_id: u32, data_addr: usize, map_addr: usize, data_id: u32, src_id: u32) -> Result<usize, ErrorStatus> {
        let mut ret: Result<usize, ErrorStatus> = Ok(0);

        let rd_ptr = granule_map_with_id(rd_id as usize,  BufferSlot::SLOT_RD) as *mut Rd;
        let rd = unsafe {&mut (*rd_ptr)};
        if rd.state != 0 {ret = Err(ErrorStatus::StatusErrorRealmAttr);}
        else {
            match TableUtil::data_create_op(rd_id, data_addr, map_addr, data_id, src_id) {
                Ok(_) => {GranuleUtil::granule_set_state(data_id, GranuleState::GranuleStateData);}
                Err(err) => {ret = Err(err);}
            }
        }

        unsafe {buffer_unmap(rd_ptr as usize)};
        GranuleUtil::unlock_granule(rd_id);
        return ret;
    }

    // Call data unmap before data destroy
    pub fn data_destroy(rd_id: u32, map_addr: usize) -> Result<usize, ErrorStatus> {
        let mut ret: Result<usize, ErrorStatus> = Ok(0);
        // level == 3
        table_walk_lock_unlock(rd_id, map_addr, NR_TABLE_LEVELS -1);
        let v_percpu_list = VPERCPU_LOCK.lock();
        let g_llt_id = v_percpu_list[crate::cpuid!()].wi.g_llt_id;
        let index = v_percpu_list[crate::cpuid!()].wi.index;
        drop(v_percpu_list);

        if g_llt_id == NR_GRANULES as u32 {
            crate::println!("ERROR: data_destroy: last level page table is invalid");
            return Err(ErrorStatus::StatusErrorTableWalk);
        }
        else {
            // Read the last level3 page table content
            let table_ptr = granule_map_with_id(g_llt_id as usize, BufferSlot::SLOT_RTT) as *mut Page;
            let ll_table = unsafe {&mut (*table_ptr)};
            let mut pte_val = ll_table.entry[index];

            if PTE_TO_IPA_STATE(pte_val) != IpaState::IpaStateAbsent {
                crate::println!("ERROR: data_destroy: invalid IPA state");
                ret = Err(ErrorStatus::StatusErrorTableWalk);
            }
            else {
                let data_addr = PTE_TO_PA(pte_val);
                let g_data = RmmUtil::find_lock_granule(data_addr, GranuleState::GranuleStateData);

                match g_data {
                    Ok(data_id) => {
                        // clear the pte entry
                        pte_val = IPA_STATE_TO_PTE(IpaState::IpaStateDestroyed);
                        ll_table.entry[index] = pte_val;
                        GranuleUtil::put_granule(g_llt_id);
                        //clear the granule state and value
                        if  map_addr >= SHARED_ADDR_BEGIN && map_addr < SHARED_ADDR_END {
                            // crate::println!("data_destroy: destroy shared memory {:x} !!!!!!!!!!!!", map_addr);
                            GranuleUtil::granule_set_state(data_id, GranuleState::GranuleStateNs);
                        }
                        else {
                            granule_map_zero(data_id, BufferSlot::SLOT_DELEGATED);
                            GranuleUtil::granule_set_state(data_id, GranuleState::GranuleStateDelegated);
                        }

                        GranuleUtil::unlock_granule(data_id);
                    }
                    Err(_) => {
                        crate::println!("ERROR: data_destroy: find data granule is failed");
                        ret = Err(ErrorStatus::StatusErrorTableWalk);
                    }
                }
            }

            unsafe {buffer_unmap(table_ptr as usize);}
            // Lock pair with table_walk_lock_unlock
            GranuleUtil::unlock_granule(g_llt_id);
        }

        return ret;
    }

    pub fn data_create_unknown_op(rd_id: u32, data_addr: usize, map_addr: usize, data_id: u32) -> Result<usize, ErrorStatus> {
        let mut ret: Result<usize, ErrorStatus> = Ok(0);
        // level == 3
        table_walk_lock_unlock(rd_id, map_addr, NR_TABLE_LEVELS -1);
        let v_percpu_list = VPERCPU_LOCK.lock();
        let g_llt_id = v_percpu_list[crate::cpuid!()].wi.g_llt_id;
        let index = v_percpu_list[crate::cpuid!()].wi.index;
        drop(v_percpu_list);

        if g_llt_id == NR_GRANULES as u32 {
            return Err(ErrorStatus::StatusErrorTableWalk);
        }
        else {
            // Read the last level3 page table content
            let table_ptr = granule_map_with_id(g_llt_id as usize, BufferSlot::SLOT_RTT) as *mut Page;
            let ll_table = unsafe {&mut (*table_ptr)};
            let mut pte_val = ll_table.entry[index];

            if PTE_TO_IPA_STATE(pte_val) != IpaState::IpaStateVacant {
                ret = Err(ErrorStatus::StatusErrorTableEntry);
            }
            else {
                pte_val = IPA_STATE_TO_PTE(IpaState:: IpaStateAbsent) | data_addr;
                ll_table.entry[index] = pte_val;
                GranuleUtil::get_granule(g_llt_id);
            }
            
            unsafe {buffer_unmap(table_ptr as usize);}
            // Lock pair with table_walk_lock_unlock
            GranuleUtil::unlock_granule(g_llt_id);
        }
        return ret;
    }

    pub fn data_create_shared_op(rd_id: u32, data_addr: usize, map_addr: usize, data_id: u32) -> Result<usize, ErrorStatus> {
        let mut ret: Result<usize, ErrorStatus> = Ok(0);
        // level == 3
        table_walk_lock_unlock(rd_id, map_addr, NR_TABLE_LEVELS -1);
        let v_percpu_list = VPERCPU_LOCK.lock();
        let g_llt_id = v_percpu_list[crate::cpuid!()].wi.g_llt_id;
        let index = v_percpu_list[crate::cpuid!()].wi.index;
        drop(v_percpu_list);

        if g_llt_id == NR_GRANULES as u32 {
            return Err(ErrorStatus::StatusErrorTableWalk);
        }
        else {
            // Read the last level3 page table content
            let table_ptr = granule_map_with_id(g_llt_id as usize, BufferSlot::SLOT_RTT) as *mut Page;
            let ll_table = unsafe {&mut (*table_ptr)};
            let mut pte_val = ll_table.entry[index];

            if PTE_TO_IPA_STATE(pte_val) != IpaState::IpaStateVacant {
                ret = Err(ErrorStatus::StatusErrorTableEntry);
            }
            else {
                pte_val = IPA_STATE_TO_PTE(IpaState:: IpaStateAbsent) | data_addr;
                ll_table.entry[index] = pte_val;
                // crate::println!("data_create_shared_op pte_val {:x}", pte_val);
                GranuleUtil::get_granule(g_llt_id);
            }
            
            unsafe {buffer_unmap(table_ptr as usize);}
            // Lock pair with table_walk_lock_unlock
            GranuleUtil::unlock_granule(g_llt_id);
        }
        return ret;
    }
    
    pub fn data_create_unknown(rd_id: u32, data_addr: usize, map_addr: usize, data_id: u32) -> Result<usize, ErrorStatus> {
        let mut ret: Result<usize, ErrorStatus> = Ok(0);


        match TableUtil::data_create_unknown_op(rd_id, data_addr, map_addr, data_id) {
            Ok(_) => {GranuleUtil::granule_set_state(data_id, GranuleState::GranuleStateData);}
            Err(err) => {ret = Err(err);}
        }
        
        GranuleUtil::unlock_granule(rd_id);
        return ret;
    }

    pub fn data_create_shared(rd_id: u32, data_addr: usize, map_addr: usize, data_id: u32) -> Result<usize, ErrorStatus> {
        let mut ret: Result<usize, ErrorStatus> = Ok(0);

        match TableUtil::data_create_shared_op(rd_id, data_addr, map_addr, data_id) {
            Ok(_) => {
                GranuleUtil::granule_set_state(data_id, GranuleState::GranuleStateData);
            }
            Err(err) => {ret = Err(err);}
        }
        
        GranuleUtil::unlock_granule(rd_id);
        return ret;
    }


}

