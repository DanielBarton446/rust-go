use colored::Colorize;

pub(crate) trait Icon {
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

impl Stone {
    pub fn get_opponent(&self) -> Option<Stone> {
        match &self {
            Stone::Black => Some(Stone::White),
            Stone::White => Some(Stone::Black),
            Stone::Empty => None,
        }
    }
}
