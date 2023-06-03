#[repr(C)]
pub struct ListHead {
    pub prev: *mut ListHead,
    pub next: *mut ListHead,
}

use alloc::boxed::Box;
impl ListHead {
    pub fn new() -> Self {
        let mut ret = ListHead {
            prev: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
        };
        ret.init();
        ret
    }

    pub fn init(&mut self) {
        self.next = self as *mut ListHead;
        self.prev = self as *mut ListHead;
    }

    pub fn is_empty(&mut self) -> bool {
        self.next == self as *mut ListHead
    }

    pub fn remove_self(&mut self) {
        unsafe {
            (*self.next).prev = self.prev;
            (*self.prev).next = self.next;
        }
        self.prev = core::ptr::null_mut();
        self.next = core::ptr::null_mut();
    }

    pub fn push(&mut self, node: *mut ListHead) {
        unsafe {
            (*node).next = self.next;
            (*node).prev = self;
            (*(*node).next).prev = node;
            (*(*node).prev).next = node;
        }
    }

    pub fn pop(&mut self) -> *mut ListHead {
        match self.is_empty() {
            true => return core::ptr::null_mut(),
            false => {
                let mut node = unsafe { Box::from_raw(self.next) };
                node.as_mut().remove_self();
                return Box::into_raw(node);
            }
        }
    }
}
