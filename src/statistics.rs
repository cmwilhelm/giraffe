use gnuplot;
use gnuplot::AxesCommon;
use statistical;

use traits::HasColor;
use traits::HasHeight;
use traits::HasSpeed;
use world;


pub struct Statistics {
    color_means:           Vec<f64>,
    color_std_deviations:  Vec<f64>,
    height_means:          Vec<f64>,
    height_std_deviations: Vec<f64>,
    leg_means:             Vec<f64>,
    neck_means:            Vec<f64>,
    speed_means:           Vec<f64>,
    speed_std_deviations:  Vec<f64>,
    world_colors:          Vec<u32>,
    lion_speeds:           Vec<u32>,
    tree_heights:          Vec<u32>,
    generations:           u32
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            color_means:           vec![],
            color_std_deviations:  vec![],
            height_means:          vec![],
            height_std_deviations: vec![],
            leg_means:             vec![],
            neck_means:            vec![],
            speed_means:           vec![],
            speed_std_deviations:  vec![],
            world_colors:          vec![],
            lion_speeds:           vec![],
            tree_heights:          vec![],
            generations:           0
        }
    }

    pub fn update(&mut self, world: &world::World) {
        let colors = world.tower.iter().map(|giraffe| {
            giraffe.color() as f64
        }).collect::<Vec<f64>>();

        let sizes = world.tower.iter().map(|giraffe| {
            giraffe.height() as f64
        }).collect::<Vec<f64>>();

        let leg_lengths = world.tower.iter().map(|giraffe| {
            giraffe.leg_length() as f64
        }).collect::<Vec<f64>>();

        let neck_lengths = world.tower.iter().map(|giraffe| {
            giraffe.neck_length() as f64
        }).collect::<Vec<f64>>();

        let speeds = world.tower.iter().map(|giraffe| {
            giraffe.speed() as f64
        }).collect::<Vec<f64>>();

        let color_mean    = statistical::mean(&colors);
        let color_stddev  = statistical::standard_deviation(&colors, None);
        let height_mean   = statistical::mean(&sizes);
        let height_stddev = statistical::standard_deviation(&sizes, None);
        let leg_mean      = statistical::mean(&leg_lengths);
        let neck_mean     = statistical::mean(&neck_lengths);
        let speed_mean    = statistical::mean(&speeds);
        let speed_stddev  = statistical::standard_deviation(&speeds, None);

        self.color_means.push(color_mean);
        self.color_std_deviations.push(color_stddev);
        self.height_means.push(height_mean);
        self.height_std_deviations.push(height_stddev);
        self.leg_means.push(leg_mean);
        self.neck_means.push(neck_mean);
        self.speed_means.push(speed_mean);
        self.speed_std_deviations.push(speed_stddev);

        self.world_colors.push(world.options.color);
        self.lion_speeds.push(world.options.lion_speed);
        self.tree_heights.push(world.options.tree_height);

        self.generations = world.generation;
    }

    pub fn print_latest(&self) {
        let latest_color_mean    = self.color_means.last();
        let latest_height_mean   = self.height_means.last();
        let latest_speed_mean    = self.speed_means.last();
        let latest_world_color   = self.world_colors.last();
        let latest_lion_speed    = self.lion_speeds.last();
        let latest_tree_height   = self.tree_heights.last();

        if self.generations != 0 {
            println!(
                "g-color: {}, color: {}, g-height: {}, t-height, {}, g-speed: {}, l-speed: {}",
                latest_color_mean.unwrap(),
                latest_world_color.unwrap(),
                latest_height_mean.unwrap(),
                latest_tree_height.unwrap(),
                latest_speed_mean.unwrap(),
                latest_lion_speed.unwrap()
            );
        } else {
            println!("No stats yet");
        }
    }

    pub fn generate_color_figure(&self, destination_file: &str) {
        let x: Vec<f64> = (0..self.generations).into_iter().map(|i| {
            i as f64
        }).collect();

        let mut figure = gnuplot::Figure::new();

        figure.set_terminal("png", destination_file);
        figure.axes2d()
            .set_x_label("Generation", &vec![])
            .set_y_label("Color", &vec![])
            .lines(
                &x,
                &self.color_means,
                &[gnuplot::Caption("Mean Giraffe Color"), gnuplot::Color("black")]
            )
            .lines(
                &x,
                &self.world_colors,
                &[gnuplot::Caption("Tree Color"), gnuplot::Color("red")]
            );

        figure.show();
    }

    pub fn generate_height_figure(&self, destination_file: &str) {
        let x: Vec<f64> = (0..self.generations).into_iter().map(|i| {
            i as f64
        }).collect();

        let mut figure = gnuplot::Figure::new();

        figure.set_terminal("png", destination_file);
        figure.axes2d()
            .set_x_label("Generation", &vec![])
            .set_y_label("Height", &vec![])
            .lines(
                &x,
                &self.height_means,
                &[gnuplot::Caption("Mean Giraffe Height"), gnuplot::Color("black")]
            )
            .lines(
                &x,
                &self.leg_means,
                &[gnuplot::Caption("Mean Leg Length"), gnuplot::Color("blue")]
            )
            .lines(
                &x,
                &self.neck_means,
                &[gnuplot::Caption("Mean Neck Length"), gnuplot::Color("green")]
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
            .set_x_label("Generation", &vec![])
            .set_y_label("Speed", &vec![])
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
