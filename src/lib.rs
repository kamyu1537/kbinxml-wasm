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
    #[wasm_bindgen(typescript_type = "{ data: string, encoding: number }")]
    pub type IResult;
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub data: String,
    pub encoding: u8,
}

// XML -> Binary
#[wasm_bindgen]
pub fn to_bin(data: &[u8]) -> IResult {
    let (collection, encoding) = kbinxml::from_text_xml(data).unwrap();
    let options = Options::with_encoding(encoding);
    let result = kbinxml::to_binary_with_options(options, &collection).unwrap();

    let result = Result {
        data: base64::encode(result),
        encoding: encoding.to_byte(),
    };

    serde_wasm_bindgen::to_value(&result)
        .unwrap()
        .unchecked_into::<IResult>()
}

// XML -> Binary
#[wasm_bindgen]
pub fn to_bin_with_encoding(data: &[u8], encoding_byte: u8) -> IResult {
    let (collection, _encoding) = kbinxml::from_text_xml(data).unwrap();
    let encoding_type = EncodingType::from_byte(encoding_byte).unwrap();
    let options = Options::with_encoding(encoding_type);
    let result = kbinxml::to_binary_with_options(options, &collection).unwrap();

    let result = Result {
        data: base64::encode(result),
        encoding: encoding_type.to_byte(),
    };

    serde_wasm_bindgen::to_value(&result)
        .unwrap()
        .unchecked_into::<IResult>()
}

// Binary -> XML
#[wasm_bindgen]
pub fn to_xml(data: &[u8]) -> IResult {
    let (collection, encoding) = kbinxml::from_slice(data).unwrap();
    let xml_vec = kbinxml::to_text_xml(&collection).unwrap();
    let xml_str = String::from_utf8(xml_vec).unwrap();

    // xml_str split \n and trim all lines and join
    let xml = xml_str
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("");

    let result = Result {
        data: xml,
        encoding: encoding.to_byte(),
    };

    serde_wasm_bindgen::to_value(&result)
        .unwrap()
        .unchecked_into::<IResult>()
}
