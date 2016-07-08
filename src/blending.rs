use giraffe_lib::random_proportion;

#[derive(Copy, Clone)]
pub enum BlendingMode {
    Mean,
    UniformCrossOver
}

pub fn blend_chromosomes(a: &Vec<u8>, b: &Vec<u8>, mode: BlendingMode) -> Vec<u8> {
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
