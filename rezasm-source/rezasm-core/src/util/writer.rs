pub enum Writer {
    CLIApplication,
    GraphicalWriter,
}

impl Writer {
    pub fn write_string(&self, data: &str) {
        match self {
            Self::CLIApplication => print!("{}", data),
            Self::GraphicalWriter => todo!(),
        }
    }

    pub fn write_char(&self, data: &char) {
        match self {
            Self::CLIApplication => print!("{}", data),
            Self::GraphicalWriter => todo!(),
        }
    }

    pub fn write_integer(&self, data: &i64) {
        match self {
            Self::CLIApplication => print!("{}", data),
            Self::GraphicalWriter => todo!(),
        }
    }

    pub fn write_float(&self, data: &f64) {
        match self {
            Self::CLIApplication => print!("{}", data),
            Self::GraphicalWriter => todo!(),
        }
    }
}
