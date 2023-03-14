use crate::types::{BinaryOptions, BinaryOptionsType, BinaryResult, XmlResult};
use kbinxml::{EncodingType, Options};
use wasm_bindgen::prelude::*;

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

#[allow(dead_code)]
pub fn set_log_level() {
    use std::sync::Once;
    static SET_LOG_LEVEL: Once = Once::new();
    SET_LOG_LEVEL.call_once(|| {
        console_log::init_with_level(log::Level::Trace).expect("error initializing log");
    });
}

pub fn from_text_xml(xml: String) -> Result<(kbinxml::NodeCollection, EncodingType), JsError> {
    let data = xml.as_bytes();
    match kbinxml::from_text_xml(data) {
        Ok((collection, encoding)) => Ok((collection, encoding)),
        Err(err) => Err(JsError::new(format!("KbinXMLError: {}", err).as_str())),
    }
}

pub fn from_slice_xml(xml: &[u8]) -> Result<(kbinxml::NodeCollection, EncodingType), JsError> {
    match kbinxml::from_text_xml(xml) {
        Ok((collection, encoding)) => Ok((collection, encoding)),
        Err(err) => Err(JsError::new(format!("KbinXMLError: {}", err).as_str())),
    }
}

pub fn to_binary_with_options(
    options: Options,
    collection: &kbinxml::NodeCollection,
) -> Result<Vec<u8>, JsError> {
    match kbinxml::to_binary_with_options(options, collection) {
        Ok(binary) => Ok(binary),
        Err(err) => Err(JsError::new(format!("KbinXMLError: {}", err).as_str())),
    }
}

pub fn to_xml_result(data: &XmlResult) -> Result<JsValue, JsError> {
    match serde_wasm_bindgen::to_value(data) {
        Ok(result) => Ok(result),
        Err(err) => Err(JsError::new(format!("KbinXMLError: {}", err).as_str())),
    }
}

pub fn to_binary_result(data: &BinaryResult) -> Result<JsValue, JsError> {
    match serde_wasm_bindgen::to_value(data) {
        Ok(result) => Ok(result),
        Err(err) => Err(JsError::new(format!("KbinXMLError: {}", err).as_str())),
    }
}

pub fn get_to_binary_options(opts: BinaryOptionsType) -> Result<BinaryOptions, JsError> {
    match serde_wasm_bindgen::from_value(JsValue::from(opts)) {
        Ok(options) => Ok(options),
        Err(err) => Err(JsError::new(format!("KbinXMLError: {}", err).as_str())),
    }
}

pub fn build_to_binary_options(opts: BinaryOptions) -> Result<Options, JsError> {
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
        match EncodingType::from_byte(encoding) {
            Ok(encoding) => options.encoding(encoding),
            Err(err) => return Err(JsError::new(format!("KbinXMLError: {}", err).as_str())),
        };
    };

    Ok(options.build())
}

pub fn get_binary_from_slice(
    data: &[u8],
) -> Result<(kbinxml::NodeCollection, EncodingType), JsError> {
    match kbinxml::from_slice(data) {
        Ok((collection, encoding)) => Ok((collection, encoding)),
        Err(err) => Err(JsError::new(format!("KbinXMLError: {}", err).as_str())),
    }
}

pub fn to_text_xml(collection: &kbinxml::NodeCollection) -> Result<String, JsError> {
    match kbinxml::to_text_xml(collection) {
        Ok(buf) => match String::from_utf8(buf) {
            Ok(str) => Ok(str),
            Err(err) => Err(JsError::new(format!("KbinXMLError: {}", err).as_str())),
        },
        Err(err) => Err(JsError::new(format!("KbinXMLError: {}", err).as_str())),
    }
}

pub fn remove_indent(str: &str) -> String {
    str.split('\n')
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("")
}
