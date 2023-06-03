pub fn ti_memset(start: *mut u8, val: u8, len: usize) {
    for i in 0..len {
        let addr = start as usize + i;
        let ptr = addr as *mut u8;
        unsafe {
            (*ptr) = val;
        }
    }
}
