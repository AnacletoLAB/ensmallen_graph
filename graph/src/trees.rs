use super::*;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use vec_rand::xorshift::xorshift as rand_u64;

const NOT_PRESENT: u32 = u32::MAX;

/// # Implementation of algorithms relative to trees.
impl Graph {
    fn iter_edges_from_random_state(
        &self,
        random_state: u64,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        let edges_number = self.get_edges_number();
        // We execute two times the xorshift to improve the randomness of the seed.
        let updated_random_state = rand_u64(rand_u64(random_state ^ SEED_XOR as u64));
        (updated_random_state..edges_number + updated_random_state).filter_map(move |i| {
            let edge_id = i % edges_number;
            let (src, dst) = self.get_edge_from_edge_id(edge_id);
            match src == dst || !self.directed && src > dst {
                true => None,
                false => Some((edge_id, src, dst)),
            }
        })
    }

    fn iter_on_edges_with_preference<'a>(
        &'a self,
        random_state: u64,
        unwanted_edge_types: &'a Option<HashSet<EdgeTypeT>>,
        verbose: bool,
    ) -> impl Iterator<Item = (NodeT, NodeT)> + 'a {
        let pb = get_loading_bar(
            verbose,
            format!("Building random spanning tree for graph {}", self.name).as_ref(),
            self.get_edges_number() as usize,
        );
        let result: Box<dyn Iterator<Item = (NodeT, NodeT)>> =
            if let (Some(uet), _) = (unwanted_edge_types, &self.edge_types) {
                Box::new(
                    self.iter_edges_from_random_state(random_state)
                        .filter_map(move |(edge_id, src, dst)| {
                            if uet.contains(&self.get_unchecked_edge_type(edge_id).unwrap()) {
                                return None;
                            }
                            Some((src, dst))
                        })
                        .chain(self.iter_edges_from_random_state(random_state).filter_map(
                            move |(edge_id, src, dst)| {
                                if !uet.contains(&self.get_unchecked_edge_type(edge_id).unwrap()) {
                                    return None;
                                }
                                Some((src, dst))
                            },
                        )),
                )
            } else {
                Box::new(
                    self.iter_edges_from_random_state(random_state)
                        .map(|(_, src, dst)| (src, dst)),
                )
            };

        result.progress_with(pb)
    }

    /// Returns set of edges composing a spanning tree and connected components.
    ///
    /// # Arguments
    ///
    /// TODO: Updated docstrings.
    ///
    pub fn kruskal<'a>(
        &self,
        edges: impl Iterator<Item = (NodeT, NodeT)> + 'a,
    ) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
        let nodes_number = self.get_nodes_number() as usize;
        let mut tree = HashSet::with_capacity(self.get_nodes_number() as usize);
        let mut components = vec![NOT_PRESENT; nodes_number];
        let mut merged_component_number = 0;
        let mut component_sizes: Vec<usize> = Vec::new();
        let mut components_remapping: Vec<NodeT> = Vec::new();

        edges.for_each(|(src, dst)| {
            if src == dst {
                return;
            }
            let src_component = components[src as usize];
            let dst_component = components[dst as usize];
            match (src_component == NOT_PRESENT, dst_component == NOT_PRESENT) {
                // If neither nodes have a component, they must be inserted
                // both in the components vector and in the tree.
                // The edge must be added to the three.
                (true, true) => {
                    let component_number = components_remapping.len() as NodeT;
                    components[src as usize] = component_number;
                    components[dst as usize] = component_number;
                    components_remapping.push(component_number);
                    component_sizes.push(2);
                    tree.insert((src, dst));
                }
                // If both nodes have a component, the two components must be merged
                // if they are not the same one.
                // The edge must be added to the three.
                // The components mapping must be updated and afterwards the other nodes
                // must be updated accordingly to this update.
                (false, false) => {
                    if src_component == dst_component {
                        return;
                    }
                    let src_component = components_remapping[src_component as usize];
                    let dst_component = components_remapping[dst_component as usize];
                    components[src as usize] = dst_component;
                    components[dst as usize] = dst_component;
                    if src_component == dst_component {
                        return;
                    }
                    let (min_component, max_component) = match src_component < dst_component {
                        true => (src_component, dst_component),
                        false => (dst_component, src_component),
                    };
                    merged_component_number += 1;
                    component_sizes[min_component as usize] +=
                        component_sizes[max_component as usize];

                    components_remapping
                        .iter_mut()
                        .enumerate()
                        .for_each(|(comp, remapped)| {
                            if *remapped == max_component {
                                *remapped = min_component;
                                component_sizes[comp] = 0;
                            }
                        });
                    tree.insert((src, dst));
                }
                // If only one node has a component, the second model must be added.
                _ => {
                    let (component_number, not_inserted_node) = match src_component == NOT_PRESENT {
                        true => (dst_component, src),
                        false => (src_component, dst),
                    };
                    let component_number = components_remapping[component_number as usize];
                    component_sizes[component_number as usize] += 1;
                    components[not_inserted_node as usize] = component_number as NodeT;
                    tree.insert((src, dst));
                }
            };
        });

        let components_number = AtomicUsize::new(component_sizes.len());
        components.par_iter_mut().for_each(|remapped| {
            if *remapped == NOT_PRESENT {
                *remapped = (components_number.fetch_add(1, Ordering::SeqCst)
                    - merged_component_number) as NodeT;
            } else {
                *remapped = components_remapping[*remapped as usize];
            }
        });

        let (min_component_size, max_component_size) = component_sizes
            .iter()
            .cloned()
            .filter(|c| *c != 0)
            .minmax()
            .into_option()
            .unwrap();

        let total_components_number = component_sizes.len() - merged_component_number;

        (
            tree,
            components,
            total_components_number as NodeT,
            min_component_size as NodeT,
            max_component_size as NodeT,
        )
    }

    /// Returns set of edges composing a spanning tree and connected components.
    ///
    /// The spanning tree is NOT minimal.
    /// The given random_state is NOT the root of the tree.
    ///
    /// # Arguments
    ///
    /// * `random_state`:NodeT - The random_state to use for the holdout,
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `unwanted_edge_types`: &Option<HashSet<EdgeTypeT>> - Which edge types id to try to avoid.
    /// * `verbose`: bool - Wethever to show a loading bar or not.
    ///
    pub fn random_spanning_arborescence_kruskal(
        &self,
        random_state: EdgeT,
        unwanted_edge_types: &Option<HashSet<EdgeTypeT>>,
        verbose: bool,
    ) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
        self.kruskal(self.iter_on_edges_with_preference(random_state, unwanted_edge_types, verbose))
    }

    pub fn spanning_arborescence_kruskal(
        &self,
        verbose: bool,
    ) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
        let pb = get_loading_bar(
            verbose,
            &format!(
                "Computing spanning arborescence with Kruskal for graph {}",
                self.get_name()
            ),
            self.get_unique_edges_number() as usize,
        );
        self.kruskal(self.get_unique_edges_iter(self.directed).progress_with(pb))
    }

    fn scale_node_threads(&self) -> usize {
        1 + (1.0 / (1.0 + 1000000.0 / (self.get_nodes_number() as f64 * 0.8))) as usize
    }

    /// Returns set of edges composing a spanning tree.
    /// This is the implementaiton of [A Fast, Parallel Spanning Tree Algorithm for Symmetric Multiprocessors (SMPs)](https://smartech.gatech.edu/bitstream/handle/1853/14355/GT-CSE-06-01.pdf)
    /// by David A. Bader and Guojing Cong.
    pub fn spanning_arborescence(
        &self,
        verbose: bool,
    ) -> Result<(usize, impl Iterator<Item = (NodeT, NodeT)> + '_), String> {
        if self.directed {
            return Err(
                "The spanning arborescence from Bader et al. algorithm only works for undirected graphs!".to_owned(),
            );
        }
        let nodes_number = self.get_nodes_number() as usize;
        let mut parents = vec![NOT_PRESENT; nodes_number];
        let cpu_number = num_cpus::get();
        let thread_number = min!(1 + self.scale_node_threads(), cpu_number);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(thread_number)
            .build()
            .unwrap();
        let shared_stacks: Arc<Vec<Mutex<Vec<NodeT>>>> = Arc::from(
            (0..(thread_number - 1))
                .map(|_| Mutex::from(Vec::new()))
                .collect::<Vec<Mutex<Vec<NodeT>>>>(),
        );
        let active_nodes_number = AtomicUsize::new(0);
        let completed = AtomicBool::new(false);
        let total_inserted_edges = AtomicUsize::new(0);
        let thread_safe_parents = ThreadSafe {
            value: std::cell::UnsafeCell::new(&mut parents),
        };

        // since we were able to build a stub tree with cpu.len() leafs,
        // we spawn the treads and make anyone of them build the sub-trees.
        pool.scope(|s| {
            // for each leaf of the previous stub tree start a DFS keeping track
            // of which nodes we visited and updating accordingly the parents vector.
            // the nice trick here is that, since all the leafs are part of the same tree,
            // if two processes find the same node, we don't care which one of the two take
            // it so we can proceed in a lockless fashion (and maybe even without atomics
            // if we manage to remove the colors vecotr and only keep the parents one)
            s.spawn(|_| {
                let pb = get_loading_bar(
                    verbose,
                    format!("Computing spanning tree of graph {}", self.get_name()).as_ref(),
                    nodes_number,
                );
                (0..nodes_number).progress_with(pb).for_each(|src| {
                    let ptr = thread_safe_parents.value.get();
                    unsafe {
                        // If the node has already been explored we skip ahead.
                        if (*ptr)[src] != NOT_PRESENT {
                            return;
                        }
                    }
                    unsafe {
                        // find the first not explored node (this is guardanteed to be in a new component)
                        if self.has_singletons() && self.is_singleton(src as NodeT) {
                            // We set singletons as self-loops for now.
                            (*ptr)[src] = src as NodeT;
                            return;
                        }
                    }
                    loop {
                        unsafe {
                            if (*ptr)[src] != NOT_PRESENT {
                                break;
                            }
                        }
                        if active_nodes_number.load(Ordering::SeqCst) == 0 {
                            unsafe {
                                if (*ptr)[src] != NOT_PRESENT {
                                    break;
                                }
                                (*ptr)[src] = src as NodeT;
                            }
                            shared_stacks[0].lock().unwrap().push(src as NodeT);
                            active_nodes_number.fetch_add(1, Ordering::SeqCst);
                            break;
                        }
                    }
                });
                completed.store(true, Ordering::SeqCst);
            });
            (0..shared_stacks.len()).for_each(|_| {
                s.spawn(|_| 'outer: loop {
                    let thread_id = rayon::current_thread_index().unwrap();
                    let src = 'inner: loop {
                        {
                            for mut stack in (thread_id..(shared_stacks.len() + thread_id))
                                .map(|id| shared_stacks[id % shared_stacks.len()].lock().unwrap())
                            {
                                if let Some(src) = stack.pop() {
                                    break 'inner src;
                                }
                            }

                            if completed.load(Ordering::SeqCst) {
                                break 'outer;
                            }
                        }
                    };
                    self.get_source_destinations_range(src).for_each(|dst| {
                        let ptr = thread_safe_parents.value.get();
                        unsafe {
                            if (*ptr)[dst as usize] == NOT_PRESENT {
                                (*ptr)[dst as usize] = src;
                                total_inserted_edges.fetch_add(1, Ordering::SeqCst);
                                active_nodes_number.fetch_add(1, Ordering::SeqCst);
                                shared_stacks[rand_u64(dst as u64) as usize % shared_stacks.len()]
                                    .lock()
                                    .unwrap()
                                    .push(dst);
                            }
                        }
                    });
                    active_nodes_number.fetch_sub(1, Ordering::SeqCst);
                });
            });
        });

        // convert the now completed parents vector to a list of tuples representing the edges
        // of the spanning arborescense.
        Ok((
            total_inserted_edges.load(Ordering::SeqCst),
            (0..self.get_nodes_number()).filter_map(move |src| {
                let dst = parents[src as usize];
                // If the edge is NOT registered as a self-loop
                // which may happen when dealing with singletons
                // or the root nodes, we return the edge.
                if src != dst {
                    return Some((src, dst));
                }
                None
            }),
        ))
    }

    /// Returns set of roaring bitmaps representing the connected components.
    pub fn connected_components(
        &self,
        verbose: bool,
    ) -> Result<(Vec<NodeT>, NodeT, NodeT, NodeT), String> {
        if self.directed {
            return Err(
                "The connected components algorithm only works for undirected graphs!".to_owned(),
            );
        }
        let nodes_number = self.get_nodes_number() as usize;
        let mut components = vec![NOT_PRESENT; nodes_number];
        let cpu_number = num_cpus::get();
        let thread_number = min!(1 + self.scale_node_threads(), cpu_number);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(thread_number)
            .build()
            .unwrap();
        let shared_stacks: Arc<Vec<Mutex<Vec<NodeT>>>> = Arc::from(
            (0..(thread_number - 1))
                .map(|_| Mutex::from(Vec::new()))
                .collect::<Vec<Mutex<Vec<NodeT>>>>(),
        );
        let active_nodes_number = AtomicUsize::new(0);
        let current_component_nodes_number = AtomicUsize::new(0);
        let components_number = AtomicUsize::new(0);
        let max_component_nodes_number = AtomicUsize::new(1);
        let min_component_nodes_number = AtomicUsize::new(usize::MAX);
        let completed = AtomicBool::new(false);
        let thread_safe_components = ThreadSafe {
            value: std::cell::UnsafeCell::new(&mut components),
        };

        // since we were able to build a stub tree with cpu.len() leafs,
        // we spawn the treads and make anyone of them build the sub-trees.
        pool.scope(|s| {
            // for each leaf of the previous stub tree start a DFS keeping track
            // of which nodes we visited and updating accordingly the components vector.
            // the nice trick here is that, since all the leafs are part of the same tree,
            // if two processes find the same node, we don't care which one of the two take
            // it so we can proceed in a lockless fashion (and maybe even without atomics
            // if we manage to remove the colors vecotr and only keep the components one)
            s.spawn(|_| {
                let pb = get_loading_bar(
                    verbose,
                    format!(
                        "Computing connected components of graph {}",
                        self.get_name()
                    )
                    .as_ref(),
                    nodes_number,
                );
                (0..nodes_number).progress_with(pb).for_each(|src| {
                    let ptr = thread_safe_components.value.get();
                    unsafe {
                        // If the node has already been explored we skip ahead.
                        if (*ptr)[src] != NOT_PRESENT {
                            return;
                        }
                    }
                    unsafe {
                        // find the first not explored node (this is guardanteed to be in a new component)
                        if self.has_singletons() && self.is_singleton(src as NodeT) {
                            // We set singletons as self-loops for now.
                            (*ptr)[src] =
                                components_number.fetch_add(1, Ordering::SeqCst) as NodeT;
                            min_component_nodes_number.store(1, Ordering::SeqCst);
                            return;
                        }
                    }
                    loop {
                        unsafe {
                            if (*ptr)[src] != NOT_PRESENT {
                                break;
                            }
                        }
                        if active_nodes_number.load(Ordering::SeqCst) == 0 {
                            unsafe {
                                if (*ptr)[src] != NOT_PRESENT {
                                    break;
                                }
                                (*ptr)[src] =
                                    components_number.fetch_add(1, Ordering::SeqCst) as NodeT;
                            }
                            shared_stacks[0].lock().unwrap().push(src as NodeT);
                            active_nodes_number.fetch_add(1, Ordering::SeqCst);
                            let ccnn = current_component_nodes_number.swap(1, Ordering::SeqCst);
                            if ccnn != 0 {
                                if max_component_nodes_number.load(Ordering::SeqCst) < ccnn {
                                    max_component_nodes_number.store(ccnn, Ordering::SeqCst);
                                }
                                if min_component_nodes_number.load(Ordering::SeqCst) > ccnn {
                                    min_component_nodes_number.store(ccnn, Ordering::SeqCst);
                                }
                            }
                            break;
                        }
                    }
                });
                completed.store(true, Ordering::SeqCst);
            });
            (0..shared_stacks.len()).for_each(|_| {
                s.spawn(|_| 'outer: loop {
                    let thread_id = rayon::current_thread_index().unwrap();
                    let src = 'inner: loop {
                        {
                            for mut stack in (thread_id..(shared_stacks.len() + thread_id))
                                .map(|id| shared_stacks[id % shared_stacks.len()].lock().unwrap())
                            {
                                if let Some(src) = stack.pop() {
                                    break 'inner src;
                                }
                            }

                            if completed.load(Ordering::SeqCst) {
                                break 'outer;
                            }
                        }
                    };
                    self.get_source_destinations_range(src).for_each(|dst| {
                        let ptr = thread_safe_components.value.get();
                        unsafe {
                            if (*ptr)[dst as usize] == NOT_PRESENT {
                                (*ptr)[dst as usize] = (*ptr)[src as usize];
                                current_component_nodes_number.fetch_add(1, Ordering::SeqCst);
                                active_nodes_number.fetch_add(1, Ordering::SeqCst);
                                shared_stacks[rand_u64(dst as u64) as usize % shared_stacks.len()]
                                    .lock()
                                    .unwrap()
                                    .push(dst);
                            }
                        }
                    });
                    active_nodes_number.fetch_sub(1, Ordering::SeqCst);
                });
            });
        });
        let ccnn = current_component_nodes_number.load(Ordering::SeqCst);
        if max_component_nodes_number.load(Ordering::SeqCst) < ccnn {
            max_component_nodes_number.store(ccnn, Ordering::SeqCst);
        }
        if min_component_nodes_number.load(Ordering::SeqCst) > ccnn {
            min_component_nodes_number.store(ccnn, Ordering::SeqCst);
        }
        Ok((
            components,
            components_number.load(Ordering::SeqCst) as NodeT,
            min_component_nodes_number.load(Ordering::SeqCst) as NodeT,
            max_component_nodes_number.load(Ordering::SeqCst) as NodeT,
        ))
    }
}

use std::cell::UnsafeCell;

struct ThreadSafe<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for ThreadSafe<T> {}
