/* tslint:disable */
/* eslint-disable */
/**
* @param {string} xml
* @returns {BinaryResult}
*/
export function to_bin(xml: string): BinaryResult;
/**
* @param {Uint8Array} xml
* @returns {BinaryResult}
*/
export function slice_to_bin(xml: Uint8Array): BinaryResult;
/**
* @param {string} xml
* @param {BinaryOptions} opts
* @returns {BinaryResult}
*/
export function to_bin_with_options(xml: string, opts: BinaryOptions): BinaryResult;
/**
* @param {Uint8Array} xml
* @param {BinaryOptions} opts
* @returns {BinaryResult}
*/
export function slice_to_bin_with_options(xml: Uint8Array, opts: BinaryOptions): BinaryResult;
/**
* @param {Uint8Array} data
* @param {boolean | undefined} pretty
* @returns {XmlResult}
*/
export function to_xml(data: Uint8Array, pretty?: boolean): XmlResult;

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


