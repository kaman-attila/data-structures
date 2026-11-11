struct TailedLinkedList {
    head: *mut Node,
    tail: *mut Node
}

struct Node {
    value: u32,
    next: *mut Node
}

impl TailedLinkedList {
    fn new() -> Self {
        TailedLinkedList { head: std::ptr::null_mut(), tail: std::ptr::null_mut() }
    }

    fn push_front(&mut self, value: u32) {
        self.head = Node::new(value, self.head);
        if self.tail.is_null() {
            self.tail = self.head;
        }
    }

    fn pop_front(&mut self) -> u32 {

    }

    fn push_back(&mut self, value: u32) {
        let new_node = Node::new(value, std::ptr::null_mut());
        if self.tail.is_null() {
            self.head = new_node;
        }
        else {
            unsafe{(*self.tail).next = new_node;}
        }
        self.tail = new_node;
    }

    fn pop_back(&mut self) -> u32 {

    }

    fn size(&mut self) -> u32 {

    }

    fn is_empty(&mut self) -> bool {

    }

    fn contains(&mut self, value: u32) -> bool {

    }

    fn at(&mut self, index: usize) -> u32 {

    }

    fn insert(&mut self, value: u32, index: usize) {

    }
}

impl Node {
    fn new(value: u32, next: *mut Node) -> *mut Node {
        let layout = std::alloc::Layout::new::<Node>();
        unsafe {
            let pointer : *mut Node = std::alloc::alloc(layout) as *mut Node;
            (*pointer).value = value;
            (*pointer).next = next;
            pointer as *mut Node
        }
    }
}

fn main() {
    let mut tll = TailedLinkedList::new();
}
