use crate::cli::color::TerminalColor;

/// color scheme struct for colored strings output
pub struct ColorScheme {
    /// node changed
    pub changed: TerminalColor,
    /// node changed
    pub removed: TerminalColor,
    /// node changed
    pub untracked: TerminalColor,
    /// standard
    pub standard: TerminalColor,
    /// default
    pub default: TerminalColor,
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme {
            changed: TerminalColor::Yellow,
            removed: TerminalColor::Red,
            untracked: TerminalColor::Blue,
            standard: TerminalColor::Green,
            default: TerminalColor::Default,
        }
    }
}
