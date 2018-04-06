/* Structs used to add meaningful labels to data (instead of using Tuples)
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

/* Define `area` Method on the `Rectangle` Struct using an `impl` implementation block
 * with single parameter whose type is an immutable "borrow".
 * Rust knows the type of `self` is `Rectangle` since the Method is in the context of
 * `impl Rectangle`, which is the Struct instance.
 * We are only "borrowing", not taking "ownership", since only reading from the Struct data.
 *
 */
impl Rectangle {
    // Associated Functions
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/* Create instance of Struct. Call `area()` Method on the Struct instance.
 * Pass "reference" of Struct to `area` function to retain "ownership"
 * of the value so we may continue using it in `main()`.
 * `println!` macro uses `Display` default formatter when it uses `{}`
 * for primitive types. `Debug` Trait output formatter may be used with `{:?}`
 * or `{:#?}` for better readability.
 * after opting into its usage with `#[derive(Debug)]` before the Struct definition.
 */
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let square1 = Rectangle::square(3);

    println!("rect1 is {:?}", rect1);
    println!("square1 is {:?}", square1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square1? {}", rect1.can_hold(&square1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_area() {
        let actual_rect_instance: Rectangle = Rectangle { width: 30, height: 50 };
        let expected_response: u32 = 1500;
        assert_eq!(actual_rect_instance.area(), expected_response);
    }

    #[test]
    fn test_rect_can_hold_smaller_rect() {
        let actual_rect_instance_1: Rectangle = Rectangle { width: 30, height: 50 };
        let actual_rect_instance_2: Rectangle = Rectangle { width: 10, height: 40 };
        let expected_response: bool = true;
        assert_eq!(actual_rect_instance_1.can_hold(&actual_rect_instance_2), expected_response);
    }

    #[test]
    fn test_rect_can_hold_larger_rect() {
        let actual_rect_instance_1: Rectangle = Rectangle { width: 30, height: 50 };
        let actual_rect_instance_3: Rectangle = Rectangle { width: 60, height: 45 };
        let expected_response: bool = false;
        assert_eq!(actual_rect_instance_1.can_hold(&actual_rect_instance_3), expected_response);
    }

    #[test]
    fn test_rect_can_hold_smaller_square() {
        let actual_rect_instance_1: Rectangle = Rectangle { width: 30, height: 50 };
        let actual_rect_instance_4: Rectangle = Rectangle::square(3);
        let expected_response: bool = true;
        assert_eq!(actual_rect_instance_1.can_hold(&actual_rect_instance_4), expected_response);
    }
}