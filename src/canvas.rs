use super::Theme;
// use std::fmt::{self, Write as FmtWrite};
use std::fmt::{self, Write as FmtWrite};
use std::io::{self, stdin, stdout, Read, Write};
use strip_ansi_escapes::strip as normalize;
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
    pos: (u16, u16),
}

impl<'a, W> Canvas<'a, W>
where
    W: Write,
{
    pub fn new(w: &'a mut W, pos: (u16, u16)) -> Canvas<'a, W> {
        write!(w, "{}", cursor::Goto(pos.0, pos.1));
        Canvas {
            w,
            lines: 0,
            line: 1,
            col: 0,
            pos,
        }
    }

    pub fn print<S: fmt::Display>(&mut self, line: &S) -> Result<&mut Self, io::Error> {
        let line = line.to_string();
        write!(self.w, "{}{}", clear::AfterCursor, line)?;

        self.col += normalize(line)?.len() as u16;
        self.w.flush()?;
        Ok(self)
    }

    pub fn println(&mut self, line: &str) -> Result<&mut Self, io::Error> {
        write!(
            self.w,
            "{}{}{}\n",
            clear::AfterCursor,
            line,
            cursor::Left(normalize(line)?.len() as u16),
        )?;
        self.lines += 1;
        self.line += 1;
        self.col = 0;

        self.w.flush()?;

        Ok(self)
    }

    pub fn cursor_pos(&mut self) -> Result<(u16, u16), io::Error> {
        self.w.cursor_pos()
    }

    pub fn move_left(&mut self, a: u16) -> Result<&mut Self, io::Error> {
        if self.col < a {
            return Ok(self);
        }

        write!(self.w, "{}", cursor::Left(a));
        self.col -= a;

        Ok(self)
    }

    pub fn move_up(&mut self, c: u16) -> Result<&mut Self, io::Error> {
        if self.line == 0 {
            return Ok(self);
        }
        write!(self.w, "{}", cursor::Up(c))?;
        self.line -= 1;
        Ok(self)
    }

    pub fn move_right(&mut self, a: u16) -> Result<&mut Self, io::Error> {
        write!(self.w, "{}", cursor::Right(a));
        self.col += a;

        Ok(self)
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
        self.line += len;
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
        self.line = 1;
        self.col = 0;

        Ok(self)
    }

    pub fn clear_from(&mut self, col: u16) -> Result<&mut Self, io::Error> {
        write!(
            self.w,
            "{}{}",
            cursor::Goto(self.pos.0 + col, self.pos.1 + self.line - 1),
            clear::AfterCursor
        );
        self.col = col;

        Ok(self)
    }

    pub fn flush(&mut self) -> Result<(), io::Error> {
        self.w.flush()
    }

    pub fn col(&self) -> u16 {
        self.col
    }
}

impl<'a, W> Write for Canvas<'a, W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.print(&std::str::from_utf8(buf).unwrap())?;
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.w.flush()
    }
}
