use super::error::Result;
use super::Choice;
use std::fmt;
use std::io::Write;
use std::iter::FromIterator;
use strip_ansi_escapes::strip as normalize;
use termion::{color, style, clear};

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


pub struct Indicator {
    pub active: String,
    pub inactive: String
}

impl Indicator {
    pub fn as_str<'a>(&'a self, active: bool) -> &'a str {
        if active {
            self.active.as_str()
        } else {
            self.inactive.as_str()
        }
    }
}

pub struct Theme {
    results: Color,
    prefix: String,
    highlight_indicator: Indicator,
    selected_indicator: Indicator
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            results: Color::Cyan,
            prefix: Color::Green.wrap("[?] "),
            selected_indicator: Indicator {
                active: "◉".to_string(),
                inactive: "◯".to_string()
            },
            highlight_indicator: Indicator {
                active: "❯".to_string(),
                inactive: " ".to_string()
            }
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
            "{}\r{}{} {}\n\r",
            clear::CurrentLine,
            self.prefix,
            msg,
            self.results.wrap(answer),
        )?)
    }

    pub fn print_choice<R: Write, C: Choice>(
        &self,
        output: &mut R,
        choice: &C,
        highlighted: bool,
    ) -> Result<()> {
        let prefix = normalize(&self.prefix)?;
        let space = String::from_iter((0..prefix.len()).map(|_| ' '));
        Ok(write!(
            output,
            "{}\r{}{} {}",
            clear::CurrentLine,
            space,
            self.highlight_indicator.as_str(highlighted),
            choice.text()
        )?)
    }

    pub fn print_multiple_choice<R: Write, C: Choice>(
        &self,
        output: &mut R,
        choice: &C,
        highlighted: bool,
        selected: bool,
    ) -> Result<()> {
        let prefix = normalize(&self.prefix)?;
        let space = String::from_iter((0..prefix.len()).map(|_| ' '));
        Ok(write!(
            output,
            "{}\r{}{} {} {}",
            clear::CurrentLine,
            space,
            self.highlight_indicator.as_str(highlighted),
            self.selected_indicator.as_str(selected),
            choice.text()
        )?)
    }
}
