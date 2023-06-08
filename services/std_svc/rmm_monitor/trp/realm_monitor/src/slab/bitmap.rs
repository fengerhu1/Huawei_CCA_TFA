#[derive(Debug)]
pub struct BitMap<'a> {
    alloc: &'a mut [u8],
}

impl BitMap<'_> {
    pub fn new(base: usize, nbits: usize) -> Self {
        let alloc_base_ptr = base as *mut u8;
        let len = nbits / 8 + 1;
        let alloc_array = unsafe { &mut *core::ptr::slice_from_raw_parts_mut(alloc_base_ptr, len) };
        BitMap { alloc: alloc_array }
    }

    pub fn bit_set(&mut self, idx: usize) {
        let c = self.alloc[idx / 8];
        let new_c = c | (1 << idx % 8) as u8;
        self.alloc[idx / 8] = new_c;
    }

    pub fn bit_clear(&mut self, idx: usize) {
        let c = self.alloc[idx / 8];
        let new_c = c & (!(1 << (idx % 8)) as u8);
        self.alloc[idx / 8] = new_c;
    }

    pub fn bit_isset(&mut self, idx: usize) -> bool {
        let c = self.alloc[idx / 8];
        (c & (1 << idx % 8) as u8) != 0
    }
}
