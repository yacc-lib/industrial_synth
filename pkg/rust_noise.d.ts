/* tslint:disable */
/* eslint-disable */

export class IndustrialEngine {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  set_chorus(mix: number, rate_hz: number, depth_ms: number, feedback: number): void;
  set_cutoff(freq: number): void;
  set_jitter(amount: number, band_hz: number): void;
  set_limiter(amount: number): void;
  set_comb_mix(mix: number): void;
  set_feedback(fb: number): void;
  set_filter_q(q: number): void;
  set_fm_level(level: number): void;
  set_fm_ratio(ratio: number): void;
  set_ring_mix(mix: number): void;
  set_vector_x(x: number): void;
  set_vector_y(y: number): void;
  all_notes_off(): void;
  set_bit_depth(depth: number): void;
  set_comb_damp(damp: number): void;
  set_comb_freq(freq: number): void;
  set_mod_index(mod_index: number): void;
  set_post_gain(gain: number): void;
  set_resonance(res: number): void;
  set_chaos_mode(mode: number): void;
  set_chaos_rate(rate: number): void;
  set_drift_type(drift_type: number): void;
  set_grain_size(size: number): void;
  set_ring_ratio(ratio: number): void;
  set_saturation(drive: number, mix: number): void;
  set_synth_type(synth_type: number): void;
  set_drift_speed(speed: number): void;
  set_fold_amount(amount: number): void;
  set_mod_routing(lfo_cutoff: number, lfo_fold: number, sh_cutoff: number, sh_fold: number, sh_bit: number, jitter_pitch: number): void;
  set_noise_drone(enabled: boolean): void;
  set_noise_level(level: number): void;
  set_sample_hold(rate_hz: number, depth: number, slew_ms: number): void;
  set_sync_amount(amount: number): void;
  set_drift_amount(amount: number): void;
  set_filter_drive(drive: number): void;
  set_chaos_enabled(enabled: boolean): void;
  set_comb_feedback(feedback: number): void;
  set_diffusion_mix(mix: number): void;
  set_grain_density(density: number): void;
  set_filter_damping(damping: number): void;
  set_harmonics_count(count: number): void;
  set_modal_stiffness(stiffness: number): void;
  set_harmonic_rolloff(rolloff: number): void;
  set_wave_morph_speed(speed: number): void;
  set_noise_gate_follow(follow: boolean): void;
  set_phase_dist_amount(amount: number): void;
  set_wavetable_position(position: number): void;
  set_modal_inharmonicity(inharmonicity: number): void;
  set_phase_resonance_point(point: number): void;
  static new(sample_rate: number): IndustrialEngine;
  panic(): void;
  note_on(note_id: number, frequency: number, mod_index: number): void;
  process(output: Float32Array): void;
  set_lfo(rate_hz: number, depth: number, shape: number): void;
  set_sub(level: number, detune: number): void;
  note_off(note_id: number): void;
  set_adsr(attack_ms: number, decay_ms: number, sustain: number, release_ms: number): void;
  set_tilt(value: number): void;
  set_drive(val: number): void;
  set_spasm(value: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_industrialengine_free: (a: number, b: number) => void;
  readonly industrialengine_all_notes_off: (a: number) => void;
  readonly industrialengine_new: (a: number) => number;
  readonly industrialengine_note_off: (a: number, b: number) => void;
  readonly industrialengine_note_on: (a: number, b: number, c: number, d: number) => void;
  readonly industrialengine_panic: (a: number) => void;
  readonly industrialengine_process: (a: number, b: number, c: number, d: any) => void;
  readonly industrialengine_set_adsr: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly industrialengine_set_bit_depth: (a: number, b: number) => void;
  readonly industrialengine_set_chaos_enabled: (a: number, b: number) => void;
  readonly industrialengine_set_chaos_mode: (a: number, b: number) => void;
  readonly industrialengine_set_chaos_rate: (a: number, b: number) => void;
  readonly industrialengine_set_chorus: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly industrialengine_set_comb_damp: (a: number, b: number) => void;
  readonly industrialengine_set_comb_feedback: (a: number, b: number) => void;
  readonly industrialengine_set_comb_freq: (a: number, b: number) => void;
  readonly industrialengine_set_comb_mix: (a: number, b: number) => void;
  readonly industrialengine_set_cutoff: (a: number, b: number) => void;
  readonly industrialengine_set_diffusion_mix: (a: number, b: number) => void;
  readonly industrialengine_set_drift_amount: (a: number, b: number) => void;
  readonly industrialengine_set_drift_speed: (a: number, b: number) => void;
  readonly industrialengine_set_drift_type: (a: number, b: number) => void;
  readonly industrialengine_set_drive: (a: number, b: number) => void;
  readonly industrialengine_set_feedback: (a: number, b: number) => void;
  readonly industrialengine_set_filter_damping: (a: number, b: number) => void;
  readonly industrialengine_set_filter_drive: (a: number, b: number) => void;
  readonly industrialengine_set_filter_q: (a: number, b: number) => void;
  readonly industrialengine_set_fm_level: (a: number, b: number) => void;
  readonly industrialengine_set_fm_ratio: (a: number, b: number) => void;
  readonly industrialengine_set_fold_amount: (a: number, b: number) => void;
  readonly industrialengine_set_grain_density: (a: number, b: number) => void;
  readonly industrialengine_set_grain_size: (a: number, b: number) => void;
  readonly industrialengine_set_harmonic_rolloff: (a: number, b: number) => void;
  readonly industrialengine_set_harmonics_count: (a: number, b: number) => void;
  readonly industrialengine_set_jitter: (a: number, b: number, c: number) => void;
  readonly industrialengine_set_lfo: (a: number, b: number, c: number, d: number) => void;
  readonly industrialengine_set_limiter: (a: number, b: number) => void;
  readonly industrialengine_set_mod_index: (a: number, b: number) => void;
  readonly industrialengine_set_mod_routing: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly industrialengine_set_modal_inharmonicity: (a: number, b: number) => void;
  readonly industrialengine_set_modal_stiffness: (a: number, b: number) => void;
  readonly industrialengine_set_noise_drone: (a: number, b: number) => void;
  readonly industrialengine_set_noise_gate_follow: (a: number, b: number) => void;
  readonly industrialengine_set_noise_level: (a: number, b: number) => void;
  readonly industrialengine_set_phase_dist_amount: (a: number, b: number) => void;
  readonly industrialengine_set_phase_resonance_point: (a: number, b: number) => void;
  readonly industrialengine_set_post_gain: (a: number, b: number) => void;
  readonly industrialengine_set_resonance: (a: number, b: number) => void;
  readonly industrialengine_set_ring_mix: (a: number, b: number) => void;
  readonly industrialengine_set_ring_ratio: (a: number, b: number) => void;
  readonly industrialengine_set_sample_hold: (a: number, b: number, c: number, d: number) => void;
  readonly industrialengine_set_saturation: (a: number, b: number, c: number) => void;
  readonly industrialengine_set_spasm: (a: number, b: number) => void;
  readonly industrialengine_set_sub: (a: number, b: number, c: number) => void;
  readonly industrialengine_set_sync_amount: (a: number, b: number) => void;
  readonly industrialengine_set_synth_type: (a: number, b: number) => void;
  readonly industrialengine_set_tilt: (a: number, b: number) => void;
  readonly industrialengine_set_vector_x: (a: number, b: number) => void;
  readonly industrialengine_set_vector_y: (a: number, b: number) => void;
  readonly industrialengine_set_wave_morph_speed: (a: number, b: number) => void;
  readonly industrialengine_set_wavetable_position: (a: number, b: number) => void;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
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
