pub mod core {
    /// To match a element of website
    pub mod element;

    /// To find element of website and show
    /// all infomation, if this exist
    pub mod find_element;
}

/// CLI module, to define all commands of CLI
mod cli;

/// For handles all commands of CLI
pub mod cmds {
    pub mod handles;
}
/// Define Colors/LogLevel
mod colors;
/// Define all error types
mod errors;