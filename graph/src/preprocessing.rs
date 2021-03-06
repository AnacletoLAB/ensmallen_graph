use super::*;
use indicatif::ProgressIterator;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::prelude::*;
use std::collections::HashMap;
use vec_rand::gen_random_vec;
use vec_rand::xorshift::xorshift;

enum EdgeEmbeddingMethods {
    Hadamard,
    Average,
    Sum,
    L1,
    AbsoluteL1,
    L2,
    Concatenate,
}

impl EdgeEmbeddingMethods {
    fn new(method: &str) -> Result<EdgeEmbeddingMethods, String> {
        match method {
            "Hadamard" => Ok(EdgeEmbeddingMethods::Hadamard),
            "Average" => Ok(EdgeEmbeddingMethods::Average),
            "Sum" => Ok(EdgeEmbeddingMethods::Sum),
            "L1" => Ok(EdgeEmbeddingMethods::L1),
            "AbsoluteL1" => Ok(EdgeEmbeddingMethods::AbsoluteL1),
            "L2" => Ok(EdgeEmbeddingMethods::L2),
            "Concatenate" => Ok(EdgeEmbeddingMethods::Concatenate),
            _ => Err(format!(
                concat!(
                    "Given embedding method '{}' is not supported.",
                    "The supported methods are 'Hadamard', 'Average', 'Sum', 'AbsoluteL1', 'L1', 'Concatenate' and 'L2'."
                ),
                method
            )),
        }
    }
}

/// Return training batches for Word2Vec models.
///
/// The batch is composed of a tuple as the following:
///
/// - (Contexts indices, central nodes indices): the tuple of nodes
///
/// This does not provide any output value as the model uses NCE loss
/// and basically the central nodes that are fed as inputs work as the
/// outputs value.
///
/// # Arguments
///
/// * sequences: Vec<Vec<usize>> - the sequence of sequences of integers to preprocess.
/// * window_size: usize - Window size to consider for the sequences.
///
pub fn word2vec<'a>(
    sequences: impl ParallelIterator<Item = Vec<NodeT>> + 'a,
    window_size: usize,
) -> Result<impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a, String> {
    Ok(sequences.flat_map_iter(move |sequence| {
        let sequence_length = sequence.len();
        if sequence_length < window_size * 2 + 1 {
            panic!("You are providing sequences that are smaller than the the minimum amount.");
        }
        (window_size..(sequence_length - window_size - 1)).map(move |i| {
            (
                (i - window_size..i)
                    .chain(i + 1..window_size + i + 1)
                    .map(|j| sequence[j])
                    .collect(),
                sequence[i],
            )
        })
    }))
}

/// Return triple with CSR representation of cooccurrence matrix.
///
/// The first vector has the sources, the second vector the destinations
/// and the third one contains the min-max normalized frequencies.
///
/// # Arguments
///
/// * sequences:Vec<Vec<usize>> - the sequence of sequences of integers to preprocess.
/// * window_size: Option<usize> - Window size to consider for the sequences.
/// * verbose: Option<bool>,
///     Wethever to show the progress bars.
///     The default behaviour is false.
///     
pub fn cooccurence_matrix(
    sequences: impl ParallelIterator<Item = Vec<NodeT>>,
    window_size: usize,
    number_of_sequences: usize,
    verbose: bool,
) -> Result<(Words, Words, Frequencies), String> {
    let mut cooccurence_matrix: HashMap<(NodeT, NodeT), f64> = HashMap::new();
    let pb1 = get_loading_bar(verbose, "Computing frequencies", number_of_sequences);
    // TODO!: Avoid this collect and create the cooccurrence matrix in a parallel way.
    // Tommy is currently trying to develop a version of the hashmap that is able to handle this.
    let vec = sequences.collect::<Vec<Vec<NodeT>>>();
    vec.iter().progress_with(pb1).for_each(|sequence| {
        let walk_length = sequence.len();
        for (central_index, &central_word_id) in sequence.iter().enumerate() {
            for distance in 1..1 + window_size {
                if central_index + distance >= walk_length {
                    break;
                }
                let context_id = sequence[central_index + distance];
                if central_word_id < context_id {
                    *cooccurence_matrix
                        .entry((central_word_id as NodeT, context_id as NodeT))
                        .or_insert(0.0) += 1.0 / distance as f64;
                } else {
                    *cooccurence_matrix
                        .entry((context_id as NodeT, central_word_id as NodeT))
                        .or_insert(0.0) += 1.0 / distance as f64;
                }
            }
        }
    });

    let elements = cooccurence_matrix.len() * 2;
    let mut max_frequency = 0.0;
    let mut words: Vec<NodeT> = vec![0; elements];
    let mut contexts: Vec<NodeT> = vec![0; elements];
    let mut frequencies: Vec<f64> = vec![0.0; elements];
    let pb2 = get_loading_bar(
        verbose,
        "Converting mapping into CSR matrix",
        cooccurence_matrix.len(),
    );

    cooccurence_matrix
        .iter()
        .progress_with(pb2)
        .enumerate()
        .for_each(|(i, ((word, context), frequency))| {
            let (k, j) = (i * 2, i * 2 + 1);
            if *frequency > max_frequency {
                max_frequency = *frequency;
            }
            words[k] = *word;
            words[j] = words[k];
            contexts[k] = *context;
            contexts[j] = contexts[k];
            frequencies[k] = *frequency;
            frequencies[j] = frequencies[k];
        });

    frequencies
        .par_iter_mut()
        .for_each(|frequency| *frequency /= max_frequency);

    Ok((words, contexts, frequencies))
}

/// # Preprocessing for ML algorithms on graph.
impl Graph {
    /// Return training batches for Node2Vec models.
    ///
    /// The batch is composed of a tuple as the following:
    ///
    /// - (Contexts indices, central nodes indices): the tuple of nodes
    ///
    /// This does not provide any output value as the model uses NCE loss
    /// and basically the central nodes that are fed as inputs work as the
    /// outputs value.
    ///
    /// # Arguments
    ///
    /// * walk_parameters: &WalksParameters - the weighted walks parameters.
    /// * quantity: usize - Number of nodes to consider.
    /// * window_size: usize - Window size to consider for the sequences.
    ///
    pub fn node2vec<'a>(
        &'a self,
        walk_parameters: &'a WalksParameters,
        quantity: NodeT,
        window_size: usize,
    ) -> Result<impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a, String> {
        // do the walks and check the result
        word2vec(
            self.random_walks_iter(quantity, walk_parameters)?,
            window_size,
        )
    }

    /// Return triple with CSR representation of cooccurrence matrix.
    ///
    /// The first vector has the sources, the second vector the destinations
    /// and the third one contains the min-max normalized frequencies.
    ///
    /// # Arguments
    ///
    /// * parameters: &WalksParameters - the walks parameters.
    /// * window_size: Option<usize> - Window size to consider for the sequences.
    /// * verbose: Option<bool>,
    ///     Wethever to show the progress bars.
    ///     The default behaviour is false.
    ///     
    pub fn cooccurence_matrix(
        &self,
        walks_parameters: &WalksParameters,
        window_size: usize,
        verbose: bool,
    ) -> Result<(Words, Words, Frequencies), String> {
        let walks = self.complete_walks_iter(walks_parameters)?;
        cooccurence_matrix(
            walks,
            window_size,
            (self.get_unique_sources_number() * walks_parameters.iterations) as usize,
            verbose,
        )
    }

    /// Returns triple with source nodes, destination nodes and labels for training model for link prediction.
    ///
    /// # Arguments
    ///
    /// * idx:u64 - The index of the batch to generate, behaves like a random random_state,
    /// * batch_size: usize - The maximal size of the batch to generate,
    /// * method: &str - String representing the required edge embedding method.
    /// * negative_samples: f64 - The component of netagetive samples to use,
    /// * avoid_false_negatives: bool - Wether to remove the false negatives when generated.
    ///     - It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * maximal_sampling_attempts: usize - Number of attempts to execute to sample the negative edges.
    /// * graph_to_avoid: Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    pub fn link_prediction<'a>(
        &'a self,
        idx: u64,
        batch_size: usize,
        method: &str,
        negative_samples: f64,
        avoid_false_negatives: bool,
        maximal_sampling_attempts: usize,
        graph_to_avoid: &'a Option<&Graph>,
    ) -> Result<impl ParallelIterator<Item = (usize, Vec<f64>, bool)> + 'a, String> {
        // xor the random_state with a constant so that we have a good amount of 0s and 1s in the number
        // even with low values (this is needed becasue the random_state 0 make xorshift return always 0)
        let random_state = idx ^ SEED_XOR as u64;

        if negative_samples < 0.0 || !negative_samples.is_finite() {
            return Err("Negative sample must be a posive real value.".to_string());
        }

        if self.embedding.is_none() {
            return Err("Embedding object was not provided.".to_string());
        }

        let method = match EdgeEmbeddingMethods::new(method)? {
            EdgeEmbeddingMethods::Hadamard => |x1: &Vec<f64>, x2: &Vec<f64>| {
                x1.iter()
                    .cloned()
                    .zip(x2.iter().cloned())
                    .map(|(x1, x2)| x1 * x2)
                    .collect()
            },
            EdgeEmbeddingMethods::Average => |x1: &Vec<f64>, x2: &Vec<f64>| {
                x1.iter()
                    .cloned()
                    .zip(x2.iter().cloned())
                    .map(|(x1, x2)| (x1 + x2) / 2.0)
                    .collect()
            },
            EdgeEmbeddingMethods::Sum => |x1: &Vec<f64>, x2: &Vec<f64>| {
                x1.iter()
                    .cloned()
                    .zip(x2.iter().cloned())
                    .map(|(x1, x2)| x1 + x2)
                    .collect()
            },
            EdgeEmbeddingMethods::L1 => |x1: &Vec<f64>, x2: &Vec<f64>| {
                x1.iter()
                    .cloned()
                    .zip(x2.iter().cloned())
                    .map(|(x1, x2)| x1 - x2)
                    .collect()
            },
            EdgeEmbeddingMethods::AbsoluteL1 => |x1: &Vec<f64>, x2: &Vec<f64>| {
                x1.iter()
                    .cloned()
                    .zip(x2.iter().cloned())
                    .map(|(x1, x2)| (x1 - x2).abs())
                    .collect()
            },
            EdgeEmbeddingMethods::L2 => |x1: &Vec<f64>, x2: &Vec<f64>| {
                x1.iter()
                    .cloned()
                    .zip(x2.iter().cloned())
                    .map(|(x1, x2)| (x1 - x2).powi(2))
                    .collect()
            },
            EdgeEmbeddingMethods::Concatenate => {
                |x1: &Vec<f64>, x2: &Vec<f64>| x1.iter().chain(x2.iter()).cloned().collect()
            }
        };

        // The number of negatives is given by computing their fraction of batchsize
        let negative_number: usize =
            ((batch_size as f64 / (1.0 + negative_samples)) * negative_samples) as usize;
        // All the remaining values then are positives
        let positive_number: usize = batch_size - negative_number;
        let graph_has_no_self_loops = !self.has_selfloops();

        let edges_number = self.get_edges_number() as u64;
        let nodes_number = self.get_nodes_number() as u64;

        let mut rng: StdRng = SeedableRng::seed_from_u64(random_state);
        let random_values = gen_random_vec(batch_size, random_state);
        let mut indices: Vec<usize> = (0..batch_size).collect();
        indices.shuffle(&mut rng);

        match &self.embedding {
            Some(embedding) =>Ok((0..batch_size)
                .into_par_iter()
                .map(move |i| {
                    let mut sampled = random_values[i];
                    let (src, dst, label) = if i < positive_number{
                        let (src, dst) = self.get_edge_from_edge_id(sampled % edges_number);
                        (src, dst, true)
                    } else {
                        let mut attempts = 0;
                        loop {
                            if attempts > maximal_sampling_attempts {
                                panic!(format!(
                                    concat!(
                                        "Executed more than {} attempts to sample a negative edge.\n",
                                        "If your graph is so small that you see this error, you may want to consider ",
                                        "using one of the edge embedding transformer from the Embiggen library."
                                    ),
                                    maximal_sampling_attempts
                                ));
                            }
                            attempts += 1;
                            let random_src = sampled & 0xffffffff; // We need this to be an u64.
                            let random_dst = sampled >> 32; // We need this to be an u64.
                                                            // This technique is taken from:
                                                            // https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
                            let src = ((random_src * nodes_number) >> 32) as NodeT;
                            let dst = ((random_dst * nodes_number) >> 32) as NodeT;
                            if avoid_false_negatives && self.has_edge(src, dst, None) {
                                sampled = xorshift(sampled);
                                continue;
                            }
                            if let Some(g) = &graph_to_avoid {
                                if g.has_edge(src, dst, None) {
                                    sampled = xorshift(sampled);
                                    continue;
                                }
                            }
                            if graph_has_no_self_loops && src == dst {
                                sampled = xorshift(sampled);
                                continue;
                            }
                            break (src, dst, false);
                        }
                    };
                    (
                        indices[i],
                        method(&embedding[src as usize],&embedding[dst as usize]),
                        label
                    )
                })),
            None=>Err("Embedding object was not provided. Use the method 'set_embedding' to provide the embedding.".to_string())
        }
    }
}
