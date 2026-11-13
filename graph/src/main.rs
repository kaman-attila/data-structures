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

fn display(days: u32) {
    println!(
        "It takes {} days for the bunny to become completely rusty!",
        days
    );
}

fn main() {
    let chosen_triangle: usize = 8; // HARDCODED INPUT

    let mesh = load_mesh("mesh/bunny.txt");
}
