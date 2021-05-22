use std::fmt;

pub enum TerminalColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}

const BLACK_COLOR: &str = "\x1b[0;30m";
const RED_COLOR: &str = "\x1b[0;31m";
const GREEN_COLOR: &str = "\x1b[0;32m";
const YELLOW_COLOR: &str = "\x1b[0;33m";
const BLUE_COLOR: &str = "\x1b[0;34m";
const MAGENTA_COLOR: &str = "\x1b[0;35m";
const CYAN_COLOR: &str = "\x1b[0;36m";
const WHITE_COLOR: &str = "\x1b[0;37m";
const DEFAULT_COLOR: &str = "\x1b[0m";

impl fmt::Display for TerminalColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TerminalColor::Black => BLACK_COLOR,
            TerminalColor::Red => RED_COLOR,
            TerminalColor::Green => GREEN_COLOR,
            TerminalColor::Yellow => YELLOW_COLOR,
            TerminalColor::Blue => BLUE_COLOR,
            TerminalColor::Magenta => MAGENTA_COLOR,
            TerminalColor::Cyan => CYAN_COLOR,
            TerminalColor::White => WHITE_COLOR,
            TerminalColor::Default => DEFAULT_COLOR,
        };
        write!(f, "{}", s)
    }
}

impl TerminalColor {
    pub fn colorize<T: AsRef<TerminalColor>>(s: &str, color: T) -> String {
        format!("{}{}{}", color.as_ref(), s, TerminalColor::Default)
    }
}

impl AsRef<TerminalColor> for TerminalColor {
    fn as_ref(&self) -> &Self {
        self
    }
}
