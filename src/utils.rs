use crate::error::KbinXMLError;
use crate::types::{BinaryOptions, BinaryOptionsType, BinaryResult, XmlResult};
use kbinxml::{EncodingType, Options};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn from_text_xml(xml: String) -> Result<(kbinxml::NodeCollection, EncodingType), KbinXMLError> {
    let data = xml.as_bytes();
    if let Ok((collection, encoding)) = kbinxml::from_text_xml(data) {
        Ok((collection, encoding))
    } else {
        Err(KbinXMLError::InvalidXML)
    }
}

pub fn to_binary_with_options(
    options: Options,
    collection: &kbinxml::NodeCollection,
) -> Result<Vec<u8>, KbinXMLError> {
    if let Ok(binary) = kbinxml::to_binary_with_options(options, collection) {
        Ok(binary)
    } else {
        Err(KbinXMLError::ToBinary)
    }
}

pub fn to_xml_result(data: &XmlResult) -> Result<JsValue, KbinXMLError> {
    if let Ok(result) = serde_wasm_bindgen::to_value(data) {
        Ok(result)
    } else {
        Err(KbinXMLError::ResultConversion)
    }
}

pub fn to_binary_result(data: &BinaryResult) -> Result<JsValue, KbinXMLError> {
    if let Ok(result) = serde_wasm_bindgen::to_value(data) {
        Ok(result)
    } else {
        Err(KbinXMLError::ResultConversion)
    }
}

pub fn get_to_binary_options(opts: BinaryOptionsType) -> Result<BinaryOptions, KbinXMLError> {
    if let Ok(options) = serde_wasm_bindgen::from_value(JsValue::from(opts)) {
        Ok(options)
    } else {
        Err(KbinXMLError::InvalidOption)
    }
}

pub fn build_to_binary_options(opts: BinaryOptions) -> Result<Options, KbinXMLError> {
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
        if let Ok(encoding_type) = EncodingType::from_byte(encoding) {
            options.encoding(encoding_type);
        } else {
            return Err(KbinXMLError::InvalidEncodingType);
        }
    }

    Ok(options.build())
}

pub fn get_binary_from_slice(
    data: &[u8],
) -> Result<(kbinxml::NodeCollection, EncodingType), KbinXMLError> {
    if let Ok((collection, encoding)) = kbinxml::from_slice(data) {
        Ok((collection, encoding))
    } else {
        Err(KbinXMLError::InvalidXML)
    }
}

pub fn to_text_xml(collection: &kbinxml::NodeCollection) -> Result<String, KbinXMLError> {
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
