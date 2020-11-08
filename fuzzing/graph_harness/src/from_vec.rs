use super::*;

pub fn from_vec_harness(data: FromVecHarnessParams) -> Result<(), String> {
    let mut g = graph::Graph::from_string_unsorted(
        data.edges.iter().cloned(),
        match &data.nodes {
            Some(ns) => Some(ns.iter().cloned()),
            None => None,
        },
        data.directed,
        false,
        "Graph".to_owned(),
        data.ignore_duplicated_nodes,
        data.ignore_duplicated_edges,
        false,
        data.numeric_edge_types_ids,
        data.numeric_node_ids,
        data.numeric_node_types_ids,
    )?;
    // We ignore this error because we execute only the fuzzing to find
    // the panic situations that are NOT just errors, but unhandled errors.
    let _ = graph::test_utilities::default_test_suite(&mut g, false);

    Ok(())
}
