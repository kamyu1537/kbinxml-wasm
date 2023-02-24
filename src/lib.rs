mod utils;

use gloo_utils::format::JsValueSerdeExt;
use kbinxml::{EncodingType, Options};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct DecodeResult {
    pub base64: String,
    pub encoding: u8,
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
pub fn decode(data: &[u8]) -> JsValue {
    let (collection, encoding) = kbinxml::from_slice(data).unwrap();
    let xml = kbinxml::to_text_xml(&collection).unwrap();

    let result = DecodeResult {
        base64: base64::encode(xml),
        encoding: encoding.to_byte(),
    };

    JsValue::from_serde(&result).unwrap()
}
