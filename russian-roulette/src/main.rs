use rand::Rng;

use crate::revolver::cylinder::create_cylinder;
use crate::revolver::revolver::Revolver;

pub mod revolver;

fn main() {
    let mut rng = rand::thread_rng();
    let cylinder = create_cylinder(rng.gen(), &mut rng);
    let mut revolver = Revolver { cylinder };
    revolver.reload(&mut rng);
}
