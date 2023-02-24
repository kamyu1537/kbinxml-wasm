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
* @returns {{ xml: string, encoding: number }}
*/
export function decode(data: Uint8Array): { xml: string, encoding: number };
