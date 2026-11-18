enum VertexColor {
    WHITE,
    GREY,
    BLACK,
}

struct Vertex {
    id: usize,
    color: VertexColor,
}

impl Vertex {
    fn new(mut id_counter: usize) -> Self {
        id_counter += 1;
        Vertex {
            id: id_counter,
            color: VertexColor::WHITE,
        }
    }

    fn recolor(&mut self, new_color: VertexColor) {
        self.color = new_color;
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
    adjacency_list: Vec<Vec<Arc>>,
}

impl WeightedGraph {
    fn new() -> Self {
        WeightedGraph {
            adjacency_list: vec![],
        }
    }

    fn add_arc(&mut self, start: &Vertex, end: &Vertex, weight: i32) {
        self.adjacency_list[start.id].push(Arc { start, end, weight });
    }

    fn shortest_path(start: &mut Vertex, end: &mut Vertex) -> Vec<Arc> {}
}

fn main() {
    let mut weighted_graph = WeightedGraph::new();

    let mut id_counter = 0;

    let vertex_a = Vertex::new(id_counter);
    let vertex_b = Vertex::new(id_counter);
    let vertex_c = Vertex::new(id_counter);
    let vertex_d = Vertex::new(id_counter);
    let vertex_e = Vertex::new(id_counter);
    let vertex_f = Vertex::new(id_counter);
    let vertex_g = Vertex::new(id_counter);
    let vertex_h = Vertex::new(id_counter);

    weighted_graph.add_arc(&vertex_a, &vertex_b, 3);
    weighted_graph.add_arc(&vertex_a, &vertex_c, 4);
    weighted_graph.add_arc(&vertex_c, &vertex_b, -4);
    weighted_graph.add_arc(&vertex_c, &vertex_d, 2);
    weighted_graph.add_arc(&vertex_b, &vertex_d, 4);
    weighted_graph.add_arc(&vertex_d, &vertex_h, 9);
    weighted_graph.add_arc(&vertex_a, &vertex_e, 5);
    weighted_graph.add_arc(&vertex_e, &vertex_f, 1);
    weighted_graph.add_arc(&vertex_f, &vertex_g, 1);
    weighted_graph.add_arc(&vertex_g, &vertex_h, 1);
}
