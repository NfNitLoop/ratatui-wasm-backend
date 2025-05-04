use std::fmt::Display;

use wasm_bindgen::{prelude::wasm_bindgen, JsCast as _};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Error;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Error) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn message(this: &Error) -> String;

    #[wasm_bindgen(js_name = string)]
    pub type JsString;


    // We usually expect an `Error` to be thrown from exceptions, but JS doesn't guarantee that.
    #[wasm_bindgen]
    pub type MaybeError;
}

impl Display for MaybeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(error) = self.dyn_ref::<Error>() {
            let name = error.name();
            let message = error.message();
            return write!(f, "{name}: {message}");
        }

        if let Some(str_err) = self.as_string() {
            return write!(f, "raw-string: {str_err}")
        }

        let js_type = self.js_typeof().as_string().unwrap_or_else(|| "unknown throw type".into());
        write!(f, "thrown type: {js_type}")
    }
}