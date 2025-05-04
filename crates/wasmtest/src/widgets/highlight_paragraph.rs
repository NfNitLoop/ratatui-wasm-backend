use std::mem;

use ratatui_wasm_backend::ratatui::{buffer::Buffer, layout::Rect, style::{Color, Stylize as _}, text::{Line, Span, Text, ToSpan}, widgets::{Block, Paragraph, Widget, WidgetRef, Wrap}};
use wasm_bindgen::prelude::wasm_bindgen;

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
        let mut lines = vec![];
        let mut spans = vec![];

        let mut matches = self.matches.iter().peekable();


        for (char_index, (byte_index, ch)) in self.text.char_indices().enumerate() {
            loop {
                let Some(mat) = matches.peek() else {
                    break;
                };
                if mat.end <= char_index {
                    matches.next();
                } else {
                    break;
                }
            }

            // No need to handle CR/LF: can't input LF.
            if ch == '\n' {
                let owned_spans = spans.clone();
                lines.push(Line::from(owned_spans));
                spans.clear();
            }

            let hi = match matches.peek() {
                None => false,
                Some(mat) => {
                    mat.start <= char_index && char_index < mat.end
                },
            };

            // quite brute force, but will work with multi-line strings.
            let end_char = (1..).map(|offset| offset + byte_index).filter(|it| self.text.is_char_boundary(*it)).next().unwrap();
            let Some(substr) = self.text.get(byte_index..end_char) else {
                panic!("slice failed");
            };
            if hi {
                spans.push(substr.black().on_yellow())
            } else {
                spans.push(substr.into())
            }
        }

        if !spans.is_empty() {
            lines.push(Line::from(spans));
        }

        let text = Text::from(lines);

        Paragraph::new(text).block(self.block.clone()).wrap(Wrap{trim: false}).render(area, buf);
    }
}