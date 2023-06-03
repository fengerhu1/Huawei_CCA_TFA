#[derive(Debug)]
pub struct LinkedList {
    pub head: Option<*mut Node>,
    pub tail: Option<*mut Node>,
    pub len: usize,
}
#[derive(Debug)]
pub struct Node {
    pub next: Option<*mut Node>,
    pub prev: Option<*mut Node>,
}

impl LinkedList {
    pub const fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, node_addr: usize) {
        let node_ptr = node_addr as *mut Node;
        unsafe {
            (*node_ptr).next = self.head;
            (*node_ptr).prev = None;
            let node = Some(node_ptr);
            match self.head {
                None => self.tail = node,
                Some(head) => (*head).prev = node,
            }
            self.head = node;
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.head.is_none() {
            return None;
        }
        unsafe {
            let node_ptr = self.head.unwrap();
            self.head = (*node_ptr).next;
            (*node_ptr).next = None;
            (*node_ptr).prev = None;
            match self.head {
                None => self.tail = None,
                Some(head) => (*head).prev = None,
            }
            self.len -= 1;
            return Some(node_ptr as usize);
        }
    }

    pub fn peek(&self) -> Option<usize> {
        if self.head.is_none() {
            return None;
        }
        let res_ptr = self.head.unwrap();
        let res_addr = res_ptr as usize;
        return Some(res_addr);
    }

    pub fn remove(&mut self, node_addr: usize) -> bool {
        if !self.contains(node_addr) {
            return false;
        }
        unsafe {
            let node_ptr = node_addr as *mut Node;
            if (*node_ptr).prev.is_some() {
                (*(*node_ptr).prev.unwrap()).next = (*node_ptr).next;
            }
            if (*node_ptr).next.is_some() {
                (*(*node_ptr).next.unwrap()).prev = (*node_ptr).prev;
            }
            if self.head.unwrap() == node_ptr {
                self.head = (*node_ptr).next;
            }
            if self.tail.unwrap() == node_ptr {
                self.tail = (*node_ptr).prev;
            }
            (*node_ptr).next = None;
            (*node_ptr).prev = None;
            self.len -= 1;
        }

        true
    }

    pub fn contains(&self, node_addr: usize) -> bool {
        if self.len <= 0 {
            return false;
        }
        let node_ptr = node_addr as *mut Node;
        unsafe {
            let mut curr_node = self.head;
            while curr_node.is_some() && curr_node.unwrap() != node_ptr {
                curr_node = (*curr_node.unwrap()).next;
            }
            return curr_node.is_some();
        }
    }
}
