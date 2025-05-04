use std::{fmt::Display, io::Write as _, mem};

use ratatui::{
    backend::WindowSize,
    layout::Position,
    style::{Modifier, Style},
};
use std::io::Error as IOError;
use std::io::Result as IOResult;
use wasm_bindgen::JsValue;

use crate::types::{JsWriter, JsTermSizeCallback, log_value};

pub struct AnsiBackendOptions {
    /// A way to get the terminal size from JavaScript
    pub get_size: JsTermSizeCallback,

    /// A method to write to stdout in JavaScript
    pub stdout_writer: JsWriter,
}


/// A pure ANSI implementation of RataTUI's backend.
///
/// The caller must provide a callback for fetching window size, and writing output to stdout.
pub struct AnsiBackend {
    get_size: JsTermSizeCallback,
    pos: Option<Position>,
    buf: Vec<u8>,
    stdout_writer: JsWriter,
}

impl AnsiBackend {
    pub fn new(options: AnsiBackendOptions) -> Self {
        let AnsiBackendOptions{get_size, stdout_writer} = options;
        Self {
            get_size,
            stdout_writer,
            pos: None,
            buf: Vec::new()
        }
    }
}

impl ratatui::backend::Backend for AnsiBackend {
    fn draw<'a, I>(&mut self, content: I) -> IOResult<()>
    where
        I: Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>,
    {
        let mut prev_style: Option<Style> = None;

        for (x, y, cell) in content {
            if cell.skip {
                continue;
            }

            let mut new_pos = Position { x, y };
            if Some(new_pos) != self.pos {
                self.set_cursor_position(new_pos)?;
            }

            self.diff_style(&mut prev_style, cell.style())?;

            self.buf.extend_from_slice(cell.symbol().as_bytes());
            // TODO: Check unicode width. (Why doesn't Ratatui give us this?)
            let width = 1;
            new_pos.x += width;
            // TODO: Wrap?
            self.pos = Some(new_pos)
        }
        self.flush()
    }

    fn hide_cursor(&mut self) -> IOResult<()> {
        write!(self.buf, "{}", anes::HideCursor)
    }

    fn show_cursor(&mut self) -> IOResult<()> {
        write!(self.buf, "{}", anes::ShowCursor)
    }

    fn get_cursor_position(&mut self) -> IOResult<ratatui::prelude::Position> {
        let pos = match self.pos {
            Some(pos) => pos,
            None => {
                let new_pos = Position { x: 0, y: 0 };
                self.set_cursor_position(new_pos)?;
                self.pos = Some(new_pos);
                new_pos
            }
        };

        Ok(pos)
    }

    fn set_cursor_position<P: Into<ratatui::prelude::Position>>(
        &mut self,
        new_pos_into: P,
    ) -> IOResult<()> {
        let new_pos: Position = new_pos_into.into();
        if Some(new_pos) == self.pos {
            return Ok(());
        }

        self.push(anes::MoveCursorTo(new_pos.x + 1, new_pos.y + 1))?;
        self.pos = Some(new_pos);
        Ok(())
    }

    fn clear(&mut self) -> IOResult<()> {
        // If there's a remaining color it'll set the whole screen to that color. We don't want that:
        self.push(anes::ResetAttributes)?;

        self.push(anes::ClearBuffer::All)
    }

    fn size(&self) -> IOResult<ratatui::prelude::Size> {
        let size = self.get_size.get().map_err(|_| io_err("Error getting the size of the terminal"))?;
        Ok(size.into())
    }

    fn window_size(&mut self) -> IOResult<ratatui::backend::WindowSize> {
        Ok(WindowSize {
            columns_rows: self.size()?,
            pixels: Default::default(),
        })
    }

    fn flush(&mut self) -> IOResult<()> {
        if self.buf.is_empty() {
            return Ok(());
        }
        let bytes = mem::take(&mut self.buf);
        self.stdout_writer
            .call(JsValue::NULL, bytes.into_boxed_slice())
            .map_err(|err| {
                log_value(err);
                io_err("Writing to stdout threw an error")
            })?;
        Ok(())
    }
}

fn io_err<E>(message: E) -> std::io::Error 
where E: Into<Box<dyn std::error::Error + Send + Sync>>
{
    use std::io::ErrorKind::Other;
    IOError::new(Other, message)
}

impl AnsiBackend {
    pub fn exclusive(&mut self) -> IOResult<()> {
        self.push(anes::SwitchBufferToAlternate)
    }

    pub fn normal(&mut self) -> IOResult<()> {
        self.push(anes::SwitchBufferToNormal)
    }

    pub fn beep(&mut self) -> IOResult<()> {
        self.push('\u{7}')
    }

    fn diff_style(&mut self, old: &mut Option<Style>, new: Style) -> IOResult<()> {
        if old.is_none() {
            // If there was no previous style, this is likely the beginning of a render.
            // Don't carry over any styles from the previous render:
            self.push(anes::ResetAttributes)?;
        }
        let base = old.unwrap_or_default();

        if base == new {
            return Ok(());
        }

        // Modifiers need to come before colors:
        // Thsi whole add/sub thing is a nightmare.
        // Especially given that Bold/Faint/Normal all cancel each other out.
        // Just reset styles and apply the "add" ones when they differ:
        let base_mod = base.add_modifier - base.sub_modifier;
        let new_mod = new.add_modifier - new.sub_modifier;
        if base_mod != new_mod {
            self.push(anes::ResetAttributes)?;
            for modi in new_mod.iter() {
                use anes::Attribute as Attr;
                use anes::SetAttribute as Set;
                let attr = match modi {
                    Modifier::BOLD => Attr::Bold,
                    Modifier::CROSSED_OUT => Attr::Crossed,
                    Modifier::DIM => Attr::Faint,
                    Modifier::HIDDEN => Attr::Conceal,
                    Modifier::ITALIC => Attr::Italic,
                    Modifier::RAPID_BLINK | Modifier::SLOW_BLINK => Attr::Blink,
                    Modifier::REVERSED => Attr::Reverse,
                    Modifier::UNDERLINED => Attr::Underline,
                    _ => panic!("unknown modifier"),
                };
                self.push(Set(attr))?;
            }
        }
        self.push(anes::SetBackgroundColor(ansi_color(new.bg)))?;
        self.push(anes::SetForegroundColor(ansi_color(new.fg)))?;

        // TODO: more styles?

        *old = Some(new);

        Ok(())
    }

    fn push(&mut self, ansi: impl Display) -> IOResult<()> {
        write!(self.buf, "{}", ansi)
    }
}

fn ansi_color(color: Option<ratatui::style::Color>) -> anes::Color {
    use anes::Color as AColor;
    use ratatui::style::Color as RColor;

    let Some(color) = color else {
        return anes::Color::Default;
    };

    match color {
        RColor::Reset => AColor::Default,
        RColor::Black => AColor::Black,
        RColor::Red => AColor::DarkRed,
        RColor::Green => AColor::DarkGreen,
        RColor::Yellow => AColor::DarkYellow,
        RColor::Blue => AColor::DarkBlue,
        RColor::Magenta => AColor::DarkMagenta,
        RColor::Cyan => AColor::DarkCyan,
        RColor::Gray => AColor::DarkGray,
        RColor::DarkGray => AColor::DarkGray,
        RColor::LightRed => AColor::Red,
        RColor::LightGreen => AColor::Green,
        RColor::LightYellow => AColor::Yellow,
        RColor::LightBlue => AColor::Blue,
        RColor::LightMagenta => AColor::Magenta,
        RColor::LightCyan => AColor::Cyan,
        RColor::White => AColor::White,
        RColor::Rgb(r, g, b) => AColor::Rgb(r, g, b),
        RColor::Indexed(code) => AColor::Ansi(code),
    }
}
