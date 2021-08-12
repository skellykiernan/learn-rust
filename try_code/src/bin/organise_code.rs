#[path = "../try_a_module/mod.rs"] mod try_a_module;

fn main() {
    try_a_module::pub_function();
    // A private function will error at compile time
    //try_a_module::function();
}
