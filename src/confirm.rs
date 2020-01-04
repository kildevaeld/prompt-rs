use super::error::{ErrorKind, Result};
use super::theme::{Color, Theme};
use std::io::{stdin, stdout, Write};
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct ConfirmBuilder<'de> {
    msg: &'de str,
    theme: Theme,
    default: bool,
}

impl<'de> ConfirmBuilder<'de> {
    pub fn new(msg: &'de str) -> ConfirmBuilder<'de> {
        ConfirmBuilder {
            msg,
            default: true,
            theme: Theme::default(),
        }
    }

    pub fn build(self) -> Confirm<'de> {
        Confirm {
            msg: self.msg,
            theme: self.theme,
            default: self.default,
        }
    }
}

pub struct Confirm<'de> {
    msg: &'de str,
    theme: Theme,
    default: bool,
}

impl<'de> Confirm<'de> {
    pub fn run(&self) -> Result<bool> {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;

        let msg = format!(
            "{} {}",
            self.msg,
            Color::Magenta.wrap(if self.default { "[Yn]" } else { "[yN]" })
        );

        self.theme.print_question(&mut stdout, &msg)?;

        let mut input = stdin.keys();

        let mut choice = self.default;

        stdout.lock().flush();

        loop {
            let next = input.next().unwrap();

            match next? {
                Key::Char('\n') => {
                    // Enter
                    break;
                }
                Key::Ctrl('c') => {
                    write!(stdout, "\n\r{}", cursor::Show)?;
                    return Err(ErrorKind::UserAborted.into());
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

        self.theme
            .print_results(&mut stdout, self.msg, if choice { "yes" } else { "no" })?;

        Ok(choice)
    }
}

pub fn confirm<'de>(msg: &'de str) -> Result<bool> {
    ConfirmBuilder::new(msg).build().run()
}
