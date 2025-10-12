/// Font import behavior settings.
///
/// This struct specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
///
/// # Example
///
/// ```no_run
/// use std::io;
/// use std::io::prelude::*;
/// use std::fs;
///
/// fn main() -> bmfont_rs::Result<()> {
///     let src = fs::read_to_string("font.txt")?;
///     let settings = bmfont_rs::LoadSettings::default().ignore_counts();
///     let font = bmfont_rs::text::from_str_ext(&src, &settings)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct LoadSettings {
    /// Allow String control characters.
    pub allow_string_control_characters: bool,
    /// Ignore incorrect character and kerning counts.
    pub ignore_counts: bool,
    /// Ignore invalid tags.
    pub ignore_invalid_tags: bool,
}

impl LoadSettings {
    // As we have exhaustive fields, we'll rely on builder type setter functions as opposed to
    // a 'new' function. This should enable us to add additional fields without breaking the
    // existing API.

    /// Set ignore_counts to true. Returns self.
    pub fn ignore_counts(mut self) -> Self {
        self.ignore_counts = true;
        self
    }

    /// Set ignore_invalid_tags to true. Returns self.
    pub fn ignore_invalid_tags(mut self) -> Self {
        self.ignore_invalid_tags = true;
        self
    }

    /// Set allow_string_control_characters to true. Returns self.
    pub fn allow_string_control_characters(mut self) -> Self {
        self.allow_string_control_characters = true;
        self
    }
}
