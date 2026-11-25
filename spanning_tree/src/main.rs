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

fn merge_sort(edges: &mut Vec<Edge>, descending: bool) {
    if edges.len() < 2 {
        return;
    }

    let mid = edges.len() / 2;
    let mut left = edges[..mid].to_vec();
    let mut right = edges[mid..].to_vec();

    merge_sort(&mut left, descending);
    merge_sort(&mut right, descending);

    *edges = merge(&left, &right, descending);
}

fn merge(left: &[Edge], right: &[Edge], descending: bool) -> Vec<Edge> {
    let mut sorted_list = Vec::with_capacity(left.len() + right.len());

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        let take_left = if descending {
            left[i].weight >= right[j].weight
        } else {
            left[i].weight <= right[j].weight
        };

        if take_left {
            sorted_list.push(left[i]);
            i += 1;
        } else {
            sorted_list.push(right[j]);
            j += 1;
        }
    }

    sorted_list.extend_from_slice(&left[i..]);
    sorted_list.extend_from_slice(&right[j..]);

    sorted_list
}

fn name(i: usize) -> char;

fn main() {
    todo!();
}
