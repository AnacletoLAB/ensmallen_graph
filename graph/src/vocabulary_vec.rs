use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VocabularyVec<IndexT: ToFromUsize, CountT: ToFromUsize> {
    pub ids: Vec<IndexT>,
    pub vocabulary: Vocabulary<IndexT>,
    pub counts: Vec<CountT>,
}

impl<IndexT: ToFromUsize, CountT: ToFromUsize> VocabularyVec<IndexT, CountT> {
    pub fn default() -> VocabularyVec<IndexT, CountT> {
        VocabularyVec {
            ids: Vec::new(),
            vocabulary: Vocabulary::default(),
            counts: Vec::new(),
        }
    }

    pub fn from_structs(
        ids: Vec<IndexT>,
        vocabulary: Option<Vocabulary<IndexT>>,
    ) -> Option<VocabularyVec<IndexT, CountT>> {
        match vocabulary {
            Some(vocab) => {
                let mut vocabvec = VocabularyVec {
                    ids,
                    vocabulary: vocab,
                    counts: Vec::new(),
                };
                vocabvec.build_counts();
                Some(vocabvec)
            }
            None => None,
        }
    }

    pub fn build_counts(&mut self) {
        self.counts = vec![CountT::from_usize(0); self.vocabulary.len()];
        for index in self.ids.iter() {
            self.counts[IndexT::to_usize(*index)] += CountT::from_usize(1);
        }
    }

    pub fn build_reverse_mapping(&mut self) -> Result<(), String> {
        self.vocabulary.build_reverse_mapping()
    }

    /// Returns id of given value inserted.
    ///
    /// # Arguments
    ///
    /// * `value`: String - The value to be inserted.
    pub fn insert(&mut self, value: String) -> Result<IndexT, String> {
        self.vocabulary.insert(value.clone())?;
        let id = *self.get(&value).unwrap();
        self.ids.push(id);
        Ok(id)
    }

    /// Returns wethever the value is empty or not.
    pub fn is_empty(&self) -> bool {
        self.vocabulary.is_empty()
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - Id to be translated.
    pub fn translate(&self, id: IndexT) -> &str {
        self.vocabulary.translate(id)
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<&IndexT> {
        self.vocabulary.get(key)
    }

    /// Return vector of keys of the map.
    pub fn keys(&self) -> Vec<String> {
        self.vocabulary.keys()
    }

    /// Return length of the vocabulary.    
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Return boolean representing if values are numeric.
    pub fn has_numeric_ids(&self) -> bool {
        self.vocabulary.has_numeric_ids()
    }

    /// Set wether to load IDs as numeric.
    ///
    /// # Arguments
    /// * numeric_ids: bool - Wether to load the IDs as numeric
    ///
    pub fn set_numeric_ids(mut self, numeric_ids: bool) -> VocabularyVec<IndexT, CountT> {
        self.vocabulary = self.vocabulary.set_numeric_ids(numeric_ids);
        self
    }
}
