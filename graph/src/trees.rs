use super::*;
use indicatif::ProgressIterator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use roaring::{RoaringBitmap, RoaringTreemap};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use vec_rand::xorshift::xorshift as rand_u64;

const NOT_PRESENT: u32 = u32::MAX;

// Return component of given node, including eventual remapping.
fn get_node_component(component: usize, components_remapping: &HashMap<usize, usize>) -> usize {
    match components_remapping.get(&component) {
        Some(c) => *c,
        None => component,
    }
}

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
        verbose: bool,
        unwanted_edge_types: &'a Option<HashSet<EdgeTypeT>>,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + 'a {
        // TODO! FIX THIS CRASH if called with unwanted_edge_types and the graph does not have edge types.
        let result: Box<dyn Iterator<Item = (EdgeT, NodeT, NodeT)>> =
            if let Some(uet) = unwanted_edge_types {
                Box::new(
                    self.iter_edges_from_random_state(random_state)
                        .filter(move |(edge_id, _, _)| {
                            !uet.contains(&self.get_unchecked_edge_type(*edge_id).unwrap())
                        })
                        .chain(self.iter_edges_from_random_state(random_state).filter(
                            move |(edge_id, _, _)| {
                                uet.contains(&self.get_unchecked_edge_type(*edge_id).unwrap())
                            },
                        )),
                )
            } else {
                Box::new(self.iter_edges_from_random_state(random_state))
            };

        let pb = get_loading_bar(
            verbose,
            format!("Building spanning tree for {}", self.name).as_ref(),
            self.get_edges_number() as usize,
        );
        result.progress_with(pb)
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
    pub fn random_spanning_tree(
        &self,
        random_state: EdgeT,
        include_all_edge_types: bool,
        unwanted_edge_types: &Option<HashSet<EdgeTypeT>>,
        verbose: bool,
    ) -> (RoaringTreemap, Vec<RoaringBitmap>) {
        // Create vector of sets of the single nodes.
        let mut components: Vec<Option<RoaringBitmap>> = Vec::new();
        // Create vector of nodes component numbers.
        let mut nodes_components: Vec<Option<usize>> = vec![None; self.get_nodes_number() as usize];
        // Create the empty tree (this will be sparse on most graphs so roaring can save memory).
        let mut tree = RoaringTreemap::new();
        // Components remapping
        let mut components_remapping: HashMap<usize, usize> = HashMap::new();

        // Iterate over all the edges and add and edge to the mst
        // iff the edge create, expand or merge components.
        for (edge_id, src, dst) in
            self.iter_on_edges_with_preference(random_state, verbose, unwanted_edge_types)
        {
            let mut update_tree = false;
            let src_component = nodes_components[src as usize];
            let dst_component = nodes_components[dst as usize];
            // if both nodes are not covered then the edge is isolated
            // and must start its own component
            match (src_component, dst_component) {
                (None, None) => {
                    update_tree = true;
                    nodes_components[src as usize] = Some(components.len());
                    nodes_components[dst as usize] = Some(components.len());
                    components.push(Some(RoaringBitmap::from_iter(vec![src, dst])));
                }
                (Some(src_component), Some(dst_component)) => {
                    // if the components are different then we add it because it will merge them
                    if src_component == dst_component {
                        continue;
                    }
                    let src_component = get_node_component(src_component, &components_remapping);
                    let dst_component = get_node_component(dst_component, &components_remapping);
                    if src_component != dst_component {
                        let removed_component = components[src_component].take().unwrap();
                        if let Some(component) = &mut components[dst_component] {
                            component.union_with(&removed_component);
                        }
                        components_remapping.par_iter_mut().for_each(
                            |(component, remapped_component)| {
                                if *component == src_component
                                    || *remapped_component == src_component
                                {
                                    *remapped_component = dst_component;
                                }
                            },
                        );
                        components_remapping.insert(src_component, dst_component);
                        update_tree = true;
                    }
                }
                _ => {
                    let (inserted_component, not_inserted, not_inserted_component) =
                        if src_component.is_some() {
                            (src_component, dst, &mut nodes_components[dst as usize])
                        } else {
                            (dst_component, src, &mut nodes_components[src as usize])
                        };
                    let inserted_component =
                        get_node_component(inserted_component.unwrap(), &components_remapping);
                    if let Some(component) = &mut components[inserted_component] {
                        component.insert(not_inserted);
                    }
                    *not_inserted_component = Some(inserted_component);
                    update_tree = true;
                }
            };

            if update_tree {
                tree.extend(self.compute_edge_ids_vector(edge_id, src, dst, include_all_edge_types))
            }
        }

        let components = components.iter().filter_map(|c| c.clone()).collect();

        (tree, components)
    }

    pub fn connected_components_from_spanning_arborescence(
        &self,
        spanning_arborescence: &[(NodeT, NodeT)],
    ) -> Vec<RoaringBitmap> {
        // Create vector of sets of the single nodes.
        let mut components: Vec<Option<RoaringBitmap>> = Vec::new();
        // Create vector of nodes component numbers.
        let mut nodes_components: Vec<Option<usize>> = vec![None; self.get_nodes_number() as usize];
        // Components remapping
        let mut components_remapping: HashMap<usize, usize> = HashMap::new();
        // Compute the connected components.
        spanning_arborescence.iter().for_each(|(src, dst)| {
            let src_component = nodes_components[*src as usize];
            let dst_component = nodes_components[*dst as usize];
            // if both nodes are not covered then the edge is isolated
            // and must start its own component
            match (src_component, dst_component) {
                (None, None) => {
                    let new_component_number = components.len();
                    let component = [*src, *dst]
                        .iter()
                        .flat_map(|node| {
                            self.get_source_destinations_range(*node)
                                .collect::<Vec<NodeT>>()
                        })
                        .collect::<RoaringBitmap>();
                    component.iter().for_each(|node| {
                        nodes_components[node as usize] = Some(new_component_number);
                    });
                    components.push(Some(component));
                }
                (Some(src_component), Some(dst_component)) => {
                    // if the components are different then we add it because it will merge them
                    if src_component == dst_component {
                        return;
                    }
                    let src_component = get_node_component(src_component, &components_remapping);
                    let dst_component = get_node_component(dst_component, &components_remapping);
                    if src_component != dst_component {
                        let removed_component = components[src_component].take().unwrap();
                        if let Some(component) = &mut components[dst_component] {
                            component.union_with(&removed_component);
                        }
                        components_remapping.iter_mut().for_each(
                            |(component, remapped_component)| {
                                if *component == src_component
                                    || *remapped_component == src_component
                                {
                                    *remapped_component = dst_component;
                                }
                            },
                        );
                        components_remapping.insert(src_component, dst_component);
                    }
                }
                _ => {
                    let (inserted_component, not_inserted) = if src_component.is_some() {
                        (src_component, *dst)
                    } else {
                        (dst_component, *src)
                    };
                    let inserted_component =
                        get_node_component(inserted_component.unwrap(), &components_remapping);
                    let destinations = self
                        .get_source_destinations_range(not_inserted)
                        .collect::<Vec<NodeT>>();

                    destinations.iter().for_each(|node| {
                        nodes_components[*node as usize] = Some(inserted_component);
                    });
                    nodes_components[not_inserted as usize] = Some(inserted_component);
                    if let Some(component) = &mut components[inserted_component] {
                        component.insert(not_inserted);
                        component.extend(destinations);
                    }
                }
            };
        });
        components.iter().cloned().filter_map(|c| c).collect()
    }

    /// Returns set of edges composing a spanning tree.
    /// This is the implementaiton of [A Fast, Parallel Spanning Tree Algorithm for Symmetric Multiprocessors (SMPs)](https://smartech.gatech.edu/bitstream/handle/1853/14355/GT-CSE-06-01.pdf)
    /// by David A. Bader and Guojing Cong.
    pub fn spanning_arborescence(&self) -> Vec<(NodeT, NodeT)> {
        let nodes_number = self.get_nodes_number() as usize;
        let mut parents = vec![NOT_PRESENT; nodes_number];
        let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
        let shared_stack: Arc<Mutex<Vec<NodeT>>> = Arc::from(Mutex::from(Vec::new()));
        let number_of_working_threads = AtomicUsize::new(0);
        let thread_safe_parents = ThreadSafe {
            value: std::cell::UnsafeCell::new(&mut parents),
        };
        (0..nodes_number).for_each(|src| {
            let ptr = thread_safe_parents.value.get();
            unsafe {
                // If the node has already been explored we skip ahead.
                if (*ptr)[src] != NOT_PRESENT {
                    return;
                }
                (*ptr)[src] = src as NodeT;
            }
            unsafe {
                // find the first not explored node (this is guardanteed to be in a new component)
                if self.has_singletons() && self.is_singleton(src as NodeT) {
                    // We set singletons as self-loops for now.
                    (*ptr)[src] = src as NodeT;
                    return;
                }
            }
            shared_stack.lock().unwrap().push(src as NodeT);
            // since we were able to build a stub tree with cpu.len() leafs,
            // we spawn the treads and make anyone of them build the sub-trees.
            pool.scope(|s| {
                // for each leaf of the previous stub tree start a DFS keeping track
                // of which nodes we visited and updating accordingly the parents vector.
                // the nice trick here is that, since all the leafs are part of the same tree,
                // if two processes find the same node, we don't care which one of the two take
                // it so we can proceed in a lockless fashion (and maybe even without atomics
                // if we manage to remove the colors vecotr and only keep the parents one)

                (0..pool.current_num_threads()).for_each(|_| {
                    s.spawn(|_| 'outer: loop {
                        let src = loop {
                            {
                                let mut stack = shared_stack.lock().unwrap();
                                if let Some(src) = stack.pop() {
                                    number_of_working_threads.fetch_add(1, Ordering::SeqCst);
                                    break src;
                                }
                                if number_of_working_threads.load(Ordering::Relaxed) == 0 {
                                    break 'outer;
                                }
                            }
                        };
                        self.get_source_destinations_range(src).for_each(|dst| {
                            let ptr = thread_safe_parents.value.get();
                            unsafe {
                                if (*ptr)[dst as usize] == NOT_PRESENT {
                                    (*ptr)[dst as usize] = src;
                                    shared_stack.lock().unwrap().push(dst);
                                }
                            }
                        });
                        number_of_working_threads.fetch_sub(1, Ordering::SeqCst);
                    });
                });
            });
        });

        // convert the now completed parents vector to a list of tuples representing the edges
        // of the spanning arborescense.
        parents
            .par_iter()
            .enumerate()
            .filter_map(|(dst, src)| {
                // If the edge is NOT registered as a self-loop
                // which may happen when dealing with singletons
                // or the root nodes, we return the edge.
                if *src != dst as NodeT {
                    return Some((*src, dst as NodeT));
                }
                None
            })
            .collect::<Vec<(NodeT, NodeT)>>()
    }
}

use std::cell::UnsafeCell;

struct ThreadSafe<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for ThreadSafe<T> {}
