/* tslint:disable */
/* eslint-disable */
/**
 * @param {string} csv_content
 * @returns {any}
 */
export function parse_csv(csv_content: string): any;
/**
 * @param {string} csv_content
 * @param {string} column_name
 * @returns {Float64Array}
 */
export function get_column_values(csv_content: string, column_name: string): Float64Array;
/**
 * @param {Float64Array} values
 * @returns {any}
 */
export function calculate_stats(values: Float64Array): any;
/**
 * @param {Float64Array} x_values
 * @param {Float64Array} y_values
 * @returns {any}
 */
export function linear_regression(x_values: Float64Array, y_values: Float64Array): any;
/**
 * @param {string} x_col
 * @param {string} y_column
 * @param {string} csv_content
 * @returns {any}
 */
export function analyze_data(x_col: string, y_column: string, csv_content: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly parse_csv: (a: number, b: number) => Array;
  readonly get_column_values: (a: number, b: number, c: number, d: number) => Array;
  readonly calculate_stats: (a: number, b: number) => Array;
  readonly linear_regression: (a: number, b: number, c: number, d: number) => Array;
  readonly analyze_data: (a: number, b: number, c: number, d: number, e: number, f: number) => Array;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
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
