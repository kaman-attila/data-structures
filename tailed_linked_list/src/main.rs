struct Tailed_Linked_List {
    head: *mut Node,
    tail: *mut Node
}

struct Node {
    value: u32,
    next: *mut Node
}

impl Tailed_Linked_List {
    fn new() -> Self {
        Tailed_Linked_List { head: std::ptr::null_mut(), tail: std::ptr::null_mut() }
    }

    fn push_front() {

    }

    fn pop_front() {

    }

    fn push_back() {

    }

    fn pop_back() {

    }

    fn size() {

    }

    fn is_empty() {

    }

    fn contains() {

    }

    fn at() {

    }

    fn insert() {
        
    }
}

fn main() {
    let mut tll = Tailed_Linked_List::new();
}
