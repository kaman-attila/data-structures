struct GapBuffer {
    capacity: usize,
    data: *mut char,
    left: usize,
    right: usize,
}

fn initialize_buffer(size: usize) -> GapBuffer {
    let layout = std::alloc::Layout::array::<char>(size).unwrap();
    unsafe {
        GapBuffer {
            capacity: size,
            data: std::alloc::alloc(layout) as *mut char,
            left: 0,
            right: size,
        }
    }
}

fn debug(gb: &GapBuffer) {
    for i in 0..gb.capacity {
        if i >= gb.left && i < gb.right {
            print!("_");
        } else {
            print!("{}", at(gb, i));
        }
    }
    print!("\n");
}

fn type_char(gb: &mut GapBuffer, c: char) {
    if gb.left == gb.right {
        resize(gb);
    }

    unsafe {
        *gb.data.add(gb.left) = c;
    }
    gb.left += 1;
}

fn resize(gb: &mut GapBuffer) {
    let old_capacity = gb.capacity;
    let new_capacity = old_capacity * 2;

    let old_layout = std::alloc::Layout::array::<char>(old_capacity).unwrap();
    let new_layout = std::alloc::Layout::array::<char>(new_capacity).unwrap();

    let data = unsafe { std::alloc::alloc(new_layout) as *mut char };
    for i in 0..gb.left {
        unsafe {
            *data.add(i) = *gb.data.add(i);
        }
    }

    let right_len = old_capacity - gb.right;
    let new_right = new_capacity - right_len;
    for i in 0..right_len {
        unsafe {
            *data.add(new_right + i) = *gb.data.add(gb.right + i);
        }
    }

    unsafe { std::alloc::dealloc(gb.data as *mut u8, old_layout) }
    gb.data = data;
    gb.capacity = new_capacity;
    gb.right = new_right;
}

fn insert_text(gb: &mut GapBuffer, text: &str) {
    for c in text.chars() {
        type_char(gb, c);
    }
}

fn move_left(gb: &mut GapBuffer) {
    if gb.left == 0 {
        return;
    }

    let temp = get_last_left(gb);

    unsafe {
        *gb.data.add(gb.right - 1) = temp;
    }

    gb.left -= 1;
    gb.right -= 1;
}

fn move_right(gb: &mut GapBuffer) {
    if gb.right == gb.capacity {
        return;
    }

    let temp = get_first_right(gb);

    unsafe {
        *gb.data.add(gb.left) = temp;
    }

    gb.left += 1;
    gb.right += 1;
}

fn get_last_left(gb: &GapBuffer) -> char {
    at(gb, gb.left - 1)
}

fn get_first_right(gb: &GapBuffer) -> char {
    at(gb, gb.right)
}

fn at(gb: &GapBuffer, index: usize) -> char {
    unsafe { *gb.data.add(index) }
}

fn backspace(gb: &mut GapBuffer) {
    gb.left -= 1;
}

fn main() {
    let mut gb = initialize_buffer(10);
    insert_text(&mut gb, "This!");
    debug(&gb);
    move_left(&mut gb);
    debug(&gb);
    insert_text(&mut gb, " is a ni");
    debug(&gb);
    insert_text(&mut gb, "ce Rap buffer");
    debug(&gb);
    for _ in 0..9 {
        move_left(&mut gb);
        debug(&gb);
    }
    backspace(&mut gb);
    type_char(&mut gb, 'G');
    for _ in 0..10 {
        move_right(&mut gb);
        debug(&gb);
    }
    insert_text(&mut gb, " ;-) ");
    debug(&gb);
}
