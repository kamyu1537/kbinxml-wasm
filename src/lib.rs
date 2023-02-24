mod utils;

use kbinxml::{EncodingType, Options};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "{ xml: string, encoding: number }")]
    pub type IDecodeResult;
}

#[derive(Serialize, Deserialize)]
pub struct DecodeResult {
    pub xml: String,
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
pub fn decode(data: &[u8]) -> IDecodeResult {
    let (collection, encoding) = kbinxml::from_slice(data).unwrap();
    let xml_vec = kbinxml::to_text_xml(&collection).unwrap();
    let xml_str = String::from_utf8(xml_vec).unwrap();

    // xml_str split \n and trim all lines and join
    let xml = xml_str
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("");

    let result = DecodeResult {
        xml,
        encoding: encoding.to_byte(),
    };

    serde_wasm_bindgen::to_value(&result)
        .unwrap()
        .unchecked_into::<IDecodeResult>()
}
