mod utils;

use core::fmt;
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
    pub type ResultDataType;

    #[wasm_bindgen(typescript_type = "ToBinOptionType")]
    pub type ToBinOptionType;
}

#[derive(Serialize, Deserialize)]
pub struct ResultData {
    pub data: String,
    pub encoding: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToBinOptions {
    pub compression: Option<bool>,
    pub encoding: Option<u8>,
}

#[derive(Debug, Clone)]
enum KbinXMLError {
    InvalidXML,
    ToBinary,
    ToXml,
    ResultConversion,
    InvalidOption,
    Utf8Error,
}

impl std::error::Error for KbinXMLError {}
impl fmt::Display for KbinXMLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KbinXMLError::InvalidXML => write!(f, "invalid xml"),
            KbinXMLError::ToBinary => write!(f, "to_binary error"),
            KbinXMLError::ToXml => write!(f, "to_xml error"),
            KbinXMLError::ResultConversion => write!(f, "result data conversion error"),
            KbinXMLError::InvalidOption => write!(f, "invalid option"),
            KbinXMLError::Utf8Error => write!(f, "from_utf8 error"),
        }
    }
}

fn from_text_xml(xml: String) -> Result<(kbinxml::NodeCollection, EncodingType), KbinXMLError> {
    let data = xml.as_bytes();
    if let Ok((collection, encoding)) = kbinxml::from_text_xml(data) {
        Ok((collection, encoding))
    } else {
        Err(KbinXMLError::InvalidXML)
    }
}

fn to_binary_with_options(
    options: Options,
    collection: &kbinxml::NodeCollection,
) -> Result<Vec<u8>, KbinXMLError> {
    if let Ok(binary) = kbinxml::to_binary_with_options(options, collection) {
        Ok(binary)
    } else {
        Err(KbinXMLError::ToBinary)
    }
}

fn to_value(data: &ResultData) -> Result<JsValue, KbinXMLError> {
    if let Ok(result) = serde_wasm_bindgen::to_value(data) {
        Ok(result)
    } else {
        Err(KbinXMLError::ResultConversion)
    }
}

fn get_to_binary_options(opts: ToBinOptionType) -> Result<ToBinOptions, KbinXMLError> {
    if let Ok(options) = serde_wasm_bindgen::from_value(JsValue::from(opts)) {
        Ok(options)
    } else {
        Err(KbinXMLError::InvalidOption)
    }
}

fn build_to_binary_options(opts: ToBinOptions) -> Result<Options, KbinXMLError> {
    let mut options = Options::builder();

    if let Some(compression) = opts.compression {
        options.compression(match compression {
            true => kbinxml::CompressionType::Compressed,
            false => kbinxml::CompressionType::Uncompressed,
        });
    } else {
        options.compression(kbinxml::CompressionType::Uncompressed);
    }

    if let Some(encoding) = opts.encoding {
        options.encoding(EncodingType::from_byte(encoding).unwrap());
    }

    Ok(options.build())
}

fn get_binary_from_slice(
    data: &[u8],
) -> Result<(kbinxml::NodeCollection, EncodingType), KbinXMLError> {
    if let Ok((collection, encoding)) = kbinxml::from_slice(data) {
        Ok((collection, encoding))
    } else {
        Err(KbinXMLError::InvalidXML)
    }
}

fn to_text_xml(collection: &kbinxml::NodeCollection) -> Result<String, KbinXMLError> {
    if let Ok(buf) = kbinxml::to_text_xml(collection) {
        if let Ok(str) = String::from_utf8(buf) {
            Ok(str)
        } else {
            Err(KbinXMLError::Utf8Error)
        }
    } else {
        Err(KbinXMLError::ToXml)
    }
}

// XML -> Binary
#[wasm_bindgen]
pub fn to_bin(xml: String) -> Result<ResultDataType, JsError> {
    let (collection, encoding) = from_text_xml(xml)?;
    let options = Options::with_encoding(encoding);
    let binary = to_binary_with_options(options, &collection)?;

    let result = ResultData {
        data: base64::encode(binary),
        encoding: encoding.to_byte(),
    };

    let result = to_value(&result)?;
    Ok(result.unchecked_into::<ResultDataType>())
}

// XML -> Binary
#[wasm_bindgen]
pub fn to_bin_with_options(xml: String, opts: ToBinOptionType) -> Result<ResultDataType, JsError> {
    let opts = get_to_binary_options(opts)?;
    let options = build_to_binary_options(opts.clone())?;
    let (collection, _encoding) = from_text_xml(xml)?;
    let binary = to_binary_with_options(options, &collection)?;

    let result = ResultData {
        data: base64::encode(binary),
        encoding: match opts.encoding {
            Some(encoding) => encoding,
            None => EncodingType::None.to_byte(),
        },
    };

    let result = to_value(&result)?;
    Ok(result.unchecked_into::<ResultDataType>())
}

// Binary -> XML
#[wasm_bindgen]
pub fn to_xml(data: &[u8]) -> Result<ResultDataType, JsError> {
    let (collection, encoding) = get_binary_from_slice(data)?;
    let xml_str = to_text_xml(&collection)?;

    //  값 변환
    let str = xml_str
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("");

    let result = ResultData {
        data: str,
        encoding: encoding.to_byte(),
    };

    let result = to_value(&result)?;
    Ok(result.unchecked_into::<ResultDataType>())
}
