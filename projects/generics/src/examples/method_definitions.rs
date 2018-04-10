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

    // fn distance_from_origin<f32>(&self) -> f32 {
    //     (self.x.powi(2) + self.y.powi(2)).sqrt()
    // }
}

struct PointMixed<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointMixed<T, U> {
    // `V` and `W` types are only relevant to the method definition
    fn mixup<V, W>(self, other: PointMixed<V, W>) -> PointMixed<T, W> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let p = Point { x: 5, y: 10 };

    println!("Point with p.x = {}", p.x());
    // println!("p.distance_from_origin = {}", p.distance_from_origin());

    let p1 = PointMixed { x: 5, y: 10.4 };
    let p2 = PointMixed { x: "Hello", y: 'c'}; // String Slice + char
    let p3 = p1.mixup(p2);

    println!("PointMixed p3.x = {}, p3.y = {}", p3.x, p3.y);
}
