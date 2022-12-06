use rand::{rngs::ThreadRng, Rng};

use super::cylinder::Cylinder;

pub struct Revolver {
    pub cylinder: Cylinder,
}

impl Revolver {
    pub fn reload(self: &mut Revolver, rng_seed: &mut ThreadRng) {
        self.cylinder.deadly_cartridge = rng_seed.gen_range(1..self.cylinder.size);
    }

    pub fn shoot(self: &mut Revolver) {
        self.cylinder.deadly_cartridge += 1;
    }
}