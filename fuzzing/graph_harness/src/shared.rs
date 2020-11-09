use super::*;
use arbitrary_rust::Arbitrary;


#[derive(Arbitrary, Debug, PartialEq)]
pub struct FromVecHarnessParams {
    pub edges: Vec::<Result<StringQuadruple, String>>,
    pub nodes: Option::<Vec<Result<(String, Option<String>), String>>>,
    pub directed: bool,
    pub ignore_duplicated_nodes: bool,
    pub ignore_duplicated_edges: bool,
    pub numeric_edge_types_ids: bool,
    pub numeric_node_ids: bool,
    pub numeric_node_types_ids: bool
}

#[derive(Arbitrary, Debug, PartialEq)]
pub struct FromCsvHarnessParams {
    pub edge_reader: EdgeFileReaderParams,
    pub nodes_reader: Option::<NodeFileReaderParams>,
    pub directed: bool,
}

#[derive(Arbitrary, Debug, PartialEq)]
pub struct CSVFileReaderParams {
    pub verbose: Option::<bool>,
    pub separator: Option::<String>,
    pub header: Option::<bool>,
    pub rows_to_skip: Option::<usize>,
    pub ignore_duplicates: Option::<bool>,
}

#[derive(Arbitrary, Debug, PartialEq)]
pub struct NodeFileReaderParams {
    pub file: String,
    pub reader: CSVFileReaderParams,
    pub default_node_type: Option::<String>,
    pub nodes_column_number: Option::<usize>,
    pub nodes_column: Option::<String>,
    pub node_types_column_number: Option::<usize>,
    pub node_types_column: Option::<String>,
}


#[derive(Arbitrary, Debug, PartialEq)]
pub struct EdgeFileReaderParams {
    pub file: String,
    pub reader: CSVFileReaderParams,
    pub sources_column_number: Option::<usize>,
    pub sources_column: Option::<String>,
    pub destinations_column_number: Option::<usize>,
    pub destinations_column: Option::<String>,
    pub edge_types_column_number: Option::<usize>,
    pub edge_types_column: Option::<String>,
    pub weights_column_number: Option::<usize>,
    pub weights_column: Option::<String>,
    pub default_weight: Option::<WeightT>,
    pub default_edge_type: Option::<String>,
    pub skip_self_loops: Option::<bool>,
}