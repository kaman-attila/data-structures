use std::fmt::write;

#[derive(Clone, Copy, Debug)]
struct Edge {
    weight: u32,
    u: usize,
    v: usize,
}

impl Edge {
    fn new(weight: u32, u: usize, v: usize) -> Self {
        Edge { weight, u, v }
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{} {}", name(self.u), name(self.v), self.weight)
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..=n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        if self.rank[root_a] > self.rank[root_b] {
            self.parent[root_b] = root_a;
        } else if self.rank[root_a] < self.rank[root_b] {
            self.parent[root_a] = root_b;
        } else {
            self.parent[root_b] = root_a;
            self.rank[root_a] += 1;
        }

        true
    }

    fn components(&self) -> usize {
        self.components
    }

    fn debug(&self) {
        println!("{:?}", self.parent);
    }
}

struct SpanningTree {
    edges: Vec<Edge>,
    total: u32,
    components: usize,
}

impl SpanningTree {
    fn is_connected(&self) -> bool {
        self.components == 1
    }
}

impl std::fmt::Display for SpanningTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

struct Graph {
    n: usize,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            n,
            edges: Vec::new(),
        }
    }

    fn add_edge(&mut self, weight: u32, u: usize, v: usize) {
        self.edges.push(Edge::new(weight, u, v));
    }

    fn adjacency_matrix(&self) -> Vec<Vec<Option<u32>>> {
        let mut matrix = vec![vec![None; self.n]; self.n];

        for edge in &self.edges {
            matrix[edge.u][edge.v] = Some(edge.weight);
            matrix[edge.v][edge.u] = Some(edge.weight);
        }

        matrix
    }

    fn kruskal(&self) -> SpanningTree {
        if self.n == 0 {
            return SpanningTree {
                edges: vec![],
                total: 0,
                components: 0,
            };
        }
        let mut edges = self.edges.clone();
        merge_sort(&mut edges, false);

        let mut spanning_tree = SpanningTree {
            edges: Vec::with_capacity(self.n - 1),
            total: 0,
            components: 0,
        };
        let mut union_find = UnionFind::new(self.n);

        for &edge in &edges {
            if spanning_tree.edges.len() + 1 == self.n {
                break;
            }

            if union_find.union(edge.u, edge.v) {
                spanning_tree.edges.push(edge);
                spanning_tree.total += edge.weight;
            }

            union_find.debug();
        }

        spanning_tree.components = union_find.components();
        spanning_tree
    }

    fn maximum_spanning_tree(&self) -> SpanningTree;

    fn prim(&self, start: usize) -> SpanningTree {
        if self.n == 0 || start >= self.n {
            return SpanningTree {
                edges: vec![],
                total: 0,
                components: 0,
            };
        }

        let mut spanning_tree = SpanningTree {
            edges: Vec::with_capacity(self.n - 1),
            total: 0,
            components: 0,
        };

        let adjacency_matrix = self.adjacency_matrix();

        let mut in_tree = vec![false; self.n];
        let mut distances = vec![u32::MAX; self.n];
        let mut parents: Vec<Option<usize>> = vec![None; self.n];

        distances[start] = 0;

        loop {
            let mut smallest: Option<usize> = None;

            for vertex in 0..self.n {
                if in_tree[vertex] || distances[vertex] == u32::MAX {
                    continue;
                }

                let is_better = match smallest {
                    None => true,
                    Some(smallest_index) => distances[vertex] < distances[smallest_index],
                };

                if is_better {
                    smallest = Some(vertex);
                }
            }

            let index = match smallest {
                Some(i) => i,
                None => break,
            };

            in_tree[index] = true;
            if let Some(p) = parents[index] {
                spanning_tree
                    .edges
                    .push(Edge::new(distances[index], p, index));
                spanning_tree.total += distances[index];
            }

            for v in 0..self.n {
                if in_tree[v] {
                    continue;
                }

                if let Some(weight) = adjacency_matrix[index][v] {
                    if weight < distances[v] {
                        distances[v] = weight;
                        parents[v] = Some(index);
                    }
                }
            }
        }

        spanning_tree
    }
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

fn name(i: usize) -> char {
    (b'A' + i as u8) as char
}

fn main() {
    todo!();
}
