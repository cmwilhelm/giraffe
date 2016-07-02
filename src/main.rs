mod giraffe;
mod simulation;
mod world;

const SIMULATION_LENGTH: u32 = 1000;


fn main () {
    let mut world = simulation::build_initial_world();

    for _ in 0..SIMULATION_LENGTH {
        world = simulation::evolve_world(world);
    }
}
