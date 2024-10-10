/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
* @returns {string}
*/
export function greet(name: string): string;
/**
* @param {bigint} left
* @param {bigint} right
* @returns {bigint}
*/
export function add(left: bigint, right: bigint): bigint;
/**
*/
export class GranularSynth {
  free(): void;
/**
* @param {number} sample_rate
*/
  constructor(sample_rate: number);
/**
* @param {Float32Array} sample_data
*/
  load_sample(sample_data: Float32Array): void;
/**
* @param {number} position
* @param {number} duration
*/
  add_grain(position: number, duration: number): void;
/**
* @returns {Float32Array}
*/
  generate(): Float32Array;
}
/**
*/
export class KarplusStrong {
  free(): void;
/**
* @param {number} frequency
* @param {number} sample_rate
*/
  constructor(frequency: number, sample_rate: number);
/**
* @returns {number}
*/
  tick(): number;
/**
* @param {number} decay
*/
  set_decay(decay: number): void;
}
/**
*/
export class Random {
  free(): void;
/**
*/
  constructor();
/**
* @param {number} min
* @param {number} max
* @returns {number}
*/
  gen_range(min: number, max: number): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_granularsynth_free: (a: number, b: number) => void;
  readonly granularsynth_new: (a: number) => number;
  readonly granularsynth_load_sample: (a: number, b: number, c: number) => void;
  readonly granularsynth_add_grain: (a: number, b: number, c: number) => void;
  readonly granularsynth_generate: (a: number, b: number) => void;
  readonly greet: (a: number, b: number, c: number) => void;
  readonly add: (a: number, b: number) => number;
  readonly __wbg_karplusstrong_free: (a: number, b: number) => void;
  readonly karplusstrong_new: (a: number, b: number) => number;
  readonly karplusstrong_tick: (a: number) => number;
  readonly karplusstrong_set_decay: (a: number, b: number) => void;
  readonly __wbg_random_free: (a: number, b: number) => void;
  readonly random_new: () => number;
  readonly random_gen_range: (a: number, b: number, c: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
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
