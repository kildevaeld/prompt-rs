use super::Theme;
// use std::fmt::{self, Write as FmtWrite};
use std::io::{self, stdin, stdout, Read, Write};
use termion::{
    clear,
    cursor::{self, DetectCursorPos},
    input::TermRead,
};

pub struct Canvas<'a, W> {
    w: &'a mut W,
    lines: u16,
    line: u16,
    col: u16,
    theme: &'a Theme,
    pos: (u16, u16),
}

impl<'a, W> Canvas<'a, W>
where
    W: Write,
{
    pub fn new(w: &'a mut W, theme: &'a Theme, pos: (u16, u16)) -> Canvas<'a, W> {
        write!(w, "{}", cursor::Goto(pos.0, pos.1));
        Canvas {
            w,
            theme,
            lines: 0,
            line: 0,
            col: 0,
            pos,
        }
    }

    pub fn print(&mut self, line: &str) -> Result<&mut Self, io::Error> {
        write!(self.w, "{}{}", clear::AfterCursor, line)?;
        self.col += line.len() as u16;
        self.w.flush()?;
        Ok(self)
    }

    pub fn println(&mut self, line: &str) -> Result<&mut Self, io::Error> {
        write!(
            self.w,
            "{}{}{}\n",
            clear::AfterCursor,
            line,
            cursor::Left(line.len() as u16),
        )?;
        self.lines += 1;
        self.col = 0;

        self.w.flush()?;

        Ok(self)
    }

    pub fn cursor_pos(&mut self) -> Result<(u16, u16), io::Error> {
        self.w.cursor_pos()
    }

    pub fn print_lines(&mut self, lines: &[&str]) -> Result<&mut Self, io::Error> {
        let len = lines.len() as u16;
        for line in lines {
            write!(
                self.w,
                "{}{}{}\n",
                clear::AfterCursor,
                line,
                cursor::Left(line.len() as u16 + self.col),
            )?;
            self.col = 0;
        }

        self.lines += len;
        //self.col = 0;

        self.w.flush()?;

        Ok(self)
    }

    pub fn clear(&mut self) -> Result<&mut Self, io::Error> {
        write!(
            self.w,
            "{}",
            cursor::Goto(self.pos.0, self.pos.1 + self.lines)
        );

        for _ in 0..self.lines {
            write!(self.w, "{}{}", clear::CurrentLine, cursor::Up(1))?;
        }

        self.lines = 0;
        self.col = 0;

        Ok(self)
    }
}
