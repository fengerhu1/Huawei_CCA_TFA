use core::cmp::{max, min};
use crate::slab::bitmap::BitMap;
use crate::slab::buddy::BuddyAllocator;
use crate::slab::list::{LinkedList, Node};

pub mod bitmap;
pub mod buddy;
pub mod list;

#[derive(Debug)]
struct Slab<'a>{
    list:Node,
    base:usize,
    obj_base:usize,
    slab_sz:usize,
    grid_sz:usize,
    align_sz:usize,
    free_num:usize,
    total_num:usize,
    first_free_idx:usize,
    alloc: BitMap<'a>,
}

#[derive(Debug)]
struct MemCache{
    list: Node,
    slab_free: LinkedList,
    slab_partial: LinkedList,
    slab_full: LinkedList,
    obj_sz:usize,
    align_sz:usize,
    grid_sz:usize,
}

#[derive(Debug)]
pub struct SlabAllocator<'a> {
    mem_cache_list: LinkedList,
    pub allocator: Option<BuddyAllocator<'a>>,
    slab_sz: usize
}

impl Slab<'_>{

    fn new(base:usize,size:usize,belong_cache:&MemCache)->Self{
        let map_base = base + core::mem::size_of::<Slab>();
        let map_nbits = size / belong_cache.grid_sz;
        let obj_base = map_base + map_nbits / 8 + 8; //+8 to reserve space to bit map 
        let obj_base = roundup(obj_base,belong_cache.grid_sz);
        let meta_sz = core::mem::size_of::<Slab>()
                                + (obj_base - map_base);
        let real_sz = size - meta_sz;
        let mut ret = Slab{
            list: Node {next:None,prev:None},
            base,
            obj_base,
            slab_sz: size,
            grid_sz:belong_cache.grid_sz,
            align_sz:belong_cache.align_sz,
            free_num: real_sz/belong_cache.grid_sz,
            total_num: real_sz/belong_cache.grid_sz,
            alloc: BitMap::new(map_base,map_nbits),
            first_free_idx:0,
        };
        for i in 0..map_nbits{
            ret.alloc.bit_clear(i);
        }
        return ret;
    }

    fn alloc(&mut self)->Option<usize>{
        if self.is_full() {
            return None;
        }
        let ret_addr = self.obj_base + (self.grid_sz * self.first_free_idx);
        self.alloc.bit_set(self.first_free_idx);
        self.free_num -= 1;
        // if full then no first_free_idx
        if self.free_num <= 0 {
            self.first_free_idx = usize::MAX;
            return Some(ret_addr);
        }
        // if not full , update first_free_idx
        for offset in 1..self.total_num{
            let curr_idx = (self.first_free_idx + offset) % self.total_num;
            if !self.alloc.bit_isset(curr_idx) {
                self.first_free_idx = curr_idx;
                break;
            }
        }
        return Some(ret_addr);
    }

    fn free(&mut self,obj_addr:usize)->bool{
        if !self.contain(obj_addr){
            panic!("obj_addr is wrong");
        }
        let idx = (obj_addr - self.obj_base) / self.grid_sz;
        self.alloc.bit_clear(idx);
        self.free_num+=1;
        self.first_free_idx = min(idx,self.first_free_idx);
        true
    }

    fn contain(&self, obj_addr:usize)->bool{
        return (self.base <= obj_addr) && (obj_addr <= self.base + self.slab_sz);
    }

    fn is_full(&self)->bool{
        return self.free_num <= 0;
    }
}

impl MemCache{

    fn new(obj_sz:usize,align_bit:usize)->Self{
        MemCache{
            list: Node {next:None,prev:None},
            slab_free:LinkedList::new(),
            slab_full:LinkedList::new(),
            slab_partial:LinkedList::new(),
            obj_sz,
            align_sz:1 << align_bit,
            grid_sz: max(obj_sz,1 << align_bit),
        }
    }

    fn alloc(&mut self) ->Option<usize>{
        //Part 1: Determine Slab
        let mut curr_slab_opt = None;
        if !self.slab_partial.is_empty() {
            curr_slab_opt = self.slab_partial.pop();
        }
        else if !self.slab_free.is_empty(){
            curr_slab_opt = self.slab_free.pop();
        }
        if curr_slab_opt.is_none() {
            return None;
        }
        //Part 2: alloc
        let curr_slab_addr = curr_slab_opt.unwrap();
        let curr_slab = curr_slab_addr as *mut Slab;
        let res =unsafe{(*curr_slab).alloc()};
        unsafe{
            match (*curr_slab).is_full() {
                true => self.slab_full.push(curr_slab_addr),
                false=> self.slab_partial.push(curr_slab_addr),
            }
        }
        return res;
    }

    fn free(&mut self,obj_addr:usize)->bool{
        let curr_slab = self.obj2slab(obj_addr);
        if curr_slab.is_none() {
            return false;
        }
        let res = unsafe{
            (*curr_slab.unwrap()).free(obj_addr)
        };
        return res;
    }

    /// refill current mem_cache, by given base and size
    fn refill(&mut self,base:usize,size:usize) ->bool{
        /* Part 1: Prepare slab structure*/
        let align_base = roundup(base,self.align_sz);
        let shrink_sz = size - (align_base - base);
        let new_slab = Slab::new(align_base,shrink_sz,self);
        /* Part 2: set new slab into target addr & attach to slab_free*/
        let slab_addr = align_base;
        let slab_ptr = slab_addr as *mut Slab;
        unsafe{*slab_ptr = new_slab};
        self.slab_free.push(slab_addr);
        return true;
    }

    fn obj2slab(&mut self,obj_addr:usize)->Option<*mut Slab>{
        // slab_free doesn't contains obj , so we don't search in it
        // Part 1: Search in slab_partial
        unsafe{
            let mut curr_slab_node = self.slab_partial.head;
            while curr_slab_node.is_some() {
                let curr_slab_addr = curr_slab_node.unwrap() as usize;
                let curr_slab_ptr = curr_slab_addr as *mut Slab;
                if (*curr_slab_ptr).contain(obj_addr) {
                    return Some(curr_slab_ptr);
                }
                curr_slab_node = (*curr_slab_node.unwrap()).next;

            }
        }
        // Part 2: Search in slab_full
        unsafe{
            let mut curr_slab_node = self.slab_full.head;
            while curr_slab_node.is_some() {
                let curr_slab_addr = curr_slab_node.unwrap() as usize;
                let curr_slab_ptr = curr_slab_addr as *mut Slab;
                if (*curr_slab_ptr).contain(obj_addr) {
                    return Some(curr_slab_ptr);
                }

                curr_slab_node = (*curr_slab_node.unwrap()).next;

            }
        }
        // return None if search failed
        None
    }

    fn contains(&mut self, obj_addr:usize) ->bool{
        let res = self.obj2slab(obj_addr);
        return res.is_some();
    }

}

impl SlabAllocator<'_>{

    ///Constructor
    pub const fn new()->Self{
        SlabAllocator{
            mem_cache_list:LinkedList::new(),
            // allocator:Some(BuddyAllocator::new(0x06A0_0000,0x0800_0000)),
            allocator: None,
            slab_sz: 1 << 12,
        }
    }

    pub fn init(&mut self){
        crate::println!("SlabAllocator init");
        self.allocator = Some(BuddyAllocator::new(0x06A0_0000,0x0800_0000));
        crate::println!("SlabAllocator init2");
        self.allocator.as_mut().unwrap().init();
        crate::println!("SlabAllocator init3");
    }

    ///alloc size with align, align may be less than size
    pub fn slab_alloc(&mut self, obj_sz:usize, align_bit:usize) ->Option<usize>{
        match &self.allocator {
            Some(x)=> {},
            _ => {
                self.init();
            }
        }
        crate::println!("SlabAllocator slab_alloc");
        // Part 0 : Sanity Check
        let curr_grid_sz = max(obj_sz, 1<<align_bit);
        if curr_grid_sz > self.slab_sz {
            panic!("obj_sz or align is too large! obj_sz = {}, align_bit = {}, grid_sz = {}",obj_sz,align_bit,curr_grid_sz);
        }
        //Part 1 : Search in list to find suitable mem_cache, create if not
        // suitable i.e. min(x) s.t. x.grid_sz >= max(obj_sz,1 << align_bit)
        let mut min_cache_opt:Option<*mut MemCache> = None;
        let mut min_grid_sz = usize::MAX;
        let mut curr_cache_node = self.mem_cache_list.head;
        unsafe{
            while curr_cache_node.is_some() {
                let curr_cache_addr = curr_cache_node.unwrap() as usize;
                let curr_cache_ptr = curr_cache_addr as *mut MemCache;
                let curr_cache_grid_sz = (*curr_cache_ptr).grid_sz;
                if  curr_cache_grid_sz >= curr_grid_sz
                    && curr_cache_grid_sz < min_grid_sz{
                    min_cache_opt = Some(curr_cache_ptr);
                    min_grid_sz = curr_cache_grid_sz;
                }
                curr_cache_node = (*curr_cache_node.unwrap()).next;
            }
        }
        // Part 2: create Memcache if None
        let curr_cache =  match min_cache_opt {
            Some(x)=> x,
            _ => {
                //create if None
                let new_cache = MemCache::new(obj_sz, align_bit);
                let cache_addr = self.allocator.as_mut().unwrap().alloc_with_align(1 << 12,12).unwrap();
                let cache_ptr = cache_addr as *mut MemCache;
                unsafe{*cache_ptr = new_cache};
                self.mem_cache_list.push(cache_addr);
                cache_ptr
            }
        };
        //Part 3: Alloc with the found mem_cache
        let mut alloc_res = unsafe{(*curr_cache).alloc()};
        if let None = alloc_res{
            let size = self.slab_sz;
            let base = self.allocator.as_mut().unwrap().alloc(size).unwrap();
            unsafe{(*curr_cache).refill(base,size)};
            alloc_res = unsafe{(*curr_cache).alloc()};
        }
        return alloc_res;
    }

    pub fn free(&mut self,obj_addr:usize)-> bool{
        //Part 1: Determine which mem_cache
        let curr_cache_opt = self.obj2cache(obj_addr);
        if curr_cache_opt.is_none() {
            return false;
        }
        //Part 2: free with the found mem_cache
        let curr_cache = curr_cache_opt.unwrap();
        return unsafe{(*curr_cache).free(obj_addr)};
    }

    /// find MemCache by obj_addr,
    /// should be optimized according to virt_to_cache()
    fn obj2cache(&mut self,obj_addr:usize)-> Option<*mut MemCache>{
        let mut curr_cache_node = self.mem_cache_list.head;
        unsafe{
            while curr_cache_node.is_some() {
                let curr_cache_addr = curr_cache_node.unwrap() as usize;
                let curr_cache_ptr = curr_cache_addr as *mut MemCache;
                if (*curr_cache_ptr).contains(obj_addr) {
                    return Some(curr_cache_ptr);
                }
                curr_cache_node = (*curr_cache_node.unwrap()).next;
            }
        }
        None
    }

    pub fn bd_alloc(&mut self, obj_sz:usize, align_bit:usize)->Option<usize>{
        self.allocator.as_mut().unwrap().alloc_with_align(obj_sz,align_bit)
    }

    pub fn bd_free(&mut self, obj_addr:usize)->bool{
        self.allocator.as_mut().unwrap().free(obj_addr)
    }
}

fn roundup(n:usize,sz:usize)->usize{
    let _n = n as isize;
    let _sz = sz as isize;
    (((_n  - 1) / _sz + 1) * _sz) as usize
}



