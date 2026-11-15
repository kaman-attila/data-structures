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
        let node = self.find(key);
        unsafe {
            if !node.is_null() {
                (*node).deleted = true;
                self.size -= 1;
                return Some((*node).value);
            }
        }

        None
    }

    fn get(&self, key: &str) -> Option<i32> {
        let node = self.find(key);
        unsafe {
            if !node.is_null() {
                return Some((*node).value);
            }
        }

        None
    }

    fn contains(&self, key: &str) -> bool {
        !self.find(key).is_null()
    }

    fn find(&self, key: &str) -> *mut Node {
        if self.is_empty() {
            return std::ptr::null_mut();
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
                        return std::ptr::null_mut();
                    }
                    return current;
                }
            }
        }

        std::ptr::null_mut()
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

fn help_drop(node: *mut Node, layout: std::alloc::Layout) {
    if node.is_null() {
        return;
    }

    unsafe {
        help_drop((*node).left_child, layout);
        help_drop((*node).right_child, layout);

        std::ptr::drop_in_place(node);
        std::alloc::dealloc(node as *mut u8, layout);
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::new::<Node>();

        help_drop(self.root, layout);
    }
}

fn in_order_traverse(f: &mut std::fmt::Formatter<'_>, node: *mut Node) -> std::fmt::Result {
    if node.is_null() {
        return Ok(());
    }

    unsafe {
        in_order_traverse(f, (*node).left_child)?;

        if !(*node).deleted {
            writeln!(f, "    {}: {},", (*node).key, (*node).value)?;
        }

        in_order_traverse(f, (*node).right_child)
    }
}

impl std::fmt::Display for Dictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        in_order_traverse(f, self.root)?;
        write!(f, "}}")
    }
}

fn main() {
    let mut dictionary = Dictionary::new();
    println!("{}", dictionary);

    if dictionary.is_empty() {
        println!("This dictionary is empty.");
    } else {
        println!("This dictionary is not empty.");
    }

    dictionary.insert("age".to_string(), 19);
    println!("{}", dictionary);

    println!("The size of the dictionary is: {}.", dictionary.size());

    dictionary.insert("height".to_string(), 186);
    dictionary.insert("weight".to_string(), 87);
    println!("{}", dictionary);

    dictionary.insert("height".to_string(), 187);
    println!("{}", dictionary);

    dictionary.insert("speed".to_string(), 22);
    println!("{}", dictionary);

    dictionary.remove("weight");
    println!("{}", dictionary);

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

    drop(dictionary);
}
