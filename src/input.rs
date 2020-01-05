use super::editor::{Editor, IntoEditor};
use super::error::{Error, Result};
use super::theme::{Theme, DEFAULT_THEME};
use std::io::{stdin, stdout, Read, Write};
use termion::cursor;
use termion::input::TermRead;

pub struct InputBuilder<'a> {
    msg: &'a str,
    theme: Option<Theme>,
}

impl<'a> InputBuilder<'a> {
    pub fn new(msg: &'a str) -> InputBuilder<'a> {
        InputBuilder { msg, theme: None }
    }

    pub fn theme(mut self, theme: Theme) -> InputBuilder<'a> {
        self.theme = Some(theme);
        self
    }

    pub fn build(self) -> Input<'a> {
        Input {
            msg: self.msg,
            theme: self.theme,
        }
    }
}

impl<'de> IntoEditor for InputBuilder<'de> {
    type Editor = Input<'de>;
    fn into_editor(self) -> Self::Editor {
        self.build()
    }
}

pub struct Input<'a> {
    msg: &'a str,
    theme: Option<Theme>,
}

impl<'a> Input<'a> {
    pub fn new(msg: &'a str) -> InputBuilder<'a> {
        InputBuilder::new(msg)
    }

    pub fn run(&self) -> Result<String> {
        <Input as Editor>::run(
            self,
            &mut stdin(),
            &mut stdout(),
            self.theme.as_ref().unwrap_or(&DEFAULT_THEME),
        )
    }
}

impl<'a> Editor for Input<'a> {
    type Output = String;
    fn run<R: Read, W: Write>(
        &self,
        stdin: &mut R,
        stdout: &mut W,
        theme: &Theme,
    ) -> Result<Self::Output> {
        theme.print_question(stdout, self.msg)?;

        stdout.flush()?;

        let input = match TermRead::read_line(stdin) {
            Ok(Some(s)) => s,
            Ok(None) => return Err(Error::NoMoreInput),
            Err(e) => return Err(Error::IoError(e)),
        };

        write!(stdout, "{}", cursor::Up(1))?;
        theme.print_results(stdout, self.msg, &input)?;

        Ok(input)
    }
}

pub fn input(msg: &str) -> Result<String> {
    InputBuilder::new(msg).build().run()
}
