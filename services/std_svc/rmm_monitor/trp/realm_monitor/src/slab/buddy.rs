use crate::slab::list::LinkedList;

const LEAF_SIZE: usize = 1 << 12;

#[derive(Debug)]
pub struct SzInfo<'a> {
    free_list: LinkedList,
    alloc: &'a mut [u8],
}

impl<'a> SzInfo<'a> {
    pub fn new(alloc_base: usize, alloc_len: usize) -> Self {
        /* Create alloc_array from raw */
        let alloc_base_ptr = alloc_base as *mut u8;
        let alloc_array =
            unsafe { &mut *core::ptr::slice_from_raw_parts_mut(alloc_base_ptr, alloc_len) };
        for i in 0..alloc_len {
            alloc_array[i] = 0;
        }
        SzInfo {
            free_list: LinkedList::new(),
            alloc: alloc_array,
        }
    }
}

#[derive(Debug)]
pub struct BuddyAllocator<'a> {
    base: usize,
    end: usize,
    nlevel: u8,
    leaf_size: usize,
    sz_array: &'a mut [SzInfo<'a>],
}

impl<'a> BuddyAllocator<'a> {
    pub fn new(base: usize, end: usize) -> Self {
        /* Part 1 : calc some metadata */
        let mut align_base = roundup(base, LEAF_SIZE);
        let align_end = rounddown(end, LEAF_SIZE);
        let length = align_end - align_base;
        let mut nlevel = log2(length / LEAF_SIZE) as u8;
        if (LEAF_SIZE << nlevel as usize) < length {
            nlevel += 1;
        }
        /* Part 2 : alloc sz_array , by adding to base*/
        let sz_array_base: usize = align_base;
        let sz_array_len: usize = nlevel as usize;
        align_base += sz_array_len * core::mem::size_of::<SzInfo>();
        align_base = roundup(align_base, LEAF_SIZE);
        //TODO: should do memset
        let sz_array_base_ptr = sz_array_base as *mut SzInfo;
        let sz_array =
            unsafe { &mut *core::ptr::slice_from_raw_parts_mut(sz_array_base_ptr, sz_array_len) };
        /* Part 3: new a bd */
        let bd: BuddyAllocator = BuddyAllocator {
            base: align_base,
            end: align_end,
            nlevel,
            leaf_size: LEAF_SIZE,
            sz_array,
        };
        /* Part 4 : call init to setup internal structure*/
        // bd.init();
        return bd;
    }

    /*init sz_array, alloc array, free_list */
    pub fn init(&mut self) {
        /* Part 1: Alloc mem for alloc_array for n level */
        crate::println!("BuddyAllocator init");
        for cur_level in 0..self.nlevel {
            let alloc_base = self.base;
            let nblk = nblk_of_level(self.nlevel, cur_level);
            let alloc_len = match nblk {
                1..=7 => 1,
                _ => nblk / 8,
            };
            self.base += alloc_len;
            let sz_info = SzInfo::new(alloc_base, alloc_len);
            self.sz_array[cur_level as usize] = sz_info;
        }
        /* Part 2: init free list*/
        self.base = roundup(self.base, LEAF_SIZE);
        self.init_free_list(self.base, self.end);
    }

    pub fn alloc_with_align(&mut self, nbytes: usize, align_bit: usize) -> Option<usize> {
        /*Part 0: Sanity Check*/
        if (1 << align_bit) >= (self.end - self.base) {
            panic!("Align bit out of range : align_bit = {}", align_bit);
        }
        /*Part 1 : Try without align*/
        let raw_addr = self.alloc(nbytes).unwrap();
        let align_addr = roundup(raw_addr, 1 << align_bit);
        let sz = blk_sz_of_level(self.level_of_addr(raw_addr) as u8);
        let remain_sz = sz - (align_addr - raw_addr);
        if remain_sz >= nbytes {
            return Some(align_addr);
        }
        /* Part 2: Re-alloc with double-sized demand*/
        self.free(raw_addr);
        let new_raw_addr = self.alloc(nbytes * 2).unwrap();
        let new_align_addr = roundup(new_raw_addr, 1 << align_bit);
        Some(new_align_addr)
    }

    pub fn alloc(&mut self, nbyte: usize) -> Option<usize> {
        if nbyte == 0 {
            return None;
        }
        /* Part 1: determine level */
        let mut k = 0;
        while k < self.nlevel {
            let sz_info: &SzInfo = &self.sz_array[k as usize];
            if blk_sz_of_level(k) >= nbyte && !sz_info.free_list.is_empty() {
                //sz fit and free exists
                break;
            }
            k += 1;
        }
        if k >= self.nlevel {
            panic!("out of memory");
        }
        /* Part 2 : split size k and alloc */
        while k > 0 {
            if blk_sz_of_level(k - 1) < nbyte {
                break;
            }
            //2.1 fetch from free list
            let cur_sz_info: &mut SzInfo = &mut self.sz_array[k as usize];
            let curr_addr = cur_sz_info.free_list.pop().unwrap();
            let bd_left = curr_addr;
            let bd_right = curr_addr + blk_sz_of_level(k - 1);
            bit_set(&mut cur_sz_info.alloc, idx_of_addr(self.base, k, curr_addr));
            //2.2 insert into next level
            let next_sz_info: &mut SzInfo = &mut self.sz_array[(k - 1) as usize];
            next_sz_info.free_list.push(bd_left);
            next_sz_info.free_list.push(bd_right);
            k -= 1;
        }
        /* Part 3: Final Fetch */
        let sz_info: &mut SzInfo = &mut self.sz_array[k as usize];
        let ret_addr = sz_info.free_list.pop().unwrap();
        let idx = idx_of_addr(self.base, k, ret_addr);
        bit_set(&mut sz_info.alloc, idx);

        Some(ret_addr)
    }

    pub fn free(&mut self, addr: usize) -> bool {
        /*Part 1 : determine which level*/
        let mut k = 0;
        while k < self.nlevel {
            let sz_info: &mut SzInfo = &mut self.sz_array[k as usize];
            if bit_isset(&mut sz_info.alloc, idx_of_addr(self.base, k, addr)) {
                break;
            }
            k += 1;
        }
        if k == self.nlevel {
            panic!("Internal Error");
        }
        /*Part 2 : Merge buddy if possible*/
        let mut new_addr = addr;
        while k < self.nlevel - 1 {
            let curr_idx = idx_of_addr(self.base, k, addr);
            let bd_idx = match curr_idx % 2 == 0 {
                true => curr_idx + 1,
                false => curr_idx - 1,
            };
            let sz_info: &mut SzInfo = &mut self.sz_array[k as usize];
            bit_clear(&mut sz_info.alloc, curr_idx);
            if bit_isset(&mut sz_info.alloc, bd_idx) {
                // Buddy is free
                // 1. remove buddy free list
                let bd_addr = addr_of_idx(self.base, k, bd_idx);
                sz_info.free_list.remove(bd_addr);
                // 2. update new_addr
                new_addr = match curr_idx % 2 == 0 {
                    true => addr,
                    false => bd_addr,
                };
                k += 1;
                continue;
            }
            break;
        }
        /*Part 3: insert into Lv.k free_list*/
        let sz_info: &mut SzInfo = &mut self.sz_array[k as usize];
        sz_info.free_list.push(new_addr);
        return true;
    }

    fn init_free_list(&mut self, base: usize, end: usize) {
        let isz: isize = (end - base) as isize;
        if isz <= 0 || isz < blk_sz_of_level(0) as isize {
            return;
        }
        let sz = isz as usize;
        let mut level = 0;
        for lv in (0..self.nlevel).rev() {
            if blk_sz_of_level(lv) <= sz && blk_sz_of_level(lv + 1) > sz {
                //Attach to No.k free list
                let sz_info: &mut SzInfo = &mut self.sz_array[lv as usize];
                sz_info.free_list.push(base);
                level = lv;
                //set buddy alloc
                let curr_idx = idx_of_addr(self.base, level, base);
                let buddy_idx = curr_idx + 1;
                bit_set(self.sz_array[level as usize].alloc, buddy_idx);
                break;
            }
        }
        /*Part 2: recursive call */
        let next_base = base + blk_sz_of_level(level);
        self.init_free_list(next_base, end);
    }

    fn level_of_addr(&self, addr: usize) -> usize {
        for k in 0..self.nlevel {
            if bit_isset(
                &self.sz_array[k as usize].alloc,
                idx_of_addr(self.base, k as u8, addr),
            ) {
                return k as usize;
            }
        }
        panic!("Internal Error in level_of_addr: level overflow");
    }
}

fn idx_of_addr(base: usize, level: u8, addr: usize) -> usize {
    (addr - base) / blk_sz_of_level(level)
}
fn addr_of_idx(base: usize, level: u8, idx: usize) -> usize {
    base + idx * blk_sz_of_level(level)
}

fn nblk_of_level(nlevel: u8, level: u8) -> usize {
    1 << ((nlevel - level + 1) as usize)
}
fn blk_sz_of_level(level: u8) -> usize {
    (LEAF_SIZE) << (level as usize)
}

pub const fn log2(num: usize) -> usize {
    let mut k = 0;
    let mut n = num;
    while n > 1 {
        k += 1;
        n >>= 1;
    }
    return k;
}

fn bit_set(array: &mut [u8], idx: usize) {
    let c = array[idx / 8];
    let new_c = c | (1 << idx % 8) as u8;
    array[idx / 8] = new_c;
}
fn bit_clear(array: &mut [u8], idx: usize) {
    let c = array[idx / 8];
    let new_c = c & !(1 << idx % 8) as u8;
    array[idx / 8] = new_c;
}
fn bit_isset(array: &[u8], idx: usize) -> bool {
    let c = array[idx / 8];
    (c & (1 << idx % 8) as u8) != 0
}

const fn roundup(n: usize, sz: usize) -> usize {
    let _n = n as isize;
    let _sz = sz as isize;
    (((_n - 1) / _sz + 1) * _sz) as usize
}
const fn rounddown(n: usize, sz: usize) -> usize {
    n / sz * sz
}
