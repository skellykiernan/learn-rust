mod try_a_module;

fn main() {
    let x:u32 = 3;
    println!("Hello, world!, {}", x);
    try_a_module::pub_function();
    // A private function will error at compile time
    //try_a_module::function();

}
