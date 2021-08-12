pub fn pub_function() {
    println!("called `try_a_module::pub_function()`");
    // can be called within the module
    function();
    // use the local module
    local_module::pub_function();
}


// Can only be called from within the module
fn function() {
    println!("called `try_a_module::function()`");
}


// a local module
mod local_module {
    pub fn pub_function() {
        println!("called `try_a_module::local_module::pub_function()`");
    }
}

