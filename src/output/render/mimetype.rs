use ansi_term::Style;

use crate::fs::fields as f;
use crate::output::cell::TextCell;

impl f::MimeType {
    pub fn render(self) -> TextCell {
        let style = Style::new();

        TextCell::paint(style, self.mime_type.to_string())
    }
}
