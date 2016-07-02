use giraffe::Giraffe;

pub struct World {
    giraffes:    Vec<Giraffe>,
    tree_height: u32
}

pub fn build_world() -> World {
    let giraffes = vec![
        Giraffe { legs: vec![1,2,3,4], neck: vec![3,4] }
    ];

    World {
        giraffes: giraffes,
        tree_height: 48

    }
}
