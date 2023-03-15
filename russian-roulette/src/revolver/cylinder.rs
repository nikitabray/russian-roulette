use rand::Rng;

use super::Shoot;

pub struct Cylinder {
    pub size: u32,
    deadly_cartridge: u32,
    current_cartridge: u32,
}

impl Cylinder {
    pub fn new(size: u32) -> Self {
        let deadly_cartridge = rand::thread_rng().gen_range(1..size);
        println!("Deadly cartridge is at position {}", deadly_cartridge);
        Cylinder {size, 
        deadly_cartridge,
        current_cartridge: 1,
        }
    }
    pub fn spin(self: &mut Cylinder) {
        self.current_cartridge = rand::thread_rng().gen_range(1..self.size)
    }

    pub fn reload(self: &mut Cylinder) {
        self.deadly_cartridge = rand::thread_rng().gen_range(1..self.size);
    }

    pub fn shoot(self: &mut Cylinder) -> Shoot {
        if self.current_cartridge == self.size {
            self.current_cartridge = 1;
        };
        self.current_cartridge += 1;
        match self.current_cartridge == self.deadly_cartridge {
            true => Shoot::DEADLY,
            false => Shoot::BLANK,
        }
    }
}
