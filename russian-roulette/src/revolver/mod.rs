pub mod cylinder;
use cylinder::Cylinder;

pub enum Shoot {
    BLANK,
    DEADLY,
}

pub struct Revolver {
    pub cylinder: Cylinder,
}

impl Revolver {
    pub fn new(cylinder: Cylinder) -> Self {
        Revolver {cylinder}
    }
    pub fn spin(self: &mut Revolver) {
        self.cylinder.spin()
    }

    pub fn reload(self: &mut Revolver) {
        self.cylinder.reload()
    }

    pub fn shoot(self: &mut Revolver) -> Shoot {
        self.cylinder.shoot()
    }
}
