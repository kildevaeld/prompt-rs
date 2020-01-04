use std::fmt;

/// An option the user can choose
///
/// (Since the name "Option" is reserved for the well-known type representing
/// nullability, we are calling this one "Choice".)
pub trait Choice {
    /// User visible text
    type Text: fmt::Display;
    /// Internal value representing this choice
    type Value;

    /// Get a reference to the text
    fn text(&self) -> &Self::Text;

    /// Get a reference to the value of this choice
    fn value(&self) -> &Self::Value;
}

impl<'a> Choice for &'a str {
    type Text = &'a str;
    type Value = &'a str;

    fn text(&self) -> &Self::Text {
        self
    }

    fn value(&self) -> &Self::Value {
        self
    }
}

impl Choice for String {
    type Text = String;
    type Value = String;

    fn text(&self) -> &Self::Text {
        self
    }

    fn value(&self) -> &Self::Value {
        self
    }
}

impl<'a, T, V> Choice for (T, V)
where
    T: fmt::Display,
    
{
    type Text = T;
    type Value = V;

    fn text(&self) -> &T {
        &self.0
    }

    fn value(&self) -> &V {
        &self.1
    }
}

// impl<'a, T, V> Display for Choice<(T, V)
// where
//     T: Display,
// {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0 as T)
//     }
// }
