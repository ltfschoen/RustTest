// Experimenting with Sharding from https://github.com/Drops-of-Diamond/diamond_drops

// Internal Crates
extern crate sharding;

fn main() {
    sharding::examples::collator::collator_example();
    sharding::examples::threads::threads_example();
}