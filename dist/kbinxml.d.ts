/* tslint:disable */
/* eslint-disable */
/**
* @param {string} xml
* @returns {ResultType}
*/
export function to_bin(xml: string): ResultType;
/**
* @param {string} xml
* @param {ToBinOptionType} opts
* @returns {ResultType}
*/
export function to_bin_with_options(xml: string, opts: ToBinOptionType): ResultType;
/**
* @param {Uint8Array} data
* @returns {ResultType}
*/
export function to_xml(data: Uint8Array): ResultType;

export type ResultType = {
    data: string,
    encoding: EncodingType,
};

export type ToBinOptionType = {
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


