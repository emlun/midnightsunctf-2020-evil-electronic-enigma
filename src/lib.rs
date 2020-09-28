mod leg_computer;
mod leg_computer_parse;

pub use leg_computer::LegComputer;
pub use leg_computer::RegisterRef;
pub use leg_computer::Word;
pub use leg_computer_parse::assemble_program;
pub use leg_computer_parse::generate_code;
