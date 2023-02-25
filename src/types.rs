use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct XmlResult {
    pub data: String,
    pub encoding: u8,
}

#[derive(Serialize, Deserialize)]
pub struct BinaryResult {
    pub data: Box<[u8]>,
    pub encoding: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinaryOptions {
    pub compression: Option<bool>,
    pub encoding: Option<u8>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "XmlResult")]
    pub type XmlResultType;

    #[wasm_bindgen(typescript_type = "BinaryResult")]
    pub type BinaryResultType;

    #[wasm_bindgen(typescript_type = "BinaryOptions")]
    pub type BinaryOptionsType;
}
