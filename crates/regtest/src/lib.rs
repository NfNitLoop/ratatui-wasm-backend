mod widgets;
mod js;
mod texts;

use std::collections::VecDeque;

use js::regexp::{Match, RegExp};
use ratatui_wasm_backend::{
    anes::parser::{KeyCode, KeyModifiers, Parser, Sequence}, backend::{ AnsiBackend, AnsiBackendOptions }, ctrl::{Ctrl, GetCtrl as _}, ratatui:: {
        self,
        border,
        layout::{Constraint, Direction, Layout},
        style::Color,
        text::{ToLine as _, ToText},
        widgets::{Borders, Padding, Wrap},
    }, types
};

use ratatui::{
    buffer::Buffer, layout::Rect, prelude::Backend, style::Stylize, text::{Line, Text}, widgets::{Block, Paragraph, Widget}
};
use texts::SAMPLE;
use types::{JsTermSizeCallback, JsWriter, log};
use wasm_bindgen::prelude::*;
use widgets::{Hilighted, ToDynLayout};



pub type Result<T, E = JsValue> = std::result::Result<T, E>;

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
        if self.app.beep {
            self.term.backend_mut().beep().map_err(|e| format!("{e}"))?;
            self.app.beep = false;
        }

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
pub struct App {
    // as input by the user:
    regex: String,

    // Sample search text.
    body: String,

    // Should we beep the terminal on the next render?
    beep: bool,

    /// Was there an error compiling the regex or making the match?
    error: Option<String>,

    matches: Vec<Match>,

    // Show the debug pane
    debug: bool,

    // Used for debugging
    seqs: VecDeque<Sequence>,
    
}

impl Default for App {
    fn default() -> Self {
        let mut new_self = Self { 
            seqs: Default::default(), 
            regex: "\\w+ism\\b|\\w+ist?s\\b|nation(al)?|\\d?[02468][.]".into(),
            
            body: SAMPLE.into(),
            debug: false,
            beep: false,
            error: None,
            matches: vec![],
        };
        new_self.calc_matches();
        new_self
    }
}

impl App {
    fn recv_sequence(&mut self, seq: Sequence) -> Result<()> {
        match seq {
            Sequence::Key(code, modifiers) => match code {
                KeyCode::Esc => {
                    Err("quit")?;
                },
                seq if seq.ctrl() == Some(Ctrl::C) => {
                    Err("quit")?;
                },
                seq if seq.ctrl().is_some() => {
                    self.beep = true;
                }
                KeyCode::Char('d') if modifiers == KeyModifiers::ALT => {
                    self.toggle_debug();
                },
                _ if !modifiers.is_empty() => {
                    self.beep = true;
                }
                KeyCode::Char(c) => {
                    self.got_char(c);
                },
                KeyCode::Backspace | KeyCode::Delete=> {
                    self.backspace();
                }
                _ => {
                    self.beep = true;
                }
            },
            Sequence::Mouse(_,_) => {},
            Sequence::CursorPosition(_,_) => {},
        };

        self.add_debug_seq(seq);

        Ok(())
    }

    fn add_debug_seq(&mut self, seq: Sequence) {
        self.seqs.push_back(seq);
        if self.seqs.len() > 10 {
            self.seqs.pop_front();
        }
    }

    fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    fn got_char(&mut self, c: char) {
        // TODO: Dispatch depending on active pane:
        self.regex.push(c);
        self.calc_matches();
    }

    fn backspace(&mut self) {
        // TODO: Dispatch depend on active pane:
        self.regex.pop();
        self.calc_matches();
    }

    fn calc_matches(&mut self) {
        // TODO: Allow dynamically setting the flags:
        let flags = "digm";

        let re = match RegExp::new(self.regex.as_str(), flags) {
            Ok(re) => re,
            Err(err) => {
                self.error = Some(format!("{err}"));
                self.matches.clear();
                return;
            }
        };

        self.matches = re.match_all(self.body.as_str());
        self.error = None;
    }
}

fn block() -> Block<'static> {
    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Color::DarkGray)
        .borders(border!(TOP, LEFT, RIGHT))
        .padding(Padding::bottom(1))
}

// TODO: Switch to a statefulWidgetRef so that we can get back the cursor position after render.
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut layout = Layout::default().direction(Direction::Vertical).dynamic();

        // title:
        layout.add(
            Constraint::Length(1),
            "🔬 JavaScript Regex Tester 🧪 ".bold().fg(Color::Yellow).into_centered_line()
        );

        // regex
        layout.add(
            Constraint::Length(3), 
            Paragraph::new( Text::from(self.regex.as_str()) )
                .block(block().title(" RegEx ".to_line().right_aligned()))
        );

        if let Some(error) = &self.error {
            let title = " Error ".to_line().right_aligned();
            let block = block().title(title).border_style(Color::Red);
            layout.add(
                Constraint::Length(3),
                Paragraph::new(error.to_text())
                    .block(block)
                    .wrap(Wrap{trim: false})
            )
        }

        // Main text box:
        layout.add(Constraint::Fill(5), {
            let title = " Text ".to_line().right_aligned();
            let matches = self.matches.len();
            let match_txt = if matches == 0 { "".to_string() } else {
                format!(" {matches} Matches ")
            };
            let match_txt = Line::from(match_txt).centered();
            Hilighted {
                block: block()
                    .title(title)
                    .title_bottom(match_txt)
                    .borders(Borders::ALL),
                matches: &self.matches,
                text: &self.body,
            }
        });


        if self.debug {
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
                " ".into(),
                "Debug".into(),
                " ".into(),
                "<Alt-D>".fg(Color::Yellow).bold(),
            ]).centered()
        });


        layout.render(area, buf);       
    }
}
