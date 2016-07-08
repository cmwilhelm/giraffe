use blending::BlendingMode;
use mutation::MutationDecay;

pub const SIMULATION_LENGTH: u32           = 1500;
pub const WORLD_SIZE:        u16           = 1000;
pub const TREE_HEIGHT:       u32           = 1500;
pub const MUTATION_RATE:     f32           = 0.001;
pub const LION_SPEED:        u32           = 500;
pub const COLOR:             u32           = 500;
pub const BLENDING_MODE:     BlendingMode  = BlendingMode::OnePointCrossover;
pub const MUTATION_DECAY:    MutationDecay = MutationDecay::None;
