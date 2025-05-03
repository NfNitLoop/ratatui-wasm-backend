//! Types we expect to receive from JavaScript:

use ratatui::layout::Size;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type JsTermSize;

    #[wasm_bindgen(method, getter)]
    pub fn columns(this: &JsTermSize) -> u16;

    #[wasm_bindgen(method, getter)]
    pub fn rows(this: &JsTermSize) -> u16;
}

impl Into<Size> for JsTermSize {
    fn into(self) -> Size {
        Size {
            width: self.columns(),
            height: self.rows(),
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TSTermSizeCallback: &'static str = r#"
/**
 * Gets the current size of the terminal.
 * 
 * Must be provided so that the WASM Terminal Backend can properly support resizing.
 */
type TerminalSizeCallback = {
    (): { columns: number, rows: number }
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "TerminalSizeCallback")]
    pub type JsTermSizeCallback;

    #[wasm_bindgen(method, catch)]
    fn call(this: &JsTermSizeCallback) -> Result<JsTermSize, JsValue>;
}

impl JsTermSizeCallback {
    pub fn get(&self) -> Result<Size, JsValue> {
        Ok(self.call()?.into())
    }
}

/// console.log:
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: String);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_value(v: JsValue);
}

#[wasm_bindgen(typescript_custom_section)]
const TSWriter: &'static str = r#"
/**
 * A writer that we will output terminal commands to.
 */
type Writer = {
    (bytes: Uint8Array): number
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Writer")]
    pub type JsWriter;

    #[wasm_bindgen(method,catch)]
    pub fn call(this: &JsWriter, value: JsValue, bytes: Box<[u8]>) -> Result<usize, JsValue>;
}