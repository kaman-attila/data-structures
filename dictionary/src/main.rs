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
            size: 0,
        }
    }

    fn insert(&mut self, key: String, value: i32) {}

    fn remove(&mut self, key: &str) -> Option<i32> {}

    fn get(&self, key: &str) -> Option<i32> {
        if self.is_empty() { return None; }

        let mut current = self.root;
        unsafe {
            while !current.is_null() {
                if (*current).key > key.to_string() {
                    current = (*current).left_child;
                }
                else if (*current).key < key.to_string() {
                    current = (*current).right_child;
                }
                else {
                    return Some((*current).value);
                }
            }
        }

        None
    }

    fn contains(&self, key: &str) -> bool {
        if self.is_empty() { return false; }

        let mut current = self.root;
        unsafe {
            while !current.is_null() {
                if (*current).key > key.to_string() {
                    current = (*current).left_child;
                }
                else if (*current).key < key.to_string() {
                    current = (*current).right_child;
                }
                else {
                    return true;
                }
            }
        }

        false
    }

    fn min(&self) -> Option<(String, i32)> {
        if self.is_empty() { return None; }

        let mut current = self.root;
        unsafe {
            while !(*current).left_child.is_null() {
                current = (*current).left_child;
            }

            Some(((*current).key.clone(), (*current).value))
        }
    }

    fn max(&self) -> Option<(String, i32)> {
        if self.is_empty() { return None; }

        let mut current = self.root;
        unsafe {
            while !(*current).right_child.is_null() {
                current = (*current).right_child;
            }

            Some(((*current).key.clone(), (*current).value))
        }
    }

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
