use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    
    /// path to register definition file, an optional argument using the std::Option, see https://doc.rust-lang.org/std/option/  
    #[structopt(short, long, parse(from_os_str))]
    defs: Option<std::path::PathBuf>,
    /// register name, required positional argument
    reg_name: String,
}

fn main() {

	let args = Cli::from_args();

    // Display the args then add some simple code
    println!("args.defs {:?}  args.reg_name {:?}", 
              args.defs, 
              args.reg_name);

    println!("");
    println!("---- Checking arguments");
    println!("");

    // Checking an optional argument from the command line
    match args.defs {
        Some(x) => println!("There was SOME args.defs {:?}", x),
        None    => println!("There is no(NONE) args.defs"),
    }

    // TODO how to handle multiple options


}