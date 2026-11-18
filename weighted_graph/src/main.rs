#[derive(Clone, Copy, PartialEq, Eq)]
enum VertexColor {
    WHITE,
    GREY,
    BLACK,
}

#[derive(Clone, Copy)]
struct Vertex {
    id: usize,
}

impl Vertex {
    fn new(id: usize) -> Self {
        Vertex { id: id }
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

    fn add_vertex(&mut self) -> Vertex {
        let id = self.adjacency_list.len();
        let vertex = Vertex::new(id);
        self.adjacency_list.push(Vec::new());
        vertex
    }

    fn add_arc(&mut self, start: Vertex, end: Vertex, weight: i32) {
        self.adjacency_list[start.id].push(Arc::new(start, end, weight));
    }

    fn shortest_path(&self, start: &mut Vertex, end: &mut Vertex) -> Option<Vec<Arc>> {
        self.topological_sort()
    }
}

fn main() {
    let mut weighted_graph = WeightedGraph::new();

    let mut id_counter = 0;

    let vertex_a = weighted_graph.add_vertex();
    let vertex_b = weighted_graph.add_vertex();
    let vertex_c = weighted_graph.add_vertex();
    let vertex_d = weighted_graph.add_vertex();
    let vertex_e = weighted_graph.add_vertex();
    let vertex_f = weighted_graph.add_vertex();
    let vertex_g = weighted_graph.add_vertex();
    let vertex_h = weighted_graph.add_vertex();

    weighted_graph.add_arc(vertex_a, vertex_b, 3);
    weighted_graph.add_arc(vertex_a, vertex_c, 4);
    weighted_graph.add_arc(vertex_c, vertex_b, -4);
    weighted_graph.add_arc(vertex_c, vertex_d, 2);
    weighted_graph.add_arc(vertex_b, vertex_d, 4);
    weighted_graph.add_arc(vertex_d, vertex_h, 9);
    weighted_graph.add_arc(vertex_a, vertex_e, 5);
    weighted_graph.add_arc(vertex_e, vertex_f, 1);
    weighted_graph.add_arc(vertex_f, vertex_g, 1);
    weighted_graph.add_arc(vertex_g, vertex_h, 1);

    let shortest_path = weighted_graph.shortest_path(&mut vertex_a, &mut vertex_h);

    println!("Shortest pat from A to H is: {:?}", shortest_path);
}
