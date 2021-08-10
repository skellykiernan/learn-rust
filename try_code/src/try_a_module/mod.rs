pub fn pub_function() {
    println!("called `try_a_module::pub_function()`");
    // can be called within the module
    function();
}


// Can pnly be called from within the module
fn function() {
    println!("called `try_a_module::function()`");
}
