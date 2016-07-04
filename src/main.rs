extern crate gnuplot;
extern crate statistical;

mod giraffe;
mod simulation;
mod world;


const SIMULATION_LENGTH: u32 = 1500;

struct Statistics {
    means:          Vec<f64>,
    std_deviations: Vec<f64>,
    tree_heights:   Vec<u32>
}

impl Statistics {
    fn new() -> Self {
        Statistics {
            means:          vec![],
            std_deviations: vec![],
            tree_heights:   vec![]
        }
    }
}

fn update_state(world: &world::World, statistics: &mut Statistics) {
    let sizes = world.tower.iter().map(|giraffe| {
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
    let stddev = statistical::standard_deviation(&sizes, None);
    println!("mean: {}, std: {}, height: {}", mean, stddev, world.tree_height);

    statistics.means.push(mean);
    statistics.std_deviations.push(stddev);
    statistics.tree_heights.push(world.tree_height);
}


fn main () {
    let mut statistics = Statistics::new();

    let mut world = simulation::build_initial_world();

    for _ in 0..SIMULATION_LENGTH {
        update_state(&world, &mut statistics);
        world = simulation::evolve_world(world);
    }

    let x: Vec<f64> = (0..SIMULATION_LENGTH).into_iter().map(|i| i as f64).collect();
    let mut fg = gnuplot::Figure::new();
    fg.set_terminal("png", "output.png");

    fg.axes2d()
        .lines(&x, &statistics.means, &[gnuplot::Caption("Mean Giraffe Height"), gnuplot::Color("black")])
        .lines(&x, &statistics.tree_heights, &[gnuplot::Caption("Tree Height"), gnuplot::Color("red")]);

    fg.show();
}
