/* Import communicator library crate into scope
 */
extern crate communicator;

/* Cargo treats main.rs as the root file of a binary crate
 * in same directory as the existing library crate in root module file lib.rs
 */
fn main() {
    communicator::client::connect();
    communicator::network::server::connect();
}
