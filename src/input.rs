use super::error::ErrorKind;
use super::error::Result;
use super::theme::Theme;
use std::io::{stdin, stdout, Write};
use termion::cursor;
use termion::input::TermRead;

pub struct InputBuilder<'a> {
    msg: &'a str,
    theme: Theme,
}

impl<'a> InputBuilder<'a> {
    pub fn new(msg: &'a str) -> InputBuilder<'a> {
        InputBuilder {
            msg,
            theme: Theme::default(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> InputBuilder<'a> {
        self.theme = theme;
        self
    }

    pub fn build(self) -> Input<'a> {
        Input {
            msg: self.msg,
            theme: self.theme,
        }
    }

}

pub struct Input<'a> {
    msg: &'a str,
    theme: Theme,
}

impl<'a> Input<'a> {

    pub fn new(msg: &'a str) -> InputBuilder<'a> {
        InputBuilder::new(msg)
    }

    pub fn run(&self) -> Result<String> {
        let mut stdin = stdin();
        let mut stdout = stdout();

        self.theme.print_question(&mut stdout, self.msg)?;

        stdout.lock().flush()?;

        let input = match TermRead::read_line(&mut stdin) {
            Ok(Some(s)) => s,
            Ok(None) => return Err(ErrorKind::NoMoreInput.into()),
            Err(e) => return Err(ErrorKind::Io(e).into()),
        };

        write!(stdout, "{}", cursor::Up(1))?;
        self.theme.print_results(&mut stdout, self.msg, &input)?;

        Ok(input)
    }
}

pub fn input(msg: &str) -> Result<String> {
    InputBuilder::new(msg).build().run()
}
