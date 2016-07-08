use blending::BlendingMode;


pub trait CanMutate {
    fn mutate(&self, mutation_rate: f32) -> Self;
}

pub trait CanMate : CanMutate {
    fn mate(a: &Self, b: &Self, mutation_rate: f32, blending_mode: BlendingMode) -> Self;
}

pub trait HasColor {
    fn color(&self) -> u32;
}

pub trait HasHeight {
    fn height(&self) -> u32;
}

pub trait HasSpeed {
    fn speed(&self) -> u32;
}
