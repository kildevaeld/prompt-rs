use super::choice::Choice;
use super::editor::Editor;
use super::error::{Error, Result};
use super::theme::{Theme, DEFAULT_THEME};
use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

pub struct MultiSelectBuilder<'de, C, V>
where
    C: Choice<Value = V>,
{
    msg: &'de str,
    choices: &'de [C],
    page_size: usize,
    theme: Option<Theme>,
    min: usize,
    max: usize,
}

impl<'de, C, V> MultiSelectBuilder<'de, C, V>
where
    C: Choice<Value = V>,
{
    pub fn new(msg: &'de str, choices: &'de [C]) -> MultiSelectBuilder<'de, C, V> {
        MultiSelectBuilder {
            msg,
            choices,
            page_size: 8,
            theme: None,
            min: 0,
            max: 0,
        }
    }

    pub fn page_size(mut self, size: usize) -> MultiSelectBuilder<'de, C, V> {
        self.page_size = size;
        self
    }

    pub fn max(mut self, max: usize) -> MultiSelectBuilder<'de, C, V> {
        self.max = max;
        self
    }

    pub fn min(mut self, min: usize) -> MultiSelectBuilder<'de, C, V> {
        self.min = min;
        self
    }

    pub fn build(self) -> MultiSelect<'de, C, V> {
        MultiSelect {
            msg: self.msg,
            choices: self.choices,
            page_size: self.page_size,
            theme: self.theme,
            min: self.min,
            max: self.max,
        }
    }
}

pub struct MultiSelect<'de, C, V>
where
    C: Choice<Value = V>,
{
    msg: &'de str,
    choices: &'de [C],
    page_size: usize,
    theme: Option<Theme>,
    min: usize,
    max: usize,
}

impl<'de, C, V> MultiSelect<'de, C, V>
where
    C: Choice<Value = V>,
{
    pub fn new(msg: &'de str, choices: &'de [C]) -> MultiSelectBuilder<'de, C, V> {
        MultiSelectBuilder::new(msg, choices)
    }

    pub fn run(&self) -> Result<Vec<&'de C>> {
        <MultiSelect<'de, C, V> as Editor>::run(
            self,
            &mut stdin(),
            &mut stdout(),
            self.theme.as_ref().unwrap_or(&DEFAULT_THEME),
        )
    }
}

impl<'de, C, V> Editor for MultiSelect<'de, C, V>
where
    C: Choice<Value = V>,
{
    type Output = Vec<&'de C>;
    fn run<R: Read, W: Write>(
        &self,
        stdin: &mut R,
        stdout: &mut W,
        theme: &Theme,
    ) -> Result<Self::Output> {
        let mut stdout = stdout.into_raw_mode()?;

        theme.print_question(&mut stdout, self.msg)?;
        write!(stdout, "\n{}", cursor::Hide)?;

        let rows = std::cmp::min(self.choices.len(), self.page_size);

        for _ in 0..rows - 1 {
            write!(stdout, "\n")?;
        }

        let mut cur: usize = 0;
        let mut offset: usize = 0;

        let mut input = stdin.keys();

        let mut choices: HashMap<usize, &C> = HashMap::default();

        loop {
            write!(stdout, "{}", cursor::Up((rows + 0) as u16))?;
            let cur_idx = offset + cur;

            for (i, s) in self.choices.iter().skip(offset).take(rows).enumerate() {
                write!(stdout, "\n\r{}", clear::CurrentLine)?;
                theme.print_multiple_choice(
                    &mut stdout,
                    s,
                    cur == i,
                    choices.contains_key(&(offset + i)),
                )?;
            }

            stdout.flush()?;

            let next = input.next().unwrap();

            match next? {
                // Enter
                Key::Char('\n') => {
                    if choices.len() >= self.min {
                        break;
                    }
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
                Key::Char(' ') => {
                    if choices.contains_key(&cur_idx) {
                        choices.remove(&cur_idx);
                    } else if self.max == 0 || choices.len() < self.max {
                        choices.insert(cur_idx, &self.choices[cur_idx]);
                    }
                }
                Key::Ctrl('c') => {
                    write!(stdout, "\n\r{}", cursor::Show)?;
                    return Err(Error::UserAborted);
                }
                _ => {}
            }
        }
        // Clears choices
        for _ in 0..(rows + 1) {
            write!(stdout, "{}{}", clear::CurrentLine, cursor::Up(1))?;
        }

        write!(stdout, "\n\r{}", cursor::Show)?;

        let mut choices = choices.iter().map(|m| *m.0).collect::<Vec<_>>();
        choices.sort();

        theme.print_results(
            &mut stdout,
            self.msg,
            choices
                .iter()
                .map(|m| self.choices[*m].text().to_string())
                .collect::<Vec<_>>()
                .join(", ")
                .as_str(), //.text().to_string().as_str(),
        )?;

        let choices = choices
            .iter()
            .map(|m| &self.choices[*m])
            .collect::<Vec<_>>();

        Ok(choices)
    }
}

pub fn multi_select<'de, C, V>(msg: &'de str, choices: &'de [C]) -> Result<Vec<&'de C>>
where
    C: Choice<Value = V>,
{
    MultiSelectBuilder::new(msg, choices).build().run()
}
