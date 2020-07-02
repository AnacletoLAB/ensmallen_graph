use super::*;
use rayon::prelude::*;
extern crate rand;
use rand::Rng;  
use rand::seq::SliceRandom;
use rand::thread_rng;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use hashbrown::{HashMap};

fn gen_random_usize_vec(num: usize, max: usize) -> Vec<usize> {
    // TODO! substitute with xorshiro
    let mut rng = rand::thread_rng();
    let vals: Vec<usize> = (0..num).map(|_| rng.gen_range(0, max)).collect();
    vals
}

#[macro_export]
macro_rules! max {
    ($a: expr, $b: expr) => {
        if $a >= $b {
            $a
        } else {
            $b
        }
    };
}
#[macro_export]
macro_rules! min {
    ($a: expr, $b: expr) => {
        if $a < $b {
            $a 
        } else {
            $b
        }
    };
}

fn skipgram_vector_length(walk_length:usize, window_size:usize)->usize{
    (0..walk_length)
        .map(|i| min!(walk_length, i + window_size + 1) - if i>window_size{i - window_size}else{0} - 1)
        .sum()
}


/// Preprocessing for ML algorithms on graph.
impl Graph {

    fn skipgram(
        &self,
        walk: &[usize],
        window_size: Option<usize>,
        negative_samples: Option<f64>,
        shuffle: Option<bool>,
    ) -> (
        (
            Vec<usize>,
            Vec<usize>
        ),
        Vec<u8>   
    ){
        let _negative_samples = negative_samples.unwrap_or(1.0);
        let _window_size = window_size.unwrap_or(4);
        let _shuffle = shuffle.unwrap_or(true);
    
        let vector_length: usize = skipgram_vector_length(walk.len(), _window_size);
    
        // create the positive data
        let total_length = (vector_length as f64 * (1.0 + _negative_samples)) as usize;
        let mut words: Vec<NodeT> = Vec::with_capacity(total_length);
        let mut contexts: Vec<NodeT> = Vec::with_capacity(total_length);
        
        for (i, wi) in walk.iter().enumerate() {
            let window_start = if i > _window_size {
                i - _window_size
            } else {
                0
            };
            let window_end = min!(walk.len(), i + _window_size + 1); 
            let delta = window_end - window_start - 1;
    
            words.extend_from_slice(&vec![*wi; delta][..]);
            contexts.extend_from_slice(&walk[window_start..i]);
            contexts.extend_from_slice(&walk[i + 1..window_end]);
        }
    
        let mut labels = vec![1; vector_length];
    
        // create negative data
        // In this implementation, negative samples ARE MANDATORY.
        
        // TODO! This thing can create false negatives!!
        // The issue was already present in the original TensorFlow implementation.
        let num_negatives = (vector_length as f64 *_negative_samples) as usize;
        let words_neg: Vec<NodeT> = gen_random_usize_vec(num_negatives, walk.len())
            .iter()
            .map(|i| walk[*i])
            .collect();
        let contexts_neg: Vec<NodeT> = gen_random_usize_vec(
            num_negatives,
            self.get_nodes_number()
        );
        let labels_neg = vec![0; num_negatives];
    
        // merge positives and negatives labels
        words.extend(words_neg.iter());
        contexts.extend(contexts_neg.iter());
        labels.extend(labels_neg.iter());
    
        if _shuffle {
            let mut indices: Vec<usize> = (0..words.len() as usize).collect();
            indices.shuffle(&mut thread_rng());
    
            words = indices.iter().map(|i| words[*i]).collect();
            contexts = indices.iter().map(|i| contexts[*i]).collect();
            labels = indices.iter().map(|i| labels[*i]).collect();
        }
    
        ((words, contexts), labels)
    }
    
    // TODO docstring
    pub fn skipgrams(
        &self,
        idx: usize,
        batch_size: usize,
        length: usize,
        iterations: Option<usize>,
        window_size: Option<usize>,
        negative_samples: Option<f64>,
        shuffle: Option<bool>,
        min_length: Option<usize>,
        return_weight: Option<ParamsT>,
        explore_weight: Option<ParamsT>,
        change_node_type_weight: Option<ParamsT>,
        change_edge_type_weight: Option<ParamsT>
    ) -> Result<((Vec<usize>,Vec<usize>),Vec<u8>), String>{

        if idx*batch_size >= self.get_nodes_number(){
            return Err(format!(
                concat!(
                    "The given walk index {idx} with batch size {batch_size} ",
                    "is larger than the number of nodes {nodes} in the graph."
                ),
                idx=idx,
                batch_size=batch_size,
                nodes=self.get_nodes_number()
            ));
        }

        let walks = self.walk(
            length,
            iterations,
            Some(idx*batch_size),
            Some(min!(self.get_nodes_number(), (idx+1)*batch_size)),
            min_length,
            return_weight,
            explore_weight,
            change_node_type_weight,
            change_edge_type_weight,
            Some(false)
        ).unwrap();

        let mut cumsum:Vec<usize> = Vec::with_capacity(walks.len());
        let _window_size = window_size.unwrap_or(4);
        let _negative_samples = negative_samples.unwrap_or(1.0);
        
        for i in 0..walks.len(){
            let new_value = (skipgram_vector_length(walks[i].len(), _window_size) as f64 * (1.0 + _negative_samples)) as usize;
            cumsum.push(
                if i==0 {
                    new_value
                } else {
                    cumsum[i-1] + new_value
                }
            );
        }
        
        let vector_length = cumsum[cumsum.len()-1];

        let mut words = vec![0; vector_length];
        let mut contexts = vec![0; vector_length];
        let mut labels = vec![1; vector_length];
        {
            let mut words_indices = Vec::new();
            let mut remaining_words_array = words.as_mut_slice();
            let mut contexts_indices = Vec::new();
            let mut remaining_contexts_array = contexts.as_mut_slice();
            let mut labels_indices = Vec::new();
            let mut remaining_labels_array = labels.as_mut_slice();
            for i in 0..cumsum.len() {
                let start = if i==0{
                    0
                } else {
                    cumsum[i-1]
                };
                let (words_left, words_right) = remaining_words_array.split_at_mut(cumsum[i] - start);
                let (contexts_left, contexts_right) = remaining_contexts_array.split_at_mut(cumsum[i] - start);
                let (labels_left, labels_right) = remaining_labels_array.split_at_mut(cumsum[i] - start);
                words_indices.push(words_left);
                contexts_indices.push(contexts_left);
                labels_indices.push(labels_left);
                remaining_words_array = words_right;
                remaining_contexts_array = contexts_right;
                remaining_labels_array = labels_right;
            }

            walks
                .par_iter()
                .zip(words_indices.par_iter_mut())
                .zip(contexts_indices.par_iter_mut())
                .zip(labels_indices.par_iter_mut())
                .for_each(|(((walk, words_index), contexts_index), labels_index)|{
                let ((_words, _contexts), _labels) = self.skipgram(
                    walk,
                    window_size,
                    Some(_negative_samples),
                    shuffle
                );
                (*words_index).copy_from_slice(&_words);
                (*contexts_index).copy_from_slice(&_contexts);
                (*labels_index).copy_from_slice(&_labels);
            });
        }
        let false_negatives:Vec<bool> = words.par_iter()
            .zip(contexts.par_iter())
            .zip(labels.par_iter())
            .map(
                |((src, dst), label)|
                (*label == 1) || self.has_edge(*src, *dst)
            )
            .collect();
        words = false_negatives.par_iter()
            .zip(words.par_iter())
            .filter(|(false_negative, _)| **false_negative)
            .map(|(_, src)| *src)
            .collect();
        contexts = false_negatives.par_iter()
            .zip(contexts.par_iter())
            .filter(|(false_negative, _)| **false_negative)
            .map(|(_, src)| *src)
            .collect();
        labels = false_negatives.par_iter()
            .zip(labels.par_iter())
            .filter(|(false_negative, _)| **false_negative)
            .map(|(_, src)| *src)
            .collect();
        Ok(((words, contexts), labels))
    }

    // TODO docstring
    pub fn cooccurence_matrix(
        &self,
        length: usize,
        window_size: Option<usize>,
        iterations: Option<usize>,
        min_length: Option<usize>,
        return_weight: Option<ParamsT>,
        explore_weight: Option<ParamsT>,
        change_node_type_weight: Option<ParamsT>,
        change_edge_type_weight: Option<ParamsT>,
        verbose: Option<bool>
    ) -> Result<(Vec<NodeT>, Vec<NodeT>, Vec<f64>), String> {

        let _verbose = verbose.unwrap_or(true);
        let _window_size = window_size.unwrap_or(4);

        // TODO: if in Rust is possible to return a generator, we could
        // iterate directly on the walks without storing them into an array.
        let walks = self.walk(
            length,
            iterations,
            None,
            None,
            min_length,
            return_weight,
            explore_weight,
            change_node_type_weight,
            change_edge_type_weight,
            Some(_verbose)
        ).unwrap();

        let mut cooccurence_matrix: HashMap<(NodeT, NodeT), f64> = HashMap::new();
        let pb1 = if _verbose {
            let pb1 = ProgressBar::new(walks.len() as u64);
            pb1.set_style(ProgressStyle::default_bar().template(
                "Computing cooccurrence mapping {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb1.set_draw_delta(walks.len() as u64 / 100);
            pb1
        } else {
            ProgressBar::hidden()
        };

        for i in (0..walks.len()).progress_with(pb1){
            let walk = &walks[i];
            let walk_length = walk.len();
            for (central_index, &central_word_id) in walk.iter().enumerate(){
                for distance in 1..1+_window_size{
                    if central_index + distance >= walk_length{
                        break;
                    }
                    let context_id = walk[central_index + distance];
                    if central_word_id < context_id{
                        *cooccurence_matrix.entry((central_word_id, context_id)).or_insert(0.0) += 1.0 / distance as f64;
                    } else {
                        *cooccurence_matrix.entry((context_id, central_word_id)).or_insert(0.0) += 1.0 / distance as f64;
                    }
                }
            }
        }

        let elements = cooccurence_matrix.len()*2;
        let mut max_frequency = 0.0;
        let mut words: Vec<NodeT> = vec![0; elements];
        let mut contexts: Vec<NodeT> = vec![0; elements];
        let mut frequencies: Vec<f64> = vec![0.0; elements];
        let pb2 = if _verbose {
            let pb2 = ProgressBar::new(cooccurence_matrix.len() as u64);
            pb2.set_style(ProgressStyle::default_bar().template(
                "Converting mapping into CSR matrix {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb2.set_draw_delta(cooccurence_matrix.len() as u64 / 100);
            pb2
        } else {
            ProgressBar::hidden()
        };

        cooccurence_matrix
            .iter()
            .progress_with(pb2)
            .enumerate()
            .for_each(|(i, ((word, context), frequency))|{
            let (k, j) = (i*2, i*2+1);
            if *frequency > max_frequency{
                max_frequency = *frequency;
            }
            words[k] = *word;
            words[j] = words[k];
            contexts[k] = *context;
            contexts[j] = contexts[k];
            frequencies[k] = *frequency;
            frequencies[j] = frequencies[k];
        });

        frequencies.par_iter_mut().for_each(|frequency| {*frequency/=max_frequency});

        Ok((words, contexts, frequencies))
    }

    pub fn link_prediction(
        &self,
        batch_size:u64,
        negative_samples: Option<f64>,
        graph_to_avoid: Option<&Graph>,
        shuffle: Option<bool>
    )->(Vec<NodeT>, Vec<NodeT>, Vec<u8>){
        let _negative_samples = negative_samples.unwrap_or(1.0);
        let _shuffle = shuffle.unwrap_or(true);
        let negatives_number:u64 = ((batch_size as f64 / (1.0 + _negative_samples)) * _negative_samples) as u64;
        let positives_number:u64 = batch_size - negatives_number;

        let edges_number = self.get_edges_number() as u64;
        let positives:Vec<(NodeT, NodeT)> = (0..positives_number)
            .into_par_iter()
            .map(|_| {
                let edge = (random_u64() % edges_number) as EdgeT;
                let src = self.sources[edge];
                let dst = self.destinations[edge];
                (src, dst)
            })
            .filter(|(src, dst)| if let Some(g) = &graph_to_avoid{
                !g.has_edge(*src, *dst)
            } else {
                true
            })
            .collect();

        let negatives:Vec<(NodeT, NodeT)> = (0..negatives_number)
            .into_par_iter()
            .map(|_| (
                self.sources[(random_u64() % edges_number) as EdgeT],
                self.destinations[(random_u64() % edges_number) as EdgeT]
                )
            )
            .filter(|(src, dst)| ! (self.has_edge(*src, *dst) || if let Some(g) = &graph_to_avoid{
                g.has_edge(*src, *dst)
            } else {
                false
            }))
            .collect();
        
        let mut labels:Vec<u8> = [vec![1 as u8; positives.len()], vec![0 as u8; negatives.len()]]
            .iter()
            .flatten()
            .cloned()
            .collect();

        let mut edges:Vec<(NodeT, NodeT)> = [positives, negatives]
            .iter()
            .flatten()
            .cloned()
            .collect();
        
        if _shuffle {
            let mut indices: Vec<usize> = (0..edges.len() as usize).collect();
            indices.shuffle(&mut thread_rng());

            labels = indices.par_iter().map(|i| labels[*i]).collect();
            edges = indices.par_iter().map(|i| edges[*i]).collect();
        }

        let sources:Vec<NodeT> = edges.par_iter().map(|(src, _)| *src).collect();
        let destinations:Vec<NodeT> = edges.par_iter().map(|(_, dst)| *dst).collect();

        (sources, destinations, labels)
    }
}