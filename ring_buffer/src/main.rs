struct RingBuffer {
    data: *mut u32,
    start: usize,
    size: usize,
    capacity: usize,
}

impl RingBuffer {
    fn new(capacity: usize) -> Self {
        let capacity = capacity.max(1);

        let layout = std::alloc::Layout::array::<u32>(capacity).unwrap();
        let data = unsafe { std::alloc::alloc(layout) as *mut u32 };
        if data.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        RingBuffer {
            data,
            start: 0,
            size: 0,
            capacity,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
