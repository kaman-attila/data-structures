struct Node {
    key: String,
    value: i32,
    left_child: *mut Node,
    right_child: *mut Node,
}

impl Node {
    fn new(key: String, value: i32) -> *mut Node {
        let layout = std::alloc::Layout::new::<Node>();
        unsafe {
            let pointer: *mut Node = std::alloc::alloc(layout) as *mut Node;
            (*pointer).key = key;
            (*pointer).value = value;
            (*pointer).left_child = std::ptr::null_mut();
            (*pointer).right_child = std::ptr::null_mut();
            pointer as *mut Node
        }
    }
}

struct Dictionary {
    root: *mut Node,
}

impl Dictionary {
    fn new() -> Self {
        Dictionary {
            root: std::ptr::null_mut(),
        }
    }
}

fn main() {
    let mut dictionary = Dictionary::new();
}
