use giraffe::Giraffe;


pub struct World {
    pub tower:            Vec<Giraffe>,
    pub mutation_percent: f32,
    pub tree_height:      u32
}
