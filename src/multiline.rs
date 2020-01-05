use super::choice::Choice;
use super::editor::Editor;
use super::error::{ErrorKind, Result};
use super::theme::{Theme, DEFAULT_THEME};
use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor::{self, DetectCursorPos}};

pub struct Multiline<'de> {
    msg: &'de str,
    height: usize
}

impl<'de> Multiline<'de> {

    pub fn new(msg: &'de str) -> Multiline<'de> {
        Multiline {
            msg, height: 8
        }
    }

    pub fn run(&self) -> Result<String> {
        let theme = Theme::default();
        let mut stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;

       

        theme.print_question(&mut stdout, self.msg)?;
        
        let (_, y) = stdout.cursor_pos()?;

        for _ in 0..self.height {
            write!(&mut stdout, "\n")?;
        }
    

        let base_y = y - self.height as u16 + 1;
        let mut scroll_offset = 0;
        let mut cur_x = 1;
        let mut cur_y = base_y;


        let mut buffer: Vec<String> = vec![String::new()];

        let mut input = stdin.keys();

        

        loop {

            //write!(stdout, "{}", cursor::Up(cur_y as u16 - base_y as u16))?;
            write!(stdout, "{}", cursor::Goto(1, base_y))?;
            for (i, s) in buffer.iter().skip(scroll_offset).take(self.height).enumerate() {
                write!(&mut stdout, "\n\r{}", clear::CurrentLine)?;
                write!(&mut stdout, "{}", s)?;
            }

            write!(stdout, "{}", cursor::Goto(cur_x, cur_y))?;

            stdout.flush()?;

            let next = input.next().unwrap();

            match next? {
                Key::Char('\n') => {
                    // Enter
                    buffer.push(String::new());
                    cur_y += 1;
                    cur_x = 1;
                    //break;
                }
                Key::Up if cur_y > base_y => {
                    cur_y -= 1;
                }
                Key::Up if cur_y == base_y && scroll_offset > 0 => {
                    scroll_offset -= 1;
                }
                Key::Down if cur_y < self.height as u16 => {
                    cur_y += 1;
                }
                Key::Down if cur_y == self.height as u16 && scroll_offset < self.height => {
                    scroll_offset += 1;
                }
                Key::Esc => {
                    break;
                }
                // Key::Backspace => {

                // }
                Key::Ctrl('c') => {
                    write!(stdout, "\n\r{}", cursor::Show)?;
                    return Err(ErrorKind::UserAborted.into());
                }
                Key::Char(c) => {
                    buffer.last_mut().unwrap().push(c);
                    cur_x += 1;
                }
                _ => {}
            }

        }


        Ok("".to_string())
    }
}