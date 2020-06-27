use super::error::Result;
use super::theme::Theme;
use std::io::{Read, Write};

pub trait Editor {
    type Output;

    fn run<R: Read, W: Write>(
        &self,
        stdin: &mut R,
        stdout: &mut W,
        theme: &Theme,
    ) -> Result<Self::Output>;
}

pub trait IntoEditor {
    type Editor: Editor;
    fn into_editor(self) -> Self::Editor;
}

impl<T> IntoEditor for T
where
    T: Editor,
{
    type Editor = T;
    fn into_editor(self) -> Self::Editor {
        self
    }
}

// pub struct EditorPrinter<'a> {
//     inline: bool,
//     label: &'a str,

// }
