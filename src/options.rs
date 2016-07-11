use blending::BlendingMode;
use mutation::MutationDecay;
use defaults;

#[derive(Debug)]
pub struct Options {
    pub blending_mode:     BlendingMode,
    pub color:             u32,
    pub lion_speed:        u32,
    pub mutation_decay:    MutationDecay,
    pub mutation_rate:     f32,
    pub simulation_length: u32,
    pub herd_size:         u16,
    pub tree_height:       u32
}

impl Default for Options {
    fn default() -> Self {
        Options {
            blending_mode:     defaults::BLENDING_MODE,
            color:             defaults::COLOR,
            lion_speed:        defaults::LION_SPEED,
            mutation_decay:    defaults::MUTATION_DECAY,
            mutation_rate:     defaults::MUTATION_RATE,
            simulation_length: defaults::SIMULATION_LENGTH,
            herd_size:         defaults::WORLD_SIZE,
            tree_height:       defaults::TREE_HEIGHT
        }
    }
}
