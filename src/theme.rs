use super::error::Result;
use std::fmt;
use std::io::Write;
use termion::{color, style};

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
}

impl Color {
    pub fn wrap<S: AsRef<str>>(&self, msg: S) -> String {
        format!("{}{}{}", self, msg.as_ref(), style::Reset)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "{}", color::Fg(color::Black)),
            Color::Blue => write!(f, "{}", color::Fg(color::Blue)),
            Color::Cyan => write!(f, "{}", color::Fg(color::Cyan)),
            Color::Green => write!(f, "{}", color::Fg(color::Green)),
            Color::Magenta => write!(f, "{}", color::Fg(color::Magenta)),
            Color::Red => write!(f, "{}", color::Fg(color::Red)),
            Color::White => write!(f, "{}", color::Fg(color::White)),
            Color::Yellow => write!(f, "{}", color::Fg(color::Yellow)),
        }
    }
}

pub struct Theme {
    results: Color,
    prefix: String,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            results: Color::Cyan,
            prefix: Color::Green.wrap("[?] "),
        }
    }
}

impl Theme {
    pub fn prefix<S: AsRef<str>>(mut self, prefix: S) -> Theme {
        self.prefix = prefix.as_ref().to_string();
        self
    }

    pub fn result_color(mut self, color: Color) -> Theme {
        self.results = color;
        self
    }

    pub fn print_question<R: Write>(&self, output: &mut R, msg: &str) -> Result<()> {
        Ok(write!(output, "{}{} ", self.prefix, msg)?)
    }

    pub fn print_results<R: Write>(&self, output: &mut R, msg: &str, answer: &str) -> Result<()> {
        Ok(write!(
            output,
            "\r{}{} {}\n\r",
            self.prefix,
            msg,
            self.results.wrap(answer),
        )?)
    }
}
