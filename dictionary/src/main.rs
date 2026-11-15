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
    size: u32,
}

impl Dictionary {
    fn new() -> Self {
        Dictionary {
            root: std::ptr::null_mut(),
        }
    }

    fn insert(&mut self, key: String, value: i32) {}

    fn remove(&mut self, key: &str) -> Option<i32> {}

    fn get(&self, key: &str) -> Option<i32> {}

    fn contains(&self, key: &str) -> bool {}

    fn min(&self) -> Option<(String, i32)> {}

    fn max(&self) -> Option<(String, i32)> {}

    fn size(&self) -> u32 {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.root.is_null()
    }
}

fn main() {
    let mut dictionary = Dictionary::new();
}
