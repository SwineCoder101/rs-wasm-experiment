/* tslint:disable */
/* eslint-disable */

/**
 * Computes the FFT of a time series data array
 * 
 * # Arguments
 * * `time_series` - A vector of f32 values representing the time series data
 * 
 * # Returns
 * A vector of f32 values representing the FFT result, interleaved as [real0, imag0, real1, imag1, ...]
 */
export function compute_fft(time_series: Float32Array): Float32Array;

/**
 * Computes the FFT magnitude spectrum of a time series data array
 * 
 * # Arguments
 * * `time_series` - A vector of f32 values representing the time series data
 * 
 * # Returns
 * A vector of f32 values representing the magnitude of each frequency bin
 */
export function compute_fft_magnitude(time_series: Float32Array): Float32Array;

/**
 * Convenience function: Computes FFT magnitude from a file content string
 * Combines parsing and FFT computation in one step
 * 
 * # Arguments
 * * `file_content` - String content of a text file with one number per line
 * 
 * # Returns
 * A vector of f32 values representing the magnitude of each frequency bin
 */
export function compute_fft_magnitude_from_string(file_content: string): Float32Array;

/**
 * Parses a text file content string into a time series array
 * Each line should contain a single numeric value
 * 
 * # Arguments
 * * `file_content` - String content of a text file with one number per line
 * 
 * # Returns
 * A vector of f32 values parsed from the file content, or empty vector on error
 */
export function parse_time_series_from_string(file_content: string): Float32Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly compute_fft: (a: number, b: number) => [number, number];
  readonly compute_fft_magnitude: (a: number, b: number) => [number, number];
  readonly compute_fft_magnitude_from_string: (a: number, b: number) => [number, number];
  readonly parse_time_series_from_string: (a: number, b: number) => [number, number];
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
