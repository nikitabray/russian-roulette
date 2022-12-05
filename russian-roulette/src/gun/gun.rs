use rand::{rngs::ThreadRng, Rng};

use super::clip::Clip;

pub struct Gun {
    pub clip: Clip,
}

impl Gun {
    pub fn reload(self: &mut Gun, rng_seed: &mut ThreadRng) {
        self.clip.deadly_cartridge = rng_seed.gen_range(1..self.clip.size);
    }
}