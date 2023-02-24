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

/**
 * @param compression
 * @param encoding 0x00: UTF-8, 0x20: UTF-16LE, 0x40: UTF-16BE, 0x60: UTF-32LE, 0x80: UTF-32BE, 0xA0: Shift-JIS
 */
export type BinaryOptions = {
    compression?: boolean,
    encoding?: 0x00 | 0x20 | 0x40 | 0x60 | 0x80 | 0xA0,
};


