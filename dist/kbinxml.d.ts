/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} data
* @param {number} encoding_byte
* @returns {string}
*/
export function encode(data: Uint8Array, encoding_byte: number): string;
/**
* @param {Uint8Array} data
* @returns {DecodeResult}
*/
export function decode(data: Uint8Array): DecodeResult;
/**
*/
export class DecodeResult {
  free(): void;
/**
* @param {string} base64
* @param {number} encoding
* @returns {DecodeResult}
*/
  static new(base64: string, encoding: number): DecodeResult;
/**
* @returns {string}
*/
  base64(): string;
/**
* @returns {number}
*/
  encoding(): number;
}
