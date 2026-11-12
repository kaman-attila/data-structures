use std::fs::File;
use std::io::{BufWriter, Write};

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

fn distance_pp(p1: Point, p2: Point) -> f32 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt()
}

fn distance_pt(p: Point, t: Triangle) -> f32 {
    let d0 = distance_pp(p, t[0]);
    let d1 = distance_pp(p, t[1]);
    let d2 = distance_pp(p, t[2]);
    d0.min(d1).min(d2)
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

fn free_mesh(mesh: &mut Mesh) {
    mesh.triangles.clear();
    mesh.triangles.shrink_to_fit();
}

fn save_to_file(mesh: &Mesh, filename: &str) {
    let file = File::create(filename)
        .unwrap_or_else(|error| panic!("failed to create mesh file '{filename}': {error}"));
    let mut writer = BufWriter::new(file);
    let _ = mesh;
    writeln!(writer, "{}", mesh.triangles.len()).expect("failed to write triangle count");
    for triangle in mesh.triangles.iter() {
        for point in triangle {
            write!(writer, "{} {} {} ", point.x, point.y, point.z)
                .expect("failed to write vertex coordinates");
        }
        write!(writer, "\n").expect("failed to write new line");
    }
}

fn closest_triangles(mesh: &Mesh, p: Point, count: usize) -> Mesh {
    let mut closest_mesh = Mesh::new(count);
    let mut distances: Vec<(f32, Triangle)> = vec![];

    for triangle in mesh.triangles.iter() {
        distances.push((distance_pt(p, *triangle), *triangle));
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let limit = count.min(distances.len());
    for i in 0..limit {
        closest_mesh.push(distances[i].1);
    }

    closest_mesh
}

fn main() {
    let mesh = load_mesh("mesh/bunny.txt");
    save_to_file(&mesh, "mesh/test_bunny_copy.txt");

    let players = [
        Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Point {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        },
    ];
    let counts = [1, 5, 10, 100, 1000, 10000, 100000];

    for (player_index, player) in players.iter().enumerate() {
        for count in counts {
            println!("Player {}, Count {}", player_index + 1, count);
            let mut closest = closest_triangles(&mesh, *player, count);
            let filename = format!("mesh/player_{}_count_{}.mesh", player_index + 1, count);
            save_to_file(&closest, &filename);
            free_mesh(&mut closest);
        }
    }
}
