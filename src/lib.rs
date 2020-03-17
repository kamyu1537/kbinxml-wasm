use wasm_bindgen::prelude::*;
use kbinxml::{Options};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);
}

#[wasm_bindgen]
pub fn encode(data: &[u8]) -> String {
    let (collection, encoding) = match kbinxml::from_text_xml(data) {
        Ok(v) => v,
        Err(_e) => panic!(_e)
    };
    let options = Options::with_encoding(encoding);
    let buf = kbinxml::to_binary_with_options(options, &collection);
    let result = match buf {
        Ok(value)  => value,
        Err(_e) => vec![],
    };
    return base64::encode(result)
}

#[wasm_bindgen]
pub fn decode(data: &[u8]) -> String {
    let (collection, _encoding) = match kbinxml::from_slice(data) {
        Ok(v) => v,
        Err(_e) => panic!(_e)
    };
    let text_original = match kbinxml::to_text_xml(&collection) {
        Ok(v) => v,
        Err(_e) => vec![]
    };
    return base64::encode(text_original)
}
