use super::*;

/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
pub struct EdgeFileWriter {
    pub(crate) writer: CSVFileWriter,
    pub(crate) sources_column: String,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column: String,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column: String,
    pub(crate) edge_types_column_number: usize,
    pub(crate) weights_column: String,
    pub(crate) weights_column_number: usize,
    pub(crate) numeric_node_ids: bool,
    pub(crate) directed: Option<bool>,
}

impl EdgeFileWriter {
    /// Return new EdgeFileWriter object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    ///
    pub fn new(path: String) -> EdgeFileWriter {
        EdgeFileWriter {
            writer: CSVFileWriter::new(path),
            sources_column: "subject".to_string(),
            sources_column_number: 0,
            destinations_column: "object".to_string(),
            destinations_column_number: 1,
            edge_types_column: "label".to_string(),
            edge_types_column_number: 2,
            weights_column: "weight".to_string(),
            weights_column_number: 3,
            numeric_node_ids: false,
            directed: None,
        }
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column(mut self, sources_column: Option<String>) -> EdgeFileWriter {
        if let Some(column) = sources_column {
            self.sources_column = column;
        }
        self
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column_number: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column_number(
        mut self,
        sources_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = sources_column_number {
            self.sources_column_number = column_number;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destinations_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column(
        mut self,
        destinations_column: Option<String>,
    ) -> EdgeFileWriter {
        if let Some(column) = destinations_column {
            self.destinations_column = column;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destinations_column_number: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column_number(
        mut self,
        destinations_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = destinations_column_number {
            self.destinations_column_number = column_number;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * edge_types_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column(mut self, edge_type_column: Option<String>) -> EdgeFileWriter {
        if let Some(column) = edge_type_column {
            self.edge_types_column = column;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * edge_types_column_number: Option<usize> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column_number(
        mut self,
        edge_type_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = edge_type_column_number {
            self.edge_types_column_number = column_number;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * weights_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_weights_column(mut self, weights_column: Option<String>) -> EdgeFileWriter {
        if let Some(column) = weights_column {
            self.weights_column = column;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * weights_column_number: Option<usize> - The node types column to use for the file.
    ///
    pub fn set_weights_column_number(
        mut self,
        weights_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = weights_column_number {
            self.weights_column_number = column_number;
        }
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - Wethever to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> EdgeFileWriter {
        if let Some(v) = verbose {
            self.writer.verbose = v;
        }
        self
    }

    /// Set the numeric_id.
    ///
    /// # Arguments
    ///
    /// * numeric_id: Option<bool> - Wethever to convert numeric Ids to Node Id.
    ///
    pub fn set_numeric_node_ids(mut self, numeric_node_ids: Option<bool>) -> EdgeFileWriter {
        if let Some(nni) = numeric_node_ids {
            self.numeric_node_ids = nni;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<String>) -> EdgeFileWriter {
        if let Some(v) = separator {
            self.writer.separator = v;
        }
        self
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - Wethever to write out an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> EdgeFileWriter {
        if let Some(v) = header {
            self.writer.header = v;
        }
        self
    }

    /// Set the directed.
    ///
    /// # Arguments
    ///
    /// * directed: Option<bool> - Wethever to write out the graph as directed or not.
    ///
    pub fn set_directed(mut self, directed: Option<bool>) -> EdgeFileWriter {
        self.directed = directed;
        self
    }

    /// Write edge file.
    ///  
    /// # Arguments
    ///
    /// * `graph`: &Graph - the graph to write out.
    pub fn dump(&self, graph: &Graph) -> Result<(), String> {
        let directed: bool = self.directed.unwrap_or_else(|| graph.is_directed());
        // build the header
        let mut header = vec![
            (self.sources_column.clone(), self.sources_column_number),
            (
                self.destinations_column.clone(),
                self.destinations_column_number,
            ),
        ];

        if graph.has_edge_types() {
            header.push((
                self.edge_types_column.clone(),
                self.edge_types_column_number,
            ));
        }

        if graph.has_weights() {
            header.push((self.weights_column.clone(), self.weights_column_number));
        }

        let number_of_columns = 1 + header.iter().map(|(_, i)| i).max().unwrap();

        self.writer.write_lines(
            graph.get_edges_number() as usize,
            compose_lines(number_of_columns, header),
            graph
                .get_edges_quadruples(directed)
                .map(|(_, src, dst, edge_type, weight)| {
                    let mut line = vec![
                        (
                            match self.numeric_node_ids {
                                true => src.to_string(),
                                false => graph.nodes.translate(src).to_string(),
                            },
                            self.sources_column_number,
                        ),
                        (
                            match self.numeric_node_ids {
                                true => dst.to_string(),
                                false => graph.nodes.translate(dst).to_string(),
                            },
                            self.destinations_column_number,
                        ),
                    ];

                    if let Some(ets) = &graph.edge_types {
                        line.push((
                            ets.translate(edge_type.unwrap()).to_string(),
                            self.edge_types_column_number,
                        ));
                    }

                    if let Some(w) = weight {
                        line.push((w.to_string(), self.weights_column_number));
                    }

                    compose_lines(number_of_columns, line)
                }),
        )
    }
}
