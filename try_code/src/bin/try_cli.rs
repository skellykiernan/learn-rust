use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    
    /// path to register definition file  
    #[structopt(short, long, parse(from_os_str))]
    defs: Option<std::path::PathBuf>,
    /// register name
    reg_name: String,
}

fn main() {

	let args = Cli::from_args();

    // Display the args then add some simple code
    println!("args.defs {:?}  args.reg_name {:?}", 
              args.defs, 
              args.reg_name);
}