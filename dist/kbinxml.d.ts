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
    encoding: number,
};

export type ToBinOptionType = {
    compression?: boolean,
    encoding?: number,
};


