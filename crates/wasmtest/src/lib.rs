use std::collections::VecDeque;

use ratatui_wasm_backend::anes::parser::KeyCode;
use ratatui_wasm_backend::ctrl::Ctrl;
use ratatui_wasm_backend::ratatui;
use ratatui_wasm_backend::ratatui::border;
use ratatui_wasm_backend::ratatui::layout::Constraint;
use ratatui_wasm_backend::ratatui::layout::Layout;
use ratatui_wasm_backend::ratatui::style::Color;
use ratatui_wasm_backend::ratatui::text::ToLine as _;
use ratatui_wasm_backend::ratatui::widgets::Borders;
use ratatui_wasm_backend::types;
use ratatui_wasm_backend::backend::{ AnsiBackend, AnsiBackendOptions };
use ratatui_wasm_backend::anes::parser::{Parser, Sequence};
use ratatui_wasm_backend::ctrl::GetCtrl as _;

use ratatui::{
    buffer::Buffer, layout::Rect, prelude::Backend, style::Stylize, text::{Line, Text}, widgets::{Block, Paragraph, Widget}, Frame, Terminal
};
use types::{JsTermSizeCallback, JsWriter, log};
use wasm_bindgen::prelude::*;
use widgets::DynLayout;

mod widgets;

pub type Result<T, E = JsValue> = std::result::Result<T, E>;

// TODO: Process keyboard input in Rust? 
// Maybe: https://docs.rs/terminal-keycode/latest/terminal_keycode/
// Anes, which I'm already using, also has a parser. Nice.

/// The main entrypoint into your TUI.
/// 
/// You can access .app to update its state, then render() to view the results.
#[wasm_bindgen]
pub struct Main {
    term: ratatui::Terminal<AnsiBackend>,
    parser: Parser,
    app: App,
}

#[wasm_bindgen]
impl Main {
    #[wasm_bindgen(constructor)]
    pub fn new(get_size: JsTermSizeCallback, stdout_writer: JsWriter) -> Result<Main> {
        let mut be = AnsiBackend::new(AnsiBackendOptions { get_size, stdout_writer });

        be.exclusive().map_err(|err| err.to_string())?;
        be.clear().map_err(|err| err.to_string())?;

        let term = ratatui::Terminal::new(be).map_err(|err| err.to_string())?;
        Ok(Self { 
            term, 
            app: App::default(),
            parser: Parser::default()
        })
    }

    pub fn push_stdin_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.parser.advance(bytes, false);

        let mut got_token = false;
        while let Some(seq) = self.parser.next() {
            self.app.recv_sequence(seq)?;
            got_token = true;
        }
        if got_token {
            self.render()?;
        }
        Ok(())
    }


    pub fn render(&mut self) -> Result<()> {
        self.term.draw( |frame| {
            self.app.render(frame.area(), frame.buffer_mut())
        }).map_err(|err| err.to_string())?;
        Ok(())
    }
}



impl Drop for Main {
    fn drop(&mut self) {
        if let Err(err) = self.term.backend_mut().clear() {
            log(err.to_string());
        };
        if let Err(err) = self.term.backend_mut().normal() {
            log(err.to_string());
        };
        if let Err(err) = self.term.flush() {
            log(err.to_string());
        }
    }
}


/// Application state.
#[derive(Default)]
pub struct App {
    counter: u16,
    seqs: VecDeque<Sequence>,
    regex: String,
}

impl App {
    fn recv_sequence(&mut self, seq: Sequence) -> Result<()> {
        match seq {
            Sequence::Key(code, _modifiers) => match code {
                KeyCode::Up => {
                    self.counter += 1;
                },
                KeyCode::Down => {
                    self.counter -= 1;
                },
                KeyCode::Left => {
                    self.counter -= 10;
                },
                KeyCode::Right => {
                    self.counter += 10;
                },
                KeyCode::Esc => {
                    Err("quit")?;
                },
                seq if seq.ctrl() == Some(Ctrl::C) => {
                    Err("quit")?;
                }
                code @ KeyCode::Char(c) if code.ctrl().is_none() => {
                    self.regex.push(c);
                },
                KeyCode::Backspace => {
                    self.regex.pop();
                }
                KeyCode::Delete => {
                    self.regex.pop();
                }
                _ => {
                    // TODO: set beep?
                }
            },
            Sequence::Mouse(_,_) => {},
            Sequence::CursorPosition(_,_) => {},
        };

        self.add_seq(seq);

        Ok(())
    }

    fn add_seq(&mut self, seq: Sequence) {
        self.seqs.push_back(seq);
        if self.seqs.len() > 10 {
            self.seqs.pop_front();
        }
    }
}

fn block() -> Block<'static> {
    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Color::DarkGray)
        .borders(border!(TOP, LEFT, RIGHT))        
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {


        let mut layout = DynLayout::from(Layout::default().direction(ratatui::layout::Direction::Vertical));

        // title:
        layout.add(
            Constraint::Length(1),
            "🔬 JavaScript Regex Tester 🧪 ".bold().fg(Color::Yellow).into_centered_line()
        );

        // regex
        layout.add(
            Constraint::Length(2), 
            Paragraph::new( Text::from(self.regex.as_str()) )
                .block(block().title(" RegEx ".to_line().right_aligned()))
        );



        // Body:
        layout.add(Constraint::Fill(5), {
            let title = " Body ".to_line().right_aligned();
            Paragraph::new(Text::from("body goes here"))
                .block(block().title(title).borders(Borders::ALL))
        });


        if true {
            layout.add( Constraint::Min(12), {
                let title = " Event Debug ".to_line().right_aligned();
                let seq_lines = self.seqs.iter()
                    .map(|s| {
                        let ctrl = s.ctrl();
                        format!("{s:?} {ctrl:?}")
                    })
                    .map(|s| Line::from(s))
                    .collect::<Vec<_>>()
                ;
                let seq_text = Text::from(seq_lines);
                Paragraph::new(seq_text)
                    // .centered()
                    .block(block().title(title).borders(Borders::all()))
            });
        }

        layout.add(Constraint::Length(1), {
            Line::from(vec![
                "Quit ".into(),
                "<Esc>".fg(Color::Yellow).bold(),
            ]).centered()
        });


        layout.render(area, buf);       
    }
}
