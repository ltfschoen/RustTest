use std::fmt;
// note: associated functions do not use `&self` like methods do

// trait
trait Pilot: fmt::Display {
    // associated function
    fn name() -> String;

    // method
    fn fly(&self);

    // it is necessary to implement `Display` on Point otherwise get error:
    // `method cannot be called on `&Self` due to unsatisfied trait bounds` and
    // `Point cannot be formatted with the default formatter`
    // `Human cannot be formatted with the default formatter`
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{:?}", len);
    }
}

// type
struct Human;

// type struct
struct Point {
    x: i32,
    y: i32,
}

// type vector
// note: it is a tuple struct with a single element
struct Wrapper (
    Vec<String>
);

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Pilot for Point {
    // associated function of type
    fn name() -> String {
        String::from("Dot")
    }

    // method of type
    fn fly(&self) {
        println!("Point - Pilot::fly(&self)");
    }
}

impl Human {
    // associated function of type
    fn name() -> String {
        String::from("Fred")
    }

    // method of type implemented directly
    fn fly(&self) {
        println!("Human::fly(&self)");
    }
}

impl Pilot for Human {
    // associated function of type
    fn name() -> String {
        String::from("Maverick")
    }

    // method of type
    fn fly(&self) {
        println!("Human - Pilot::fly(&self)");
    }
}

fn main() {
    let person = Human;
    let point = Point { x: 0, y: 0 };
    // call method of specific type implementation using trait (explicit syntax)
    Pilot::fly(&person);
    // call method of type `Human` directly
    person.fly();
    Human::fly(&person); // equivalent to `person.fly()` to disambiguate
    // call associated function of type `Human` directly
    println!("{:?}", <Human>::name());
    // call associated function of `Pilot` trait that is implemented on type `Human`
    // we need to disambiguate using **fully qualified syntax**
    // `<Type as Trait>::function(receiver_if_method, next_arg, ...)`
    println!("{:?}", <Human as Pilot>::name());
    println!("{:?}", <Point as Pilot>::name());

    Pilot::outline_print(&person);
    Pilot::outline_print(&point);
    <Point as Pilot>::fly(&point);

    // using the "newtype pattern" to create a "wrapper type" to wrap type `Vec<T>` that we
    // want to implement trait `Display` for to overcome the "orphan rule" restriction
    let w = Wrapper(vec![
        (String::from("hello")),
        (String::from("world")),
    ]);
    println!("w = {}", w);
}