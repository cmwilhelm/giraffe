extern crate docopt;
extern crate rustc_serialize;
extern crate nalgebra;
extern crate kiss3d;
extern crate gnuplot;
extern crate rand;
extern crate statistical;

mod blending;
mod defaults;
mod mutation;
mod giraffe;
mod giraffe_lib;
mod options;
mod solution_space;
mod statistics;
mod traits;
mod world;

const USAGE: &'static str = "
Giraffe!

Usage:
  giraffe
  giraffe terrain (plot|3d)

Options:
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_terrain: bool,
    cmd_plot:    bool,
    cmd_3d:      bool
}

fn run_simulation() {
    let mut statistics = statistics::Statistics::new();
    let options        = options::Options::default();
    let mut world      = world::World::new(options);

    statistics.update(&world);
    statistics.print_latest();

    for _ in 0..world.options.simulation_length {
        world = world.evolve();
        statistics.update(&world);
        statistics.print_latest();
    }

    statistics.generate_color_figure("color_output.png");
    statistics.generate_height_figure("height_output.png");
    statistics.generate_speed_figure("speed_output.png");
}

fn handle_terrain_command(args: &Args) {
    if args.cmd_plot {
        let destination = "fitness_terrain.png";
        solution_space::render_plot(destination);
        println!("Generated fitness terrain plot to {:?}", destination);
    } else {
        println!("Preparing to render 3d environment...");
        solution_space::render_3d();
    }
}

fn main () {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_terrain {
        handle_terrain_command(&args);
    } else {
        run_simulation();
    }
}
