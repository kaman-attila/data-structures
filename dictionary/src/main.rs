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
            (*pointer).key = key;
            (*pointer).value = value;
            (*pointer).deleted = false;
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

    fn insert(&mut self, key: String, value: i32) {
        if self.is_empty() {
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
                } else {
                    (*current).value = value;
                    (*current).deleted = false;
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
                if (*current).key > key.to_string() {
                    current = (*current).left_child;
                } else if (*current).key < key.to_string() {
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
                if (*current).key > key.to_string() {
                    current = (*current).left_child;
                } else if (*current).key < key.to_string() {
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
                if (*current).key > key.to_string() {
                    current = (*current).left_child;
                } else if (*current).key < key.to_string() {
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
        if self.is_empty() {
            return None;
        }

        let mut current = self.root;
        unsafe {
            while !(*current).left_child.is_null() {
                current = (*current).left_child;
            }

            Some(((*current).key.clone(), (*current).value))
        }
    }

    fn max(&self) -> Option<(String, i32)> {
        if self.is_empty() {
            return None;
        }

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

    if dictionary.is_empty() {
        println!("This dictionary is empty.");
    }
    else {
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

    if dictionary.contains("length") {
        println!("This dictionary contains the key: {}.", key);
    }
    else {
        println!("This dictionary does not contain the key: {}.", key);
    }

    let minimum = dictionary.min();
    if let Some((key, value)) = minimum {
        println!("The minimum of this dictionary is key: {}, value: {}.", key, value)
    } else {
        println!("Dictionary has no minimum.")
    }

    let maximum = dictionary.max();
    if let Some((key, value)) = maximum {
        println!("The maximum of this dictionary is key: {}, value: {}.", key, value)
    } else {
        println!("Dictionary has no maximum.")
    }
}
