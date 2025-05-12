use std::{cell::RefCell, rc::Rc};

use ratatui::{buffer::Buffer, layout::Rect, text::{Line, ToLine}, widgets::{Widget, WidgetRef}};
use textwrap::{self, Options};

/// Renders an editable text box with text wrapping.
/// TODO: Editable. 
/// TODO: Track cursor position. (Deal with trimmed spaces?)
pub struct TextBox {
    text: String,
    /// Text position within the string, according to Rust, which uses utf8-byte offsets.
    text_pos_in_bytes: usize,

    // May need to be updated during rendering.
    state: RefCell<State>,

}

/// State we need to update during render.
struct State {
    // TODO: Save the Buffer position of the cursor
    // pos: Option<usize>,

    // Whether state has been changed, which might need a recalculation for re-render.
    dirty: bool,
    last_wrap_width: u16,
    
    // "calculated" render state:
    lines: Vec<Line<'static>>
}

impl TextBox {
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
    
    pub fn handle_input(&mut self, seq: ratatui_wasm_backend::anes::parser::KeyCode) {
        use ratatui_wasm_backend::anes::parser::KeyCode as Code;
        match seq {
            Code::Backspace => {
                self.backspace()
            },
            Code::Delete => {},
            Code::Enter => {
                self.insert('\n');
            },
            Code::Left => {},
            Code::Right => {},
            Code::Up => {},
            Code::Down => {},
            Code::Home => {},
            Code::End => {},
            Code::PageUp => {},
            Code::PageDown => {},
            Code::BackTab => {},

            Code::Tab
            | Code::Insert => {
                // Do nothing
            },
            Code::F(_) => {},
            Code::Char(c) => {
                self.insert(c);
            },
            Code::Null => {},
            Code::Esc => {},
        }
    }
    
    fn dirty(&mut self) {
        let mut state = self.state.borrow_mut();
        state.dirty = true;
    }
    
    fn insert(&mut self, c: char) {
        self.text.insert(self.text_pos_in_bytes, c);
        self.text_pos_in_bytes += c.len_utf8();
        self.dirty();
    }

    fn backspace(&mut self) {
        if self.text_pos_in_bytes == 0 {
            // Set beep?
            return
        }
        if self.text_pos_in_bytes > self.text.len() {
            self.text_pos_in_bytes = self.text.len()
        }

        for char_offset in 1..=4 {
            let idx = self.text_pos_in_bytes - char_offset;
            if self.text.is_char_boundary(idx) {
                self.text.remove(idx);
                self.text_pos_in_bytes = idx;
                break;
            }
        }

        self.dirty();
    }
}

impl TextBox {
    fn my_render(&self, area: Rect, buf: &mut Buffer) {
        self.rewrap(area.width);

        // TODO: avoid double borrow.
        let m = self.state.borrow_mut();
        
        // TODO: support text-alignment:
        for (row, line) in area.rows().zip(m.lines.iter()) {
            line.render(row, buf);
        }
    }
}

impl <T: ToString> From<T> for TextBox {
    fn from(value: T) -> Self {
        Self {
            text: value.to_string(),
            text_pos_in_bytes: 0,
            state: RefCell::new(State {
                dirty: true,
                last_wrap_width: 0,
                lines: vec![],
            })
        }
    }
}

// impl Widget for &TextBox {
//     fn render(self, area: Rect, buf: &mut Buffer)
//     {
//         self.my_render(area, buf)
//     }
// }

impl WidgetRef for TextBox {
    fn render_ref(&self,area:Rect,buf: &mut Buffer) {
        self.my_render(area, buf);
    }
}



impl TextBox {
    fn rewrap(&self, new_width: u16) {
        let mut state = self.state.borrow_mut();
        
        if !state.dirty && state.last_wrap_width == new_width {
            return;
        }

        let opts = Options::new(new_width as usize).break_words(true);
        let lines = textwrap::wrap(&self.text, opts);
        let lines = lines.into_iter()
            .map(|it| it.into_owned())
            .map(|it| Line::from(it))
            .collect::<Vec<_>>();

        state.lines = lines;
        state.last_wrap_width = new_width;
        state.dirty = false;
    }
}