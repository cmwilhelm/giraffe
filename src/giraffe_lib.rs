use rand;

pub fn random_proportion() -> f32 {
    let result = (rand::random::<u8>() as f32) / ((!0 as u8) as f32);
    result
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

pub enum BlendingMode {
    Mean,
    UniformCrossOver
}

pub fn blend_chromosomes(a: &Vec<u8>, b: &Vec<u8>, mode: &BlendingMode) -> Vec<u8> {
    match mode {
        _ => uniform_crossover(&a, &b)
    }
}

fn uniform_crossover(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b).map(|(a_gene, b_gene)| {
        if random_proportion() <= 0.5 {
            *a_gene
        } else {
            *b_gene
        }
    }).collect::<Vec<u8>>()
}
