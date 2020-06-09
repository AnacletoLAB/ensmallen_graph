use itertools::Itertools;
use log::debug;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

type NodeT = usize;
type EdgeT = usize;
type WeightT = f64;
type ParamsT = f64;
type NodeTypeT = u16;
type EdgeTypeT = u16;

#[derive(Debug, Clone)]
pub struct Graph {
    sources: Vec<NodeT>,
    destinations: Vec<NodeT>,
    nodes_mapping: HashMap<String, NodeT>,
    reverse_nodes_mapping: Vec<String>,
    unique_edges: HashSet<(NodeT, NodeT)>,
    outbounds: Vec<EdgeT>,
    weights: Option<Vec<WeightT>>,
    node_types: Option<Vec<NodeTypeT>>,
    edge_types: Option<Vec<EdgeTypeT>>,
    edge_types_mapping: Option<HashMap<String, EdgeTypeT>>,
    node_types_mapping: Option<HashMap<String, NodeTypeT>>,
}

fn check_uniqueness(values: Vec<&[String]>) {
    let vector_len = values[0].len();

    if !values.iter().all(|x| x.len() == vector_len) {
        panic!(
            "All the vectors must have the same number of records. But got instead {:?}",
            values.iter().map(|x| x.len()).collect::<Vec<usize>>()
        );
    }

    let uniques_number = (0..vector_len)
        .map(|i| values.iter().map(|v| v[i].clone()).collect::<Vec<String>>())
        .unique()
        .count();

    if uniques_number != vector_len {
        panic!("The vectors have duplicated rows.");
    }
}

fn validate(
    nodes: &[String],
    sources_names: &[String],
    destinations_names: &[String],
    node_types: &Option<Vec<String>>,
    edge_types: &Option<Vec<String>>,
    weights: &Option<Vec<WeightT>>,
) {

    if let Some(nt) = node_types{
        debug!("Checking that nodes and node types are of the same length.");
        if nodes.len() != nt.len() {
            panic!("The number of given nodes does not match the number of node_types");
        }
    }

    debug!("Computing that edges are contained within given nodes.");
    let unique_nodes: HashSet<String> = 
        sources_names
            .iter()
            .chain(destinations_names.iter())
            .cloned()
            .collect();

    debug!("Checking if every node used by the edges exists.");
    nodes.iter().for_each(|node| {
        if !unique_nodes.contains(node) {
            panic!(
                "A node provided with the edges ('{}') does not exists within given nodes.",
                node
            );
        }
    });

    debug!("Checking that nodes must be uniques.");
    if nodes.len() != nodes.iter().unique().count() {
        panic!("The nodes must be uniques. Duplicates were found in the data.")
    }

    debug!("Checking if that the edges must be uniques.");
    if let Some(et) = edge_types {
        check_uniqueness(vec![&sources_names, &destinations_names, &et]);
    }

    debug!("Checking for non-zero weights.");
    if let Some(w) = weights {
        w.iter().for_each(|weight| {
            if *weight == 0.0 {
                panic!("One of the provided weights is either 0 or within float error to zero.");
            }
        });
    }
}

impl Graph {
    pub fn new_directed(
        nodes: Vec<String>,
        sources_names: Vec<String>,
        destinations_names: Vec<String>,
        node_types: Option<Vec<String>>,
        edge_types: Option<Vec<String>>,
        weights: Option<Vec<WeightT>>,
        validate_input_data: Option<bool>,
    ) -> Graph {
        if validate_input_data.unwrap_or_else(|| true) {
            validate(
                &nodes,
                &sources_names,
                &destinations_names,
                &node_types,
                &edge_types,
                &weights,
            );
        }

        debug!("Computing nodes to node IDs mapping.");
        let nodes_mapping: HashMap<String, NodeT> =
            HashMap::from_iter(nodes.iter().cloned().zip(0..nodes.len()));

        debug!("Computing node types to node type IDs mapping.");
        let (node_types_mapping, remapped_node_types) = if let Some(nt) = &node_types {
            let unique_node_types: Vec<String> = nt.iter().cloned().unique().collect();
            let node_types_mapping: HashMap<String, NodeTypeT> = unique_node_types
                .iter()
                .enumerate()
                .map(|(i, n)| (n.clone(), i as NodeTypeT))
                .collect();
            let remapped_node_types: Vec<NodeTypeT> = nt
                .iter()
                .cloned()
                .map(|node| *node_types_mapping.get(&node).unwrap() as NodeTypeT)
                .collect();
            (Some(node_types_mapping), Some(remapped_node_types))
        } else {
            (None, None)
        };

        debug!("Computing edge types to edge type IDs mapping.");
        let (edge_types_mapping, remapped_edge_types) = if let Some(nt) = &edge_types {
            let unique_edge_types: Vec<String> = nt.iter().cloned().unique().collect();
            let edge_types_mapping: HashMap<String, EdgeTypeT> = unique_edge_types
                .iter()
                .enumerate()
                .map(|(i, n)| (n.clone(), i as EdgeTypeT))
                .collect();
            let remapped_edge_types: Vec<EdgeTypeT> = nt
                .iter()
                .cloned()
                .map(|edge| *edge_types_mapping.get(&edge).unwrap() as EdgeTypeT)
                .collect();
            (Some(edge_types_mapping), Some(remapped_edge_types))
        } else {
            (None, None)
        };

        debug!("Computing sources node IDs.");
        let sources: Vec<NodeT> = sources_names
            .iter()
            .map(|dst| *nodes_mapping.get(dst).unwrap())
            .collect();

        debug!("Computing destinations node IDs.");
        let destinations: Vec<NodeT> = destinations_names
            .iter()
            .map(|dst| *nodes_mapping.get(dst).unwrap())
            .collect();

        debug!("Computing unique edges.");
        let unique_edges: HashSet<(NodeT, NodeT)> =
            HashSet::from_iter(sources.iter().cloned().zip(destinations.iter().cloned()));

        debug!("Computing sorting of given edges based on sources.");
        let permutation = permutation::sort(&sources[..]);
        debug!("Sorting given sources.");
        let sorted_sources = permutation.apply_slice(&sources[..]);
        debug!("Sorting given destinations.");
        let sorted_destinations = permutation.apply_slice(&destinations[..]);
        debug!("Sorting given weights.");
        let sorted_weights = weights.map(|w| permutation.apply_slice(&w[..]));
        debug!("Sorting given edge types.");
        let sorted_edge_types = remapped_edge_types.map(|et| permutation.apply_slice(&et[..]));

        Graph {
            nodes_mapping,
            node_types_mapping,
            node_types: remapped_node_types,
            unique_edges,
            sources: sorted_sources,
            destinations: sorted_destinations,
            edge_types: sorted_edge_types,
            edge_types_mapping,
            outbounds: Graph::compute_outbounds(nodes.len(), sources),
            reverse_nodes_mapping: nodes,
            weights: sorted_weights,
        }
    }

    pub fn new_undirected(
        nodes: Vec<String>,
        sources_names: Vec<String>,
        destinations_names: Vec<String>,
        node_types: Option<Vec<String>>,
        edge_types: Option<Vec<String>>,
        weights: Option<Vec<WeightT>>,
        validate_input_data: Option<bool>,
    ) -> Graph {
        if validate_input_data.unwrap_or_else(|| true) {
            validate(
                &nodes,
                &sources_names,
                &destinations_names,
                &node_types,
                &edge_types,
                &weights,
            );
        };

        debug!("Identifying self-loops present in given graph.");
        let loops_mask: Vec<bool> = sources_names
            .iter()
            .zip(destinations_names.iter())
            .map(|(a, b)| a == b)
            .collect();

        debug!("Building undirected graph sources.");
        let mut full_sources: Vec<String> = sources_names.clone();
        full_sources.extend(
            sources_names
                .iter()
                .zip(loops_mask.iter())
                .filter(|&(_, &mask)| mask)
                .map(|(value, _)| value.clone())
                .collect::<Vec<String>>(),
        );

        debug!("Building undirected graph destinations.");
        let mut full_destinations: Vec<String> = destinations_names.clone();
        full_destinations.extend(
            destinations_names
                .iter()
                .zip(loops_mask.iter())
                .filter(|&(_, &mask)| mask)
                .map(|(value, _)| value.clone())
                .collect::<Vec<String>>(),
        );

        let mut full_edge_types = edge_types;
        if let Some(e) = &mut full_edge_types {
            debug!("Building undirected graph edge types.");
            e.extend(
                e.iter()
                    .zip(loops_mask.iter())
                    .filter(|&(_, &mask)| mask)
                    .map(|(value, _)| value.clone())
                    .collect::<Vec<String>>(),
            );
        };

        let mut full_weights = weights;
        if let Some(w) = &mut full_weights {
            debug!("Building undirected graph weights.");
            w.extend(
                w.iter()
                    .zip(loops_mask.iter())
                    .filter(|&(_, &mask)| mask)
                    .map(|(value, _)| *value)
                    .collect::<Vec<WeightT>>(),
            );
        };

        Graph::new_directed(
            nodes,
            full_sources,
            full_destinations,
            node_types,
            full_edge_types,
            full_weights,
            Some(false)
        )
    }

    fn compute_outbounds(nodes_number: NodeT, sources: Vec<NodeT>) -> Vec<EdgeT> {
        debug!("Computing outbound edges ranges from each node.");
        let mut last_src: NodeT = 0;
        // Instead of fixing the last values after the loop, we set directly
        // all values to the length of the sources, which is the sum of all
        // possible neighbors.
        let mut outbounds: Vec<EdgeT> = vec![sources.len(); nodes_number];

        for (i, src) in sources.iter().enumerate() {
            if last_src != *src {
                // Assigning to range instead of single value, so that traps
                // have as delta between previous and next node zero.
                for o in &mut outbounds[last_src..*src] {
                    *o = i;
                }
                last_src = *src;
            }
        }

        outbounds
    }

    fn get_min_max_edge(&self, node: NodeT) -> (EdgeT, EdgeT) {
        let min_edge: EdgeT = if node == 0 {
            0
        } else {
            self.outbounds[node - 1]
        };
        let max_edge: EdgeT = self.outbounds[node];
        (min_edge, max_edge)
    }

    fn is_node_trap(&self, node: NodeT) -> bool {
        let (_min, _max) = self.get_min_max_edge(node);
        _min == _max
    }

    fn is_edge_trap(&self, edge: EdgeT) -> bool {
        self.is_node_trap(self.destinations[edge])
    }

    fn get_node_transition(
        &self,
        node: NodeT,
        change_node_type_weight: ParamsT,
    ) -> (Vec<WeightT>, Vec<NodeT>, EdgeT, EdgeT) {
        // Retrieve edge boundaries.
        let (min_edge, max_edge) = self.get_min_max_edge(node);
        // If weights are given
        let mut transition: Vec<WeightT> = if let Some(w) = &self.weights {
            w[min_edge..max_edge].to_vec()
        } else {
            vec![1.0; max_edge - min_edge]
        };

        let destinations: Vec<NodeT> = self.destinations[min_edge..max_edge].to_vec();

        //############################################################
        //# Handling of the change node type parameter               #
        //############################################################

        // If the node types were given:
        if let Some(nt) = &self.node_types {
            // if the destination node type matches the neighbour
            // destination node type (we are not changing the node type)
            // we weigth using the provided change_node_type_weight weight.
            let this_type: NodeTypeT = nt[node];

            transition
                .iter_mut()
                .zip(destinations.iter().map(|dst| nt[*dst]))
                .filter(|(_, neigh_type)| this_type == *neigh_type)
                .for_each(|(transition_value, _)| *transition_value /= change_node_type_weight);
            // credo non serva collect perche' modifichiamo i valori direttamente
        }

        (transition, destinations, min_edge, max_edge)
    }

    fn get_edge_transition(
        &self,
        edge: EdgeT,
        change_edge_type_weight: ParamsT,
        change_node_type_weight: ParamsT,
        return_weight: ParamsT,
        explore_weight: ParamsT,
    ) -> (Vec<WeightT>, Vec<NodeT>, EdgeT, EdgeT) {
        // Get the source and destination for current edge.
        let (src, dst) = (&self.sources[edge], self.destinations[edge]);

        // Compute the transition weights relative to the node weights.
        let (mut transition, destinations, min_edge, max_edge) =
            self.get_node_transition(dst, change_node_type_weight);

        //############################################################
        //# Handling of the change edge type parameter               #
        //############################################################

        // If the edge types were given:
        if let Some(et) = &self.edge_types {
            //# If the neighbour edge type matches the previous
            //# edge type (we are not changing the edge type)
            //# we weigth using the provided change_edge_type_weight weight.
            let this_type: EdgeTypeT = et[edge];
            transition
                .iter_mut()
                .zip(et[min_edge..max_edge].iter())
                .filter(|(_, &neigh_type)| this_type == neigh_type)
                .for_each(|(transition_value, _)| *transition_value /= change_edge_type_weight);
        }

        //############################################################
        //# Handling of the Q parameter: the return coefficient      #
        //############################################################

        //# If the neigbour matches with the source, hence this is
        //# a backward loop like the following:
        //# SRC -> DST
        //#  ▲     /
        //#   \___/
        //#
        //# We weight the edge weight with the given return weight.

        transition
            .iter_mut()
            .zip(destinations.iter())
            .filter(|&(_, ndst)| src == ndst)
            .for_each(|(transition_value, _)| *transition_value *= return_weight);

        //############################################################
        //# Handling of the P parameter: the exploration coefficient #
        //############################################################

        transition
            .iter_mut()
            .zip(destinations.iter())
            .filter(|&(_, ndst)| !self.unique_edges.contains(&(*ndst, *src)))
            .for_each(|(transition_value, _)| *transition_value *= explore_weight);

        (transition, destinations, min_edge, max_edge)
    }

    fn extract(&self, weights: Vec<WeightT>) -> NodeT {
        WeightedIndex::new(&weights)
            .unwrap()
            .sample(&mut thread_rng())
    }

    fn extract_node(&self, node: NodeT, change_node_type_weight: ParamsT) -> (NodeT, EdgeT) {
        let (weights, dsts, min_edge, _) = self.get_node_transition(node, change_node_type_weight);
        let index = self.extract(weights);
        (dsts[index], min_edge + index)
    }

    fn extract_edge(
        &self,
        edge: EdgeT,
        change_edge_type_weight: ParamsT,
        change_node_type_weight: ParamsT,
        return_weight: ParamsT,
        explore_weight: ParamsT,
    ) -> (NodeT, EdgeT) {
        let (weights, dsts, min_edge, _) = self.get_edge_transition(
            edge,
            change_edge_type_weight,
            change_node_type_weight,
            return_weight,
            explore_weight,
        );
        let index = self.extract(weights);
        (dsts[index], min_edge + index)
    }

    pub fn walk(
        &self,
        number: usize,
        length: usize,
        change_edge_type_weight: ParamsT,
        change_node_type_weight: ParamsT,
        return_weight: ParamsT,
        explore_weight: ParamsT,
    ) -> Vec<Vec<Vec<NodeT>>> {
        (0..number)
            .into_par_iter()
            .map(|_| {
                (0..self.outbounds.len())
                    .into_par_iter()
                    .map(|node| {
                        self.single_walk(
                            length,
                            node,
                            change_edge_type_weight,
                            change_node_type_weight,
                            return_weight,
                            explore_weight,
                        )
                    })
                    .collect::<Vec<Vec<NodeT>>>()
            })
            .collect::<Vec<Vec<Vec<NodeT>>>>()
    }

    fn single_walk(
        &self,
        length: usize,
        node: NodeT,
        change_edge_type_weight: ParamsT,
        change_node_type_weight: ParamsT,
        return_weight: ParamsT,
        explore_weight: ParamsT,
    ) -> Vec<NodeT> {
        let mut walk: Vec<NodeT> = Vec::with_capacity(length);
        walk.push(node);

        if self.is_node_trap(node) {
            return walk;
        }

        let (dst, mut edge) = self.extract_node(node, change_node_type_weight);
        walk.push(dst);

        for _ in 2..length {
            if self.is_edge_trap(edge) {
                break;
            }
            let (dst, inner_edge) = self.extract_edge(
                edge,
                change_edge_type_weight,
                change_node_type_weight,
                return_weight,
                explore_weight,
            );
            edge = inner_edge;
            walk.push(dst);
        }
        walk
    }
}
