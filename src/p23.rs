use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Default, Debug)]
struct Graph {
    data: HashMap<CallSign, HashSet<CallSign>>,
}

impl Graph {
    fn get_full_subgraph(&self) -> SubGraph {
        SubGraph::new(&self, self.data.keys().copied().collect())
    }
    fn get_empty_subgraph(&self) -> SubGraph {
        SubGraph::new(&self, Default::default())
    }
    fn neighbors(&self, vertex: CallSign) -> &HashSet<CallSign> {
        self.data.get(&vertex).unwrap()
    }
}

#[derive(Debug)]
struct SubGraph<'a> {
    parent: &'a Graph,
    vertices: HashSet<CallSign>,
}

impl<'a> SubGraph<'a> {
    fn build_str(self) -> String {
        let mut vertices: Vec<_> = self.vertices.into_iter().collect();
        vertices.sort();
        let out: Vec<_> = vertices.into_iter().map(|x| format!("{:?}", x)).collect();
        out.join(",")
    }
    fn new(parent: &'a Graph, vertices: HashSet<CallSign>) -> Self {
        Self { parent, vertices }
    }
    fn len(&self) -> usize {
        self.vertices.len()
    }
    fn vertex_union(&self, vertex: CallSign) -> Self {
        let mut out = self.vertices.clone();
        out.insert(vertex);
        Self::new(self.parent, out)
    }
    fn subset_intersection(&self, other: &HashSet<CallSign>) -> Self {
        Self::new(
            self.parent,
            self.vertices.intersection(other).copied().collect(),
        )
    }
    fn union(&self, other: &SubGraph) -> Self {
        Self::new(
            self.parent,
            self.vertices.union(&other.vertices).copied().collect(),
        )
    }
    /// Returns any element from the vertices set of the subgraph
    fn get_pivot(&self) -> CallSign {
        *self.vertices.iter().next().unwrap()
    }
    fn set_minus(&self, other: &HashSet<CallSign>) -> Self {
        Self::new(
            self.parent,
            self.vertices.difference(other).copied().collect(),
        )
    }
    fn pop_vertex(&mut self, vertex: CallSign) {
        self.vertices.remove(&vertex);
    }
    fn push_vertex(&mut self, vertex: CallSign) {
        self.vertices.insert(vertex);
    }
}

// type Graph = HashMap<CallSign, HashSet<CallSign>>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct CallSign {
    data: [u8; 2],
}

impl CallSign {
    fn new(input: &[u8]) -> Self {
        let mut data = [0; 2];
        data.copy_from_slice(input);
        Self { data }
    }
    fn t_start(&self) -> bool {
        self.data[0] == b't'
    }
}

impl std::fmt::Debug for CallSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &std::str::from_utf8(&self.data).expect("invalid utf-8 sequence")
        )
    }
}

/// Bron-Kerbosch algorithm
fn maximal_clique<'a>(
    r: SubGraph<'a>,
    mut p: SubGraph<'a>,
    mut x: SubGraph<'a>,
) -> Option<SubGraph<'a>> {
    if p.len() == 0 && x.len() == 0 {
        return Some(r);
    }
    let parent_graph = r.parent;
    let u = p.union(&x).get_pivot();
    let mut out = None;
    let mut out_score = -999;
    for v in p.set_minus(parent_graph.neighbors(u)).vertices {
        let v_neighbors = parent_graph.neighbors(v);
        let best = maximal_clique(
            r.vertex_union(v),
            p.subset_intersection(&v_neighbors),
            x.subset_intersection(&v_neighbors),
        );
        if let Some(best) = best {
            if best.len() as i32 > out_score {
                out_score = best.len() as i32;
                out = Some(best);
            }
        }
        p.pop_vertex(v);
        x.push_vertex(v);
    }
    out
}

fn three_cliques(k: CallSign, graph: &Graph) -> u64 {
    let neighbors = graph.neighbors(k);
    let mut sum = 0;
    for n1 in neighbors {
        for n2 in neighbors {
            // Do not double count
            if n1 <= n2 {
                continue;
            }
            if n1.t_start() && *n1 > k {
                continue;
            }
            if n2.t_start() && *n2 > k {
                continue;
            }
            if graph.data.get(n1).unwrap().contains(n2) {
                sum += 1;
            }
        }
    }
    sum
}

pub fn solve(input: &str) -> Option<(u64, String)> {
    let mut graph: Graph = Default::default();
    for line in input.lines() {
        let (left, right) = line
            .split("-")
            .map(|x| CallSign::new(x.as_bytes()))
            .collect_tuple()?;
        graph.data.entry(left).or_default().insert(right);
        graph.data.entry(right).or_default().insert(left);
    }
    let mut part1 = 0;
    for k in graph.data.keys() {
        if k.t_start() {
            part1 += three_cliques(*k, &graph);
        }
    }
    let clique = maximal_clique(
        graph.get_empty_subgraph(),
        graph.get_full_subgraph(),
        graph.get_empty_subgraph(),
    )?;

    Some((part1, clique.build_str()))
}

#[cfg(test)]
mod tests {
    static INPUT: &'static str = include_str!("../input/p23.txt");
    #[test]
    fn day23_solve() {
        // crate::simple_bench(INPUT, super::solve);
        dbg!(super::solve(INPUT));
    }
}
