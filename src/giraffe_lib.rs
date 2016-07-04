extern crate rand;


pub fn random_proportion() -> f32 {
    let result = (rand::random::<u8>() as f32) / ((!0 as u8) as f32);
    result
}
