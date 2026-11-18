#[derive(Clone, Copy, PartialEq, Eq)]
enum VertexColor {
    WHITE,
    GREY,
    BLACK,
}

struct PathResult {
    path: Vec<Vertex>,
    total_distance: i32,
}

impl PathResult {
    fn new(path: Vec<Vertex>, total_distance: i32) -> Self {
        PathResult {
            path,
            total_distance,
        }
    }
}

struct AlgorithmResult {
    distances: Vec<i32>,
    predecessors: Vec<Option<usize>>,
}

impl AlgorithmResult {
    fn new(distances: Vec<i32>, predecessors: Vec<Option<usize>>) -> Self {
        AlgorithmResult {
            distances,
            predecessors,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Vertex {
    id: usize,
    name: char,
}

impl Vertex {
    fn new(id: usize, name: char) -> Self {
        Vertex { id: id, name: name }
    }
}

#[derive(Clone, Copy)]
struct Arc {
    end: Vertex,
    weight: i32,
}

impl Arc {
    fn new(end: Vertex, weight: i32) -> Self {
        Arc { end, weight }
    }
}

struct WeightedGraph {
    vertices: Vec<Vertex>,
    adjacency_list: Vec<Vec<Arc>>,
}

impl WeightedGraph {
    fn new() -> Self {
        WeightedGraph {
            vertices: vec![],
            adjacency_list: vec![],
        }
    }

    fn add_vertex(&mut self, name: char) -> Vertex {
        let id = self.adjacency_list.len();
        let vertex = Vertex::new(id, name);
        self.vertices.push(vertex);
        self.adjacency_list.push(Vec::new());
        vertex
    }

    fn add_arc(&mut self, start: Vertex, end: Vertex, weight: i32) {
        self.adjacency_list[start.id].push(Arc::new(end, weight));
    }

    fn print_shortest_path_between(&self, start: Vertex, end: Vertex) {
        if let Some(result) = self.shortest_path(start, end) {
            print!(
                "The shortest path between {} and {} is: ",
                start.name, end.name
            );
            for vertex in result.path {
                print!("{} -> ", vertex.name);
            }

            print!("Done!\n");

            println!("Total distance is {}.", result.total_distance);
            return;
        }

        println!(
            "It seems like the graph has no shortest path between {} and {}.",
            start.name, end.name
        );
    }

    fn shortest_path(&self, start: Vertex, end: Vertex) -> Option<PathResult> {
        let result = match self.topological_sort() {
            Some(order) => self.topological_shortest_path(start, &order),
            None => {
                if self.has_negative_weight() {
                    self.bellman_ford(start)?
                } else {
                    self.dijkstra(start)
                }
            }
        };

        if result.distances[end.id] == i32::MAX {
            return None;
        }

        let mut path = vec![];
        let mut current = end.id;
        while let Some(pred) = result.predecessors[current] {
            path.push(self.vertices[current]);
            current = pred;
        }
        path.push(self.vertices[current]);

        path.reverse();
        Some(PathResult::new(path, result.distances[end.id]))
    }

    fn has_negative_weight(&self) -> bool {
        for starting_arcs in self.adjacency_list.iter() {
            for arc in starting_arcs {
                if arc.weight < 0 {
                    return true;
                }
            }
        }

        false
    }

    fn topological_sort(&self) -> Option<Vec<Vertex>> {
        let n = self.vertices.len();
        let mut vertex_colors = vec![VertexColor::WHITE; n];
        let mut post_order = Vec::with_capacity(n);

        for i in 0..n {
            if vertex_colors[i] == VertexColor::WHITE
                && !self.depth_first_search(i, &mut vertex_colors, &mut post_order)
            {
                return None;
            }
        }

        post_order.reverse();
        Some(post_order)
    }

    fn depth_first_search(
        &self,
        start: usize,
        vertex_colors: &mut Vec<VertexColor>,
        post_order: &mut Vec<Vertex>,
    ) -> bool {
        vertex_colors[start] = VertexColor::GREY;

        for arc in &self.adjacency_list[start] {
            let v = arc.end.id;
            match vertex_colors[v] {
                VertexColor::WHITE => {
                    if !self.depth_first_search(v, vertex_colors, post_order) {
                        return false;
                    }
                }
                VertexColor::GREY => return false,
                VertexColor::BLACK => {}
            }
        }

        vertex_colors[start] = VertexColor::BLACK;
        post_order.push(self.vertices[start]);
        true
    }

    fn topological_shortest_path(&self, start: Vertex, order: &[Vertex]) -> AlgorithmResult {
        let mut distances = vec![i32::MAX; self.vertices.len()];
        let mut predecessors = vec![None; self.vertices.len()];

        distances[start.id] = 0;

        for vertex in order {
            for arc in self.adjacency_list[vertex.id].iter() {
                self.relax(vertex.id, arc, &mut distances, &mut predecessors);
            }
        }

        AlgorithmResult::new(distances, predecessors)
    }

    fn dijkstra(&self, start: Vertex) -> AlgorithmResult {
        let mut distances = vec![i32::MAX; self.vertices.len()];
        let mut predecessors = vec![None; self.vertices.len()];
        let mut visited = vec![false; self.vertices.len()];

        distances[start.id] = 0;

        loop {
            let mut smallest: Option<usize> = None;

            for vertex in &self.vertices {
                let vertex_index = vertex.id;

                if visited[vertex_index] || distances[vertex_index] == i32::MAX {
                    continue;
                }

                let is_better = match smallest {
                    None => true,
                    Some(smallest_index) => distances[vertex_index] < distances[smallest_index],
                };

                if is_better {
                    smallest = Some(vertex_index);
                }
            }

            let Some(index) = smallest else {
                break;
            };

            visited[index] = true;
            for arc in self.adjacency_list[index].iter() {
                self.relax(index, arc, &mut distances, &mut predecessors);
            }
        }

        AlgorithmResult::new(distances, predecessors)
    }

    fn bellman_ford(&self, start: Vertex) -> Option<AlgorithmResult> {
        let n = self.vertices.len();
        let mut distances = vec![i32::MAX; n];
        let mut predecessors = vec![None; n];

        distances[start.id] = 0;

        for _ in 1..n {
            for i in 0..n {
                for arc in &self.adjacency_list[i] {
                    self.relax(i, arc, &mut distances, &mut predecessors);
                }
            }
        }

        for i in 0..n {
            for arc in &self.adjacency_list[i] {
                if self.relax(i, arc, &mut distances, &mut predecessors) {
                    return None;
                }
            }
        }

        Some(AlgorithmResult::new(distances, predecessors))
    }

    fn relax(
        &self,
        start: usize,
        arc: &Arc,
        distances: &mut [i32],
        predecessors: &mut [Option<usize>],
    ) -> bool {
        if distances[start] == i32::MAX {
            return false;
        }

        if distances[arc.end.id] > distances[start] + arc.weight {
            distances[arc.end.id] = distances[start] + arc.weight;
            predecessors[arc.end.id] = Some(start);
            return true;
        }

        false
    }
}

fn main() {
    let mut weighted_graph = WeightedGraph::new();

    let vertex_a = weighted_graph.add_vertex('A');
    let vertex_b = weighted_graph.add_vertex('B');
    let vertex_c = weighted_graph.add_vertex('C');
    let vertex_d = weighted_graph.add_vertex('D');
    let vertex_e = weighted_graph.add_vertex('E');
    let vertex_f = weighted_graph.add_vertex('F');
    let vertex_g = weighted_graph.add_vertex('G');
    let vertex_h = weighted_graph.add_vertex('H');

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

    weighted_graph.print_shortest_path_between(vertex_a, vertex_h);
}
