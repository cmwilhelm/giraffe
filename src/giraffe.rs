extern crate rand;

use giraffe_lib;
use traits::CanMate;
use traits::CanMutate;
use traits::HasHeight;
use traits::HasSpeed;


const LEG_SEGMENTS:  usize = 4;
const NECK_SEGMENTS: usize = 8;

pub struct Giraffe {
    legs: Vec<u8>,
    neck: Vec<u8>
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
}

impl CanMutate for Giraffe {
    fn mutate(&self, mutation_rate: f32) -> Self {
        let mutated_legs_chromosome = giraffe_lib::apply_mutations(
            &self.legs, mutation_rate
        );

        let mutated_neck_chromosome = giraffe_lib::apply_mutations(
            &self.neck, mutation_rate
        );

        Giraffe {
            legs: mutated_legs_chromosome,
            neck: mutated_neck_chromosome
        }
    }
}

impl CanMate for Giraffe {
    fn mate(giraffe1: &Self, giraffe2: &Self, mutation_rate: f32) -> Self {
        let mutated1 = giraffe1.mutate(mutation_rate);
        let mutated2 = giraffe2.mutate(mutation_rate);

        Giraffe {
            legs: giraffe_lib::blend_chromosomes(&mutated1.legs, &mutated2.legs),
            neck: giraffe_lib::blend_chromosomes(&mutated1.neck, &mutated2.legs)
        }
    }
}

impl HasHeight for Giraffe {
    fn height(&self) -> u32 {
        let mut total_height: u32 = 0;

        for l1 in self.legs.iter() {
            total_height = total_height + (*l1 as u32);
        }

        for l2 in self.neck.iter() {
            total_height = total_height + (*l2 as u32);
        }

        total_height
    }
}

impl HasSpeed for Giraffe {
    fn speed(&self) -> u32 {
        let leg_height = self.legs.iter().fold(0, |acc, &x| {
            acc + (x as u32)
        });

        leg_height
    }
}
