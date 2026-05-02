//! Small Markdown builder used by layouts.

/// Fluent Markdown builder.
///
/// TODO(Phase 2.c): grow helpers for tables, collapsible `<details>` blocks,
///                  and code fences with language hints.
#[derive(Default)]
pub struct MarkdownBuilder {
    buf: String,
}

impl MarkdownBuilder {
    /// Create an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a heading of the given level (1-6).
    pub fn heading(&mut self, level: u8, text: &str) -> &mut Self {
        let level = level.clamp(1, 6) as usize;
        self.buf.push_str(&"#".repeat(level));
        self.buf.push(' ');
        self.buf.push_str(text);
        self.buf.push_str("\n\n");
        self
    }

    /// Append a paragraph.
    pub fn paragraph(&mut self, text: &str) -> &mut Self {
        self.buf.push_str(text);
        self.buf.push_str("\n\n");
        self
    }

    /// Append a fenced code block tagged with `lang`.
    pub fn code_block(&mut self, lang: &str, body: &str) -> &mut Self {
        self.buf.push_str("```");
        self.buf.push_str(lang);
        self.buf.push('\n');
        self.buf.push_str(body);
        if !body.ends_with('\n') {
            self.buf.push('\n');
        }
        self.buf.push_str("```\n\n");
        self
    }

    /// Consume the builder and return the final string.
    pub fn finish(self) -> String {
        self.buf
    }
}
