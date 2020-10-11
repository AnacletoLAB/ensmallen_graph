use super::*;

impl Graph {
    /// Drop all edge types (if presents) and set all the edge to edge_type.
    /// 
    /// # Arguments
    /// - `edge_type`: String - The edge type to assing to all the edges.
    pub fn set_all_edge_types(&mut self, edge_type: String){
        let mut vocabulary = Vocabulary::new(false);
        vocabulary.insert(edge_type).unwrap();
        vocabulary.build_reverse_mapping().unwrap();
        self.edge_types = VocabularyVec::from_structs( vec![0; self.get_edges_number() as usize], Some(vocabulary));
    }
}