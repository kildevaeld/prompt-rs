use super::error::Result;
use super::theme::{Theme, DEFAULT_THEME};
use super::{Editor, IntoEditor};
use std::io::{stdin, stdout, Read, Stdin, Stdout, Write};

pub struct StdoutWrap(Stdout);

impl Write for StdoutWrap {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.lock().flush()
    }
}

pub struct Form<R, W> {
    theme: Theme,
    stdin: R,
    stdout: W,
}

impl<R: Read, W: Write> Form<R, W> {
    pub fn new(stdin: R, stdout: W, theme: Theme) -> Form<R, W> {
        Form {
            stdin,
            stdout,
            theme,
        }
    }

    pub fn run<E: IntoEditor>(&mut self, editor: E) -> Result<<E::Editor as Editor>::Output> {
        editor
            .into_editor()
            .run(&mut self.stdin, &mut self.stdout, &self.theme)
    }
}

impl Default for Form<Stdin, StdoutWrap> {
    fn default() -> Self {
        Form {
            theme: DEFAULT_THEME.clone(),
            stdin: stdin(),
            stdout: StdoutWrap(stdout()),
        }
    }
}

pub struct FormBuilder {}
