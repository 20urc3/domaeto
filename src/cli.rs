use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(value_enum, help = "Operating mode: 'generator' or 'fuzzer'")]
    mode: Mode,
    
    #[arg(long, required_if_eq("mode", "generator"), 
          required_if_eq("mode", "fuzzer"),
          help = "Output directory")]
    output_dir: Option<std::path::PathBuf>,

    // Generator mode arguments
    #[arg(long, required_if_eq("mode", "generator"), 
          help = "Number of files to generate (required in generator mode)")]
    files: Option<u32>,

    // Fuzzer mode arguments
    #[arg(long, required_if_eq("mode", "fuzzer"), 
          help = "Timeout in seconds for fuzzing (required in fuzzer mode)")]
    timeout: Option<u32>,
    
    #[arg(long, required_if_eq("mode", "fuzzer"), 
          help = "Number of parallel fuzzing jobs (required in fuzzer mode)")]
    jobs: Option<u32>,
    
    #[arg(long, required_if_eq("mode", "fuzzer"), 
          help = "Path to the target binary to fuzz (required in fuzzer mode)")]
    target: Option<std::path::PathBuf>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Mode {
    #[value(help = "Generate test files")]
    Generator,
    #[value(help = "Fuzz a target with Domaeto")]
    Fuzzer,
}