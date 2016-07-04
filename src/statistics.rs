extern crate gnuplot;
extern crate statistical;

use world;


pub struct Statistics {
    means:          Vec<f64>,
    std_deviations: Vec<f64>,
    tree_heights:   Vec<u32>,
    generations:    u32
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            means:          vec![],
            std_deviations: vec![],
            tree_heights:   vec![],
            generations:    0
        }
    }

    pub fn update(&mut self, world: &world::World) {
        let sizes = world.tower.iter().map(|giraffe| {
            giraffe.height() as f64
        }).collect::<Vec<f64>>();

        let mean   = statistical::mean(&sizes);
        let stddev = statistical::standard_deviation(&sizes, None);

        self.means.push(mean);
        self.std_deviations.push(stddev);
        self.tree_heights.push(world.tree_height);

        self.generations = world.generation;
    }

    pub fn print_latest(&self) {
        let latest_mean        = self.means.last();
        let latest_stddev      = self.std_deviations.last();
        let latest_tree_height = self.tree_heights.last();

        let mut has_stats = [latest_mean, latest_stddev]
            .into_iter()
            .all(|stat| !stat.is_none() );

        if latest_tree_height.is_none() {
            has_stats = false;
        }

        if has_stats {
            println!(
                "mean: {}, std: {}, height: {}",
                latest_mean.unwrap(),
                latest_stddev.unwrap(),
                latest_tree_height.unwrap()
            );
        } else {
            println!("No stats yet");
        }
    }

    pub fn generate_figure(&self, destination_file: &str) {
        let x: Vec<f64> = (0..self.generations).into_iter().map(|i| {
            i as f64
        }).collect();

        let mut figure = gnuplot::Figure::new();

        figure.set_terminal("png", destination_file);
        figure.axes2d()
            .lines(
                &x,
                &self.means,
                &[gnuplot::Caption("Mean Giraffe Height"), gnuplot::Color("black")]
            )
            .lines(
                &x,
                &self.tree_heights,
                &[gnuplot::Caption("Tree Height"), gnuplot::Color("red")]
            );

        figure.show();
    }
}
