pub trait CanMate {
    fn mate(a: &Self, b: &Self, mutation_rate: f32) -> Self;
}

pub trait CanMutate {
    fn mutate(&self, mutation_rate: f32) -> Self;
}

pub trait HasHeight {
    fn height(&self) -> u32;
}
