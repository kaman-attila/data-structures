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

    fn compare_with(self, point: Point) -> (Point, Point) {
        if self.x > point.x {
            return (self, point);
        } else if self.x == point.x {
            if self.y > point.y {
                return (self, point);
            } else if self.y == point.y {
                if self.z > point.z {
                    return (self, point);
                }
            }
        }

        (point, self)
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

struct Vertex {
    a: Point,
    b: Point,
    triangle_index: usize,
}

struct Graph {
    adjacency_list: Vec<Vec<Vertex>>,
}

impl Graph {
    fn new(vertex_count: usize) -> Self {
        Graph {
            adjacency_list: Vec::with_capacity(vertex_count),
        }
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

fn compare_points(points: (Point, Point)) -> (Point, Point) {
    if points.0.x > points.1.x {
        return (points.0, points.1);
    } else if points.0.x == points.1.x {
        if points.0.y > points.1.y {
            return (points.0, points.1);
        } else if points.0.y == points.1.y {
            if points.0.z > points.1.z {
                return (points.0, points.1);
            }
        }
    }

    (points.1, points.0)
}

fn build_adjacency(mesh: &Mesh) -> Vec<Vec<Vertex>> {
    let mut vertices: Vec<Vertex> = Vec::with_capacity(mesh.len() * 3);

    for (i, triangle) in mesh.triangles.iter().enumerate() {
        for j in 0..3 {
            let points: (Point, Point) = if j < 2 {
                triangle[j].compare_with(triangle[j + 1])
            } else {
                triangle[j].compare_with(triangle[0])
            };

            vertices.push(Vertex {
                a: points.0,
                b: points.1,
                triangle_index: i,
            });
        }
    }

    let mut adjacency_list = Vec::with_capacity(mesh.len());

    adjacency_list
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

    load_graph(&mesh);
}
