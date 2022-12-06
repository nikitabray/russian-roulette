use rand::{rngs::ThreadRng, Rng};

pub struct Cylinder {
    pub size: u32,
    pub deadly_cartridge: u32,
    current_cartridge: u32
}

impl Cylinder {
    pub fn spin(self: &mut Cylinder, rng_seed: &mut ThreadRng) {
        self.current_cartridge = rng_seed.gen_range(1..self.size);
    }
}

pub fn create_cylinder(size: u32, rng_seed: &mut ThreadRng) -> Cylinder {
    Cylinder {
        size,
        deadly_cartridge: rng_seed.gen_range(1..size),
        current_cartridge: 1
    }
}
