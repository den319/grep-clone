use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Config {
    pub query: String,
    pub file_path:String,
    
    #[arg(short, long)]
    pub ignore_case: bool,

    #[arg(short, long)]
    pub recursive: bool,

    #[arg(short = 'n', long)]
    pub line_numbers: bool,
}
