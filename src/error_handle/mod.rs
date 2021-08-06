use crate::error_handle::error_type::ErrorType;
use crate::lexer::position::Position;

pub mod error_type;

pub fn dispatch_error(error: ErrorType, pos: Option<Position>) {
    eprintln!("\n\tInvalid script -> \"{}{}\"", error.to_string(), match pos {
        Some(unwrapped) => format!(" at {}", unwrapped.to_string()),
        None => String::new()
    });

    std::process::exit(1);
}