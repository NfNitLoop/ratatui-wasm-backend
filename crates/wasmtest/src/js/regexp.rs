
use wasm_bindgen::prelude::wasm_bindgen;

use super::error::MaybeError;

#[wasm_bindgen]
extern "C" {
    /// Like js-sys's RegExp, but can handle exceptions from invalid patterns.
    #[wasm_bindgen]
    pub type RegExp;

    /// Might return an error if we've provided invalid arguments.
    #[wasm_bindgen(constructor, catch)]
    fn js_new(pattern: &str, flags: &str) -> Result<RegExp, MaybeError>;

    #[wasm_bindgen(method, getter, js_name = lastIndex)]
    fn last_index(this: &RegExp) -> Option<usize>;

    #[wasm_bindgen(method, setter, js_name = lastIndex)]
    fn set_last_index(this: &RegExp, index: Option<usize>);

    #[wasm_bindgen(method, getter)]
    fn global(this: &RegExp) -> bool;

    #[wasm_bindgen(method)]
    fn exec(this: &RegExp, subject: &str) -> Option<JsMatch>;
}

#[wasm_bindgen]
extern "C" {
    /// Result of [`RegExp.exec``]
    #[wasm_bindgen]
    type JsMatch;

    // Always present because we always set "d" flag.
    #[wasm_bindgen(method, getter)]
    fn indices(this: &JsMatch) -> Vec<JsIndexTuple>;

    #[wasm_bindgen]
    type JsIndexTuple;

    #[wasm_bindgen(method, indexing_getter)]
    fn get(this: &JsIndexTuple, index: usize) -> Option<usize>;
}

impl RegExp {
    /// Create a RegExp.
    /// Always adds the "d" flag so we can know the start/end indices of matches.
    pub fn new(pattern: &str, flags: &str) -> Result<RegExp, MaybeError> {
        if flags.contains("d") {
            return Self::js_new(pattern, flags);
        }

        let flags = format!("{flags}d");
        Self::js_new(pattern, flags.as_str())
    }

    /// RegExp objects are mutable in JavaScript, you must reset them before each use.
    fn reset(&self) {
        self.set_last_index(None)
    }

    pub fn match_all(&self, value: &str) -> Vec<Match> {
        let global = self.global();
        let mut matches = vec![];
        
        self.reset();
        loop {
            let Some(m) = self.exec(value) else {
                break
            };
            let indices = &m.indices()[0];
            let start = indices.get(0).unwrap();
            let end = indices.get(1).unwrap();

            // This case never increases the indexAt, so we get into an infinite loop:
            if start == end {
                break
            }

            matches.push(Match{start, end});
            if !global {
                break;
            }
        }

        matches
    }
}

#[derive(Debug)]
pub struct Match {
    pub start: usize,
    pub end: usize,
}

