use super::choice::Choice;
use super::editor::Editor;
use super::error::{Error, Result};
use super::theme::{Theme, DEFAULT_THEME};
use std::io::{stdin, stdout, Read, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

pub struct SelectBuilder<'de, C, V>
where
    C: Choice<Value = V>,
{
    msg: &'de str,
    choices: &'de [C],
    page_size: usize,
    theme: Option<Theme>,
}

impl<'de, C, V> SelectBuilder<'de, C, V>
where
    C: Choice<Value = V>,
{
    pub fn new(msg: &'de str, choices: &'de [C]) -> SelectBuilder<'de, C, V> {
        SelectBuilder {
            msg,
            choices,
            page_size: 8,
            theme: None,
        }
    }

    pub fn page_size(mut self, size: usize) -> SelectBuilder<'de, C, V> {
        self.page_size = size;
        self
    }

    pub fn build(self) -> Select<'de, C, V> {
        Select {
            msg: self.msg,
            choices: self.choices,
            page_size: self.page_size,
            theme: self.theme,
        }
    }
}

pub struct Select<'de, C, V>
where
    C: Choice<Value = V>,
{
    msg: &'de str,
    choices: &'de [C],
    page_size: usize,
    theme: Option<Theme>,
}

impl<'de, C, V> Select<'de, C, V>
where
    C: Choice<Value = V>,
{
    pub fn new(msg: &'de str, choices: &'de [C]) -> SelectBuilder<'de, C, V> {
        SelectBuilder::new(msg, choices)
    }

    pub fn run(&self) -> Result<&'de C> {
        <Select<'de, C, V> as Editor>::run(
            self,
            &mut stdin(),
            &mut stdout(),
            self.theme.as_ref().unwrap_or(&DEFAULT_THEME),
        )
    }
}

impl<'de, C, V> Editor for Select<'de, C, V>
where
    C: Choice<Value = V>,
{
    type Output = &'de C;
    fn run<R: Read, W: Write>(
        &self,
        stdin: &mut R,
        stdout: &mut W,
        theme: &Theme,
    ) -> Result<Self::Output> {
        let mut stdout = stdout.into_raw_mode()?;

        theme.print_question(&mut stdout, self.msg, None)?;
        write!(&mut stdout, "\n{}", cursor::Hide)?;

        let rows = std::cmp::min(self.choices.len(), self.page_size);

        for _ in 0..rows - 1 {
            write!(&mut stdout, "\n")?;
        }

        let mut cur: usize = 0;
        let mut offset: usize = 0;

        let mut input = stdin.keys();

        loop {
            write!(stdout, "{}", cursor::Up(rows as u16))?;

            for (i, s) in self.choices.iter().skip(offset).take(rows).enumerate() {
                write!(&mut stdout, "\n\r{}", clear::CurrentLine)?;
                theme.print_choice(&mut stdout, s, cur == i)?;
            }

            stdout.flush()?;

            let next = input.next().unwrap();

            match next? {
                Key::Char('\n') => {
                    // Enter
                    break;
                }
                Key::Up if cur != 0 => {
                    cur -= 1;
                }
                Key::Up if cur == 0 && offset > 0 => {
                    offset -= 1;
                }
                Key::Down if cur != rows - 1 => {
                    cur += 1;
                }
                Key::Down if (cur == rows - 1) && offset < (self.choices.len() - rows) => {
                    offset += 1;
                }
                Key::Ctrl('c') => {
                    write!(stdout, "\n\r{}", cursor::Show)?;
                    return Err(Error::UserAborted);
                }
                _ => {}
            }
        }

        for _ in 0..(rows + 1) {
            write!(stdout, "{}{}", clear::CurrentLine, cursor::Up(1))?;
        }

        write!(stdout, "\n\r{}", cursor::Show)?;

        theme.print_results(
            &mut stdout,
            self.msg,
            self.choices[offset + cur].text().to_string().as_str(),
        )?;

        Ok(&self.choices[offset + cur])
    }
}

pub fn select<'de, C, V>(msg: &'de str, choices: &'de [C]) -> Result<&'de C>
where
    C: Choice<Value = V>,
{
    SelectBuilder::new(msg, choices).build().run()
}
