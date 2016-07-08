use giraffe_lib::random_proportion;

#[derive(Copy, Clone)]
pub enum BlendingMode {
    Mean,
    OnePointCrossover,
    UniformCrossover
}

pub fn blend_chromosomes(a: &Vec<u8>, b: &Vec<u8>, mode: BlendingMode) -> Vec<u8> {
    match mode {
        BlendingMode::Mean              => mean(&a, &b),
        BlendingMode::OnePointCrossover => one_point_crossover(&a, &b),
        BlendingMode::UniformCrossover  => uniform_crossover(&a, &b)
    }
}

fn mean(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b).map(|(a_gene, b_gene)| {
        ((*a_gene as u16 + *b_gene as u16) / 2) as u8
    }).collect::<Vec<u8>>()
}

fn one_point_crossover(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let crossover_point = (random_proportion() * (a.len() as f32)) as usize;

    let mut new_chromosome = vec![];

    for i in 0..a.len() {
        let source = if i < crossover_point {
            a
        } else {
            b
        };

        new_chromosome.push(source[i]);
    }

    new_chromosome
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
