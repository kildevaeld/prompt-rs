use super::canvas::Canvas;
use super::error::Error;
use super::theme::{Color, Style, StyledString};
use super::util;
// use super::validator::Validator;
use std::fmt::Write as FmtWrite;
use std::io::{self, stdin, stdout, Read, Write};
use std::iter::FromIterator;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{
    clear,
    cursor::{self, DetectCursorPos},
};

pub struct Input2<'a> {
    echo: bool,
    mask: Option<&'a str>,
    style: Style,
}

impl<'a> Input2<'a> {
    pub fn new(echo: bool, mask: Option<&'a str>) -> Input2<'a> {
        Input2 {
            echo,
            mask,
            style: Style::default(),
        }
    }
}

impl<'a> Input2<'a> {
    pub fn run<W, R>(&self, input: R, canvas: &mut Canvas<'_, W>) -> Result<String, Error>
    where
        W: Write,
        R: Read,
    {
        let mut input = input.keys();

        let mut buffer = String::new();

        let start_col = canvas.col();

        let print_buffer = |buffer: &mut String, canvas: &mut Canvas<'_, W>| {
            if let Some(mask) = self.mask {
                let s = String::from_iter((0..buffer.len()).map(|_| mask));
                let ss = StyledString::new(self.style, s);
                canvas.clear_from(start_col)?.print(&ss)?;
            } else if self.echo {
                let ss = StyledString::new(self.style, buffer.as_str());
                canvas.clear_from(start_col)?.print(&ss)?;
            }

            Result::<_, Error>::Ok(())
        };

        loop {
            canvas.flush()?;

            let next = input.next().unwrap();
            match next? {
                Key::Char('\n') => {
                    // Enter
                    break;
                }
                Key::Ctrl('c') => {
                    return Err(Error::UserAborted);
                }
                Key::Left => {
                    if canvas.col() > start_col {
                        canvas.move_left(1)?;
                    }
                }
                Key::Right => {
                    if (canvas.col() - start_col) < buffer.len() as u16 {
                        canvas.move_right(1)?;
                    }
                }
                Key::Backspace => {
                    let col = canvas.col() - start_col;
                    if col == buffer.len() as u16 {
                        buffer.pop();
                    } else if col == 0 {
                        continue;
                    } else {
                        buffer.remove((col - 1) as usize);
                    }
                    print_buffer(&mut buffer, canvas)?;
                    let newcol = canvas.col();
                    if newcol > (col + start_col) {
                        canvas.move_left(newcol - col + 1)?;
                    }
                }
                Key::Char(c) => {
                    if util::match_char(&c) {
                        continue;
                    }

                    let col = canvas.col();

                    if (col - start_col) == buffer.len() as u16 {
                        buffer.push(c);
                    } else {
                        buffer.insert((col - start_col) as usize, c);
                    }

                    print_buffer(&mut buffer, canvas)?;

                    let newcol = canvas.col();
                    if newcol > (col + 1) {
                        canvas.move_left(newcol - col - 1)?;
                    }
                }
                _ => {}
            }
        }

        Ok(buffer)
    }
}
