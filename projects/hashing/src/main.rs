use rand::prelude::*;
use rand_core::{CryptoRng, RngCore};

fn pick_usize<R: RngCore, C: CryptoRng + ?Sized>(rng: &mut R, max: usize) -> usize {
    // use `Rng::gen_range` instead of `%` otherwise it generates biases in results
    // since the modulo of a uniform distribution makes it not uniform anymore
    rng.gen_range(0..=max)
}
    
fn main() {
    let max = 100;
    let mut rng = thread_rng();
    let random_number = pick_usize::<ThreadRng, dyn CryptoRng>(&mut rng, max);
    println!("{}", random_number);
}
