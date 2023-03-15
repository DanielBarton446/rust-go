use colored::Colorize;

pub trait Icon {
    fn get_icon(&self) -> colored::ColoredString;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum Stone {
    Black,
    White,
    Empty,
}

impl Icon for Stone {
    fn get_icon(&self) -> colored::ColoredString {
        match &self {
            // Larger ⬤
            // Stone::Black => "⬤ ".green(),
            // Stone::White => "⬤ ".blue(),
            Stone::Black => "●".blue(),
            Stone::White => "●".green(),
            _ => ".".bright_white(),
        }
    }
}
