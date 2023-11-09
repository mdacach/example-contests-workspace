use std::collections::VecDeque;

use crate::misc::recursive_function::{Callable2, RecursiveFunction2, RecursiveFunction3};

#[derive(Debug)]
pub struct Graph {
    pub adjacency_list: Vec<Vec<usize>>,
    pub num_edges: usize,
    pub num_vertices: usize,
}

#[derive(Debug)]
pub struct LCA {
    parent: Vec<usize>,
    time_in: Vec<usize>,
    time_out: Vec<usize>,
    up: Vec<Vec<usize>>,
}

impl LCA {
    /// Returns `true` if `b` is an ancestor of `a`.
    pub fn is_ancestor(&self, a: usize, b: usize) -> bool {
        self.time_in[a] <= self.time_in[b] && self.time_out[a] >= self.time_out[b]
    }

    pub fn of(&self, mut a: usize, mut b: usize) -> usize {
        if self.is_ancestor(a, b) {
            return a;
        }
        if self.is_ancestor(b, a) {
            return b;
        }

        for e in (0..32).rev() {
            let candidate = self.up[a][e];
            if !self.is_ancestor(candidate, b) {
                a = candidate;
            }
        }

        self.parent[a]
    }

    pub fn up(&self, vertex: usize, levels: usize) -> usize {
        let mut current = vertex;
        for e in (0..32).rev() {
            if (levels >> e) & 1 == 1 {
                current = self.up[current][e];
            }
        }
        current
    }
}

impl Graph {
    pub fn new(num_vertices: usize) -> Self {
        Self {
            adjacency_list: vec![vec![]; num_vertices],
            num_edges: 0,
            num_vertices,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        debug_assert!(from < self.num_vertices);
        self.adjacency_list[from].push(to);
    }

    pub fn breadth_first_search(&self, start: usize) -> Vec<Option<usize>> {
        let mut distance = vec![None; self.num_vertices];
        distance[start] = Some(0);
        let mut to_be_processed = VecDeque::new();
        to_be_processed.push_back(start);
        while let Some(current) = to_be_processed.pop_front() {
            debug_assert!(&self.adjacency_list.get(current).is_some());
            for &adjacent in &self.adjacency_list[current] {
                if distance[adjacent].is_none() {
                    distance[adjacent] = Some(distance[current].unwrap() + 1);
                    to_be_processed.push_back(adjacent);
                }
            }
        }
        distance
    }

    pub fn lowest_common_ancestor_table(&self, root: usize) -> LCA {
        let mut parent = vec![0; self.num_vertices];
        let mut time_in = vec![0; self.num_vertices];
        let mut time_out = vec![0; self.num_vertices];
        let mut current_time = 0;
        let mut dfs = RecursiveFunction2::new(|f, current_node: usize, parent_node| {
            time_in[current_node] = current_time;
            current_time += 1;
            parent[current_node] = parent_node;

            for &child in &self.adjacency_list[current_node] {
                if child == parent_node {
                    continue;
                }
                f.call(child, current_node);
            }

            time_out[current_node] = current_time;
            current_time += 1;
        });
        dfs.call(root, root);

        let mut up = vec![vec![0; 32]; self.num_vertices];
        // let mut original_parent_sparse_table = Array2d::new(n, 32, 0);
        for node in 0..self.num_vertices {
            up[node][0] = parent[node];
        }

        for exponent in 1..32 {
            for node in 0..self.num_vertices {
                let mid = up[node][exponent - 1];
                up[node][exponent] = up[mid][exponent - 1];
            }
        }

        LCA {
            parent,
            time_in,
            time_out,
            up,
        }
    }
}
