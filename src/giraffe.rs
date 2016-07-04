extern crate rand;

use giraffe_lib::random_proportion;


const LEG_SEGMENTS:  usize = 4;
const NECK_SEGMENTS: usize = 8;

pub struct Giraffe {
    pub legs: Vec<u8>,
    pub neck: Vec<u8>
}

impl Giraffe {
    pub fn random() -> Self {
        let legs_chromosome: Vec<u8> = (0..LEG_SEGMENTS).map(|_| {
            rand::random::<u8>()
        }).collect();

        let neck_chromosome: Vec<u8> = (0..NECK_SEGMENTS).map(|_| {
            rand::random::<u8>()
        }).collect();

        Giraffe { legs: legs_chromosome, neck: neck_chromosome }
    }

    pub fn mate(giraffe1: &Self, giraffe2: &Self, mutation_rate: f32) -> Self {
        let mutated1 = giraffe1.mutate(mutation_rate);
        let mutated2 = giraffe2.mutate(mutation_rate);

        Giraffe {
            legs: blend_characteristics(&mutated1.legs, &mutated2.legs),
            neck: blend_characteristics(&mutated1.neck, &mutated2.legs)
        }
    }

    fn mutate(&self, mutation_rate: f32) -> Self {
        let mutated_legs_chromosome = apply_mutations(&self.legs, mutation_rate);
        let mutated_neck_chromosome = apply_mutations(&self.neck, mutation_rate);

        Giraffe {
            legs: mutated_legs_chromosome,
            neck: mutated_neck_chromosome
        }
    }
}


fn apply_mutations(characteristics: &Vec<u8>, mutation_rate: f32) -> Vec<u8> {
    characteristics.iter().map(|i| {
        if random_proportion() * 100.0 <= mutation_rate as f32 {
            rand::random::<u8>()
        } else {
            *i
        }
    }).collect()
}

fn blend_characteristics(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(i, j)| {
        if random_proportion() <= 0.5 {
            *i
        } else {
            *j
        }
    }).collect::<Vec<u8>>()
}
