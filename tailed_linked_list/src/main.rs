struct Tailed_Linked_list {
    head: *mut Node,
    tail: *mut Node
}

struct Node {
    value: u32,
    next: *mut Node
}

fn main() {
    println!("Hello, world!");
}
