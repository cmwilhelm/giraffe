use giraffe_lib::random_proportion;
use rand;

#[derive(Copy, Clone, Debug)]
pub enum MutationDecay {
    None,
    Linear,
    Quadratic
}

pub fn apply_mutations(chromosome: &Vec<u8>, mutation_rate: f32) -> Vec<u8> {
    chromosome.iter().map(|gene| {
        if random_proportion() * 100.0 <= mutation_rate as f32 {
            rand::random::<u8>()
        } else {
            *gene
        }
    }).collect()
}

pub fn calculate_mutation_rate(
    decay:             MutationDecay,
    base_rate:         f32,
    generation:        u32,
    simulation_length: u32
) -> f32 {
    match decay {
        MutationDecay::None      => base_rate,
        MutationDecay::Linear    => linear_decay(base_rate, generation, simulation_length),
        MutationDecay::Quadratic => quadratic_decay(base_rate, generation, simulation_length)
    }
}

fn linear_decay(base_rate: f32, generation: u32, simulation_length: u32) -> f32 {
    (1.0 - generation as f32 / simulation_length as f32) * base_rate
}

fn quadratic_decay(base_rate: f32, generation: u32, simulation_length: u32) -> f32 {
    let quadratic_factor = base_rate / (simulation_length as u64).pow(2) as f32;

    -1.0 * quadratic_factor * (generation.pow(2) as f32) + base_rate
}
