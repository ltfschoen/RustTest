extern {
    fn hello_function();
}

pub fn call() {
    unsafe {
        hello_function();
    }
}

fn main() {
    call();
}