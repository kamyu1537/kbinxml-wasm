/* tslint:disable */
/* eslint-disable */
/**
* @param {string} xml
* @returns {BinaryResult}
*/
export function to_bin(xml: string): BinaryResult;
/**
* @param {string} xml
* @param {BinaryOptions} opts
* @returns {BinaryResult}
*/
export function to_bin_with_options(xml: string, opts: BinaryOptions): BinaryResult;
/**
* @param {Uint8Array} data
* @returns {XmlResult}
*/
export function to_xml(data: Uint8Array): XmlResult;

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

export enum EncodingType {
    None = 0x00,
    ASCII = 0x20,
    ISO_8859_1 = 0x40,
    EUC_JP = 0x60,
    SHIFT_JIS = 0x80,
    UTF_8 = 0xA0,
}


