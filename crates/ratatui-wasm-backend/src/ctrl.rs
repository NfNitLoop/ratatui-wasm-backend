//! Utilities for handling control characters
//! 
//! See: <https://github.com/qwandor/anes-rs/issues/41>
//! 
//! In the terminal, Ctrl-$key is not transmitted as a separate modifier and keycode,
//! they are transmitted as ASCII control characters, which may have meaning. (ex: Ctrl-m is carriage-return/enter).
//! 
//! Anes just returns these as KeyCodes, but you may need need to detect them and handle them separately.
//! 

use anes::parser::{KeyCode, Sequence};

pub trait GetCtrl {
    fn ctrl(&self) -> Option<Ctrl>;
}

impl GetCtrl for Sequence {
    fn ctrl(&self) -> Option<Ctrl> {
        match self {
            Sequence::Key(key_code, _) => key_code.ctrl(),
            Sequence::Mouse(_, _) => None,
            Sequence::CursorPosition(_, _) => None,
        }
    }
}

impl GetCtrl for KeyCode {
    fn ctrl(&self) -> Option<Ctrl> {
        let ch =match self {
            KeyCode::Enter => {
                // Anes maps both Ctrl-J and Ctrl-M to Enter.
                // So I'd rather return None here than choose one:
                return None;
            },
            KeyCode::Tab => return Some(Ctrl::I),

            // May technically be control codes. TODO: check?
            KeyCode::Esc => return None,
            KeyCode::Backspace
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Up
            | KeyCode::Down
            | KeyCode::Home
            | KeyCode::End
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::BackTab
            | KeyCode::Delete
            | KeyCode::Insert 
            | KeyCode::Null
            | KeyCode::F(_)
            => {
                return None
            },
            KeyCode::Char(ch) => ch
        };

        let ctrl = match ch {
            '\u{1}' => Ctrl::A,
            '\u{2}' => Ctrl::B,
            '\u{3}' => Ctrl::C,
            '\u{4}' => Ctrl::D,
            '\u{5}' => Ctrl::E,
            '\u{6}' => Ctrl::F,
            '\u{7}' => Ctrl::G,
            '\u{8}' => Ctrl::H,
            '\u{9}' => Ctrl::I,
            '\u{a}' => Ctrl::J,
            '\u{b}' => Ctrl::K,
            '\u{c}' => Ctrl::L,
            '\u{d}' => Ctrl::M,
            '\u{e}' => Ctrl::N,
            '\u{f}' => Ctrl::O,
            '\u{10}' => Ctrl::P,
            '\u{11}' => Ctrl::Q,
            '\u{12}' => Ctrl::R,
            '\u{13}' => Ctrl::S,
            '\u{14}' => Ctrl::T,
            '\u{15}' => Ctrl::U,
            '\u{16}' => Ctrl::V,
            '\u{17}' => Ctrl::W,
            '\u{18}' => Ctrl::X,
            '\u{19}' => Ctrl::Y,
            '\u{1a}' => Ctrl::Z,
            _ => return None,
        };

        Some(ctrl)
    }
}

/// Represents the keyboard character that was pressed with ctrl. 
///
/// Ex: CtrlKey::Char('c') is Ctrl-C
/// 
/// Note: ASCII and Unicode have many more "control codes" than what [CtrlKey] handles. We're specifically
/// limiting its scope to ctrl-$key combos.
#[derive(Debug, PartialEq)]
pub enum Ctrl {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,

    /// AKA: "Tab". Some terminals (ex: Ghostty v1.1.3) may not transmit a ctrl-i keypress.
    I,

    /// You will not ever see this in practice, because it's parsed as KeyCode::Enter.
    J,
    K,
    L,

    /// AKA: "Carriage Return" AKA "Enter".
    /// 
    /// Some terminals (ex: Ghostty v1.1.3) may not transmit a ctrl-m keypress.
    /// You will not ever see this in practice, because it's parsed as KeyCode::Enter.
    M,

    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

