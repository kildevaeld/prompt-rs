use super::editor::{Editor, IntoEditor};
use super::error::{Error, Result};
use super::theme::{Theme, DEFAULT_THEME};
use std::io::{stdin, stdout, Read, Write};
use termion::input::TermRead;
use termion::{
    clear,
    cursor::{self},
};
use valid::{self, Required, Valid, Validation};

pub type Validator = Valid<Box<dyn Validation<String>>, String>;

pub struct InputBuilder<'a> {
    msg: &'a str,
    default: Option<&'a str>,
    theme: Option<Theme>,
    validations: Option<Valid<Box<dyn Validation<String>>, String>>,
}

impl<'a> InputBuilder<'a> {
    pub fn new(msg: &'a str) -> InputBuilder<'a> {
        InputBuilder {
            msg,
            theme: None,
            default: None,
            validations: None,
        }
    }

    pub fn validate<V: Validation<String> + 'static>(mut self, v: V) -> Self {
        if let Some(valid) = self.validations.take() {
            self.validations = Some(valid.and_boxed(v));
        } else {
            self.validations = Some(Valid::new(Box::new(v)));
        }
        self
    }

    pub fn required(mut self) -> Self {
        if let Some(v) = self.validations.take() {
            self.validations = Some(v.and_boxed(valid::MinLen(1)));
        } else {
            self.validations = Some(Valid::new(Box::new(valid::MinLen(1))));
        }
        self
    }

    pub fn theme(mut self, theme: Theme) -> InputBuilder<'a> {
        self.theme = Some(theme);
        self
    }

    pub fn default(mut self, name: &'a str) -> InputBuilder<'a> {
        self.default = Some(name);
        self
    }

    pub fn build(self) -> Input<'a> {
        Input {
            msg: self.msg,
            theme: self.theme,
            default: self.default,
            validations: self.validations,
        }
    }
}

impl<'de> IntoEditor for InputBuilder<'de> {
    type Editor = Input<'de>;
    fn into_editor(self) -> Self::Editor {
        self.build()
    }
}

pub struct Input<'a> {
    msg: &'a str,
    theme: Option<Theme>,
    default: Option<&'a str>,
    validations: Option<Valid<Box<dyn Validation<String>>, String>>,
}

impl<'a> Input<'a> {
    pub fn new(msg: &'a str) -> InputBuilder<'a> {
        InputBuilder::new(msg)
    }

    pub fn run(&self) -> Result<String> {
        <Input as Editor>::run(
            self,
            &mut stdin(),
            &mut stdout(),
            self.theme.as_ref().unwrap_or(&DEFAULT_THEME),
        )
    }
}

impl<'a> Editor for Input<'a> {
    type Output = String;
    fn run<R: Read, W: Write>(
        &self,
        stdin: &mut R,
        stdout: &mut W,
        theme: &Theme,
    ) -> Result<Self::Output> {
        //let mut input = String::default();

        let mut error: Option<String> = None;

        let input = 'ui: loop {
            let w = theme.print_question(stdout, self.msg, self.default)?;
            if let Some(error) = &error {
                let dir = if error.len() < w {
                    cursor::Right((w - error.len() - 2) as u16).to_string()
                } else {
                    cursor::Left((error.len() - w + 2) as u16).to_string()
                };
                write!(stdout, "\r\n")?;
                theme.print_error(stdout, &error)?;
                write!(stdout, "{}{}", cursor::Up(1), dir)?;
            } else {
                write!(stdout, "\n{}{}", cursor::Up(1), cursor::Right(w as u16))?;
            }
            stdout.flush()?;

            let mut input = match TermRead::read_line(stdin) {
                Ok(Some(s)) => s,
                Ok(None) => return Err(Error::NoMoreInput),
                Err(e) => return Err(Error::IoError(e)),
            };
            if input.is_empty() {
                if let Some(default) = self.default {
                    input = default.to_owned();
                    break 'ui input;
                }
            }

            error = if let Some(v) = &self.validations {
                if let Err(err) = v.validate(&input) {
                    Some(err.to_string())
                } else {
                    None
                }
            } else {
                None
            };

            if error.is_none() {
                break 'ui input;
            }

            write!(stdout, "\r{}", cursor::Up(1));
        };
        if error.is_some() {
            write!(stdout, "{}", clear::CurrentLine)?;
        }
        write!(stdout, "{}", cursor::Up(1))?;
        theme.print_results(stdout, self.msg, &input)?;
        Ok(input)
    }
}

pub fn input(msg: &str) -> Result<String> {
    InputBuilder::new(msg).build().run()
}
