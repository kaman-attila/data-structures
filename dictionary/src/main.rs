struct Node {
    key: String,
    value: i32,
    deleted: bool,
    left_child: *mut Node,
    right_child: *mut Node,
}

impl Node {
    fn new(key: String, value: i32) -> *mut Node {
        let layout = std::alloc::Layout::new::<Node>();
        unsafe {
            let pointer: *mut Node = std::alloc::alloc(layout) as *mut Node;
            if pointer.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            pointer.write(Node {
                key,
                value,
                deleted: false,
                left_child: std::ptr::null_mut(),
                right_child: std::ptr::null_mut(),
            });
            pointer
        }
    }
}

fn find_min(node: *mut Node) -> *mut Node {
    if node.is_null() {
        return node;
    }
    unsafe {
        let left = find_min((*node).left_child);
        if !left.is_null() {
            return left;
        }
        if !(*node).deleted {
            return node;
        }
        find_min((*node).right_child)
    }
}

fn find_max(node: *mut Node) -> *mut Node {
    if node.is_null() {
        return node;
    }
    unsafe {
        let right = find_max((*node).right_child);
        if !right.is_null() {
            return right;
        }
        if !(*node).deleted {
            return node;
        }
        find_max((*node).left_child)
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

    fn insert(&mut self, key: String, value: i32) {
        if self.root.is_null() {
            self.root = Node::new(key, value);
            self.size += 1;
            return;
        }

        let mut current = self.root;
        unsafe {
            loop {
                if (*current).key > key {
                    if (*current).left_child.is_null() {
                        (*current).left_child = Node::new(key, value);
                        self.size += 1;
                        return;
                    }
                    current = (*current).left_child;
                } else if (*current).key < key {
                    if (*current).right_child.is_null() {
                        (*current).right_child = Node::new(key, value);
                        self.size += 1;
                        return;
                    }
                    current = (*current).right_child;
                } else {
                    if (*current).deleted {
                        (*current).deleted = false;
                        self.size += 1;
                    }
                    (*current).value = value;
                    return;
                }
            }
        }
    }

    fn remove(&mut self, key: &str) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let mut current = self.root;
        unsafe {
            while !current.is_null() {
                if (*current).key.as_str() > key {
                    current = (*current).left_child;
                } else if (*current).key.as_str() < key {
                    current = (*current).right_child;
                } else {
                    if (*current).deleted {
                        return None;
                    }

                    (*current).deleted = true;
                    self.size -= 1;
                    return Some((*current).value);
                }
            }
        }

        None
    }

    fn get(&self, key: &str) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let mut current = self.root;
        unsafe {
            while !current.is_null() {
                if (*current).key.as_str() > key {
                    current = (*current).left_child;
                } else if (*current).key.as_str() < key {
                    current = (*current).right_child;
                } else {
                    if (*current).deleted {
                        return None;
                    }
                    return Some((*current).value);
                }
            }
        }

        None
    }

    fn contains(&self, key: &str) -> bool {
        if self.is_empty() {
            return false;
        }

        let mut current = self.root;
        unsafe {
            while !current.is_null() {
                if (*current).key.as_str() > key {
                    current = (*current).left_child;
                } else if (*current).key.as_str() < key {
                    current = (*current).right_child;
                } else {
                    if (*current).deleted {
                        return false;
                    }
                    return true;
                }
            }
        }

        false
    }

    fn min(&self) -> Option<(String, i32)> {
        let node = find_min(self.root);
        if node.is_null() {
            return None;
        }
        unsafe { Some(((*node).key.clone(), (*node).value)) }
    }

    fn max(&self) -> Option<(String, i32)> {
        let node = find_max(self.root);
        if node.is_null() {
            return None;
        }
        unsafe { Some(((*node).key.clone(), (*node).value)) }
    }

    fn size(&self) -> u32 {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

fn main() {
    let mut dictionary = Dictionary::new();

    if dictionary.is_empty() {
        println!("This dictionary is empty.");
    } else {
        println!("This dictionary is not empty.");
    }

    dictionary.insert("age".to_string(), 19);

    println!("The size of the dictionary is: {}.", dictionary.size());

    dictionary.insert("height".to_string(), 186);
    dictionary.insert("weight".to_string(), 87);

    dictionary.insert("height".to_string(), 187);

    dictionary.insert("speed".to_string(), 22);

    dictionary.remove("weight");

    let key = "height";
    let got = dictionary.get(key);
    if let Some(x) = got {
        println!("The value for key: {} is {}.", key, x);
    } else {
        println!("Dictionary has no value for key: {}.", key)
    }

    let key_2 = "length";
    if dictionary.contains(key_2) {
        println!("This dictionary contains the key: {}.", key_2);
    } else {
        println!("This dictionary does not contain the key: {}.", key_2);
    }

    let minimum = dictionary.min();
    if let Some((key, value)) = minimum {
        println!(
            "The minimum of this dictionary is key: {}, value: {}.",
            key, value
        )
    } else {
        println!("Dictionary has no minimum.")
    }

    let maximum = dictionary.max();
    if let Some((key, value)) = maximum {
        println!(
            "The maximum of this dictionary is key: {}, value: {}.",
            key, value
        )
    } else {
        println!("Dictionary has no maximum.")
    }
}
