extern "C" {
    fn c_function();
}

fn main() {
    unsafe {
        c_function();
    }
}
