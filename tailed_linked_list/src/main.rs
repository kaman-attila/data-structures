struct TailedLinkedList {
    head: *mut Node,
    tail: *mut Node,
}

struct Node {
    value: u32,
    next: *mut Node,
}

impl TailedLinkedList {
    fn new() -> Self {
        TailedLinkedList {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
        }
    }

    fn push_front(&mut self, value: u32) {
        self.head = Node::new(value, self.head);
        if self.tail.is_null() {
            self.tail = self.head;
        }
    }

    fn pop_front(&mut self) -> Option<u32> {
        if self.head.is_null() {
            return None;
        }

        let temp = self.head;
        let value;
        unsafe {
            value = (*temp).value;
            self.head = (*temp).next;
            let layout = std::alloc::Layout::new::<Node>();
            std::alloc::dealloc(temp as *mut u8, layout);
        }

        if self.head.is_null() {
            self.tail = std::ptr::null_mut();
        }

        Some(value)
    }

    fn push_back(&mut self, value: u32) {
        let new_node = Node::new(value, std::ptr::null_mut());
        if self.tail.is_null() {
            self.head = new_node;
        } else {
            unsafe {
                (*self.tail).next = new_node;
            }
        }
        self.tail = new_node;
    }

    fn pop_back(&mut self) -> Option<u32> {
        if self.head.is_null() {
            return None;
        }

        let temp = self.tail;
        let value;

        unsafe {
            value = (*temp).value;
            if (*self.head).next.is_null() {
                self.head = std::ptr::null_mut();
                self.tail = std::ptr::null_mut();
            } else {
                let mut current = self.head;
                while (*current).next != temp {
                    current = (*current).next;
                }
                (*current).next = std::ptr::null_mut();
                self.tail = current;
            }
            let layout = std::alloc::Layout::new::<Node>();
            std::alloc::dealloc(temp as *mut u8, layout);
        }

        Some(value)
    }

    fn size(&mut self) -> u32 {
        if self.head.is_null() {
            return 0;
        }

        let mut size = 1;
        let mut current = self.head;

        unsafe {
            while (*current).next != self.tail {
                size += 1;
                current = (*current).next;
            }
        }

        return size;
    }

    fn is_empty(&mut self) -> bool {
        if self.head.is_null() {
            return true;
        }

        false
    }

    fn contains(&mut self, value: u32) -> bool {
        if self.head.is_null() {
            return false;
        }

        let mut current = self.head;

        unsafe {
            while !(*current).next.is_null() {
                if (*current).value == value {
                    return true;
                }

                current = (*current).next;
            }
        }

        return false;
    }

    fn at(&mut self, index: usize) -> Option<u32> {
        if self.head.is_null() {
            return None;
        }

        let mut current = self.head;

        let value;
        unsafe {
            for _ in 0..index {
                if (*current).next.is_null() {
                    return None;
                }
                current = (*current).next;
            }

            value = (*current).value;
        }

        Some(value)
    }

    fn insert(&mut self, value: u32, index: usize) {
        if self.head.is_null() {
            return;
        }

        let mut current = self.head;

        unsafe {
            for _ in 0..index - 1 {
                if (*current).next.is_null() {
                    return;
                }
                current = (*current).next;
            }

            let new_node = Node::new(value, (*current).next);
            (*current).next = new_node;
        }
    }
}

impl Node {
    fn new(value: u32, next: *mut Node) -> *mut Node {
        let layout = std::alloc::Layout::new::<Node>();
        unsafe {
            let pointer: *mut Node = std::alloc::alloc(layout) as *mut Node;
            (*pointer).value = value;
            (*pointer).next = next;
            pointer as *mut Node
        }
    }
}

impl std::fmt::Display for TailedLinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head;
        write!(f, "[HEAD -> ")?;
        unsafe {
            while !current.is_null() {
                write!(f, "{} -> ", (*current).value)?;

                current = (*current).next;
            }
        }
        write!(f, "TAIL]")
    }
}

fn main() {
    let mut tll = TailedLinkedList::new();

    println!("{}", tll);
    tll.push_front(2);
    println!("{}", tll);
    tll.pop_front();
    println!("{}", tll);

    if tll.is_empty() {
        println!("Tailed linked list is empty.");
    } else {
        println!("Tailed linked list is not empty.");
    }

    tll.push_back(5);
    println!("{}", tll);
    tll.push_front(2);
    println!("{}", tll);
    tll.push_back(3);
    println!("{}", tll);
    tll.pop_back();
    println!("{}", tll);

    if tll.is_empty() {
        println!("Tailed linked list is empty.");
    } else {
        println!("Tailed linked list is not empty.");
    }

    tll.push_front(42);
    println!("{}", tll);
    tll.push_back(10);
    println!("{}", tll);

    if tll.contains(10) {
        println!("It contains this value!");
    } else {
        println!("It does not contain this value!");
    }

    println!("The size of this talied linked list: {}", tll.size());

    tll.insert(39, 1);
    println!("{}", tll);

    let index = 1;
    let found = tll.at(index);
    if let Some(x) = found {
        println!("The value at position {} is {}.", index, x);
    } else {
        println!("Tailed linked list has no value at position {}", index)
    }
}
