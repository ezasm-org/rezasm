use clap::Parser;

/// REzASM: An assembly like programming language for use in education
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Arguments {
    /// The number of words to allocate space for on the stack and heap each; must be larger than 0
    #[arg(short, long, default_value_t=20_000)]
    memory_size: u32,
    
    /// The size in bytes of a word (4 or 8)
    #[arg(short, long, default_value_t=4)]
    word_size: u8,

    /// A file to receive standard input from
    #[arg(short, long)]
    input_file: String,

    /// A file to print standard output to
    #[arg(short, long)]
    output_file: String,
}

pub fn get_args() -> Arguments {
    Arguments::parse()
}
