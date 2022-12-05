use rand::Rng;

use crate::gun::clip::create_clip;
use crate::gun::gun::Gun;

pub mod gun;

fn main() {
    let mut rng = rand::thread_rng();
    let clip = create_clip(rng.gen(), &mut rng);
    let mut gun = Gun { clip };
    gun.reload(&mut rng);
}
