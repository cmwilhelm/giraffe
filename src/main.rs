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
  giraffe [options]
  giraffe terrain (plot|3d) [options]

Options:
  -h --help                Show this screen.
  --length=<length>        Number of generations in simulation
  --size=<size>            Number of animals per generation
  --color=<color>          Color value, integer >= 0
  --speed=<speed>          Speed of lions, integer >= 0
  --height=<height>        Height of trees, integer >= 0
  --mutrate=<mutrate>      Rate of mutation, 0-100
  --mutdecay=<mutdecay>    One of: none | linear | quad
  --blendmode=<blendmode>  One of: mean | onepoint | uniform
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_terrain:    bool,
    cmd_plot:       bool,
    cmd_3d:         bool,
    flag_length:    Option<u32>,
    flag_size:      Option<u16>,
    flag_color:     Option<u32>,
    flag_speed:     Option<u32>,
    flag_height:    Option<u32>,
    flag_mutrate:   Option<u8>,
    flag_mutdecay:  Option<String>,
    flag_blendmode: Option<String>
}

fn run_simulation(options: options::Options) {
    let mut statistics = statistics::Statistics::new();
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

fn handle_terrain_command(args: &Args, options: options::Options) {
    if args.cmd_plot {
        let destination = "fitness_terrain.png";
        solution_space::render_plot(destination, options);
        println!("Generated fitness terrain plot to {:?}", destination);
    } else {
        println!("Preparing to render 3d environment...");
        solution_space::render_3d(options);
    }
}

fn make_options_from_args(args: &Args) -> options::Options {
    let mut options = options::Options::default();

    if let Some(length) = args.flag_length {
        options.simulation_length = length;
    }

    if let Some(size) = args.flag_size {
        options.herd_size = size;
    }

    if let Some(color) = args.flag_color {
        options.color = color;
    }

    if let Some(lion_speed) = args.flag_speed {
        options.lion_speed = lion_speed;
    }

    if let Some(tree_height) = args.flag_height {
        options.tree_height = tree_height;
    }

    if let Some(mutation_rate) = args.flag_mutrate {
        options.mutation_rate = mutation_rate as f32 / 100.0
    }

    if let Some(ref mutation_decay) = args.flag_mutdecay {
        options.mutation_decay = match mutation_decay.as_ref() {
            "linear" => mutation::MutationDecay::Linear,
            "quad"   => mutation::MutationDecay::Quadratic,
            _        => mutation::MutationDecay::None
        }
    }

    if let Some(ref blending_mode) = args.flag_blendmode {
        options.blending_mode = match blending_mode.as_ref() {
            "mean"     => blending::BlendingMode::Mean,
            "onepoint" => blending::BlendingMode::OnePointCrossover,
            _          => blending::BlendingMode::UniformCrossover
        }
    }

    println!("options: {:?}", options);

    options
}

fn main () {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let options: options::Options = make_options_from_args(&args);

    if args.cmd_terrain {
        handle_terrain_command(&args, options);
    } else {
        run_simulation(options);
    }
}
