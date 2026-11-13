#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }

    fn is_larger(self, point: Point) -> bool {
        if self.x > point.x {
            return true;
        } else if self.x == point.x {
            if self.y > point.y {
                return true;
            } else if self.y == point.y {
                if self.z > point.z {
                    return true;
                }
            }
        }

        false
    }
}

type Triangle = [Point; 3];

struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    fn new(triangle_count: usize) -> Self {
        Mesh {
            triangles: Vec::with_capacity(triangle_count),
        }
    }

    fn push(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    fn len(&self) -> usize {
        self.triangles.len()
    }
}

#[derive(Clone, Copy)]
struct Vertex {
    a: Point,
    b: Point,
    triangle_index: usize,
}

impl Vertex {
    fn new(a: Point, b: Point, i: usize) -> Self {
        Vertex {
            a,
            b,
            triangle_index: i,
        }
    }

    fn is_larger(self, vertex: Vertex) -> bool {
        if self.a.is_larger(vertex.a) {
            return true;
        }
        if vertex.a.is_larger(self.a) {
            return false;
        }

        self.b.is_larger(vertex.b)
    }
}

struct Graph {
    adjacency_list: Vec<Vec<Vertex>>,
}

impl Graph {
    fn new(adjacency_list: Vec<Vec<Vertex>>) -> Self {
        Graph { adjacency_list }
    }

    fn push(value: f32) {}
}

fn load_mesh(filename: &str) -> Mesh {
    let contents = std::fs::read_to_string(filename)
        .unwrap_or_else(|error| panic!("failed to read mesh file '{filename}': {error}"));
    let mut values = contents.split_whitespace();
    let triangle_count = values
        .next()
        .expect("mesh file is missing its triangle count")
        .parse::<usize>()
        .unwrap_or_else(|error| panic!("invalid triangle count in '{filename}': {error}"));

    let mut mesh = Mesh::new(triangle_count);

    for _ in 0..triangle_count {
        let dummy_point = Point::new(0.0, 0.0, 0.0);
        let mut triangle: Triangle = [dummy_point, dummy_point, dummy_point];
        for p in 0..3 {
            let x = values
                .next()
                .expect("mesh file is missing a vertex coordinate")
                .parse::<f32>()
                .expect("mesh vertex coordinate is not a valid number");
            let y = values
                .next()
                .expect("mesh file is missing a vertex coordinate")
                .parse::<f32>()
                .expect("mesh vertex coordinate is not a valid number");
            let z = values
                .next()
                .expect("mesh file is missing a vertex coordinate")
                .parse::<f32>()
                .expect("mesh vertex coordinate is not a valid number");

            let point = Point::new(x, y, z);
            triangle[p] = point;
        }
        mesh.push(triangle);
    }
    mesh
}

fn build_adjacency(mesh: &Mesh) -> Vec<Vec<Vertex>> {
    let mut vertices: Vec<Vertex> = Vec::with_capacity(mesh.len() * 3);

    for (i, triangle) in mesh.triangles.iter().enumerate() {
        for j in 0..3 {
            if triangle[j].is_larger(triangle[(j + 1) % 3]) {
                vertices.push(Vertex::new(triangle[j], triangle[(j + 1) % 3], i));
            } else {
                vertices.push(Vertex::new(triangle[(j + 1) % 3], triangle[j], i));
            }
        }
    }

    let sorted_vertices = merge_sort(&vertices);

    let mut adjacency_list = Vec::with_capacity(mesh.len());

    let mut i = 0;
    while i <= sorted_vertices.len() {
        let mut j = i + 1;
        while j <= sorted_vertices.len() {}
    }

    adjacency_list
}

fn merge_sort(list: &[Vertex]) -> Vec<Vertex> {
    if list.len() > 1 {
        return list.to_vec();
    }

    let mid = list.len() / 2;
    let left = merge_sort(&list[..mid]);
    let right = merge_sort(list);

    merge(&left, &right)
}

fn merge(left: &[Vertex], right: &[Vertex]) -> Vec<Vertex> {
    let mut sorted_list = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i].is_larger(right[j]) {
            sorted_list.push(left[i]);
            i += 1;
        } else {
            sorted_list.push(right[j]);
            j += 1;
        }
    }
    sorted_list.append(&mut left[i..].to_vec());
    sorted_list.append(&mut right[j..].to_vec());

    sorted_list
}

fn display_answer(days: u32) {
    println!(
        "It takes {} days for the bunny to become completely rusty!",
        days
    );
}

fn display_day(graph: &Graph) {
    for (i, list) in graph.adjacency_list.iter().enumerate() {
        println!("Day {}: {} new triangles got rusty.", i, list.len());
    }
}

fn main() {
    let chosen_triangle: usize = 8; // HARDCODED INPUT

    let mesh = load_mesh("mesh/bunny.txt");

    let graph = Graph::new(build_adjacency(&mesh));
}
