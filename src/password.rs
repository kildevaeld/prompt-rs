use super::error::{ErrorKind, Result};
use super::theme::Theme;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;

pub struct PasswordBuilder<'de> {
    msg: &'de str,
    theme: Theme,
}

impl<'de> PasswordBuilder<'de> {
    pub fn new(msg: &'de str) -> PasswordBuilder<'de> {
        PasswordBuilder {
            msg,
            theme: Theme::default(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> PasswordBuilder<'de> {
        self.theme = theme;
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
    theme: Theme,
}

impl<'de> Password<'de> {
    pub fn run(&self) -> Result<String> {
        let stdin = stdin();
        let mut stdout = stdout();

        self.theme.print_question(&mut stdout, self.msg)?;

        stdout.lock().flush()?;

        let pass = stdin.lock().read_passwd(&mut stdout)?;

        let pass = match pass {
            Some(o) => o,
            None => return Err(ErrorKind::NoMoreInput.into()),
        };

        self.theme.print_results(&mut stdout, self.msg, "")?;

        Ok(pass)
    }
}

pub fn passwd(msg: &str) -> Result<String> {
    PasswordBuilder::new(msg).build().run()
}
