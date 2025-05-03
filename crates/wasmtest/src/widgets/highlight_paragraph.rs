use ratatui_wasm_backend::ratatui::{buffer::Buffer, layout::Rect, style::{Color, Stylize as _}, text::{Line, Span, Text, ToSpan}, widgets::{Block, Paragraph, Widget, WidgetRef, Wrap}};

use crate::js::regexp::Match;

/// Hilights parts of a text based on input ranges.
pub struct Hilighted<'a> {
    pub text: &'a str,

    /// These must be ordered and non-overlapping.
    pub matches: &'a Vec<Match>,

    /// Style for the paragraph.
    pub block: Block<'a>,
}

impl <'a> WidgetRef for Hilighted<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer)
    {
        let mut spans = vec![];

        // Javascript regex gives "character" offsets, but Rust wants utf-8-byte offsets.
        let char_indices: Vec<usize> = self.text.char_indices().map(|ci| ci.0).collect();
        let mut prev_rust_index: usize = 0;

        for mat in self.matches {
            let Some(&rust_start_index) = char_indices.get(mat.start) else {
                break;
            };
            let Some(&rust_end_index) = char_indices.get(mat.end) else {
                break;
            };

            // Add span for non-matched text we've skipped:
            if prev_rust_index < rust_start_index {
                let substr = self.text.get(prev_rust_index..rust_start_index).expect("non-matched text");
                spans.push(Span::from(substr));
                prev_rust_index = rust_start_index;
            }

            let substr = self.text.get(rust_start_index..rust_end_index).expect("getting match range");
            spans.push(Span::from(substr).black().on_yellow());
            prev_rust_index = rust_end_index;
        } // for-loop

        // Add span for any remaining text:
        let substr = self.text.get(prev_rust_index..).expect("getting last unmatched range");
        spans.push(Span::from(substr));

        let text = Text::from(Line::from(spans));

        Paragraph::new(text).block(self.block.clone()).wrap(Wrap{trim: false}).render(area, buf);
    }
}