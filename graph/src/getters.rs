use super::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::once;

impl Graph {
    /// Return name of the graph.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Return the number of traps (nodes without any outgoing edges that are not singletons)
    pub fn get_traps_number(&self) -> EdgeT {
        self.not_singleton_nodes_number as EdgeT - self.unique_sources.len() as EdgeT
    }

    // Return if the graph has traps or not
    pub fn has_traps(&self) -> bool {
        self.get_traps_number() > 0
    }

    /// Returns boolean representing if graph is directed.
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Returns boolean representing if graph has weights.
    pub fn has_weights(&self) -> bool {
        self.weights.is_some()
    }

    /// Returns boolean representing if graph has edge types.
    pub fn has_edge_types(&self) -> bool {
        self.edge_types.is_some()
    }

    /// Returns boolean representing if graph has self-loops.
    pub fn has_selfloops(&self) -> bool {
        self.self_loop_number > 0
    }

    /// Returns boolean representing if graph has singletons.
    pub fn has_singletons(&self) -> bool {
        self.get_singleton_nodes_number() > 0
    }

    /// Return vector of the non-unique source nodes.
    pub fn get_sources(&self) -> Vec<NodeT> {
        self.get_sources_iter().collect()
    }

    /// Return vector of the non-unique source nodes names.
    pub fn get_source_names(&self) -> Vec<String> {
        self.get_sources_iter()
            .map(|src| self.get_node_name(src).unwrap())
            .collect()
    }

    /// Return vector on the (non unique) destination nodes of the graph.
    pub fn get_destinations(&self) -> Vec<NodeT> {
        self.get_destinations_iter().collect()
    }

    /// Return vector of the non-unique destination nodes names.
    pub fn get_destination_names(&self) -> Vec<String> {
        self.get_destinations_iter()
            .map(|dst| self.get_node_name(dst).unwrap())
            .collect()
    }

    /// Return the nodes reverse mapping.
    pub fn get_nodes_reverse_mapping(&self) -> Vec<String> {
        self.nodes.reverse_map.clone()
    }

    /// Return the nodes reverse mapping.
    ///
    /// # Arguments
    ///
    /// * k: NodeT - Number of central nodes to extract.
    pub fn get_top_k_central_nodes(&self, k: NodeT) -> Vec<NodeT> {
        let mut nodes_degrees: Vec<(NodeT, NodeT)> = (0..self.get_nodes_number())
            .map(|node_id| (self.get_node_degree(node_id), node_id))
            .collect();
        nodes_degrees.sort();
        nodes_degrees.reverse();
        nodes_degrees[0..k as usize]
            .iter()
            .map(|(_, node_id)| *node_id)
            .collect()
    }

    /// Return the edge types of the edges.
    pub fn get_edge_types(&self) -> Option<Vec<EdgeTypeT>> {
        match &self.edge_types {
            Some(ets) => Some(ets.ids.clone()),
            None => None,
        }
    }

    /// Return the edge types reverse mapping.
    pub fn get_edge_types_reverse_mapping(&self) -> Option<Vec<String>> {
        match &self.edge_types {
            Some(ets) => Some(ets.vocabulary.reverse_map.clone()),
            None => None,
        }
    }

    /// Return the node types of the nodes.
    pub fn get_node_types(&self) -> Option<Vec<NodeTypeT>> {
        match &self.node_types {
            Some(nts) => Some(nts.ids.clone()),
            None => None,
        }
    }

    /// Return the weights of the nodes.
    pub fn get_weights(&self) -> Option<Vec<WeightT>> {
        self.weights.clone()
    }

    /// Return the node types reverse mapping.
    pub fn get_node_types_reverse_mapping(&self) -> Option<Vec<String>> {
        match &self.node_types {
            Some(nts) => Some(nts.vocabulary.reverse_map.clone()),
            None => None,
        }
    }

    /// Return number of the unique edges in the graph.
    pub fn get_unique_edges_number(&self) -> EdgeT {
        self.unique_edges_number
    }

    /// Return maximum encodable edge number.
    pub fn get_max_encodable_edge_number(&self) -> EdgeT {
        encode_max_edge(
            self.get_nodes_number(),
            get_node_bits(self.get_nodes_number()),
        )
    }

    /// Return the nodes mapping.
    pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.nodes.map.clone()
    }

    /// Returs option with the edge type of the given edge id.
    pub fn get_unchecked_edge_type(&self, edge_id: EdgeT) -> Option<EdgeTypeT> {
        match &self.edge_types {
            Some(ets) => Some(ets.ids[edge_id as usize]),
            None => None,
        }
    }

    /// Returs option with the node type of the given node id.
    pub fn get_unchecked_node_type(&self, node_id: NodeT) -> Option<NodeTypeT> {
        match &self.node_types {
            Some(nts) => Some(nts.ids[node_id as usize]),
            None => None,
        }
    }

    /// Returns node type of given node.
    ///
    /// # Arguments
    ///
    /// * node_id: NodeT - node whose node type is to be returned.
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The node type id of node {} is {}", 0, graph.get_node_type(0).unwrap());
    /// ```
    ///
    pub fn get_node_type(&self, node_id: NodeT) -> Result<NodeTypeT, String> {
        if let Some(nt) = &self.node_types {
            return if node_id <= nt.ids.len() as NodeT {
                Ok(nt.ids[node_id as usize])
            } else {
                Err(format!(
                    "The node_index {} is too big for the node_types vector which has len {}",
                    node_id,
                    nt.ids.len()
                ))
            };
        }
        Err(String::from(
            "Node types are not defined for current graph instance.",
        ))
    }

    /// Returns edge type of given edge.
    ///
    /// # Arguments
    ///
    /// * edge_id: EdgeT - edge whose edge type is to be returned.
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The edge type id of edge {} is {}", 0, graph.get_edge_type(0).unwrap());
    /// ```
    pub fn get_edge_type(&self, edge_id: EdgeT) -> Result<EdgeTypeT, String> {
        if let Some(et) = &self.edge_types {
            return if edge_id <= et.ids.len() as EdgeT {
                Ok(et.ids[edge_id as usize])
            } else {
                Err(format!(
                    "The edge_index {} is too big for the edge_types vector which has len {}",
                    edge_id,
                    et.ids.len()
                ))
            };
        }
        Err(String::from(
            "Edge types are not defined for current graph instance.",
        ))
    }

    /// Returs option with the node type of the given node id.
    pub fn get_node_type_string(&self, node_id: NodeT) -> Option<String> {
        match &self.node_types {
            Some(nts) => Some(
                nts.translate(self.get_unchecked_node_type(node_id).unwrap())
                    .to_owned(),
            ),
            None => None,
        }
    }

    /// Returs option with the edge type of the given edge id.
    pub fn get_edge_type_string(&self, edge_id: EdgeT) -> Option<String> {
        match &self.edge_types {
            Some(ets) => Some(
                ets.translate(self.get_unchecked_edge_type(edge_id).unwrap())
                    .to_owned(),
            ),
            None => None,
        }
    }

    /// Returs result with the node name.
    pub fn get_node_name(&self, node_id: NodeT) -> Result<String, String> {
        match node_id < self.get_nodes_number() {
            true => Ok(self.nodes.translate(node_id).to_string()),
            false => Err(format!(
                "Given node_id {} is greater than number of nodes in the graph ({}).",
                node_id,
                self.get_nodes_number()
            )),
        }
    }

    /// Returs result with the node id.
    pub fn get_node_id(&self, node_name: &str) -> Result<NodeT, String> {
        match self.nodes.get(node_name) {
            Some(node_id) => Ok(*node_id),
            None => Err(format!(
                "Given node name {} is not available in current graph.",
                node_name
            )),
        }
    }

    /// Returs node id raising a panic if used unproperly.
    pub fn get_unchecked_node_id(&self, node_name: &str) -> NodeT {
        *self.nodes.get(node_name).unwrap()
    }

    /// Returs option with the weight of the given edge id.
    pub fn get_edge_weight(&self, edge_id: EdgeT) -> Option<WeightT> {
        match &self.weights {
            Some(ws) => Some(ws[edge_id as usize]),
            None => None,
        }
    }

    /// Returns boolean representing if graph has node types.
    pub fn has_node_types(&self) -> bool {
        self.node_types.is_some()
    }

    /// Returns number of nodes in the graph.
    pub fn get_nodes_number(&self) -> NodeT {
        self.nodes.len() as NodeT
    }

    /// Returns number of edges in the graph.
    pub fn get_edges_number(&self) -> EdgeT {
        self.edges.len() as EdgeT
    }

    /// Returns number of edge types in the graph.
    pub fn get_edge_types_number(&self) -> EdgeTypeT {
        if let Some(etm) = &self.edge_types {
            etm.len() as EdgeTypeT
        } else {
            0
        }
    }

    /// Returns number of node types in the graph.
    pub fn get_node_types_number(&self) -> NodeTypeT {
        if let Some(etm) = &self.node_types {
            etm.len() as NodeTypeT
        } else {
            0
        }
    }

    /// Returns the degree of every node in the graph.
    pub fn get_node_degrees(&self) -> Vec<NodeT> {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| self.get_node_degree(node as NodeT))
            .collect::<Vec<NodeT>>()
    }

    pub fn get_not_singletons(&self) -> Vec<NodeT> {
        self.get_edges_iter()
            .flat_map(|(_, src, dst)| once(src).chain(once(dst)))
            .unique()
            .collect()
    }

    /// Return mapping from instance not trap nodes to dense nodes.
    pub fn get_dense_node_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.get_not_singletons()
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, node)| (node as NodeT, i as NodeT))
            .collect()
    }

    pub fn get_edge_type_number(&self, edge_type: EdgeTypeT) -> EdgeT {
        match &self.edge_types {
            None => 0,
            Some(ets) => ets.counts[edge_type as usize],
        }
    }

    pub fn get_node_type_number(&self, node_type: NodeTypeT) -> NodeT {
        match &self.node_types {
            None => 0 as NodeT,
            Some(nts) => nts.counts[node_type as usize],
        }
    }

    /// Return if there are multiple edges between two nodes
    pub fn is_multigraph(&self) -> bool {
        self.unique_edges_number != self.get_edges_number()
    }

    pub fn get_outbounds(&self) -> Vec<EdgeT> {
        (0..self.get_nodes_number())
            .map(|src| self.get_unchecked_edge_id_from_tuple(src as NodeT + 1, 0))
            .collect()
    }

    pub fn get_destination(&self, edge_id: EdgeT) -> NodeT {
        match &self.destinations {
            Some(destinations) => destinations[edge_id as usize],
            None => self.get_edge_from_edge_id(edge_id).1,
        }
    }

    pub fn get_destinations_range(
        &self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
    ) -> impl Iterator<Item = NodeT> + '_ {
        (min_edge_id..max_edge_id).map(move |edge_id| self.get_destination(edge_id))
    }

    pub fn get_source_destinations_range(&self, src: NodeT) -> impl Iterator<Item = NodeT> + '_ {
        self.get_unchecked_destinations_range(src)
            .map(move |edge_id| self.get_destination(edge_id))
    }

    pub fn get_unique_sources_number(&self) -> NodeT {
        self.unique_sources.len() as NodeT
    }
}