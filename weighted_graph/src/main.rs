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

    fn add_vertex(&mut self) -> Vertex {
        let id = self.adjacency_list.len();
        let vertex = Vertex::new(id);
        self.vertices.push(vertex);
        self.adjacency_list.push(Vec::new());
        vertex
    }

    fn add_arc(&mut self, start: Vertex, end: Vertex, weight: i32) {
        self.adjacency_list[start.id].push(Arc::new(end, weight));
    }

    fn shortest_path(&self, start: Vertex, end: Vertex) -> Option<Vec<Vertex>> {
        let (distances, predecessors) = match self.topological_sort() {
            Some(order) => self.topological_shortest_path(start, &order),
            None => {
                if self.has_negative_weight() {
                    self.bellman_ford(start)?
                } else {
                    self.dijkstra(start)
                }
            }
        };

        if distances[end.id] == i32::MAX {
            return None;
        }

        let mut path = vec![];
        let mut current = end.id;
        while let Some(pred) = predecessors[current] {
            path.push(self.vertices[current]);
            current = pred;
        }
        path.push(self.vertices[current]);

        path.reverse();
        Some(path)
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

    fn topological_shortest_path(
        &self,
        start: Vertex,
        order: &[Vertex],
    ) -> (Vec<i32>, Vec<Option<usize>>) {
        let mut distances = vec![i32::MAX; self.vertices.len()];
        let mut predecessors = vec![None; self.vertices.len()];

        distances[start.id] = 0;

        for vertex in order {
            for arc in self.adjacency_list[vertex.id].iter() {
                self.relax(vertex.id, &arc, &mut distances, &mut predecessors);
            }
        }

        (distances, predecessors)
    }

    fn dijkstra(&self, start: Vertex) -> (Vec<i32>, Vec<Option<usize>>) {
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

            let Some(_) = smallest else {
                break;
            };

            let vertex_index = match smallest {
                None => break,
                Some(index) if distances[index] == i32::MAX => break,
                Some(index) => index,
            };

            visited[vertex_index] = true;
            for arc in self.adjacency_list[vertex_index].iter() {
                self.relax(vertex_index, &arc, &mut distances, &mut predecessors);
            }
        }

        return (distances, predecessors);
    }

    fn bellman_ford(&self, start: Vertex) -> Option<(Vec<i32>, Vec<Option<usize>>)> {
        todo!()
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

    let shortest_path = weighted_graph.shortest_path(vertex_a, vertex_h);

    //println!("Shortest pat from A to H is: {:?}", shortest_path);
}
