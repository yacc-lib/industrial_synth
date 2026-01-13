import init, { IndustrialEngine } from './pkg/rust_noise.js';

class RustNoiseProcessor extends AudioWorkletProcessor {
    constructor(options) {
        super();
        this.engine = null;
        this.ready = false;

        // メインスレッドからのメッセージ処理
        this.port.onmessage = async (event) => {
            const { type } = event.data;
            
            if (type === 'init-wasm') {
                try {
                    // Wasmの初期化
                    await init(event.data.wasmBytes);
                    // Rust側のエンジンをインスタンス化
                    this.engine = IndustrialEngine.new(sampleRate);
                    this.ready = true;
                    console.log('[Processor] ✓ Rust Engine Ready in Worklet');
                    console.log('[Processor] ✓ BUILD 020.2 - Wave Synthesis + Filter Character ACTIVE (with polyfill.js)');
                    
                    // メインスレッドに準備完了を通知
                    this.port.postMessage({ type: 'engine-ready' });
                } catch (e) {
                    console.error('[Processor] ✗ Wasm init failed', e);
                    this.port.postMessage({ type: 'engine-error', error: e.message });
                }
            } 
            
            // ========== BASIC PARAMETERS ==========
            else if (type === 'param') {
                if (this.engine) {
                    this.engine.set_drive(event.data.value);
                    this.port.postMessage({ type: 'ack', key: 'drive', value: event.data.value, timestamp: Date.now() });
                }
            } else if (type === 'noise-level') {
                if (this.engine) {
                    this.engine.set_noise_level(event.data.value);
                    this.port.postMessage({ type: 'ack', key: 'noise_level', value: event.data.value, timestamp: Date.now() });
                }
            } else if (type === 'fm-level') {
                if (this.engine) {
                    this.engine.set_fm_level(event.data.value);
                    this.port.postMessage({ type: 'ack', key: 'fm_level', value: event.data.value, timestamp: Date.now() });
                }
            } 
            // Phase 1 追加: cutoff, resonance, feedback, fold-amount, bit-depth
            else if (type === 'cutoff') {
                if (this.engine) {
                    this.engine.set_cutoff(event.data.value);
                    this.port.postMessage({ type: 'ack', key: 'cutoff', value: event.data.value, timestamp: Date.now() });
                }
            } else if (type === 'resonance') {
                if (this.engine) {
                    this.engine.set_resonance(event.data.value);
                    this.port.postMessage({ type: 'ack', key: 'resonance', value: event.data.value, timestamp: Date.now() });
                }
            } else if (type === 'feedback') {
                if (this.engine) {
                    this.engine.set_feedback(event.data.value);
                    this.port.postMessage({ type: 'ack', key: 'feedback', value: event.data.value, timestamp: Date.now() });
                }
            } else if (type === 'fold-amount') {
                if (this.engine) {
                    this.engine.set_fold_amount(event.data.value);
                    this.port.postMessage({ type: 'ack', key: 'fold_amount', value: event.data.value, timestamp: Date.now() });
                }
            } else if (type === 'bit-depth') {
                if (this.engine) {
                    this.engine.set_bit_depth(event.data.value);
                    this.port.postMessage({ type: 'ack', key: 'bit_depth', value: event.data.value, timestamp: Date.now() });
                }
            } else if (type === 'noise-gate-follow') {
                if (this.engine) {
                    this.engine.set_noise_gate_follow(event.data.value);
                    this.port.postMessage({ type: 'ack', key: 'noise_gate_follow', value: event.data.value, timestamp: Date.now() });
                }
            }
            // Phase 1 追加: ADSR
            else if (type === 'set-adsr') {
                if (this.engine) {
                    const { attack, decay, sustain, release } = event.data;
                    this.engine.set_adsr(attack, decay, sustain, release);
                    this.port.postMessage({ type: 'ack', key: 'adsr', value: { attack, decay, sustain, release }, timestamp: Date.now() });
                }
            } 
            
            // ========== WAVE SYNTHESIS (BUILD 019) ==========
            else if (type === 'set-synth-type') {
                if (this.engine) {
                    this.engine.set_synth_type(event.data.synthType || event.data.value);
                    console.log(`[Processor] Synth Type: ${event.data.synthType || event.data.value}`);
                }
            } else if (type === 'set-fm-ratio') {
                if (this.engine) {
                    this.engine.set_fm_ratio(event.data.value);
                    console.log(`[Processor] FM Ratio: ${event.data.value}`);
                }
            } else if (type === 'set-wavetable-pos') {
                if (this.engine) {
                    this.engine.set_wavetable_position(event.data.value);
                    console.log(`[Processor] Wavetable Position: ${event.data.value}`);
                }
            } else if (type === 'set-wave-morph') {
                if (this.engine) {
                    this.engine.set_wave_morph_speed(event.data.value);
                    console.log(`[Processor] Wave Morph Speed: ${event.data.value}`);
                }
            } else if (type === 'set-harmonics-count') {
                if (this.engine) {
                    this.engine.set_harmonics_count(event.data.value);
                    console.log(`[Processor] Harmonics Count: ${event.data.value}`);
                }
            } else if (type === 'set-harmonic-rolloff') {
                if (this.engine) {
                    this.engine.set_harmonic_rolloff(event.data.value);
                    console.log(`[Processor] Harmonic Rolloff: ${event.data.value}`);
                }
            } else if (type === 'set-phase-dist-amt') {
                if (this.engine) {
                    this.engine.set_phase_dist_amount(event.data.value);
                    console.log(`[Processor] Phase Dist Amount: ${event.data.value}`);
                }
            } else if (type === 'set-phase-resonance') {
                if (this.engine) {
                    this.engine.set_phase_resonance_point(event.data.value);
                    console.log(`[Processor] Phase Resonance: ${event.data.value}`);
                }
            } else if (type === 'set-vector-x') {
                if (this.engine) {
                    this.engine.set_vector_x(event.data.value);
                    console.log(`[Processor] Vector X: ${event.data.value}`);
                }
            } else if (type === 'set-vector-y') {
                if (this.engine) {
                    this.engine.set_vector_y(event.data.value);
                    console.log(`[Processor] Vector Y: ${event.data.value}`);
                }
            } else if (type === 'set-grain-size') {
                if (this.engine) {
                    this.engine.set_grain_size(event.data.value);
                    console.log(`[Processor] Grain Size: ${event.data.value}`);
                }
            } else if (type === 'set-grain-density') {
                if (this.engine) {
                    this.engine.set_grain_density(event.data.value);
                    console.log(`[Processor] Grain Density: ${event.data.value}`);
                }
            } else if (type === 'set-modal-stiffness') {
                if (this.engine) {
                    this.engine.set_modal_stiffness(event.data.value);
                    console.log(`[Processor] Modal Stiffness: ${event.data.value}`);
                }
            } else if (type === 'set-modal-inharmonicity') {
                if (this.engine) {
                    this.engine.set_modal_inharmonicity(event.data.value);
                    console.log(`[Processor] Modal Inharmonicity: ${event.data.value}`);
                }
            } 
            
            // ========== FILTER CHARACTER (BUILD 019) ==========
            else if (type === 'filter-q') {
                if (this.engine) {
                    this.engine.set_filter_q(event.data.value);
                    console.log(`[Processor] Filter Q: ${event.data.value}`);
                }
            } else if (type === 'filter-damping') {
                if (this.engine) {
                    this.engine.set_filter_damping(event.data.value);
                    console.log(`[Processor] Filter Damping: ${event.data.value}`);
                }
            } else if (type === 'filter-drive') {
                if (this.engine) {
                    this.engine.set_filter_drive(event.data.value);
                    console.log(`[Processor] Filter Drive: ${event.data.value}`);
                }
            }
            
            // ========== CHAOS ENGINE (BUILD 017) ==========
            else if (type === 'chaos-mode') {
                if (this.engine) {
                    this.engine.set_chaos_mode(event.data.value);
                    const modes = ['Logistic', 'Lorenz', 'DoublePendulum'];
                    console.log(`[Processor] Chaos Mode: ${modes[event.data.value] || event.data.value}`);
                }
            } else if (type === 'chaos-rate') {
                if (this.engine) {
                    this.engine.set_chaos_rate(event.data.value);
                    console.log(`[Processor] Chaos Rate: ${event.data.value}`);
                }
            } else if (type === 'chaos-enabled') {
                if (this.engine) {
                    this.engine.set_chaos_enabled(event.data.value);
                    console.log(`[Processor] Chaos Enabled: ${event.data.value}`);
                }
            } else if (type === 'drift-amount') {
                if (this.engine) {
                    this.engine.set_drift_amount(event.data.value);
                    console.log(`[Processor] Drift Amount: ${event.data.value}`);
                }
            } else if (type === 'drift-speed') {
                if (this.engine) {
                    this.engine.set_drift_speed(event.data.value);
                    console.log(`[Processor] Drift Speed: ${event.data.value}`);
                }
            } else if (type === 'drift-type') {
                if (this.engine) {
                    this.engine.set_drift_type(event.data.value);
                    const types = ['Jitter', 'Wander'];
                    console.log(`[Processor] Drift Type: ${types[event.data.value] || event.data.value}`);
                }
            } else if (type === 'diffusion-mix') {
                if (this.engine) {
                    this.engine.set_diffusion_mix(event.data.value);
                    console.log(`[Processor] Diffusion Mix: ${event.data.value}`);
                }
            } 
            // Phase 1 追加: sub, saturation, tilt, post-gain, limiter
            else if (type === 'set-sub') {
                if (this.engine) {
                    const { level, detune } = event.data;
                    this.engine.set_sub(level, detune);
                }
            } else if (type === 'set-saturation') {
                if (this.engine) {
                    const { drive, mix } = event.data;
                    this.engine.set_saturation(drive, mix);
                }
            } else if (type === 'set-tilt') {
                if (this.engine) {
                    this.engine.set_tilt(event.data.value);
                }
            } else if (type === 'set-post-gain') {
                if (this.engine) {
                    this.engine.set_post_gain(event.data.value);
                }
            } else if (type === 'set-limiter') {
                if (this.engine) {
                    this.engine.set_limiter(event.data.value);
                }
            }
            // Phase 1 追加: LFO, Sample & Hold, Jitter, Chorus
            else if (type === 'set-lfo') {
                if (this.engine) {
                    const { rate, depth, shape } = event.data;
                    this.engine.set_lfo(rate, depth, shape);
                }
            } else if (type === 'set-sample-hold') {
                if (this.engine) {
                    const { rate, depth, slew } = event.data;
                    this.engine.set_sample_hold(rate, depth, slew);
                }
            } else if (type === 'set-jitter') {
                if (this.engine) {
                    const { amount, bandHz } = event.data;
                    this.engine.set_jitter(amount, bandHz);
                }
            } else if (type === 'set-chorus') {
                if (this.engine) {
                    const { mix, rate, depth, feedback } = event.data;
                    this.engine.set_chorus(mix, rate, depth, feedback);
                }
            } else if (type === 'set-spasm') {
                if (this.engine) {
                    this.engine.set_spasm(event.data.value);
                }
            } else if (type === 'set-mod-routing') {
                if (this.engine) {
                    const { lfoCutoff, lfoFold, shCutoff, shFold, shBit, jitterPitch } = event.data;
                    this.engine.set_mod_routing(lfoCutoff, lfoFold, shCutoff, shFold, shBit, jitterPitch);
                }
            }
            // Phase 2: XYパッド対応 - modIndex
            else if (type === 'mod-index') {
                if (this.engine) {
                    this.engine.set_mod_index(event.data.value);
                }
            } 
            

            // ========== BUILD 022: INDUSTRIAL TRINITY ==========
            else if (type === 'sync-amount') {
                if (this.engine) {
                    this.engine.set_sync_amount(event.data.value);
                    this.port.postMessage({ 
                        type: 'ack', 
                        key: 'sync_amount', 
                        value: event.data.value, 
                        timestamp: Date.now() 
                    });
                }
            } else if (type === 'ring-mix') {
                if (this.engine) {
                    this.engine.set_ring_mix(event.data.value);
                    this.port.postMessage({ 
                        type: 'ack', 
                        key: 'ring_mix', 
                        value: event.data.value, 
                        timestamp: Date.now() 
                    });
                }
            } else if (type === 'ring-ratio') {
                if (this.engine) {
                    this.engine.set_ring_ratio(event.data.value);
                    this.port.postMessage({ 
                        type: 'ack', 
                        key: 'ring_ratio', 
                        value: event.data.value, 
                        timestamp: Date.now() 
                    });
                }
            } else if (type === 'comb-mix') {
                if (this.engine) {
                    this.engine.set_comb_mix(event.data.value);
                    this.port.postMessage({ 
                        type: 'ack', 
                        key: 'comb_mix', 
                        value: event.data.value, 
                        timestamp: Date.now() 
                    });
                }
            } else if (type === 'comb-freq') {
                if (this.engine) {
                    this.engine.set_comb_freq(event.data.value);
                    this.port.postMessage({ 
                        type: 'ack', 
                        key: 'comb_freq', 
                        value: event.data.value, 
                        timestamp: Date.now() 
                    });
                }
            } else if (type === 'comb-feedback') {
                if (this.engine) {
                    this.engine.set_comb_feedback(event.data.value);
                    this.port.postMessage({ 
                        type: 'ack', 
                        key: 'comb_feedback', 
                        value: event.data.value, 
                        timestamp: Date.now() 
                    });
                }
            } else if (type === 'comb-damp') {
                if (this.engine) {
                    this.engine.set_comb_damp(event.data.value);
                    this.port.postMessage({ 
                        type: 'ack', 
                        key: 'comb_damp', 
                        value: event.data.value, 
                        timestamp: Date.now() 
                    });
                }
            }

            // ========== NOTE CONTROL ==========
            else if (type === 'note-on') {
                if (this.engine) {
                    const { noteId, frequency, modIndex } = event.data;
                    this.engine.note_on(noteId, frequency, modIndex);
                }
            } else if (type === 'note-off') {
                if (this.engine) {
                    const { noteId } = event.data;
                    this.engine.note_off(noteId);
                }
            }
            
            // ========== UNKNOWN TYPE WARNING (Phase 1 - 飾り検出) ==========
            else {
                console.warn(`[Processor] ⚠️ UNKNOWN MESSAGE TYPE: "${type}"`, event.data);
                this.port.postMessage({ 
                    type: 'warning', 
                    message: `Unknown parameter type: ${type}`,
                    originalType: type,
                    timestamp: Date.now()
                });
            }
        };
    }

    process(inputs, outputs, parameters) {
        if (!this.ready || !this.engine) return true;

        const output = outputs[0];
        const channel0 = output[0]; // モノラル出力

        // Rustにバッファを渡し、DSP処理を実行
        this.engine.process(channel0);

        // ステレオの場合、右チャンネルにコピー
        if (output.length > 1) {
            output[1].set(channel0);
        }

        return true;
    }
}

registerProcessor('rust-noise-processor', RustNoiseProcessor);
