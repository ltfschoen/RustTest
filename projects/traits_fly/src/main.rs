// note: associated functions do not use `&self` like methods do

// trait
trait Pilot {
    // associated function
    fn name() -> String;

    // method
    fn fly(&self);
}

// type
struct Human;

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
        println!("Pilot::fly(&self)");
    }
}

fn main() {
    let person = Human;
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
}