struct GapBuffer {
    capacity: usize,
    data: *mut char,
    left: usize,
    right: usize,
}

fn initialize_buffer(size: usize) -> GapBuffer {
    let layout = std::alloc::Layout::array::<char>(size).unwrap();
    unsafe {GapBuffer{capacity: size, data: std::alloc::alloc(layout) as *mut char, left: 0, right: size}}
}

fn debug(gb: &GapBuffer) {
    todo!();
}

fn type_char(gb: &mut GapBuffer, c: char) {
    todo!();
}

fn insert_text(gb: &mut GapBuffer, text: &str) {
    for c in text.chars() {
        type_char(gb, c);
    }
}

fn move_left(gb: &mut GapBuffer) {
    todo!();
}

fn move_right(gb: &mut GapBuffer) {
    todo!();
}

fn backspace(gb: &mut GapBuffer) {
    todo!();
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
