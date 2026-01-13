let wasm;

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const IndustrialEngineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_industrialengine_free(ptr >>> 0, 1));

export class IndustrialEngine {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(IndustrialEngine.prototype);
        obj.__wbg_ptr = ptr;
        IndustrialEngineFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IndustrialEngineFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_industrialengine_free(ptr, 0);
    }
    /**
     * @param {number} mix
     * @param {number} rate_hz
     * @param {number} depth_ms
     * @param {number} feedback
     */
    set_chorus(mix, rate_hz, depth_ms, feedback) {
        wasm.industrialengine_set_chorus(this.__wbg_ptr, mix, rate_hz, depth_ms, feedback);
    }
    /**
     * @param {number} freq
     */
    set_cutoff(freq) {
        wasm.industrialengine_set_cutoff(this.__wbg_ptr, freq);
    }
    /**
     * @param {number} amount
     * @param {number} band_hz
     */
    set_jitter(amount, band_hz) {
        wasm.industrialengine_set_jitter(this.__wbg_ptr, amount, band_hz);
    }
    /**
     * @param {number} amount
     */
    set_limiter(amount) {
        wasm.industrialengine_set_limiter(this.__wbg_ptr, amount);
    }
    /**
     * @param {number} mix
     */
    set_comb_mix(mix) {
        wasm.industrialengine_set_comb_mix(this.__wbg_ptr, mix);
    }
    /**
     * @param {number} fb
     */
    set_feedback(fb) {
        wasm.industrialengine_set_feedback(this.__wbg_ptr, fb);
    }
    /**
     * @param {number} q
     */
    set_filter_q(q) {
        wasm.industrialengine_set_filter_q(this.__wbg_ptr, q);
    }
    /**
     * @param {number} level
     */
    set_fm_level(level) {
        wasm.industrialengine_set_fm_level(this.__wbg_ptr, level);
    }
    /**
     * @param {number} ratio
     */
    set_fm_ratio(ratio) {
        wasm.industrialengine_set_fm_ratio(this.__wbg_ptr, ratio);
    }
    /**
     * @param {number} mix
     */
    set_ring_mix(mix) {
        wasm.industrialengine_set_ring_mix(this.__wbg_ptr, mix);
    }
    /**
     * @param {number} x
     */
    set_vector_x(x) {
        wasm.industrialengine_set_vector_x(this.__wbg_ptr, x);
    }
    /**
     * @param {number} y
     */
    set_vector_y(y) {
        wasm.industrialengine_set_vector_y(this.__wbg_ptr, y);
    }
    all_notes_off() {
        wasm.industrialengine_all_notes_off(this.__wbg_ptr);
    }
    /**
     * @param {number} depth
     */
    set_bit_depth(depth) {
        wasm.industrialengine_set_bit_depth(this.__wbg_ptr, depth);
    }
    /**
     * @param {number} damp
     */
    set_comb_damp(damp) {
        wasm.industrialengine_set_comb_damp(this.__wbg_ptr, damp);
    }
    /**
     * @param {number} freq
     */
    set_comb_freq(freq) {
        wasm.industrialengine_set_comb_freq(this.__wbg_ptr, freq);
    }
    /**
     * @param {number} mod_index
     */
    set_mod_index(mod_index) {
        wasm.industrialengine_set_mod_index(this.__wbg_ptr, mod_index);
    }
    /**
     * @param {number} gain
     */
    set_post_gain(gain) {
        wasm.industrialengine_set_post_gain(this.__wbg_ptr, gain);
    }
    /**
     * @param {number} res
     */
    set_resonance(res) {
        wasm.industrialengine_set_resonance(this.__wbg_ptr, res);
    }
    /**
     * @param {number} mode
     */
    set_chaos_mode(mode) {
        wasm.industrialengine_set_chaos_mode(this.__wbg_ptr, mode);
    }
    /**
     * @param {number} rate
     */
    set_chaos_rate(rate) {
        wasm.industrialengine_set_chaos_rate(this.__wbg_ptr, rate);
    }
    /**
     * @param {number} drift_type
     */
    set_drift_type(drift_type) {
        wasm.industrialengine_set_drift_type(this.__wbg_ptr, drift_type);
    }
    /**
     * @param {number} size
     */
    set_grain_size(size) {
        wasm.industrialengine_set_grain_size(this.__wbg_ptr, size);
    }
    /**
     * @param {number} ratio
     */
    set_ring_ratio(ratio) {
        wasm.industrialengine_set_ring_ratio(this.__wbg_ptr, ratio);
    }
    /**
     * @param {number} drive
     * @param {number} mix
     */
    set_saturation(drive, mix) {
        wasm.industrialengine_set_saturation(this.__wbg_ptr, drive, mix);
    }
    /**
     * @param {number} synth_type
     */
    set_synth_type(synth_type) {
        wasm.industrialengine_set_synth_type(this.__wbg_ptr, synth_type);
    }
    /**
     * @param {number} speed
     */
    set_drift_speed(speed) {
        wasm.industrialengine_set_drift_speed(this.__wbg_ptr, speed);
    }
    /**
     * @param {number} amount
     */
    set_fold_amount(amount) {
        wasm.industrialengine_set_fold_amount(this.__wbg_ptr, amount);
    }
    /**
     * @param {number} lfo_cutoff
     * @param {number} lfo_fold
     * @param {number} sh_cutoff
     * @param {number} sh_fold
     * @param {number} sh_bit
     * @param {number} jitter_pitch
     */
    set_mod_routing(lfo_cutoff, lfo_fold, sh_cutoff, sh_fold, sh_bit, jitter_pitch) {
        wasm.industrialengine_set_mod_routing(this.__wbg_ptr, lfo_cutoff, lfo_fold, sh_cutoff, sh_fold, sh_bit, jitter_pitch);
    }
    /**
     * @param {boolean} enabled
     */
    set_noise_drone(enabled) {
        wasm.industrialengine_set_noise_drone(this.__wbg_ptr, enabled);
    }
    /**
     * @param {number} level
     */
    set_noise_level(level) {
        wasm.industrialengine_set_noise_level(this.__wbg_ptr, level);
    }
    /**
     * @param {number} rate_hz
     * @param {number} depth
     * @param {number} slew_ms
     */
    set_sample_hold(rate_hz, depth, slew_ms) {
        wasm.industrialengine_set_sample_hold(this.__wbg_ptr, rate_hz, depth, slew_ms);
    }
    /**
     * @param {number} amount
     */
    set_sync_amount(amount) {
        wasm.industrialengine_set_sync_amount(this.__wbg_ptr, amount);
    }
    /**
     * @param {number} amount
     */
    set_drift_amount(amount) {
        wasm.industrialengine_set_drift_amount(this.__wbg_ptr, amount);
    }
    /**
     * @param {number} drive
     */
    set_filter_drive(drive) {
        wasm.industrialengine_set_filter_drive(this.__wbg_ptr, drive);
    }
    /**
     * @param {boolean} enabled
     */
    set_chaos_enabled(enabled) {
        wasm.industrialengine_set_chaos_enabled(this.__wbg_ptr, enabled);
    }
    /**
     * @param {number} feedback
     */
    set_comb_feedback(feedback) {
        wasm.industrialengine_set_comb_feedback(this.__wbg_ptr, feedback);
    }
    /**
     * @param {number} mix
     */
    set_diffusion_mix(mix) {
        wasm.industrialengine_set_diffusion_mix(this.__wbg_ptr, mix);
    }
    /**
     * @param {number} density
     */
    set_grain_density(density) {
        wasm.industrialengine_set_grain_density(this.__wbg_ptr, density);
    }
    /**
     * @param {number} damping
     */
    set_filter_damping(damping) {
        wasm.industrialengine_set_filter_damping(this.__wbg_ptr, damping);
    }
    /**
     * @param {number} count
     */
    set_harmonics_count(count) {
        wasm.industrialengine_set_harmonics_count(this.__wbg_ptr, count);
    }
    /**
     * @param {number} stiffness
     */
    set_modal_stiffness(stiffness) {
        wasm.industrialengine_set_modal_stiffness(this.__wbg_ptr, stiffness);
    }
    /**
     * @param {number} rolloff
     */
    set_harmonic_rolloff(rolloff) {
        wasm.industrialengine_set_harmonic_rolloff(this.__wbg_ptr, rolloff);
    }
    /**
     * @param {number} speed
     */
    set_wave_morph_speed(speed) {
        wasm.industrialengine_set_wave_morph_speed(this.__wbg_ptr, speed);
    }
    /**
     * @param {boolean} follow
     */
    set_noise_gate_follow(follow) {
        wasm.industrialengine_set_noise_gate_follow(this.__wbg_ptr, follow);
    }
    /**
     * @param {number} amount
     */
    set_phase_dist_amount(amount) {
        wasm.industrialengine_set_phase_dist_amount(this.__wbg_ptr, amount);
    }
    /**
     * @param {number} position
     */
    set_wavetable_position(position) {
        wasm.industrialengine_set_wavetable_position(this.__wbg_ptr, position);
    }
    /**
     * @param {number} inharmonicity
     */
    set_modal_inharmonicity(inharmonicity) {
        wasm.industrialengine_set_modal_inharmonicity(this.__wbg_ptr, inharmonicity);
    }
    /**
     * @param {number} point
     */
    set_phase_resonance_point(point) {
        wasm.industrialengine_set_phase_resonance_point(this.__wbg_ptr, point);
    }
    /**
     * @param {number} sample_rate
     * @returns {IndustrialEngine}
     */
    static new(sample_rate) {
        const ret = wasm.industrialengine_new(sample_rate);
        return IndustrialEngine.__wrap(ret);
    }
    panic() {
        wasm.industrialengine_panic(this.__wbg_ptr);
    }
    /**
     * @param {number} note_id
     * @param {number} frequency
     * @param {number} mod_index
     */
    note_on(note_id, frequency, mod_index) {
        wasm.industrialengine_note_on(this.__wbg_ptr, note_id, frequency, mod_index);
    }
    /**
     * @param {Float32Array} output
     */
    process(output) {
        var ptr0 = passArrayF32ToWasm0(output, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.industrialengine_process(this.__wbg_ptr, ptr0, len0, output);
    }
    /**
     * @param {number} rate_hz
     * @param {number} depth
     * @param {number} shape
     */
    set_lfo(rate_hz, depth, shape) {
        wasm.industrialengine_set_lfo(this.__wbg_ptr, rate_hz, depth, shape);
    }
    /**
     * @param {number} level
     * @param {number} detune
     */
    set_sub(level, detune) {
        wasm.industrialengine_set_sub(this.__wbg_ptr, level, detune);
    }
    /**
     * @param {number} note_id
     */
    note_off(note_id) {
        wasm.industrialengine_note_off(this.__wbg_ptr, note_id);
    }
    /**
     * @param {number} attack_ms
     * @param {number} decay_ms
     * @param {number} sustain
     * @param {number} release_ms
     */
    set_adsr(attack_ms, decay_ms, sustain, release_ms) {
        wasm.industrialengine_set_adsr(this.__wbg_ptr, attack_ms, decay_ms, sustain, release_ms);
    }
    /**
     * @param {number} value
     */
    set_tilt(value) {
        wasm.industrialengine_set_tilt(this.__wbg_ptr, value);
    }
    /**
     * @param {number} val
     */
    set_drive(val) {
        wasm.industrialengine_set_drive(this.__wbg_ptr, val);
    }
    /**
     * @param {number} value
     */
    set_spasm(value) {
        wasm.industrialengine_set_spasm(this.__wbg_ptr, value);
    }
}
if (Symbol.dispose) IndustrialEngine.prototype[Symbol.dispose] = IndustrialEngine.prototype.free;

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg___wbindgen_copy_to_typed_array_db832bc4df7216c1 = function(arg0, arg1, arg2) {
        new Uint8Array(arg2.buffer, arg2.byteOffset, arg2.byteLength).set(getArrayU8FromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_externrefs;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedFloat32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('rust_noise_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
