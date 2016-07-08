use giraffe_lib::random_proportion;
use rand;

pub fn apply_mutations(chromosome: &Vec<u8>, mutation_rate: f32) -> Vec<u8> {
    chromosome.iter().map(|gene| {
        if random_proportion() * 100.0 <= mutation_rate as f32 {
            rand::random::<u8>()
        } else {
            *gene
        }
    }).collect()
}
