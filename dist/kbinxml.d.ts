/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} data
* @returns {{ xml: string, encoding: number }}
*/
export function to_bin(data: Uint8Array): { xml: string, encoding: number };
/**
* @param {Uint8Array} data
* @param {number} encoding_byte
* @returns {{ xml: string, encoding: number }}
*/
export function to_bin_with_encoding(data: Uint8Array, encoding_byte: number): { xml: string, encoding: number };
/**
* @param {Uint8Array} data
* @returns {{ xml: string, encoding: number }}
*/
export function to_xml(data: Uint8Array): { xml: string, encoding: number };
