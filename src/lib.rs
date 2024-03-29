mod types;
mod utils;

use kbinxml::{EncodingType, Options};
use types::{BinaryOptionsType, BinaryResult, BinaryResultType, XmlResult, XmlResultType};
use utils::{
    build_to_binary_options, from_slice_xml, from_text_xml, get_binary_from_slice,
    get_to_binary_options, remove_indent, to_binary_result, to_binary_with_options, to_text_xml,
    to_xml_result,
};

use wasm_bindgen::{prelude::*, JsCast};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// append type
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/** 0x00: None, 0x20: ASCII, 0x40: ISO_8859_1, 0x60: EUC_JP, 0x80: SHIFT_JIS, 0xA0: UTF_8 */
type EncodingType = 0x00 | 0x20 | 0x40 | 0x60 | 0x80 | 0xA0;

export type XmlResult = {
    data: string,
    encoding: EncodingType,
};

export type BinaryResult = {
    data: Uint8Array,
    encoding: EncodingType,
};

export type BinaryOptions = {
    compression?: boolean,
    encoding?: EncodingType,
};
"#;

// XML -> Binary
#[wasm_bindgen]
pub fn to_bin(xml: String) -> Result<BinaryResultType, JsError> {
    utils::set_panic_hook();

    let (collection, encoding) = from_text_xml(xml)?;
    let options = Options::with_encoding(encoding);
    let binary = to_binary_with_options(options, &collection)?;

    let result = BinaryResult {
        data: binary.into_boxed_slice(),
        encoding: encoding.to_byte(),
    };

    let result = to_binary_result(&result)?;
    Ok(result.unchecked_into::<BinaryResultType>())
}

// XML -> Binary
#[wasm_bindgen]
pub fn slice_to_bin(xml: &[u8]) -> Result<BinaryResultType, JsError> {
    utils::set_panic_hook();

    let (collection, encoding) = from_slice_xml(xml)?;
    let options = Options::with_encoding(encoding);
    let binary = to_binary_with_options(options, &collection)?;

    let result = BinaryResult {
        data: binary.into_boxed_slice(),
        encoding: encoding.to_byte(),
    };

    let result = to_binary_result(&result)?;
    Ok(result.unchecked_into::<BinaryResultType>())
}

// XML -> Binary
#[wasm_bindgen]
pub fn to_bin_with_options(
    xml: String,
    opts: BinaryOptionsType,
) -> Result<BinaryResultType, JsError> {
    utils::set_panic_hook();

    let opts = get_to_binary_options(opts)?;
    let options = build_to_binary_options(opts.clone())?;
    let (collection, _encoding) = from_text_xml(xml)?;
    let binary = to_binary_with_options(options, &collection)?;

    let result = BinaryResult {
        data: binary.into_boxed_slice(),
        encoding: match opts.encoding {
            Some(encoding) => encoding,
            None => EncodingType::None.to_byte(),
        },
    };

    let result = to_binary_result(&result)?;
    Ok(result.unchecked_into::<BinaryResultType>())
}

// XML -> Binary
#[wasm_bindgen]
pub fn slice_to_bin_with_options(
    xml: &[u8],
    opts: BinaryOptionsType,
) -> Result<BinaryResultType, JsError> {
    utils::set_panic_hook();

    let opts = get_to_binary_options(opts)?;
    let options = build_to_binary_options(opts.clone())?;
    let (collection, _encoding) = from_slice_xml(xml)?;
    let binary = to_binary_with_options(options, &collection)?;

    let result = BinaryResult {
        data: binary.into_boxed_slice(),
        encoding: match opts.encoding {
            Some(encoding) => encoding,
            None => EncodingType::None.to_byte(),
        },
    };

    let result = to_binary_result(&result)?;
    Ok(result.unchecked_into::<BinaryResultType>())
}

// Binary -> XML
#[wasm_bindgen]
pub fn to_xml(data: &[u8], pretty: Option<bool>) -> Result<XmlResultType, JsError> {
    utils::set_panic_hook();

    let (collection, encoding) = get_binary_from_slice(data)?;
    let mut str = to_text_xml(&collection)?;

    if pretty.is_none() || !pretty.unwrap() {
        str = remove_indent(&str);
    }

    let result = XmlResult {
        data: str,
        encoding: encoding.to_byte(),
    };

    let result = to_xml_result(&result)?;
    Ok(result.unchecked_into::<XmlResultType>())
}

#[wasm_bindgen]
pub fn debug_mode() {
    utils::set_panic_hook();
    utils::set_log_level();
}
