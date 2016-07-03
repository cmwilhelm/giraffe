extern crate statistical;

mod giraffe;
mod simulation;
mod world;


const SIMULATION_LENGTH: u32 = 1000;


fn print_state(world: &world::World) {
    let mut total: i32 = 0;

    let sizes = world.giraffes.iter().map(|giraffe| {
        let mut total = 0;

        for leg in giraffe.legs.iter() {
            total = total + (*leg as i32);
        }

        for neck_segment in giraffe.neck.iter() {
            total = total + (*neck_segment as i32);
        }

        total as f32
    }).collect::<Vec<f32>>();

    let mean   = statistical::mean(&sizes);
    let stddev = statistical::standard_deviation(&sizes, None);
    println!("mean: {}, std: {}", mean, stddev);
}


fn main () {
    let mut world = simulation::build_initial_world();

    for _ in 0..SIMULATION_LENGTH {
        print_state(&world);
        world = simulation::evolve_world(world);
    }
}
