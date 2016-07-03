pub const LEG_SEGMENTS:  usize = 4;
pub const NECK_SEGMENTS: usize = 8;

#[derive(Debug)]
pub struct Giraffe {
    pub legs: Vec<u8>,
    pub neck: Vec<u8>
}
