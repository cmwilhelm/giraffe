use giraffe::Giraffe;


pub struct World {
    pub giraffes:         Vec<Giraffe>,
    pub mutation_percent: u8,
    pub tree_height:      u32
}
