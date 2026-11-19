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

    fn push_front(&mut self, value: u32) {}

    fn push_back(&mut self, value: u32) {}

    fn pop_front(&mut self) -> Option<u32> {}

    fn pop_back(&mut self) -> Option<u32> {}

    fn at(&self, i: usize) -> Option<u32> {}

    fn size(&self) -> u32 {}

    fn is_empty(&self) -> bool {}

    fn debug(&self) {}
}

fn main() {
    println!("Hello, world!");
}
