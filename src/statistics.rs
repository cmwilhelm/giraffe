extern crate gnuplot;
extern crate statistical;

use traits::HasHeight;
use traits::HasSpeed;
use world;


pub struct Statistics {
    height_means:          Vec<f64>,
    height_std_deviations: Vec<f64>,
    speed_means:           Vec<f64>,
    speed_std_deviations:  Vec<f64>,
    lion_speeds:           Vec<u32>,
    tree_heights:          Vec<u32>,
    generations:           u32
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            height_means:          vec![],
            height_std_deviations: vec![],
            speed_means:           vec![],
            speed_std_deviations:  vec![],
            lion_speeds:           vec![],
            tree_heights:          vec![],
            generations:           0
        }
    }

    pub fn update(&mut self, world: &world::World) {
        let sizes = world.tower.iter().map(|giraffe| {
            giraffe.height() as f64
        }).collect::<Vec<f64>>();

        let speeds = world.tower.iter().map(|giraffe| {
            giraffe.speed() as f64
        }).collect::<Vec<f64>>();

        let height_mean   = statistical::mean(&sizes);
        let height_stddev = statistical::standard_deviation(&sizes, None);
        let speed_mean    = statistical::mean(&speeds);
        let speed_stddev  = statistical::standard_deviation(&speeds, None);

        self.height_means.push(height_mean);
        self.height_std_deviations.push(height_stddev);
        self.speed_means.push(speed_mean);
        self.speed_std_deviations.push(speed_stddev);

        self.lion_speeds.push(world.lion_speed);
        self.tree_heights.push(world.tree_height);

        self.generations = world.generation;
    }

    pub fn print_latest(&self) {
        let latest_height_mean   = self.height_means.last();
        let latest_height_stddev = self.height_std_deviations.last();
        let latest_speed_mean    = self.speed_means.last();
        let latest_speed_stddev  = self.speed_std_deviations.last();
        let latest_lion_speed    = self.lion_speeds.last();
        let latest_tree_height   = self.tree_heights.last();

        let mut has_stats = [
            latest_height_mean,
            latest_height_stddev,
            latest_speed_mean,
            latest_speed_stddev
        ].into_iter().all( |stat| !stat.is_none() );

        if latest_tree_height.is_none() || latest_lion_speed.is_none() {
            has_stats = false;
        }

        if has_stats {
            println!(
                "g-height: {}, t-height, {}, g-speed: {}, l-speed: {}",
                latest_height_mean.unwrap(),
                latest_tree_height.unwrap(),
                latest_speed_mean.unwrap(),
                latest_lion_speed.unwrap()
            );
        } else {
            println!("No stats yet");
        }
    }

    pub fn generate_height_figure(&self, destination_file: &str) {
        let x: Vec<f64> = (0..self.generations).into_iter().map(|i| {
            i as f64
        }).collect();

        let mut figure = gnuplot::Figure::new();

        figure.set_terminal("png", destination_file);
        figure.axes2d()
            .lines(
                &x,
                &self.height_means,
                &[gnuplot::Caption("Mean Giraffe Height"), gnuplot::Color("black")]
            )
            .lines(
                &x,
                &self.tree_heights,
                &[gnuplot::Caption("Tree Height"), gnuplot::Color("red")]
            );

        figure.show();
    }

    pub fn generate_speed_figure(&self, destination_file: &str) {
        let x: Vec<f64> = (0..self.generations).into_iter().map(|i| {
            i as f64
        }).collect();

        let mut figure = gnuplot::Figure::new();

        figure.set_terminal("png", destination_file);
        figure.axes2d()
            .lines(
                &x,
                &self.speed_means,
                &[gnuplot::Caption("Mean Giraffe Speed"), gnuplot::Color("black")]
            )
            .lines(
                &x,
                &self.lion_speeds,
                &[gnuplot::Caption("Lion Speed"), gnuplot::Color("red")]
            );

        figure.show();
    }
}
