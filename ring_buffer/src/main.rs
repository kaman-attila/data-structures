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

    fn push_front(&mut self, value: u32) {
        if self.size == self.capacity {
            self.double_capacity();
        }

        let new_start = (self.start + self.capacity - 1) % self.capacity;

        unsafe {
            *self.data.add(new_start) = value;
        }

        self.start = new_start;
        self.size += 1;
    }

    fn push_back(&mut self, value: u32) {
        if self.size == self.capacity {
            self.double_capacity();
        }

        let position = (self.start + self.size) % self.capacity;

        unsafe {
            *self.data.add(position) = value;
        }

        self.size += 1;
    }

    fn double_capacity(&mut self) {
        let old_capacity = self.capacity;
        let new_capacity = old_capacity * 2;

        let old_layout = std::alloc::Layout::array::<u32>(old_capacity).unwrap();
        let new_layout = std::alloc::Layout::array::<u32>(new_capacity).unwrap();

        let data = unsafe { std::alloc::alloc(new_layout) as *mut u32 };
        if data.is_null() {
            std::alloc::handle_alloc_error(new_layout);
        }

        for i in 0..self.size {
            let old_idx = (self.start + i) % old_capacity;
            unsafe {
                *data.add(i) = *self.data.add(old_idx);
            }
        }

        unsafe { std::alloc::dealloc(self.data as *mut u8, old_layout) }

        self.data = data;
        self.capacity = new_capacity;
        self.start = 0;
    }

    fn pop_front(&mut self) -> Option<u32> {
        if self.is_empty() {
            return None;
        }

        let value = unsafe { *self.data.add(self.start) };

        self.start = (self.start + 1) % self.capacity;
        self.size -= 1;

        Some(value)
    }

    fn pop_back(&mut self) -> Option<u32> {
        if self.is_empty() {
            return None;
        }

        self.size -= 1;

        let index = (self.start + self.size) % self.capacity;
        let value = unsafe { *self.data.add(index) };

        if self.size() == 0 {
            self.start = 0;
        }

        Some(value)
    }

    fn at(&self, i: usize) -> Option<u32> {
        if i >= self.size {
            return None;
        }

        let position = (self.start + i) % self.capacity;

        unsafe { Some(*self.data.add(position)) }
    }

    fn size(&self) -> u32 {
        self.size as u32
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn debug(&self) {
        print!("[");

        let mut restart = false;
        let mut end = self.start + self.size;
        let mut end_2 = 0;
        if self.capacity < self.start + self.size {
            restart = true;
            end = self.capacity;
            end_2 = (self.start + self.size) % self.capacity;
        }

        for i in 0..self.capacity {
            if i == self.start {
                print!("start -> ");
            }

            if (self.start..end).contains(&i) {
                let value = unsafe { *self.data.add(i) };
                print!("{}", value);
            } else if restart && (0..end_2).contains(&i) {
                let value = unsafe { *self.data.add(i) };
                print!("{}", value);
            } else {
                print!("_");
            }

            if i != self.capacity - 1 {
                print!(", ");
            }
        }

        print!("]\n");
    }
}

fn main() {
    let mut rb = RingBuffer::new(3);
    rb.debug();
    rb.push_back(2);
    rb.debug();
    rb.push_front(5);
    rb.debug();
    rb.push_back(1);
    rb.debug();
    rb.push_back(8);
    rb.debug();
}
