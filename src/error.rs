#[derive(Debug, Clone, Copy)]
pub enum ProgramErrorKind {
    NotEnoughtArgs,
    Invalid,
    FileFailedW,
    FileFailedR,
    FileEmpty,
    FileCreationFail
}

impl ProgramErrorKind {
    pub fn get_value(self) -> String {
        match self {
            ProgramErrorKind::Invalid => "Invalid Mode!".to_string(),
            ProgramErrorKind::NotEnoughtArgs => "Not Enought args!".to_string(),
            ProgramErrorKind::FileFailedW => "Failed to write into the specified file.".to_string(),
            ProgramErrorKind::FileFailedR => "Failed to read the specified file.".to_string(),
            ProgramErrorKind::FileEmpty => "Empty file.".to_string(),
            ProgramErrorKind::FileCreationFail => "Failed to create file.".to_string(),
        }
    }

    pub fn print_err(self) -> () {
        eprintln!("ERR: {}", self.get_value());
        return;
    }

    pub fn exit_prog(self) -> () {
        std::process::exit(self as i32 + 1);
    }

    pub fn print_and_exit(self) -> () {
        self.print_err();
        self.exit_prog();
    }
}

