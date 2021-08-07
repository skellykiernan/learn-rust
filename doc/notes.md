# Overview

    - Not a first language ( is complex but more memory security)
    - Standout differences
      +   traits are used instead of inheritance(composition vs inheritance)
        - functionality for a  particular type has and can share with other types
      +   ownership and borrowing, memory safety guarantees without a garbage collector.

# Cargo

### create simple hello world project

```bash
cargo new ${NAME_PROJECT}
````

### build and run

```bash
cargo run # to build and run
cargo build # just to build
```
Note: the above will default to the debug versions adding `--release` will build and run the release optimized versions

### distribute

```bash  
cargo publish # publish a crate to crate.io 
```
when publishing ensure that the toml file has the neccessary metadata, see [Cargo's Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)


to install the crate from crates.io as follows:

```bash
cargo install <crate-name>
```
but `--git` and `--path` flags can change this source. 
***Question: Do these work with publish as well-??***


this is just a snippet of what is possible, but the doc on the [Cargo Commands](https://doc.rust-lang.org/cargo/commands/) shows what is possible and the wide.


# Syntax
## Variables

    - variables hold primitive data or references to data
    - variables are immutable by default
    - rust is a block-scoped language


```rust
  let [mut] <variable_name> : [type] = [value];
```

Also instead of let can have
    - `const` is for values that don't change, the name is replaced with the value when it's used,
    - `static` is similar to const, but has a fixed memory location and can act as a global variable.

Note: there is no type inference must be explicated declared, or shadowing allowed for `const` or `static`

## Types
    - rust is statically typed but compiler can infer what type we want to used based on the value and how we use it

## Control flow
.
### if 
```rust
  if <conditon> {
    <code>
  }
```
    - condition must be a `bool`, is explicit with no auto conversions
    - has `else` and `else if` as well, of course
  
### loops
.
    - loop : loop continously until you `break`
    - while
    - for : loop through collections

### nesting flow
.
    - can nest within statements, **I think more statements??? but not the for statement** e.g. the `let` when creating a variable 

```rust
  // let if example 
  let number = if <condition> { 5 } else { 6 }
``` 

```rust
  // let while example
  // TBD
```



## Arrays and Slices

    - Fixed sizes
    - Slices are references but with known size( unlike the C pointer)
## Functions

```rust
<pub> fn <functionName>([argIdentifier]: [argType]) [-> returnType] {

    <functionBody>

}
```
    - `pub` keyword for public, for function can access outside file(module)
    
## Methods

Methods are functions attached to objects

```rust
fn main() {
    let origin: Point = Point::set_as_origin();
    let mine: Point  = Point{ x:3.0, y: 4.0};
    let distance = mine.get_distance(origin);

    println!("Distance from my point to origin is {}", distance);
}

// Example of a point
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // example of static method
    fn set_as_origin() -> Point {
        Point {x: 0.0, y: 0.0}  // <= no semicolon means this is returned
    }

    fn get_distance(&self, from: Point) -> f64  {
        let x_diff = self.x - from.x;
        let y_diff = self.y - from.y;

        f64::sqrt(f64::powf(x_diff, 2.0)  + f64::powf(y_diff, 2.0)) // <= no semicolon means this is returned
    }

}
```


# Courses
  
## Udemy
  1.  [Ultimate Rust Crash Course](https://www.udemy.com/course/ultimate-rust-crash-course/)
      - build a console space invader game (Text console) 
  1.  


# References

  1. [learn by example](https://doc.rust-lang.org/stable/rust-by-example)
  1. [book](https://doc.rust-lang.org/book/)
  1. [Blog with list of resources](https://serokell.io/blog/learn-rust)
  1. [CLI BOOK](https://rust-cli.github.io/book/tutorial/cli-args.html)

  

            
# Learn from existing code 

  1. [dust (du + rust)](https://github.com/bootandy/dust)


# Learn by trying

##    Greek Clock device, 

### Objective
To have a console visual representation that shows positioning of the clock items when the input handle is turned

## Register Decoder CLI app

### Objective
  - Read register defs from a JSON/Config File the will allow user encode/decode the register fields values for 64/32 bit registers
  - Get a CLI Template for future utility apps ( see CLI book a)
  - 
  
### CLI
```text
reg_decode [-d defs.json] <reg_name>  [-f field_name] value [-f field_name value]...

-c, --config    : Specifies a json file that contains the register definitions,
                  if not specified the default  defs.json file will be used    

-f, --field     : specifies a field for the register, if specified the register value 
                  is displayed.  For multiple fields a register this option can be used 
                  multiple times 

```

# Interesting stuff

[Google Security blog: Rust in the Linux kernel](https://security.googleblog.com/2021/04/rust-in-linux-kernel.html)

