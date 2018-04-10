/* `Point<T>` Struct holds `x` and `y` coordinate
 * values of any type
 */
pub struct Point<T, U> {
    x: T,
    y: U,
}

pub fn run() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
