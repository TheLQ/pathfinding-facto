//! Algorithms for directed graphs.

use super::FxIndexMap;
use std::hash::Hash;

pub mod astar;
pub mod bfs;
pub mod count_paths;
pub mod cycle_detection;
pub mod dfs;
pub mod dijkstra;
pub mod edmonds_karp;
pub mod fringe;
pub mod idastar;
pub mod iddfs;
pub mod strongly_connected_components;
pub mod topological_sort;
pub mod yen;

fn reverse_path<N, V, F>(parents: &FxIndexMap<N, V>, mut parent: F, start: usize) -> Vec<N>
where
    N: Eq + Hash + Clone,
    F: FnMut(&V) -> usize,
{
    let mut i = start;
    let path = std::iter::from_fn(|| {
        parents.get_index(i).map(|(node, value)| {
            i = parent(value);
            node
        })
    })
    .collect::<Vec<&N>>();
    // Collecting the going through the vector is needed to revert the path because the
    // unfold iterator is not double-ended due to its iterative nature.
    path.into_iter().rev().cloned().collect()
}

fn reverse_path_faster<N, V, F>(parents: &FxIndexMap<N, V>, mut parent: F, start: usize) -> Vec<&N>
where
    N: Eq + Hash,
    F: FnMut(&V) -> usize,
{
    let mut i = start;
    std::iter::from_fn(|| {
        parents.get_index(i).map(|(node, value)| {
            i = parent(value);
            node
        })
    })
    .collect::<Vec<&N>>()
    // Vec::new()
}

fn reverse_path_processor<N, V, F, FX, X>(
    parents: &FxIndexMap<N, V>,
    mut parent: F,
    start: usize,
    mut processor: FX,
    xer: &mut X,
) where
    N: Eq + Hash,
    F: FnMut(&V) -> usize,
    FX: FnMut(&mut X, &N),
{
    let mut i = start;
    loop {
        let res = parents.get_index(i).map(|(node, value)| {
            i = parent(value);
            processor(xer, node)
        });
        if res.is_none() {
            break;
        }
    }
}
