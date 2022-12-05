use rand::{rngs::ThreadRng, Rng};

pub struct Clip {
    pub size: u32,
    pub deadly_cartridge: u32,
}

pub fn create_clip(size: u32, rng_seed: &mut ThreadRng) -> Clip {
    Clip {
        size,
        deadly_cartridge: rng_seed.gen_range(1..size),
    }
}
