mod utils;

use kbinxml::{EncodingType, Options};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// append type
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type ResultType = {
    data: string,
    encoding: number,
};

export type ToBinOptionType = {
    compression?: boolean,
    encoding?: number,
};
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ResultType")]
    pub type ResultType;

    #[wasm_bindgen(typescript_type = "ToBinOptionType")]
    pub type ToBinOptionType;
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub data: String,
    pub encoding: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ToBinOptions {
    pub compression: Option<bool>,
    pub encoding: Option<u8>,
}

// XML -> Binary
#[wasm_bindgen]
pub fn to_bin(data: &[u8]) -> ResultType {
    let (collection, encoding) = kbinxml::from_text_xml(data).unwrap();
    let options = Options::with_encoding(encoding);
    let result = kbinxml::to_binary_with_options(options, &collection).unwrap();

    let result = Result {
        data: base64::encode(result),
        encoding: encoding.to_byte(),
    };

    serde_wasm_bindgen::to_value(&result)
        .unwrap()
        .unchecked_into::<ResultType>()
}

// XML -> Binary
#[wasm_bindgen]
pub fn to_bin_with_options(data: &[u8], opts: ToBinOptionType) -> ResultType {
    let options: ToBinOptions = serde_wasm_bindgen::from_value(JsValue::from(opts)).unwrap();

    let (collection, _encoding) = kbinxml::from_text_xml(data).unwrap();
    let mut kbin_options = Options::builder();

    let mut encoding_type = EncodingType::UTF_8;
    if let Some(encoding) = options.encoding {
        encoding_type = EncodingType::from_byte(encoding).unwrap();
        kbin_options.encoding(encoding_type);
    }

    if let Some(compression) = options.compression {
        kbin_options.compression(match compression {
            true => kbinxml::CompressionType::Compressed,
            false => kbinxml::CompressionType::Uncompressed,
        });
    } else {
        kbin_options.compression(kbinxml::CompressionType::Uncompressed);
    }

    let result = kbinxml::to_binary_with_options(kbin_options.build(), &collection).unwrap();

    let result = Result {
        data: base64::encode(result),
        encoding: encoding_type.to_byte(),
    };

    serde_wasm_bindgen::to_value(&result)
        .unwrap()
        .unchecked_into::<ResultType>()
}

// Binary -> XML
#[wasm_bindgen]
pub fn to_xml(data: &[u8]) -> ResultType {
    let (collection, encoding) = kbinxml::from_slice(data).unwrap();
    let xml_vec = kbinxml::to_text_xml(&collection).unwrap();
    let xml_str = String::from_utf8(xml_vec).unwrap();

    // xml_str split \n and trim all lines and join
    let xml = xml_str
        .split('\n')
        .skip(1)
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("");

    let result = Result {
        data: xml,
        encoding: encoding.to_byte(),
    };

    serde_wasm_bindgen::to_value(&result)
        .unwrap()
        .unchecked_into::<ResultType>()
}
