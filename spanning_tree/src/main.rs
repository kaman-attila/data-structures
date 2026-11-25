#[derive(Clone, Copy, Debug)]
struct Edge {
    weight: u32,
    u: usize,
    v: usize,
}

impl Edge {
    fn new(weight: u32, u: usize, v: usize) -> Self;
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self;
    fn find(&mut self, x: usize) -> usize;
    fn union(&mut self, a: usize, b: usize) -> bool;
    fn components(&self) -> usize;
    fn debug(&self);
}

struct SpanningTree {
    edges: Vec<Edge>,
    total: u32,
    components: usize,
}

impl SpanningTree {
    fn is_connected(&self) -> bool;
}

impl std::fmt::Display for SpanningTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

struct Graph {
    n: usize,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(n: usize) -> Self;
    fn add_edge(&mut self, weight: u32, u: usize, v: usize);
    fn adjacency_matrix(&self) -> Vec<Vec<Option<u32>>>;
    fn kruskal(&self) -> SpanningTree;
    fn maximum_spanning_tree(&self) -> SpanningTree;
    fn prim(&self, start: usize) -> SpanningTree;
}

fn merge_sort(edges: &mut Vec<Edge>, descending: bool);
fn name(i: usize) -> char;

fn main() {
    todo!();
}