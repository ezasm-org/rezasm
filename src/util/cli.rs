use clap::Parser;

/// REzASM: An assembly like programming language for use in education
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Arguments {
    /// The file of code to open
    #[arg()]
    code_file: String,

    /// The number of words to allocate space for on the stack and heap each; must be larger than 0
    #[arg(short, long, default_value_t = 20_000)]
    memory_size: usize,

    /// The size in bytes of a word (4 or 8)
    #[arg(short, long, default_value_t = 4)]
    word_size: usize,

    /// A file to receive standard input from
    #[arg(short, long)]
    input_file: Option<String>,

    /// A file to print standard output to
    #[arg(short, long)]
    output_file: Option<String>,
}

pub fn get_args() -> Arguments {
    Arguments::parse()
}

impl Arguments {
    pub fn get_memory_size(&self) -> usize {
        self.memory_size
    }

    pub fn get_word_size(&self) -> usize {
        self.word_size
    }

    pub fn get_code_file(&self) -> &String {
        &self.code_file
    }

    pub fn get_input_file(&self) -> &Option<String> {
        &self.input_file
    }

    pub fn get_output_file(&self) -> &Option<String> {
        &self.output_file
    }
}
