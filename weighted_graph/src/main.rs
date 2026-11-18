use crate::Color::WHITE;

enum Color {
    WHITE,
    GREY,
    BLACK,
}

struct Vertex {
    id: usize,
    color: Color,
}

impl Vertex {
    fn new(id_counter: usize) -> Self {
        Vertex {
            id: id_counter,
            color: WHITE,
        }
    }
}

struct Arc {
    start: Vertex,
    end: Vertex,
    weight: i32,
}

impl Arc {
    fn new(start: Vertex, end: Vertex, weight: i32) -> Self {
        Arc { start, end, weight }
    }
}

struct WeightedGraph {
    adjacency_list: Vec<Vec<Vertex>>,
}

impl WeightedGraph {
    fn new() -> Self {
        WeightedGraph {
            adjacency_list: vec![],
        }
    }
}

fn main() {
    println!("Hello, world!");
}
