use std::borrow::Cow;
use std::str::FromStr;

/// Used for sorting languages.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Sort {
    /// Sort by number blank lines.
    Blanks,
    /// Sort by number comments lines.
    Comments,
    /// Sort by number code lines.
    Code,
    /// Sort by number files lines.
    Files,
    /// Sort by number of lines.
    Lines,
}

impl FromStr for Sort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "blanks" => Sort::Blanks,
            "comments" => Sort::Comments,
            "code" => Sort::Code,
            "files" => Sort::Files,
            "lines" => Sort::Lines,
            s => return Err(format!("Unsupported sorting option: {}", s))
        })
    }
}

impl<'a> From<Sort> for Cow<'a, Sort> {
    fn from(from: Sort) -> Self {
        Cow::Owned(from)
    }
}

impl<'a> From<&'a Sort> for Cow<'a, Sort> {
    fn from(from: &'a Sort) -> Self {
        Cow::Borrowed(from)
    }
}
