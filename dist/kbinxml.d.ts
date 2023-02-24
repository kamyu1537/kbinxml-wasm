/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} data
* @returns {ResultType}
*/
export function to_bin(data: Uint8Array): ResultType;
/**
* @param {Uint8Array} data
* @param {ToBinOptionType} opts
* @returns {ResultType}
*/
export function to_bin_with_options(data: Uint8Array, opts: ToBinOptionType): ResultType;
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


