struct Point<T> {
    x: T,
    y: T,
}

/* Declare `impl<T>` to specify we are implementing
 * methods on type `Point<T>`
 */
impl<T> Point<T> {
    /* Getter method `x` returns "reference" to the
     * data in field type `T`
     */
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn run() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
