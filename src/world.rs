use giraffe::Giraffe;
use giraffe_lib::random_proportion;


const WORLD_SIZE:    u16 = 1000;
const TREE_HEIGHT:   u32 = 1500;
const MUTATION_RATE: f32 = 0.001;

pub struct World {
    pub tower:       Vec<Giraffe>,
    pub tree_height: u32,
    mutation_rate:   f32,
    generation:      u32
}

impl World {
    pub fn new() -> Self {
        let tower: Vec<Giraffe> = (0..WORLD_SIZE).map(|_| {
            Giraffe::random()
        }).collect();

        World {
            tower:         tower,
            mutation_rate: MUTATION_RATE,
            tree_height:   TREE_HEIGHT,
            generation:    0
        }
    }

    pub fn evolve(&self) -> Self {
        let fitnesses: Vec<f32> = calculate_fitnesses(
            &self,
            &self.tower
        );

        let (cumulative_densities, total_density) =
            generate_cumulative_densities(fitnesses);

        let tower: Vec<Giraffe> = (0..WORLD_SIZE).map(|_| {
            let giraffe1 = select_giraffe(
                &cumulative_densities,
                total_density,
                &self.tower
            );

            let giraffe2 = select_giraffe(
                &cumulative_densities,
                total_density,
                &self.tower
            );

            Giraffe::mate(giraffe1, giraffe2, self.mutation_rate)
        }).collect();

        let tree_height = if random_proportion() < 0.0001 {
            (random_proportion() * 1500.0 + 500.0) as u32
        } else {
            self.tree_height
        };

        World {
            tower:         tower,
            mutation_rate: self.mutation_rate,
            tree_height:   tree_height,
            generation:    self.generation + 1
        }
    }
}


fn generate_cumulative_densities(fitnesses: Vec<f32>) -> (Vec<f64>, f64) {
    let mut total: f64 = 0.0;
    let mut cds        = vec![];

    for fitness in fitnesses.iter() {
        cds.push(total);
        total = total + (*fitness as f64);
    }

    (cds, total)
}

fn select_giraffe<'a>(cumulative_densities: &Vec<f64>, total_density: f64, tower: &'a Vec<Giraffe>) -> &'a Giraffe {
    let mut range_start = 0;
    let mut range_end   = cumulative_densities.len();
    let mut done        = false;

    let search_value = (random_proportion() as f64) * (total_density);

    let mut current: usize = 0;

    while !done {
        current = (range_start + range_end) / 2;
        if search_value >= cumulative_densities[current] {
            let upper_bound;

            if cumulative_densities.len() == current + 1 {
                upper_bound = total_density + 1.0;
            } else {
                upper_bound = cumulative_densities[current + 1]
            }

            if search_value < upper_bound {
                done = true;
            } else {
                range_start = current;
            }
        } else {
            range_end = current;
        }

        if range_end == range_start {
            done = true;
        }
    }

    &tower[current]
}

fn calculate_fitnesses(world: &World, tower: &Vec<Giraffe>) -> Vec<f32> {
    let height_deltas: Vec<i32> = tower.iter().map(|giraffe| {
        calculate_tree_delta(&world, &giraffe)
    }).collect();

    let max_delta = match height_deltas.iter().max() {
        Some(m) => *m,
        _       => 0
    };

    height_deltas.iter().map(|delta| {
        ((max_delta - *delta) as f32).cbrt().sqrt()
    }).collect::<Vec<f32>>()
}

fn calculate_tree_delta(world: &World, giraffe: &Giraffe) -> i32 {
    let mut total_height: i32 = 0;

    for l1 in giraffe.legs.iter() {
        total_height = total_height + (*l1 as i32);
    }

    for l2 in giraffe.neck.iter() {
        total_height = total_height + (*l2 as i32);
    }

    (world.tree_height as i32 - total_height as i32).abs()
}
