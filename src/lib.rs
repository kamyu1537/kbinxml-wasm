mod utils;

use wasm_bindgen::prelude::*;
use kbinxml::{Options};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// XML -> Binary
#[wasm_bindgen]
pub fn encode(data: &[u8]) -> String {
    let (collection, encoding) = kbinxml::from_text_xml(data).unwrap();
    let options = Options::with_encoding(encoding);
    let result = kbinxml::to_binary_with_options(options, &collection).unwrap();
    base64::encode(result)
}

// Binary -> XML
#[wasm_bindgen]
pub fn decode(data: &[u8]) -> String {
    let (collection, _encoding) = kbinxml::from_slice(data).unwrap();
    let result = kbinxml::to_text_xml(&collection).unwrap();
    base64::encode(result)
}
