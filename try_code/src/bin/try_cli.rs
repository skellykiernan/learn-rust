use structopt::StructOpt;

/// this is first time using StructOpt for CLI app
#[derive(StructOpt)]
struct Cli {
    
    /// register name, required positional argument
    reg_name: String,
    /// register value, this is optional as may want to specify field values instead
    reg_value: Option<String>,
    
    
    /// path to register definition file, an optional argument using the std::Option, see https://doc.rust-lang.org/std/option/  
    #[structopt(short, long, parse(from_os_str))]
    defs: Option<std::path::PathBuf>,
    /// field name within the register
    #[structopt(short, long)]
    field_names: Option<Vec<String>>,

}

fn main() {

	let args = Cli::from_args();

    // Display the args then add some simple code
    println!("
              \targs.defs {:?}  
              \targs.reg_name {:?}
              \targs.reg_value {:?}
              \targs.field_name {:?}", 
              args.defs, 
              args.reg_name,
              args.reg_value,
              args.field_names);

    println!("");
    println!("---- Checking arguments");
    println!("");

    // Checking an optional argument from the command line
    match args.defs {
        Some(defs) => println!("There was SOME args.defs {:?}", defs),
        None    => println!("There is no(NONE) args.defs"),
    }

    match args.field_names {
        Some(field_names) => println!("There was SOME field_names {:?}", field_names),
        None    => println!("There is no(NONE) args.field_names"),
    }

    match args.reg_value {
        Some(reg_value) => println!("There was SOME reg_value {:?}", reg_value),
        None    => println!("There is no(NONE) reg_value"),
    }

    // TODO how to handle multiple options


}