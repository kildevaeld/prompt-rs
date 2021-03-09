use super::choice::Choice;
use std::borrow::Cow;
use std::fmt::{self};
use std::io::{self, Write};
use std::iter::FromIterator;
use strip_ansi_escapes::strip as normalize;
use termion::{clear, color, style};

lazy_static::lazy_static! {
    pub static ref DEFAULT_THEME: Theme = Theme::new();
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Inherit,
}

impl Default for Color {
    fn default() -> Self {
        Self::Inherit
    }
}

impl Color {
    #[inline]
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "{}", color::Fg(color::Black)),
            Color::Blue => write!(f, "{}", color::Fg(color::Blue)),
            Color::Cyan => write!(f, "{}", color::Fg(color::Cyan)),
            Color::Green => write!(f, "{}", color::Fg(color::Green)),
            Color::Magenta => write!(f, "{}", color::Fg(color::Magenta)),
            Color::Red => write!(f, "{}", color::Fg(color::Red)),
            Color::White => write!(f, "{}", color::Fg(color::White)),
            Color::Yellow => write!(f, "{}", color::Fg(color::Yellow)),
            Color::LightBlack => write!(f, "{}", color::Fg(color::LightBlack)),
            Color::LightBlue => write!(f, "{}", color::Fg(color::LightBlue)),
            Color::LightCyan => write!(f, "{}", color::Fg(color::LightCyan)),
            Color::LightGreen => write!(f, "{}", color::Fg(color::LightGreen)),
            Color::LightMagenta => write!(f, "{}", color::Fg(color::LightMagenta)),
            Color::LightRed => write!(f, "{}", color::Fg(color::LightRed)),
            Color::LightWhite => write!(f, "{}", color::Fg(color::LightWhite)),
            Color::LightYellow => write!(f, "{}", color::Fg(color::LightYellow)),
            Color::Inherit => Ok(()),
        }
    }

    #[inline]
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "{}", color::Bg(color::Black)),
            Color::Blue => write!(f, "{}", color::Bg(color::Blue)),
            Color::Cyan => write!(f, "{}", color::Bg(color::Cyan)),
            Color::Green => write!(f, "{}", color::Bg(color::Green)),
            Color::Magenta => write!(f, "{}", color::Bg(color::Magenta)),
            Color::Red => write!(f, "{}", color::Bg(color::Red)),
            Color::White => write!(f, "{}", color::Bg(color::White)),
            Color::Yellow => write!(f, "{}", color::Bg(color::Yellow)),
            Color::LightBlack => write!(f, "{}", color::Bg(color::LightBlack)),
            Color::LightBlue => write!(f, "{}", color::Bg(color::LightBlue)),
            Color::LightCyan => write!(f, "{}", color::Bg(color::LightCyan)),
            Color::LightGreen => write!(f, "{}", color::Bg(color::LightGreen)),
            Color::LightMagenta => write!(f, "{}", color::Bg(color::LightMagenta)),
            Color::LightRed => write!(f, "{}", color::Bg(color::LightRed)),
            Color::LightWhite => write!(f, "{}", color::Bg(color::LightWhite)),
            Color::LightYellow => write!(f, "{}", color::Bg(color::LightYellow)),
            Color::Inherit => Ok(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Decoration {
    Bold,
    Faint,
    Italic,
    Underline,
    Blink,
    Invert,
    CrossedOut,
    Inherit,
}

impl fmt::Display for Decoration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Decoration::Bold => write!(f, "{}", style::Bold),
            Decoration::Faint => write!(f, "{}", style::Faint),
            Decoration::Italic => write!(f, "{}", style::Italic),
            Decoration::Underline => write!(f, "{}", style::Underline),
            Decoration::Blink => write!(f, "{}", style::Blink),
            Decoration::Invert => write!(f, "{}", style::Invert),
            Decoration::CrossedOut => write!(f, "{}", style::CrossedOut),
            Decoration::Inherit => Ok(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Indicator {
    active: String,
    inactive: String,
}

pub struct StyledString<'a> {
    string: Cow<'a, str>,
    style: Style,
}

impl<'a> StyledString<'a> {
    pub fn new(style: Style, string: impl Into<Cow<'a, str>>) -> StyledString<'a> {
        StyledString {
            string: string.into(),
            style,
        }
    }
}

impl<'a> fmt::Display for StyledString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.style, self.string, style::Reset)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Foreground {
    color: Color,
    decoration: Decoration,
}

impl Foreground {
    pub fn color(mut self, color: impl Into<Color>) -> Foreground {
        self.color = color.into();
        self
    }

    pub fn decoration(mut self, decoration: impl Into<Decoration>) -> Foreground {
        self.decoration = decoration.into();
        self
    }
}

impl Default for Foreground {
    fn default() -> Self {
        Foreground {
            color: Color::Inherit,
            decoration: Decoration::Inherit,
        }
    }
}

impl fmt::Display for Foreground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.color.write_fg(f)?;
        <Decoration as fmt::Display>::fmt(&self.decoration, f)
    }
}

impl From<Color> for Foreground {
    fn from(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }
}

impl From<Decoration> for Foreground {
    fn from(decoration: Decoration) -> Self {
        Self {
            decoration,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Style {
    fg: Foreground,
    bg: Color,
}

impl Style {
    pub fn bg(mut self, color: impl Into<Color>) -> Style {
        self.bg = color.into();
        self
    }

    pub fn fg(mut self, style: impl Into<Foreground>) -> Style {
        self.fg = style.into();
        self
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Foreground as fmt::Display>::fmt(&self.fg, f)?;
        self.bg.write_bg(f)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    prefix: Option<String>,
    prefix_style: Style,
    separator: String,
    default: Style,
    highlight: Style,
    selected: Style,
    result: Style,
    highlight_indicator: Indicator,
    selected_indicator: Indicator,
}

impl Theme {
    pub fn new() -> Theme {
        ThemeBuilder::default().build()
    }

    pub fn builder<'a>(&'a self) -> LineBuilder<'a> {
        LineBuilder::new(self)
    }

    pub fn print_question(
        &self,
        writer: &mut dyn Write,
        msg: &str,
        default: Option<&str>,
    ) -> Result<usize, io::Error> {
        let mut line = LineBuilder::new(self)
            .plain("\r")
            .plain(clear::CurrentLine.as_ref())
            .prefix()
            .plain(msg);
        if let Some(default) = default {
            line = line.styled(
                Style::default().fg(Color::LightBlue),
                format!("[{}]", default),
            );
        }

        let line = line.plain(&self.separator).to_string();
        write!(writer, "{}", &line)?;

        Ok(normalize(line)?.len())
    }

    pub fn print_results(
        &self,
        writer: &mut dyn Write,
        msg: &str,
        ans: &str,
    ) -> Result<&Theme, io::Error> {
        let line = LineBuilder::new(self)
            .plain(clear::CurrentLine.as_ref())
            .plain("\r")
            .prefix()
            .plain(msg)
            .plain(&self.separator)
            .result(ans);
        writeln!(writer, "{}\r", line)?;
        Ok(self)
    }

    pub fn print_choice<R: Write, C: Choice>(
        &self,
        output: &mut R,
        choice: &C,
        highlighted: bool,
    ) -> Result<&Theme, io::Error> {
        let prefix = normalize(self.prefix.as_ref().map(|m| m.as_str()).unwrap_or(""))?;
        let space = String::from_iter((0..prefix.len()).map(|_| ' '));
        let choice = choice.text().to_string();
        let mut line = self
            .builder()
            .plain(clear::CurrentLine.as_ref())
            .plain(&space)
            .highlight_indicator(highlighted)
            .plain(" ");

        line = if highlighted {
            line.highlight(&choice)
        } else {
            line.plain(&choice)
        };

        write!(output, "{}", line)?;

        Ok(self)
    }

    pub fn print_error(&self, output: &mut dyn Write, error: &str) -> Result<(), io::Error> {
        let line = self
            .builder()
            .plain(clear::CurrentLine.as_ref())
            .styled(Style::default().fg(Color::Red), "!")
            .plain(" ")
            .plain(error);
        write!(output, "{}", line)
    }

    pub fn print_multiple_choice<R: Write, C: Choice>(
        &self,
        output: &mut R,
        choice: &C,
        highlighted: bool,
        selected: bool,
    ) -> Result<&Theme, io::Error> {
        let prefix = normalize(self.prefix.as_ref().map(|m| m.as_str()).unwrap_or(""))?;
        let space = String::from_iter((0..prefix.len()).map(|_| ' '));
        let choice = choice.text().to_string();
        let mut line = self
            .builder()
            .plain(clear::CurrentLine.as_ref())
            .plain(&space)
            .highlight_indicator(highlighted)
            .plain(" ")
            .selected_indicator(selected)
            .plain(" ");

        line = if highlighted && !selected {
            line.highlight(&choice)
        } else if selected {
            line.styled(self.selected, &choice)
        } else {
            line.plain(&choice)
        };

        write!(output, "{}", line)?;

        Ok(self)
    }
}

#[derive(Clone, PartialEq)]
pub struct ThemeBuilder {
    prefix: Option<String>,
    separator: Option<String>,
    prefix_style: Style,
    default: Style,
    highlight: Style,
    selected: Style,
    result: Style,
    highlight_indicator: Option<Indicator>,
    selected_indicator: Option<Indicator>,
}

impl Default for ThemeBuilder {
    fn default() -> Self {
        Self {
            prefix: None,
            separator: None,
            prefix_style: Style::default().fg(Color::Green),
            default: Style::default(),
            result: Style::default().fg(Color::Cyan),
            highlight: Style::default().fg(Color::Cyan),
            selected: Style::default().fg(Color::Green),
            highlight_indicator: None,
            selected_indicator: None,
        }
    }
}

impl ThemeBuilder {
    pub fn prefix(mut self, prefix: impl ToString) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    pub fn separator(mut self, sep: impl ToString) -> Self {
        self.separator = Some(sep.to_string());
        self
    }

    pub fn prefix_style(mut self, prefix: impl Into<Style>) -> Self {
        self.prefix_style = prefix.into();
        self
    }

    pub fn base(mut self, default: impl Into<Style>) -> Self {
        self.default = default.into();
        self
    }

    pub fn highlight(mut self, style: impl Into<Style>) -> Self {
        self.highlight = style.into();
        self
    }

    pub fn selected(mut self, style: impl Into<Style>) -> Self {
        self.selected = style.into();
        self
    }

    pub fn result(mut self, style: impl Into<Style>) -> Self {
        self.result = style.into();
        self
    }

    pub fn highlight_indicator(mut self, indicator: impl Into<Indicator>) -> Self {
        self.highlight_indicator = Some(indicator.into());
        self
    }

    pub fn selected_indicator(mut self, indicator: impl Into<Indicator>) -> Self {
        self.selected_indicator = Some(indicator.into());
        self
    }
}

impl ThemeBuilder {
    pub fn build(self) -> Theme {
        Theme {
            prefix: Some(self.prefix.unwrap_or_else(|| String::from("? "))),
            prefix_style: self.prefix_style,
            separator: self.separator.unwrap_or_else(|| String::from(" ")), //.unwrap_or_else(|| String::from("? ")),
            default: self.default,
            highlight: self.highlight,
            selected: self.selected,
            result: self.result,
            highlight_indicator: self.highlight_indicator.unwrap_or_else(|| Indicator {
                active: "❯".to_string(),
                inactive: " ".to_string(),
            }),
            selected_indicator: self.selected_indicator.unwrap_or_else(|| Indicator {
                active: "◉".to_string(),
                inactive: "◯".to_string(),
            }),
        }
    }
}

pub struct LineBuilder<'a> {
    theme: &'a Theme,
    writer: Vec<StyledString<'a>>,
}

impl<'a> LineBuilder<'a> {
    pub fn new(theme: &'a Theme) -> LineBuilder<'a> {
        LineBuilder {
            theme,
            writer: Vec::new(),
        }
    }

    pub fn selected_indicator(mut self, on: bool) -> Self {
        self.writer.push(StyledString::new(
            if on {
                self.theme.selected
            } else {
                self.theme.default
            },
            Cow::Borrowed(if on {
                self.theme.selected_indicator.active.as_str()
            } else {
                self.theme.selected_indicator.inactive.as_str()
            }),
        ));
        self
    }

    pub fn highlight_indicator(mut self, on: bool) -> Self {
        self.writer.push(StyledString::new(
            if on {
                self.theme.highlight
            } else {
                self.theme.default
            },
            if on {
                self.theme.highlight_indicator.active.as_str()
            } else {
                self.theme.highlight_indicator.inactive.as_str()
            },
        ));
        self
    }

    pub fn highlight(mut self, msg: &'a str) -> Self {
        self.writer
            .push(StyledString::new(self.theme.highlight, msg));
        self
    }

    pub fn result(mut self, msg: &'a str) -> Self {
        self.writer.push(StyledString::new(self.theme.result, msg));
        self
    }

    pub fn plain(mut self, msg: &'a str) -> Self {
        self.writer.push(StyledString::new(self.theme.default, msg));
        self
    }

    pub fn styled(mut self, style: Style, msg: impl Into<Cow<'a, str>>) -> Self {
        self.writer.push(StyledString::new(style, msg.into()));
        self
    }

    pub fn prefix(mut self) -> Self {
        if let Some(prefix) = &self.theme.prefix {
            self.writer
                .push(StyledString::new(self.theme.prefix_style, prefix))
        }
        self
    }
}

impl<'a> fmt::Display for LineBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.writer {
            <StyledString<'a> as fmt::Display>::fmt(i, f)?;
        }
        Ok(())
    }
}
