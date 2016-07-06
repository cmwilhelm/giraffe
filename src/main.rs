mod giraffe_lib;
mod giraffe;
mod statistics;
mod traits;
mod world;


const SIMULATION_LENGTH: u32 = 1500;


fn main () {
    let mut statistics = statistics::Statistics::new();
    let mut world      = world::World::new();

    statistics.update(&world);
    statistics.print_latest();

    for _ in 0..SIMULATION_LENGTH {
        world = world.evolve();
        statistics.update(&world);
        statistics.print_latest();
    }

    statistics.generate_color_figure("color_output.png");
    statistics.generate_height_figure("height_output.png");
    statistics.generate_speed_figure("speed_output.png");
}
