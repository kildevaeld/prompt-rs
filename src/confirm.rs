use super::error::{Error, Result};
use super::theme::{Color, Theme, DEFAULT_THEME};
use super::{Editor, IntoEditor};
use std::io::{stdin, stdout, Read, Write};
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct ConfirmBuilder<'de> {
    msg: &'de str,
    theme: Option<Theme>,
    default: bool,
}

impl<'de> ConfirmBuilder<'de> {
    pub fn new(msg: &'de str) -> ConfirmBuilder<'de> {
        ConfirmBuilder {
            msg,
            default: true,
            theme: None,
        }
    }

    pub fn default(mut self, default: bool) -> ConfirmBuilder<'de> {
        self.default = default;
        self
    }

    pub fn build(self) -> Confirm<'de> {
        Confirm {
            msg: self.msg,
            theme: self.theme,
            default: self.default,
        }
    }
}

impl<'de> IntoEditor for ConfirmBuilder<'de> {
    type Editor = Confirm<'de>;
    fn into_editor(self) -> Self::Editor {
        self.build()
    }
}

pub struct Confirm<'de> {
    msg: &'de str,
    theme: Option<Theme>,
    default: bool,
}

impl<'de> Confirm<'de> {
    pub fn new(msg: &'de str) -> ConfirmBuilder<'de> {
        ConfirmBuilder::new(msg)
    }

    pub fn run(&self) -> Result<bool> {
        <Confirm as Editor>::run(
            self,
            &mut stdin(),
            &mut stdout(),
            self.theme.as_ref().unwrap_or(&DEFAULT_THEME),
        )
    }
}

impl<'de> Editor for Confirm<'de> {
    type Output = bool;

    fn run<R: Read, W: Write>(
        &self,
        stdin: &mut R,
        stdout: &mut W,
        theme: &Theme,
    ) -> Result<Self::Output> {
        let mut stdout = stdout.into_raw_mode()?;

        theme.print_question(
            &mut stdout,
            &self.msg,
            Some(if self.default { "Yn" } else { "yN" }),
        )?;

        let mut input = stdin.keys();

        let mut choice = self.default;

        stdout.flush()?;

        loop {
            let next = input.next().unwrap();

            match next? {
                Key::Char('\n') => {
                    // Enter
                    break;
                }
                Key::Ctrl('c') => {
                    write!(stdout, "\n\r{}", cursor::Show)?;
                    return Err(Error::UserAborted);
                }
                Key::Char('y') => {
                    choice = true;
                    break;
                }
                Key::Char('n') => {
                    choice = false;
                    break;
                }
                _ => {}
            }
        }

        theme.print_results(&mut stdout, self.msg, if choice { "yes" } else { "no" })?;

        Ok(choice)
    }
}

pub fn confirm<'de>(msg: &'de str) -> Result<bool> {
    ConfirmBuilder::new(msg).build().run()
}
