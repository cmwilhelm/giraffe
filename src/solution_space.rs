use gnuplot;
use gnuplot::AxesCommon;
use nalgebra;
use kiss3d;

use std::rc::Rc;
use std::cell::RefCell;

use giraffe::Giraffe;
use world;


const NUM_ROWS: u32 = (!0 as u8) as u32 * 4;
const NUM_COLS: u32 = (!0 as u8) as u32 * 8;

fn create_test_tower() -> Vec<Giraffe> {
    let mut results = vec![];

    for legs_size in 0..NUM_ROWS {
        for neck_size in 0..NUM_COLS {
            let giraffe = Giraffe::new_from_phenotypic_values(
                0, legs_size, neck_size
            );
            results.push(giraffe);
        }
    }

    results
}

fn create_test_world() -> world::World {
    let tower = create_test_tower();

    world::World::new_from_tower(tower, None)
}

fn create_fitness_matrix() -> Vec<Vec<f32>> {
    let world     = create_test_world();
    let fitnesses = world::calculate_fitnesses(&world, &world.tower);

    let mut results = vec![];

    for row_index in 0..NUM_ROWS {
        let start_value = row_index * NUM_COLS;

        let mut row = vec![];

        for col_index in 0..NUM_COLS {
            let flat_index = (start_value + col_index) as usize;

            row.push(fitnesses[flat_index]);
        }

        results.push(row);
    }

    results
}

fn create_mesh_points(matrix: &Vec<Vec<f32>>) -> Vec<nalgebra::Point3<f32>> {
    let mut results = vec![];

    for i in 0..(NUM_ROWS as usize) {
        for j in 0..(NUM_COLS as usize) {
            results.push(
                nalgebra::Point3::new(
                    i as f32 / 100.0,
                    j as f32 / 100.0,
                    (matrix[i][j] * 100.0).sqrt()
                )
            )
        }
    }

    results
}

fn create_mesh_triangles() -> Vec<nalgebra::Point3<u32>> {
    let mut results = vec![];

    for i in 0..(NUM_ROWS - 1) {
        let start_value = i * NUM_COLS;

        for j in 0..(NUM_COLS - 1) {
            results.push(
                nalgebra::Point3::new(
                    j + start_value,
                    j + 1 + start_value,
                    j + start_value + NUM_COLS
                )
            );

            results.push(
                nalgebra::Point3::new(
                    j + 1 + start_value,
                    j + 1 + start_value + NUM_COLS,
                    j + start_value + NUM_COLS
                )
            );
        }
    }

    results
}

pub fn render_plot(destination: &str) {
    let world     = create_test_world();
    let fitnesses = world::calculate_fitnesses(&world, &world.tower);

    let mut figure = gnuplot::Figure::new();

    figure.set_terminal("png", destination);
    figure.axes3d()
        .set_x_label("Neck Length", &vec![])
        .set_y_label("Leg Length", &vec![])
        .set_z_label("Fitness", &vec![])
        .surface(&fitnesses, NUM_ROWS as usize, NUM_COLS as usize, None, &vec![
            gnuplot::Caption("Fitness Terrain")
        ]);

    figure.show();
}

pub fn render_3d() {
    let mut window = kiss3d::window::Window::new("Fitness Terrain");

    let matrix    = create_fitness_matrix();
    let vertices  = create_mesh_points(&matrix);
    let triangles = create_mesh_triangles();

    let mesh = Rc::new(
        RefCell::new(
            kiss3d::resource::Mesh::new(vertices, triangles, None, None, false)
        )
    );

    let mut c = window.add_mesh(mesh, nalgebra::one());

    c.set_color(1.0, 0.0, 0.0);
    c.enable_backface_culling(false);

    window.set_light(kiss3d::light::Light::StickToCamera);

    while window.render() {
        c.prepend_to_local_rotation(&nalgebra::Vector3::new(0.0f32, 0.0, 0.001));
    }
}
