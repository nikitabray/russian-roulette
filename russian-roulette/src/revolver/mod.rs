pub mod cylinder;

pub enum Shoot {
    BLANK,
    DEADLY,
}

pub struct Revolver {
    pub cylinder: cylinder::Cylinder,
}

impl Revolver {
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
