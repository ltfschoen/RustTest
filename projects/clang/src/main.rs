extern {
    fn hello();
}

pub fn call() {
    unsafe {
        hello();
    }
}

fn main() {
    call();
}