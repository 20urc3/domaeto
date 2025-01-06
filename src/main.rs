mod cli;
mod utilities;
use clap::Parser;


fn main()
{
    let args = cli::Cli::parse();
    println!("Time to make this fuzzer!");
}
