use super::types::*;
use super::*;
use rayon::prelude::*;
use std::collections::HashMap as DefaultHashMap;
use std::collections::{HashMap, HashSet};

/// # Properties and measurements of the graph
impl Graph {
    /// Returns product of degrees of given nodes.
    ///
    /// # Arguments
    ///
    /// * `one` - Integer ID of the first node.
    /// * `two` - Integer ID of the second node.
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The degrees_product between 0 and 1 is {}", graph.degrees_product(0, 1).unwrap());
    /// ```
    pub fn degrees_product(&self, one: NodeT, two: NodeT) -> Result<usize, String> {
        if one >= self.get_nodes_number() || two >= self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "One or more of the given nodes indices ({}, {}) are ",
                    "biggen than the number of nodes present in the graph ({})."
                ),
                one,
                two,
                self.get_nodes_number()
            ));
        }
        Ok(self.get_node_degree(one) as usize * self.get_node_degree(two) as usize)
    }

    /// Returns the Jaccard index for the two given nodes.
    ///
    /// # Arguments
    ///
    /// * `one` - Integer ID of the first node.
    /// * `two` - Integer ID of the second node.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The Jaccard Index between node 1 and node 2 is {}", graph.jaccard_index(1, 2).unwrap());
    /// ```
    pub fn jaccard_index(&self, one: NodeT, two: NodeT) -> Result<f64, String> {
        if one >= self.get_nodes_number() || two >= self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "One or more of the given nodes indices ({}, {}) are ",
                    "biggen than the number of nodes present in the graph ({})."
                ),
                one,
                two,
                self.get_nodes_number()
            ));
        }

        if self.is_node_trap(one) || self.is_node_trap(two) {
            return Ok(0.0f64);
        }

        let one_neighbors: HashSet<NodeT> = self.get_source_destinations_range(one).collect();
        let two_neighbors: HashSet<NodeT> = self.get_source_destinations_range(two).collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        Ok(intersections.len() as f64 / (one_neighbors.len() + two_neighbors.len()) as f64)
    }

    /// Returns the Adamic/Adar Index for the given pair of nodes.
    ///
    /// # Arguments:
    ///
    /// * `one` - Integer ID of the first node.
    /// * `two` - Integer ID of the second node.
    ///
    /// # Implementation details
    /// Since the Adamic/Adar Index is only defined for graph not containing
    /// node traps (nodes without any outbound edge) and must support all kind
    /// of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The Adamic/Adar Index between node 1 and node 2 is {}", graph.adamic_adar_index(1, 2).unwrap());
    /// ```
    pub fn adamic_adar_index(&self, one: NodeT, two: NodeT) -> Result<f64, String> {
        if one >= self.get_nodes_number() || two >= self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "One or more of the given nodes indices ({}, {}) are ",
                    "biggen than the number of nodes present in the graph ({})."
                ),
                one,
                two,
                self.get_nodes_number()
            ));
        }

        if self.is_node_trap(one) || self.is_node_trap(two) {
            return Ok(0.0f64);
        }

        let one_neighbors: HashSet<NodeT> = self.get_source_destinations_range(one).collect();
        let two_neighbors: HashSet<NodeT> = self.get_source_destinations_range(two).collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        Ok(intersections
            .par_iter()
            .filter(|node| !self.is_node_trap(**node))
            .map(|node| 1.0 / (self.get_node_degree(*node) as f64).ln())
            .sum())
    }

    /// Returns the Resource Allocation Index for the given pair of nodes.
    ///
    /// # Arguments:
    ///
    /// * `one` - Integer ID of the first node.
    /// * `two` - Integer ID of the second node.
    ///
    /// # References
    /// [T. Zhou, L. Lu, Y.-C. Zhang.
    /// Predicting missing links via local information.
    /// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
    ///
    /// # Implementation details
    /// Since the Resource Allocation Index is only defined for graph not
    /// containing node traps (nodes without any outbound edge) and
    /// must support all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The Resource Allocation Index between node 1 and node 2 is {}", graph.resource_allocation_index(1, 2).unwrap());
    /// ```
    pub fn resource_allocation_index(&self, one: NodeT, two: NodeT) -> Result<f64, String> {
        if one >= self.get_nodes_number() || two >= self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "One or more of the given nodes indices ({}, {}) are ",
                    "biggen than the number of nodes present in the graph ({})."
                ),
                one,
                two,
                self.get_nodes_number()
            ));
        }

        if self.is_node_trap(one) || self.is_node_trap(two) {
            return Ok(0.0f64);
        }

        let one_neighbors: HashSet<NodeT> = self.get_source_destinations_range(one).collect();
        let two_neighbors: HashSet<NodeT> = self.get_source_destinations_range(two).collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        Ok(intersections
            .par_iter()
            .filter(|node| !self.is_node_trap(**node))
            .map(|node| 1.0 / self.get_node_degree(*node) as f64)
            .sum())
    }

    /// Returns the traps rate of the graph.
    ///
    /// THIS IS EXPERIMENTAL AND MUST BE PROVEN!
    ///
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The Graph rate is {}", graph.traps_rate());
    /// ```
    pub fn traps_rate(&self) -> f64 {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| {
                if !self.is_node_trap(node) {
                    self.get_source_destinations_range(node)
                        .map(|dst| self.is_node_trap(dst) as usize as f64)
                        .sum::<f64>()
                        / self.get_node_degree(node) as f64
                } else {
                    1.0
                }
            })
            .sum::<f64>()
            / self.get_nodes_number() as f64
    }

    /// Returns mean node degree of the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The mean node degree of the graph is  {}", graph.degrees_mean());
    /// ```
    pub fn degrees_mean(&self) -> f64 {
        self.get_edges_number() as f64 / self.get_nodes_number() as f64
    }

    /// Returns number of undirected edges of the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of undirected edges of the graph is  {}", graph.get_undirected_edges_number());
    /// ```
    pub fn get_undirected_edges_number(&self) -> EdgeT {
        (self.get_edges_number() - self.get_self_loop_number()) / 2 + self.get_self_loop_number()
    }

    /// Returns median node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The median node degree of the graph is  {}", graph.degrees_median());
    /// ```
    pub fn degrees_median(&self) -> NodeT {
        let mut degrees = self.get_node_degrees();
        degrees.par_sort_unstable();
        degrees[(self.get_nodes_number() / 2) as usize]
    }

    /// Returns maximum node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The maximum node degree of the graph is  {}", graph.degrees_max());
    /// ```
    pub fn degrees_max(&self) -> NodeT {
        *self.get_node_degrees().iter().max().unwrap()
    }

    /// Returns minimum node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The minimum node degree of the graph is  {}", graph.degrees_min());
    /// ```
    pub fn degrees_min(&self) -> NodeT {
        *self.get_node_degrees().iter().min().unwrap()
    }

    /// Returns mode node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The mode node degree of the graph is  {}", graph.degrees_mode());
    /// ```
    pub fn degrees_mode(&self) -> NodeT {
        let mut occurrences: HashMap<NodeT, usize> = HashMap::new();

        for value in self.get_node_degrees() {
            *occurrences.entry(value).or_insert(0) += 1;
        }

        occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .unwrap()
    }

    /// Returns number of self-loops, including also those in eventual multi-edges.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of self-loops in the graph is  {}", graph.get_self_loop_number());
    /// ```
    pub fn get_self_loop_number(&self) -> EdgeT {
        self.self_loop_number
    }

    /// Returns number of unique self-loops, excluding those in eventual multi-edges.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of unique self-loops in the graph is  {}", graph.get_unique_self_loop_number());
    /// ```
    pub fn get_unique_self_loop_number(&self) -> NodeT {
        self.unique_self_loop_number
    }

    /// Returns rate of self-loops.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The rate of self-loops in the graph is  {}", graph.get_self_loop_rate());
    /// ```
    pub fn get_self_loop_rate(&self) -> f64 {
        self.get_self_loop_number() as f64 / self.get_edges_number() as f64
    }

    /// Returns number of the source nodes.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of sources of the graph (not trap nodes) is {}", graph.get_source_nodes_number());
    /// ```
    pub fn get_source_nodes_number(&self) -> NodeT {
        self.unique_sources.len() as NodeT
    }

    /// Returns number of connected components in graph.
    pub fn connected_components_number(&self, verbose: bool) -> (NodeT, NodeT) {
        let (tree, components) = self.spanning_tree(0, false, &None, verbose);
        let connected_components_number = self.get_nodes_number() - tree.len() as NodeT;
        (connected_components_number as NodeT, match components.iter().map(|c| c.len()).max() {
            Some(max_components_number) => max_components_number,
            None=> 1
        } as NodeT)
    }

    /// Returns number of singleton nodes within the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph contains {} singleton nodes", graph.get_singleton_nodes_number());
    /// ```
    pub fn get_singleton_nodes_number(&self) -> NodeT {
        self.get_nodes_number() - self.get_not_singleton_nodes_number()
    }

    /// Returns number of not singleton nodes within the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph contains {} not singleton nodes", graph.get_not_singleton_nodes_number());
    /// ```
    pub fn get_not_singleton_nodes_number(&self) -> NodeT {
        self.not_singleton_nodes_number
    }

    /// Returns density of the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph density is {}", graph.density());
    /// ```
    pub fn density(&self) -> f64 {
        let nodes_number = self.get_nodes_number();
        self.get_edges_number() as f64 / (nodes_number * (nodes_number - 1)) as f64
    }

    /// Returns report relative to the graph metrics
    ///
    /// The report includes a few useful metrics like:
    ///
    /// * degrees_median: the median degree of the nodes.
    /// * degrees_mean: the mean degree of the nodes.
    /// * degrees_mode: the mode degree of the nodes.
    /// * degrees_max: the max degree of the nodes.
    /// * degrees_min: the min degree of the nodes.
    /// * nodes_number: the number of nodes in the graph.
    /// * edges_number: the number of edges in the graph.
    /// * unique_node_types_number: the number of different node types in the graph.
    /// * unique_edge_types_number: the number of different edge types in the graph.
    /// * traps_rate: probability to end up in a trap when starting into any given node.
    /// * selfloops_rate: pecentage of edges that are selfloops.
    /// * bidirectional_rate: rate of edges that are bidirectional.
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// graph.report();
    /// ```
    pub fn report(&self) -> DefaultHashMap<&str, String> {
        let mut report: DefaultHashMap<&str, String> = DefaultHashMap::new();
        report.insert("name", self.name.clone());
        report.insert("nodes_number", self.get_nodes_number().to_string());
        report.insert("edges_number", self.get_edges_number().to_string());
        report.insert(
            "undirected_edges_number",
            self.get_undirected_edges_number().to_string(),
        );
        report.insert("density", self.density().to_string());
        report.insert("directed", self.is_directed().to_string());
        report.insert("has_weights", self.has_weights().to_string());
        report.insert("has_edge_types", self.has_edge_types().to_string());
        report.insert("has_node_types", self.has_node_types().to_string());
        report.insert("self_loops_number", self.get_self_loop_number().to_string());
        report.insert("self_loops_rate", self.get_self_loop_rate().to_string());
        report.insert("singletons", self.get_singleton_nodes_number().to_string());
        report.insert("degree_mean", self.degrees_mean().to_string());
        report.insert(
            "unique_node_types_number",
            self.get_node_types_number().to_string(),
        );
        report.insert(
            "unique_edge_types_number",
            self.get_edge_types_number().to_string(),
        );
        report
    }

    /// Return rendered textual report of the graph.
    pub fn textual_report(&self) -> String {
        let (connected_components_number, maximum_connected_component) = self.connected_components_number(true);

        format!(
            concat!(
                "The {direction} {graph_type} {name} has {nodes_number} nodes{node_types}{singletons} and {edges_number} {weighted} edges{edge_types}, of which {self_loops}. ",
                "The graph is {quantized_density} as it has a density of {density:.5} and has {components_number} connected components, where the component with most nodes has {maximum_connected_component} nodes. ",
                "The graph median node degree is {median_node_degree}, the mean node degree is {mean_node_degree:.2} and the node degree mode is {mode_node_degree}. ",
                "The top {most_common_nodes_number} most central nodes are {central_nodes}."
            ),
            direction = match self.directed {
                true=> "directed",
                false => "undirected"
            }.to_owned(),
            graph_type = match self.is_multigraph() {
                true=> "multigraph",
                false => "graph"
            }.to_owned(),
            name = self.name,
            nodes_number = self.get_nodes_number(),
            edges_number = match self.directed {
                true => self.get_edges_number(),
                false => self.get_undirected_edges_number(),
            },
            weighted = match self.has_weights(){
                true=> "weighted",
                false=> "unweighted"
            }.to_owned(),
            self_loops = match self.has_selfloops() {
                true => format!("{} are selfloops", self.get_self_loop_number()),
                false => "none are selfloops".to_owned()
            },
            node_types= match self.has_node_types() {
                true => format!(" with {} different node types", self.get_node_types_number()),
                false => "".to_owned()
            },
            singletons = match self.has_singletons() {
                true => format!(", of which {} are singletons,", self.get_singleton_nodes_number()),
                false => "".to_owned()
            },
            edge_types= match self.has_edge_types() {
                true => format!(" with {} different edge types", self.get_edge_types_number()),
                false => "".to_owned()
            },
            quantized_density = match self.density() {
                d if d < 0.0001 => "extremely sparse".to_owned(),
                d if d < 0.001 => "quite sparse".to_owned(),
                d if d < 0.01 => "sparse".to_owned(),
                d if d < 0.1 => "dense".to_owned(),
                d if d < 0.5 => "quite dense".to_owned(),
                d if (d - 1.0 as f64).abs() < f64::EPSILON => "complete".to_owned(),
                d if d < 1.0 => "extremely dense".to_owned(),
                _ => unreachable!("Unreacheable density case")
            },
            density=self.density(),
            components_number=connected_components_number,
            maximum_connected_component=maximum_connected_component,
            median_node_degree=self.degrees_median(),
            mean_node_degree=self.degrees_mean(),
            mode_node_degree=self.degrees_mode(),
            most_common_nodes_number=min!(5, self.get_nodes_number()),
            central_nodes = {
                let top_k = self.get_top_k_central_nodes(min!(5, self.get_nodes_number()));
                let central_nodes:String = top_k[0..top_k.len()-1].iter().map(|node_id| format!("{node_name} (degree {node_degree})", node_name=self.get_node_name(*node_id).unwrap(), node_degree=self.get_node_degree(*node_id))).collect::<Vec<String>>().join(", ");
                format!(
                    "{central_nodes} and {node_name} (degree {node_degree})",
                    central_nodes=central_nodes,
                    node_name=self.get_node_name(*top_k.last().unwrap()).unwrap(),
                    node_degree=self.get_node_degree(*top_k.last().unwrap())
                )
            }
        )
    }
}