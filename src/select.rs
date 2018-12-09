use super::choice::Choice;
use super::error::{ErrorKind, Result};
use super::theme::Theme;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, style};

pub struct SelectBuilder<'de, C, V>
where
    C: Choice<Value = V>,
{
    msg: &'de str,
    choices: &'de [C],
    page_size: usize,
    theme: Theme,
}

impl<'de, C, V> SelectBuilder<'de, C, V>
where
    C: Choice<Value = V>,
{
    pub fn new(msg: &'de str, choices: &'de [C]) -> SelectBuilder<'de, C, V> {
        SelectBuilder {
            msg,
            choices,
            page_size: 5,
            theme: Theme::default(),
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
    theme: Theme,
}

impl<'de, C, V> Select<'de, C, V>
where
    C: Choice<Value = V>,
{
    pub fn run(&self) -> Result<&'de C> {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;

        self.theme.print_question(&mut stdout, self.msg)?;
        write!(stdout, "\n{}", cursor::Hide)?;

        let rows = std::cmp::min(self.choices.len(), self.page_size);

        for _ in 0..rows - 1 {
            write!(stdout, "\n")?;
        }

        let mut cur: usize = 0;
        let mut offset: usize = 0;

        let mut input = stdin.keys();

        loop {
            print!("{}", cursor::Up(rows as u16));

            for (i, s) in self.choices.iter().skip(offset).take(rows).enumerate() {
                write!(stdout, "\n\r{}", clear::CurrentLine)?;

                if cur == i {
                    write!(stdout, "{}  > {}{}", style::Bold, s.text(), style::Reset)?;
                } else {
                    write!(stdout, "    {}", s.text())?;
                }
            }

            stdout.lock().flush()?;

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
                    return Err(ErrorKind::UserAborted.into());
                }
                _ => {
                    
                }
            }
        }

        for _ in 0..(rows + 1) {
            write!(stdout, "{}{}", clear::CurrentLine, cursor::Up(1))?;
        }

        write!(stdout, "\n\r{}", cursor::Show)?;

        self.theme.print_results(
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
