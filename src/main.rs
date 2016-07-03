extern crate gnuplot;
extern crate statistical;

mod giraffe;
mod simulation;
mod world;


const SIMULATION_LENGTH: u32 = 500;


fn print_state(world: &world::World, means: &mut Vec<f64>) {
    let mut total: i32 = 0;

    let sizes = world.giraffes.iter().map(|giraffe| {
        let mut total = 0;

        for leg in giraffe.legs.iter() {
            total = total + (*leg as i64);
        }

        for neck_segment in giraffe.neck.iter() {
            total = total + (*neck_segment as i64);
        }

        total as f64
    }).collect::<Vec<f64>>();

    let mean   = statistical::mean(&sizes);

    means.push(mean);
    let stddev = statistical::standard_deviation(&sizes, None);
    println!("mean: {}, std: {}", mean, stddev);
}


fn main () {
    let mut means = vec![];
    let mut world = simulation::build_initial_world();

    for _ in 0..SIMULATION_LENGTH {
        print_state(&world, &mut means);
        world = simulation::evolve_world(world);
    }

    let x: Vec<f64> = (0..means.len()).into_iter().map(|i| i as f64).collect();
    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("png", "output.png");
    fg.axes2d()
        .lines(&x, &means, &[gnuplot::Caption("A line"), gnuplot::Color("black")]);
    fg.show();
}
