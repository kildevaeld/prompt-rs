use super::editor::Editor;
use super::error::{Error, Result};
use super::theme::{Theme, DEFAULT_THEME};
use std::io::{stdin, stdout, Read, Write};
use termion::input::TermRead;

pub struct PasswordBuilder<'de> {
    msg: &'de str,
    theme: Option<Theme>,
}

impl<'de> PasswordBuilder<'de> {
    pub fn new(msg: &'de str) -> PasswordBuilder<'de> {
        PasswordBuilder { msg, theme: None }
    }

    pub fn theme(mut self, theme: Theme) -> PasswordBuilder<'de> {
        self.theme = Some(theme);
        self
    }

    pub fn build(self) -> Password<'de> {
        Password {
            msg: self.msg,
            theme: self.theme,
        }
    }
}

pub struct Password<'de> {
    msg: &'de str,
    theme: Option<Theme>,
}

impl<'de> Password<'de> {
    pub fn new(msg: &'de str) -> PasswordBuilder<'de> {
        PasswordBuilder::new(msg)
    }

    pub fn run(&self) -> Result<String> {
        <Password as Editor>::run(
            self,
            &mut stdin(),
            &mut stdout(),
            self.theme.as_ref().unwrap_or(&DEFAULT_THEME),
        )
    }
}

impl<'de> Editor for Password<'de> {
    type Output = String;
    fn run<R: Read, W: Write>(
        &self,
        stdin: &mut R,
        stdout: &mut W,
        theme: &Theme,
    ) -> Result<Self::Output> {
        theme.print_question(stdout, self.msg)?;

        stdout.flush()?;

        let pass = stdin.read_passwd(stdout)?;

        let pass = match pass {
            Some(o) => o,
            None => return Err(Error::NoMoreInput),
        };

        theme.print_results(stdout, self.msg, "")?;

        Ok(pass)
    }
}

pub fn passwd(msg: &str) -> Result<String> {
    PasswordBuilder::new(msg).build().run()
}
