mod cli;

use structopt::StructOpt;


fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
    // cli::CommandLineArgs::from_args();
}