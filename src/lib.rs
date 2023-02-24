mod utils;

use kbinxml::{EncodingType, Options};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
#[wasm_bindgen]
pub struct DecodeResult {
    base64: String,
    encoding: u8,
}

#[wasm_bindgen]
impl DecodeResult {
    pub fn new(base64: String, encoding: u8) -> DecodeResult {
        DecodeResult { base64, encoding }
    }

    pub fn base64(&self) -> String {
        self.base64.clone()
    }

    pub fn encoding(&self) -> u8 {
        self.encoding
    }
}

// XML -> Binary
#[wasm_bindgen]
pub fn encode(data: &[u8], encoding_byte: u8) -> String {
    let (collection, _encoding) = kbinxml::from_text_xml(data).unwrap();
    let encoding_type = EncodingType::from_byte(encoding_byte).unwrap();
    let options = Options::with_encoding(encoding_type);
    let result = kbinxml::to_binary_with_options(options, &collection).unwrap();
    base64::encode(result)
}

// Binary -> XML
#[wasm_bindgen]
pub fn decode(data: &[u8]) -> DecodeResult {
    let (collection, encoding) = kbinxml::from_slice(data).unwrap();
    let result = kbinxml::to_text_xml(&collection).unwrap();
    DecodeResult::new(base64::encode(result), encoding.to_byte())
}
