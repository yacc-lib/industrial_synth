use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

/// LFO (Low Frequency Oscillator)
struct LFO {
    phase: f32,
    rate_hz: f32,
    sample_rate: f32,
}

impl LFO {
    fn new(sample_rate: f32) -> Self {
        Self { phase: 0.0, rate_hz: 1.0, sample_rate }
    }
    
    fn set_rate(&mut self, hz: f32) {
        self.rate_hz = hz.max(0.05).min(40.0);
    }
    
    fn tick(&mut self, shape: i32) -> f32 {
        let phase_inc = self.rate_hz / self.sample_rate;
        self.phase = (self.phase + phase_inc).fract();
        
        match shape {
            0 => (self.phase * 2.0 * PI).sin(),
            1 => 1.0 - (self.phase * 4.0 - 2.0).abs(),
            2 => if self.phase < 0.5 { 1.0 } else { -1.0 },
            _ => (self.phase * 2.0 * PI).sin(),
        }
    }
    
    fn reset(&mut self) {
        self.phase = 0.0;
    }
}

/// Sample & Hold
struct SampleHold {
    counter: f32,
    rate_hz: f32,
    current_value: f32,
    target_value: f32,
    slew_coeff: f32,
    sample_rate: f32,
    seed: u32,
}

impl SampleHold {
    fn new(sample_rate: f32) -> Self {
        Self {
            counter: 0.0,
            rate_hz: 5.0,
            current_value: 0.0,
            target_value: 0.0,
            slew_coeff: 1.0,
            sample_rate,
            seed: 54321,
        }
    }
    
    fn set_rate(&mut self, hz: f32) {
        self.rate_hz = hz.max(1.0).min(200.0);
    }
    
    fn set_slew(&mut self, ms: f32) {
        let clamped = ms.max(0.0).min(50.0);
        self.slew_coeff = if clamped < 0.1 {
            1.0
        } else {
            1.0 - (-1.0 / (clamped * 0.001 * self.sample_rate)).exp()
        };
    }
    
    fn tick(&mut self) -> f32 {
        let samples_per_hold = (self.sample_rate / self.rate_hz).max(1.0);
        
        if self.counter >= samples_per_hold {
            self.counter = 0.0;
            let mut x = self.seed;
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            self.seed = x;
            self.target_value = (x as f32 / std::u32::MAX as f32) * 2.0 - 1.0;
        }
        
        self.current_value += (self.target_value - self.current_value) * self.slew_coeff;
        self.counter += 1.0;
        
        self.current_value
    }
    
    fn reset(&mut self) {
        self.counter = 0.0;
        self.current_value = 0.0;
        self.target_value = 0.0;
    }
}

/// Jitter
struct Jitter {
    seed: u32,
    sample_rate: f32,
}

impl Jitter {
    fn new(sample_rate: f32) -> Self {
        Self { seed: 98765, sample_rate }
    }
    
    fn tick(&mut self, band_hz: f32) -> f32 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.seed = x;
        
        let random = (x as f32 / std::u32::MAX as f32) * 2.0 - 1.0;
        let bandwidth = (band_hz / self.sample_rate).min(0.01);
        
        random * bandwidth
    }
}

/// Chorus
struct Chorus {
    buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
    sample_rate: f32,
}

impl Chorus {
    fn new(sample_rate: f32) -> Self {
        let max_delay_ms = 50.0;
        let buffer_size = ((max_delay_ms / 1000.0) * sample_rate) as usize;
        
        Self {
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            lfo_phase: 0.0,
            sample_rate,
        }
    }
    
    fn process(&mut self, input: f32, rate_hz: f32, depth_ms: f32, feedback: f32) -> f32 {
        let delay_samples = (depth_ms / 1000.0 * self.sample_rate) as usize;
        let lfo = (self.lfo_phase * 2.0 * PI).sin();
        let modulated_delay = (delay_samples as f32 * (1.0 + lfo * 0.5)).max(1.0) as usize;
        
        let read_pos = (self.write_pos + self.buffer.len() - modulated_delay) % self.buffer.len();
        let delayed = self.buffer[read_pos];
        
        self.buffer[self.write_pos] = input + delayed * feedback.max(0.0).min(0.99);
        self.write_pos = (self.write_pos + 1) % self.buffer.len();
        
        self.lfo_phase += rate_hz / self.sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }
        
        delayed
    }
}

/// State Variable Filter
struct SVFilter {
    ic1eq: f32,
    ic2eq: f32,
    sample_rate: f32,
}

impl SVFilter {
    fn new(sample_rate: f32) -> Self {
        Self { ic1eq: 0.0, ic2eq: 0.0, sample_rate }
    }
    
    fn process(&mut self, input: f32, cutoff: f32, resonance: f32) -> f32 {
        let g = (PI * cutoff / self.sample_rate).tan();
        let k = 2.0 - 2.0 * resonance.min(0.99);
        
        let v0 = input;
        let v1 = (self.ic1eq + g * (v0 - self.ic2eq)) / (1.0 + g * (g + k));
        let v2 = self.ic2eq + g * v1;
        
        self.ic1eq = 2.0 * v1 - self.ic1eq;
        self.ic2eq = 2.0 * v2 - self.ic2eq;
        
        v2
    }
}

/// Tilt EQ
struct TiltEQ {
    low_shelf: f32,
    high_shelf: f32,
}

impl TiltEQ {
    fn new() -> Self {
        Self { low_shelf: 0.0, high_shelf: 0.0 }
    }
    
    fn process(&mut self, input: f32, tilt: f32, sample_rate: f32) -> f32 {
        let tilt_clamped = tilt.max(-1.0).min(1.0);
        
        let low_coeff = 1.0 - (-2.0 * PI * 200.0 / sample_rate).exp();
        let high_coeff = 1.0 - (-2.0 * PI * 2000.0 / sample_rate).exp();
        
        self.low_shelf += (input - self.low_shelf) * low_coeff;
        self.high_shelf += (input - self.high_shelf) * high_coeff;
        
        let bass_boost = self.low_shelf * tilt_clamped.max(0.0) * 0.5;
        let treble_boost = (input - self.high_shelf) * (-tilt_clamped).max(0.0) * 0.5;
        
        input + bass_boost + treble_boost
    }
}

/// BUILD 023: Enhanced Comb Filter with tanh in feedback loop
struct CombFilter {
    buffer: Vec<f32>,
    write_pos: usize,
    damping_state: f32,
    sample_rate: f32,
}

impl CombFilter {
    fn new(sample_rate: f32) -> Self {
        let max_delay_ms = 20.0;
        let buffer_size = ((max_delay_ms / 1000.0) * sample_rate) as usize + 1;
        
        Self {
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            damping_state: 0.0,
            sample_rate,
        }
    }
    
    fn process(&mut self, input: f32, freq: f32, feedback: f32, damp: f32) -> f32 {
        // BUILD 023: Exponential frequency mapping (50Hz - 1000Hz)
        let freq_exp = 50.0 * (20.0_f32).powf(freq / 1000.0);
        let freq_clamped = freq_exp.max(50.0).min(1000.0);
        let delay_samples = (self.sample_rate / freq_clamped) as usize;
        let delay_samples = delay_samples.min(self.buffer.len() - 1).max(1);
        
        let read_pos = (self.write_pos + self.buffer.len() - delay_samples) % self.buffer.len();
        let delayed = self.buffer[read_pos];
        
        // Damping filter
        let damp_coeff = 1.0 - damp.max(0.0).min(1.0);
        self.damping_state += (delayed - self.damping_state) * damp_coeff;
        
        // BUILD 023: tanh in feedback loop to prevent divergence
        let feedback_clamped = feedback.max(0.0).min(0.99);
        let feedback_signal = (self.damping_state * feedback_clamped).tanh();
        
        self.buffer[self.write_pos] = input + feedback_signal;
        self.write_pos = (self.write_pos + 1) % self.buffer.len();
        
        self.damping_state
    }
    
    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.damping_state = 0.0;
        self.write_pos = 0;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct Voice {
    active: bool,
    note_id: i32,
    frequency: f32,
    phase: f32,
    mod_phase: f32,
    mod_index_base: f32,
    sample_rate: f32,
    
    // Hard Sync
    master_phase: f32,
    slave_phase: f32,
    sync_amount: f32,
    
    // Ring Modulation
    ring_phase: f32,
    ring_ratio: f32,
    ring_mix: f32,
    
    // BUILD 023: Wavetable
    wavetable_position: f32,
    
    feedback: f32,
    last_output: f32,
    
    envelope_state: EnvelopeState,
    env_counter: f32,
    current_level: f32,
    attack_ms: f32,
    decay_ms: f32,
    sustain: f32,
    release_ms: f32,
    
    mod_env_counter: f32,
    mod_env_level: f32,
    
    lfo: LFO,
    sample_hold: SampleHold,
    jitter: Jitter,
    
    cutoff: f32,
    resonance: f32,
    fold_amount: f32,
    bit_depth: f32,
    
    filter: SVFilter,
    tilt_eq: TiltEQ,
    chorus: Chorus,
    comb_filter: CombFilter,
    
    comb_mix: f32,
    comb_freq: f32,
    comb_feedback: f32,
    comb_damp: f32,
    
    // BUILD 023: Granular synthesis
    grain_counter: f32,
    grain_phase: f32,
    grain_envelope: f32,
    noise_seed: u32,
}

impl Voice {
    fn new(sample_rate: f32) -> Self {
        Self {
            active: false,
            note_id: -1,
            frequency: 440.0,
            phase: 0.0,
            mod_phase: 0.0,
            mod_index_base: 2.0,
            sample_rate,
            
            master_phase: 0.0,
            slave_phase: 0.0,
            sync_amount: 0.0,
            
            ring_phase: 0.0,
            ring_ratio: 1.0,
            ring_mix: 0.0,
            
            wavetable_position: 0.0,
            
            feedback: 0.3,
            last_output: 0.0,
            
            envelope_state: EnvelopeState::Idle,
            env_counter: 0.0,
            current_level: 0.0,
            attack_ms: 5.0,
            decay_ms: 200.0,
            sustain: 0.7,
            release_ms: 300.0,
            
            mod_env_counter: 0.0,
            mod_env_level: 0.0,
            
            lfo: LFO::new(sample_rate),
            sample_hold: SampleHold::new(sample_rate),
            jitter: Jitter::new(sample_rate),
            
            cutoff: 2000.0,
            resonance: 0.5,
            fold_amount: 2.0,
            bit_depth: 12.0,
            
            filter: SVFilter::new(sample_rate),
            tilt_eq: TiltEQ::new(),
            chorus: Chorus::new(sample_rate),
            comb_filter: CombFilter::new(sample_rate),
            
            comb_mix: 0.0,
            comb_freq: 200.0,
            comb_feedback: 0.5,
            comb_damp: 0.5,
            
            grain_counter: 0.0,
            grain_phase: 0.0,
            grain_envelope: 0.0,
            noise_seed: 123456,
        }
    }
    
    pub fn note_on(&mut self, note_id: i32, freq: f32, mod_idx: f32) {
        self.active = true;
        self.note_id = note_id;
        self.frequency = freq;
        self.mod_index_base = mod_idx;
        self.envelope_state = EnvelopeState::Attack;
        self.env_counter = 0.0;
        self.mod_env_counter = 0.0;
        
        self.lfo.reset();
        self.sample_hold.reset();
        self.comb_filter.reset();
    }
    
    pub fn note_off(&mut self, note_id: i32) -> bool {
        if self.note_id == note_id {
            self.envelope_state = EnvelopeState::Release;
            self.env_counter = 0.0;
            true
        } else {
            false
        }
    }
    
    pub fn force_stop(&mut self) {
        self.active = false;
        self.envelope_state = EnvelopeState::Idle;
        self.current_level = 0.0;
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    pub fn get_note_id(&self) -> i32 {
        self.note_id
    }
    
    pub fn set_adsr(&mut self, attack_ms: f32, decay_ms: f32, sustain: f32, release_ms: f32) {
        self.attack_ms = attack_ms.max(0.1).min(5000.0);
        self.decay_ms = decay_ms.max(0.1).min(5000.0);
        self.sustain = sustain.max(0.0).min(1.0);
        self.release_ms = release_ms.max(0.1).min(10000.0);
    }
    
    pub fn set_fold_amount(&mut self, amount: f32) {
        self.fold_amount = amount.max(1.0).min(10.0);
    }
    
    pub fn set_bit_depth(&mut self, depth: f32) {
        self.bit_depth = depth.max(1.0).min(16.0);
    }
    
    pub fn set_cutoff(&mut self, freq: f32) {
        self.cutoff = freq.max(20.0).min(20000.0);
    }
    
    pub fn set_resonance(&mut self, res: f32) {
        self.resonance = res.max(0.0).min(1.0);
    }
    
    pub fn set_feedback(&mut self, fb: f32) {
        self.feedback = fb.max(0.0).min(0.99);
    }
    
    pub fn set_sync_amount(&mut self, amount: f32) {
        self.sync_amount = amount.max(0.0).min(5.0);
    }
    
    pub fn set_ring_ratio(&mut self, ratio: f32) {
        self.ring_ratio = ratio.max(0.5).min(4.0);
    }
    
    pub fn set_ring_mix(&mut self, mix: f32) {
        self.ring_mix = mix.max(0.0).min(1.0);
    }
    
    pub fn set_comb_mix(&mut self, mix: f32) {
        self.comb_mix = mix.max(0.0).min(1.0);
    }
    
    pub fn set_comb_freq(&mut self, freq: f32) {
        self.comb_freq = freq.max(50.0).min(1000.0);
    }
    
    pub fn set_comb_feedback(&mut self, feedback: f32) {
        self.comb_feedback = feedback.max(0.0).min(0.99);
    }
    
    pub fn set_comb_damp(&mut self, damp: f32) {
        self.comb_damp = damp.max(0.0).min(1.0);
    }
    
    pub fn set_wavetable_position(&mut self, position: f32) {
        self.wavetable_position = position.max(0.0).min(1.0);
    }
    
    fn wavefold(&self, input: f32, amount: f32) -> f32 {
        let x = input * amount;
        if amount > 1.0 {
            x - 2.0 * (x * 0.5).floor()
        } else {
            input
        }
    }
    
    fn bitcrush(&self, input: f32) -> f32 {
        let steps = 2.0_f32.powf(self.bit_depth);
        (input * steps).round() / steps
    }
    
    fn calculate_envelope(&mut self) -> f32 {
        let attack_samples = (self.attack_ms / 1000.0 * self.sample_rate).max(1.0);
        let decay_samples = (self.decay_ms / 1000.0 * self.sample_rate).max(1.0);
        let release_samples = (self.release_ms / 1000.0 * self.sample_rate).max(1.0);
        
        match self.envelope_state {
            EnvelopeState::Idle => {
                self.current_level = 0.0;
                0.0
            }
            EnvelopeState::Attack => {
                if self.env_counter >= attack_samples {
                    self.envelope_state = EnvelopeState::Decay;
                    self.env_counter = 0.0;
                    self.current_level = 1.0;
                } else {
                    self.current_level = self.env_counter / attack_samples;
                }
                self.env_counter += 1.0;
                self.current_level
            }
            EnvelopeState::Decay => {
                if self.env_counter >= decay_samples {
                    self.envelope_state = EnvelopeState::Sustain;
                    self.current_level = self.sustain;
                } else {
                    let progress = self.env_counter / decay_samples;
                    self.current_level = 1.0 - progress * (1.0 - self.sustain);
                }
                self.env_counter += 1.0;
                self.current_level
            }
            EnvelopeState::Sustain => {
                self.current_level = self.sustain;
                self.sustain
            }
            EnvelopeState::Release => {
                if self.env_counter >= release_samples {
                    self.active = false;
                    self.envelope_state = EnvelopeState::Idle;
                    self.current_level = 0.0;
                    0.0
                } else {
                    let progress = self.env_counter / release_samples;
                    self.current_level = self.sustain * (1.0 - progress);
                    self.env_counter += 1.0;
                    self.current_level
                }
            }
        }
    }
    
    fn calculate_mod_env(&mut self) -> f32 {
        let mod_attack = 50.0;
        let mod_decay = 500.0;
        let mod_attack_samples = (mod_attack / 1000.0 * self.sample_rate).max(1.0);
        let mod_decay_samples = (mod_decay / 1000.0 * self.sample_rate).max(1.0);
        
        if self.mod_env_counter < mod_attack_samples {
            self.mod_env_level = self.mod_env_counter / mod_attack_samples;
        } else if self.mod_env_counter < mod_attack_samples + mod_decay_samples {
            let decay_progress = (self.mod_env_counter - mod_attack_samples) / mod_decay_samples;
            self.mod_env_level = 1.0 - decay_progress;
        } else {
            self.mod_env_level = 0.0;
        }
        
        self.mod_env_counter += 1.0;
        self.mod_env_level
    }
    
    // BUILD 023: Noise generator
    fn noise(&mut self) -> f32 {
        let mut x = self.noise_seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.noise_seed = x;
        (x as f32 / std::u32::MAX as f32) * 2.0 - 1.0
    }
    
    pub fn process_sample(&mut self, sub_level: f32, sub_detune: f32, sat_drive: f32, 
                          sat_mix: f32, tilt: f32, 
                          lfo_depth: f32, lfo_shape: i32,
                          sh_depth: f32,
                          jitter_amount: f32, jitter_band_hz: f32,
                          chorus_mix: f32, chorus_rate_hz: f32, chorus_depth_ms: f32, chorus_feedback: f32,
                          lfo_to_cutoff: f32, lfo_to_fold: f32,
                          sh_to_cutoff: f32, sh_to_fold: f32, sh_to_bit: f32,
                          jitter_to_pitch: f32,
                          chaos_lfo_value: f32,
                          spectral_drift_value: f32,
                          synth_type: usize,
                          fm_ratio: f32,
                          harmonics_count: usize,
                          harmonic_rolloff: f32,
                          phase_dist_amount: f32,
                          vector_x: f32,
                          vector_y: f32,
                          grain_size: f32,
                          grain_density: f32,
                          modal_stiffness: f32,
                          modal_inharmonicity: f32,
                          filter_q: f32,
                          filter_damping: f32,
                          filter_drive: f32) -> f32 {
        if !self.active {
            return 0.0;
        }
        
        let amp_env = self.calculate_envelope();
        let mod_env = self.calculate_mod_env();
        
        if !self.active {
            return 0.0;
        }
        
        // === MODULATORS ===
        let lfo_val = self.lfo.tick(lfo_shape) * lfo_depth;
        let sh_val = self.sample_hold.tick() * sh_depth;
        let jitter_val = self.jitter.tick(jitter_band_hz) * jitter_amount;
        
        // === MODULATION ROUTING ===
        let mod_cutoff = self.cutoff * (1.0 + lfo_val * lfo_to_cutoff + sh_val * sh_to_cutoff);
        let mod_fold = self.fold_amount * (1.0 + lfo_val * lfo_to_fold + sh_val * sh_to_fold);
        let mod_bit_depth = (self.bit_depth + sh_val * sh_to_bit * 8.0).max(1.0).min(16.0);
        
        // BUILD 023: Apply Spectral Drift to frequency
        let chaos_mod = 1.0 + chaos_lfo_value * 0.02;
        let drift_mod = 1.0 + spectral_drift_value;
        let mod_freq = self.frequency * (1.0 + jitter_val * jitter_to_pitch * 0.05) * chaos_mod * drift_mod;
        
        // === ANALOG DRIFT ===
        let drift = (self.mod_phase * 12.345 + self.phase * 67.89).sin() * 0.0001;
        let drifted_phase = self.phase + drift;
        
        // === WAVE SYNTHESIS ===
        let mut signal = match synth_type {
            0 => {
                // FM Synthesis with Hard Sync
                let sync_active = self.sync_amount > 0.0;
                
                if sync_active {
                    // BUILD 023: Exponential sync_amount mapping
                    let sync_ratio = 1.0 + (2.0_f32).powf(self.sync_amount) - 1.0;
                    let master_inc = mod_freq / self.sample_rate;
                    let slave_inc = (mod_freq * sync_ratio) / self.sample_rate;
                    
                    let old_master = self.master_phase;
                    self.master_phase = (self.master_phase + master_inc).fract();
                    
                    if old_master > self.master_phase {
                        self.slave_phase = 0.0;
                    }
                    
                    self.slave_phase = (self.slave_phase + slave_inc).fract();
                    
                    let mod_freq_carrier = mod_freq * fm_ratio;
                    let modulator = (self.mod_phase * 2.0 * PI).sin();
                    let current_mod_index = self.mod_index_base * (1.0 + mod_env * 4.0);
                    
                    let carrier_phase = self.slave_phase 
                        + (modulator * current_mod_index / (2.0 * PI))
                        + (self.last_output * self.feedback);
                    
                    (carrier_phase * 2.0 * PI).sin()
                } else {
                    let mod_freq_carrier = mod_freq * fm_ratio;
                    let modulator = (self.mod_phase * 2.0 * PI).sin();
                    let current_mod_index = self.mod_index_base * (1.0 + mod_env * 4.0);
                    
                    let carrier_phase = drifted_phase 
                        + (modulator * current_mod_index) 
                        + (self.last_output * self.feedback);
                    
                    (carrier_phase * 2.0 * PI).sin()
                }
            }
            1 => {
                // BUILD 023: Wavetable Synthesis (Sine → Triangle → Saw → Square)
                let pos = self.wavetable_position;
                let phase_2pi = drifted_phase * 2.0 * PI;
                
                if pos < 0.333 {
                    // Sine → Triangle
                    let blend = pos / 0.333;
                    let sine = phase_2pi.sin();
                    let tri = 1.0 - (drifted_phase * 4.0 - 2.0).abs();
                    sine * (1.0 - blend) + tri * blend
                } else if pos < 0.666 {
                    // Triangle → Saw
                    let blend = (pos - 0.333) / 0.333;
                    let tri = 1.0 - (drifted_phase * 4.0 - 2.0).abs();
                    let saw = 1.0 - 2.0 * drifted_phase;
                    tri * (1.0 - blend) + saw * blend
                } else {
                    // Saw → Square
                    let blend = (pos - 0.666) / 0.334;
                    let saw = 1.0 - 2.0 * drifted_phase;
                    let square = if drifted_phase < 0.5 { 1.0 } else { -1.0 };
                    saw * (1.0 - blend) + square * blend
                }
            }
            2 => {
                // Additive Synthesis
                let mut additive_output = 0.0;
                let base_phase = drifted_phase * 2.0 * PI;
                let harmonics = harmonics_count.min(16);
                
                for h in 1..=harmonics {
                    let amplitude = 1.0 / (h as f32).powf(harmonic_rolloff);
                    additive_output += (base_phase * h as f32).sin() * amplitude;
                }
                
                additive_output / (harmonics as f32).sqrt()
            }
            3 => {
                // BUILD 023: Phase Distortion (Casio CZ style)
                let amount = phase_dist_amount;
                let distorted_phase = if drifted_phase < 0.5 {
                    drifted_phase * (1.0 + amount)
                } else {
                    0.5 + (drifted_phase - 0.5) * (1.0 - amount)
                };
                let wrapped_phase = distorted_phase.fract();
                (wrapped_phase * 2.0 * PI).sin()
            }
            4 => {
                // BUILD 023: Vector Synthesis (4-corner blend)
                let phase_2pi = drifted_phase * 2.0 * PI;
                
                let sine = phase_2pi.sin();
                let saw = 1.0 - 2.0 * drifted_phase;
                let square = if drifted_phase < 0.5 { 1.0 } else { -1.0 };
                let noise = self.noise();
                
                // Bilinear interpolation
                let top = sine * (1.0 - vector_x) + saw * vector_x;
                let bottom = square * (1.0 - vector_x) + noise * vector_x;
                
                top * (1.0 - vector_y) + bottom * vector_y
            }
            5 => {
                // BUILD 023: Granular Synthesis (simplified)
                let grain_size_samples = (grain_size / 1000.0 * self.sample_rate).max(10.0);
                let grain_rate = 1.0 / (grain_size_samples / self.sample_rate);
                let grains_per_sec = grain_rate * grain_density.max(0.1).min(10.0);
                
                self.grain_counter += grains_per_sec / self.sample_rate;
                
                if self.grain_counter >= 1.0 {
                    self.grain_counter = 0.0;
                    self.grain_phase = 0.0;
                    self.grain_envelope = 0.0;
                }
                
                if self.grain_phase < 1.0 {
                    self.grain_phase += 1.0 / grain_size_samples;
                    
                    // Hann window envelope
                    let env_phase = self.grain_phase * 2.0 * PI;
                    self.grain_envelope = (1.0 - env_phase.cos()) * 0.5;
                    
                    let grain_osc = (drifted_phase * 2.0 * PI).sin();
                    grain_osc * self.grain_envelope
                } else {
                    0.0
                }
            }
            6 => {
                // BUILD 023: Modal Synthesis (metallic percussion)
                let base_freq = mod_freq;
                let mut modal_output = 0.0;
                
                // 6 modal resonators with inharmonic partials
                for mode in 1..=6 {
                    let inharmonic_ratio = (mode as f32) * (1.0 + modal_inharmonicity * 0.1 * (mode as f32));
                    let mode_freq = base_freq * inharmonic_ratio;
                    let mode_phase = (drifted_phase * inharmonic_ratio).fract();
                    
                    // Exponential decay per mode
                    let decay_rate = (1.0 - modal_stiffness * 0.1) * (mode as f32);
                    let decay_env = (-decay_rate * self.env_counter / self.sample_rate).exp();
                    
                    let mode_signal = (mode_phase * 2.0 * PI).sin();
                    let mode_amplitude = 1.0 / (mode as f32).sqrt();
                    
                    modal_output += mode_signal * mode_amplitude * decay_env;
                }
                
                modal_output / 6.0_f32.sqrt()
            }
            _ => (drifted_phase * 2.0 * PI).sin(),
        };
        
        // === RING MODULATION ===
        if self.ring_mix > 0.0 {
            let ring_freq = mod_freq * self.ring_ratio;
            let ring_inc = ring_freq / self.sample_rate;
            self.ring_phase = (self.ring_phase + ring_inc).fract();
            
            // BUILD 023: Soft square wave for ring oscillator
            let ring_sine = (self.ring_phase * 2.0 * PI).sin();
            let ring_osc = (ring_sine * 3.0).tanh();
            
            let ring_signal = signal * ring_osc;
            
            // BUILD 023: Additional tanh for "dirty" metal sound
            let ring_output = (ring_signal * 1.5).tanh();
            
            signal = signal * (1.0 - self.ring_mix) + ring_output * self.ring_mix;
        }
        
        self.last_output = signal;
        
        // === SUB OSCILLATOR ===
        if sub_level > 0.0 {
            let sub_phase = (drifted_phase * 0.5) * 2.0 * PI;
            let detune_phase = (drifted_phase * 0.5 * (1.0 + sub_detune * 0.01)) * 2.0 * PI;
            
            // BUILD 023: Soft square sub (tanh waveshaping)
            let sub1 = (sub_phase.sin() * 5.0).tanh();
            let sub2 = (detune_phase.sin() * 5.0).tanh();
            let sub_signal = (sub1 + sub2) * 0.5 * sub_level;
            
            signal += sub_signal;
        }
        
        // === SATURATION ===
        let driven = signal * sat_drive;
        
        // BUILD 023: Asymmetric distortion
        let saturated = if driven > 0.0 {
            driven.tanh()
        } else {
            // Harder clipping on negative side
            (driven * 1.2).tanh() * 0.95
        };
        
        signal = signal * (1.0 - sat_mix) + saturated * sat_mix;
        
        // === FILTER ===
        // BUILD 023: Exponential cutoff mapping
        let cutoff_exp = 20.0 * (1000.0_f32).powf(mod_cutoff / 20000.0);
        let cutoff_clamped = cutoff_exp.max(20.0).min(20000.0);
        
        let effective_q = filter_q * (1.0 + filter_damping * 0.5);
        signal = self.filter.process(signal, cutoff_clamped, effective_q);
        
        let driven_signal = signal * filter_drive;
        signal = driven_signal.tanh();
        
        // === WAVEFOLD ===
        signal = self.wavefold(signal, mod_fold);
        
        // === TILT EQ ===
        signal = self.tilt_eq.process(signal, tilt, self.sample_rate);
        
        // === BITCRUSH ===
        self.bit_depth = mod_bit_depth;
        signal = self.bitcrush(signal);
        
        // === COMB FILTER ===
        if self.comb_mix > 0.0 {
            let combed = self.comb_filter.process(signal, self.comb_freq, self.comb_feedback, self.comb_damp);
            signal = signal * (1.0 - self.comb_mix) + combed * self.comb_mix;
        }
        
        // === CHORUS ===
        if chorus_mix > 0.0 {
            let chorused = self.chorus.process(signal, chorus_rate_hz, chorus_depth_ms, chorus_feedback);
            signal = signal * (1.0 - chorus_mix) + chorused * chorus_mix;
        }
        
        // === ENVELOPE ===
        signal *= amp_env;
        
        // Phase progression
        let phase_inc = mod_freq / self.sample_rate;
        self.phase = (self.phase + phase_inc).fract();
        
        let mod_phase_inc = (mod_freq * fm_ratio) / self.sample_rate;
        self.mod_phase = (self.mod_phase + mod_phase_inc).fract();
        
        signal
    }
}

#[derive(Clone, Copy)]
pub enum ChaosMode {
    Logistic = 0,
    Lorenz = 1,
    DoublePendulum = 2,
}

pub struct ChaosLfo {
    mode: ChaosMode,
    rate: f32,
    x: f32,
    y: f32,
    z: f32,
    theta1: f32,
    theta2: f32,
    p1: f32,
    p2: f32,
    sample_rate: f32,
    phase_counter: f32,
}

impl ChaosLfo {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            mode: ChaosMode::Logistic,
            rate: 1.0,
            x: 0.5,
            y: 1.0,
            z: 1.0,
            theta1: 1.0,
            theta2: 1.0,
            p1: 0.0,
            p2: 0.0,
            sample_rate,
            phase_counter: 0.0,
        }
    }
    
    pub fn set_mode(&mut self, mode: ChaosMode) {
        self.mode = mode;
        self.x = 0.5;
        self.y = 1.0;
        self.z = 1.0;
        self.theta1 = 1.0;
        self.theta2 = 1.0;
        self.p1 = 0.0;
        self.p2 = 0.0;
    }
    
    pub fn set_rate(&mut self, rate: f32) {
        self.rate = rate.max(0.1).min(20.0);
    }
    
    pub fn process(&mut self) -> f32 {
        let samples_per_update = (self.sample_rate / self.rate).max(1.0);
        
        if self.phase_counter >= samples_per_update {
            self.phase_counter = 0.0;
            
            match self.mode {
                ChaosMode::Logistic => {
                    let r = 3.9;
                    self.x = r * self.x * (1.0 - self.x);
                    self.x = self.x.max(0.0).min(1.0);
                }
                ChaosMode::Lorenz => {
                    let dt = 0.01;
                    let sigma = 10.0;
                    let rho = 28.0;
                    let beta = 8.0 / 3.0;
                    
                    let dx = sigma * (self.y - self.x);
                    let dy = self.x * (rho - self.z) - self.y;
                    let dz = self.x * self.y - beta * self.z;
                    
                    self.x += dx * dt;
                    self.y += dy * dt;
                    self.z += dz * dt;
                    
                    self.x = self.x.max(-50.0).min(50.0);
                    self.y = self.y.max(-50.0).min(50.0);
                    self.z = self.z.max(-50.0).min(50.0);
                }
                ChaosMode::DoublePendulum => {
                    let dt = 0.01;
                    let g = 9.81;
                    let l1 = 1.0;
                    let l2 = 1.0;
                    let m1 = 1.0;
                    let m2 = 1.0;
                    
                    let num1 = -g * (2.0 * m1 + m2) * self.theta1.sin()
                        - m2 * g * (self.theta1 - 2.0 * self.theta2).sin()
                        - 2.0 * (self.theta1 - self.theta2).sin() * m2
                        * (self.p2 * self.p2 * l2 + self.p1 * self.p1 * l1 * (self.theta1 - self.theta2).cos());
                    let den1 = l1 * (2.0 * m1 + m2 - m2 * (2.0 * (self.theta1 - self.theta2)).cos());
                    let alpha1 = num1 / den1;
                    
                    let num2 = 2.0 * (self.theta1 - self.theta2).sin()
                        * (self.p1 * self.p1 * l1 * (m1 + m2)
                           + g * (m1 + m2) * self.theta1.cos()
                           + self.p2 * self.p2 * l2 * m2 * (self.theta1 - self.theta2).cos());
                    let den2 = l2 * (2.0 * m1 + m2 - m2 * (2.0 * (self.theta1 - self.theta2)).cos());
                    let alpha2 = num2 / den2;
                    
                    self.p1 += alpha1 * dt;
                    self.p2 += alpha2 * dt;
                    self.theta1 += self.p1 * dt;
                    self.theta2 += self.p2 * dt;
                    
                    self.p1 = self.p1.max(-10.0).min(10.0);
                    self.p2 = self.p2.max(-10.0).min(10.0);
                }
            }
        }
        
        self.phase_counter += 1.0;
        
        match self.mode {
            ChaosMode::Logistic => (self.x - 0.5) * 2.0,
            ChaosMode::Lorenz => (self.x / 50.0).max(-1.0).min(1.0),
            ChaosMode::DoublePendulum => (self.theta1 / PI).max(-1.0).min(1.0),
        }
    }
}

pub struct SpectralDrift {
    amount: f32,
    speed: f32,
    drift_type: usize,
    phase: f32,
    sample_rate: f32,
}

impl SpectralDrift {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            amount: 0.0,
            speed: 1.0,
            drift_type: 0,
            phase: 0.0,
            sample_rate,
        }
    }
    
    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount.max(0.0).min(1.0);
    }
    
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.max(0.1).min(10.0);
    }
    
    pub fn set_type(&mut self, drift_type: usize) {
        self.drift_type = drift_type;
    }
    
    pub fn process(&mut self) -> f32 {
        let phase_inc = self.speed / self.sample_rate;
        self.phase = (self.phase + phase_inc).fract();
        
        let modulation = match self.drift_type {
            0 => (self.phase * 2.0 * PI).sin(),
            1 => {
                let lfo = (self.phase * 2.0 * PI * 0.1).sin();
                lfo * 0.5 + 0.5
            }
            _ => (self.phase * 2.0 * PI).sin(),
        };
        
        modulation * self.amount * 0.02
    }
}

struct AllpassFilter {
    buffer: f32,
}

impl AllpassFilter {
    fn new() -> Self {
        Self { buffer: 0.0 }
    }
    
    fn process(&mut self, input: f32, coeff: f32) -> f32 {
        let output = -input + self.buffer;
        self.buffer = input + coeff * output;
        output
    }
}

pub struct Diffusion {
    allpass1: AllpassFilter,
    allpass2: AllpassFilter,
    allpass3: AllpassFilter,
    allpass4: AllpassFilter,
    mix: f32,
}

impl Diffusion {
    pub fn new() -> Self {
        Self {
            allpass1: AllpassFilter::new(),
            allpass2: AllpassFilter::new(),
            allpass3: AllpassFilter::new(),
            allpass4: AllpassFilter::new(),
            mix: 0.0,
        }
    }
    
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.max(0.0).min(1.0);
    }
    
    pub fn process(&mut self, input: f32) -> f32 {
        if self.mix <= 0.0 {
            return input;
        }
        
        let mut sig = input;
        sig = self.allpass1.process(sig, 0.7);
        sig = self.allpass2.process(sig, 0.5);
        sig = self.allpass3.process(sig, 0.3);
        sig = self.allpass4.process(sig, 0.1);
        
        input * (1.0 - self.mix) + sig * self.mix
    }
}

#[wasm_bindgen]
pub struct IndustrialEngine {
    sample_rate: f32,
    seed: u32,
    drive: f32,
    voices: Vec<Voice>,
    
    noise_level: f32,
    noise_gate_follow: bool,
    noise_drone_enabled: bool,
    
    fm_level: f32,
    
    global_fold: f32,
    global_bitcrush: f32,
    global_cutoff: f32,
    global_resonance: f32,
    global_feedback: f32,
    
    sub_level: f32,
    sub_detune: f32,
    sat_drive: f32,
    sat_mix: f32,
    tilt: f32,
    post_gain: f32,
    limiter_threshold: f32,
    limiter_amount: f32,
    
    lfo_rate_hz: f32,
    lfo_depth: f32,
    lfo_shape: i32,
    
    sh_rate_hz: f32,
    sh_depth: f32,
    sh_slew_ms: f32,
    
    jitter_amount: f32,
    jitter_band_hz: f32,
    
    chorus_mix: f32,
    chorus_rate_hz: f32,
    chorus_depth_ms: f32,
    chorus_feedback: f32,
    
    spasm: f32,
    
    lfo_to_cutoff: f32,
    lfo_to_fold: f32,
    sh_to_cutoff: f32,
    sh_to_fold: f32,
    sh_to_bit: f32,
    jitter_to_pitch: f32,
    
    synth_type: usize,
    fm_ratio: f32,
    wavetable_position: f32,
    wave_morph_speed: f32,
    harmonics_count: usize,
    harmonic_rolloff: f32,
    phase_dist_amount: f32,
    phase_resonance_point: f32,
    vector_x: f32,
    vector_y: f32,
    grain_size: f32,
    grain_density: f32,
    modal_stiffness: f32,
    modal_inharmonicity: f32,
    
    filter_q: f32,
    filter_damping: f32,
    filter_drive: f32,
    
    chaos_lfo: ChaosLfo,
    chaos_enabled: bool,
    spectral_drift: SpectralDrift,
    diffusion: Diffusion,
    
    sync_amount: f32,
    ring_ratio: f32,
    ring_mix: f32,
    comb_mix: f32,
    comb_freq: f32,
    comb_feedback: f32,
    comb_damp: f32,
}

#[wasm_bindgen]
impl IndustrialEngine {
    pub fn new(sample_rate: f32) -> Self {
        let mut voices = Vec::new();
        for _ in 0..8 {
            voices.push(Voice::new(sample_rate));
        }
        
        Self {
            sample_rate,
            seed: 12345,
            drive: 1.0,
            voices,
            noise_level: 0.0,
            noise_gate_follow: true,
            noise_drone_enabled: false,
            fm_level: 1.0,
            global_fold: 2.0,
            global_bitcrush: 8.0,
            global_cutoff: 2000.0,
            global_resonance: 2.0,
            global_feedback: 0.3,
            
            sub_level: 0.0,
            sub_detune: 0.0,
            sat_drive: 1.0,
            sat_mix: 0.0,
            tilt: 0.0,
            post_gain: 1.0,
            limiter_threshold: 0.9,
            limiter_amount: 0.5,
            
            lfo_rate_hz: 1.0,
            lfo_depth: 0.0,
            lfo_shape: 0,
            
            sh_rate_hz: 5.0,
            sh_depth: 0.0,
            sh_slew_ms: 0.0,
            
            jitter_amount: 0.0,
            jitter_band_hz: 10.0,
            
            chorus_mix: 0.0,
            chorus_rate_hz: 0.5,
            chorus_depth_ms: 10.0,
            chorus_feedback: 0.3,
            
            spasm: 0.0,
            
            lfo_to_cutoff: 0.0,
            lfo_to_fold: 0.0,
            sh_to_cutoff: 0.0,
            sh_to_fold: 0.0,
            sh_to_bit: 0.0,
            jitter_to_pitch: 0.0,
            
            synth_type: 0,
            fm_ratio: 2.0,
            wavetable_position: 0.0,
            wave_morph_speed: 0.0,
            harmonics_count: 4,
            harmonic_rolloff: 1.0,
            phase_dist_amount: 0.0,
            phase_resonance_point: 0.5,
            vector_x: 0.5,
            vector_y: 0.5,
            grain_size: 50.0,
            grain_density: 0.5,
            modal_stiffness: 0.5,
            modal_inharmonicity: 0.0,
            
            filter_q: 0.5,
            filter_damping: 0.0,
            filter_drive: 1.0,
            
            chaos_lfo: ChaosLfo::new(sample_rate),
            chaos_enabled: false,
            spectral_drift: SpectralDrift::new(sample_rate),
            diffusion: Diffusion::new(),
            
            sync_amount: 0.0,
            ring_ratio: 1.0,
            ring_mix: 0.0,
            comb_mix: 0.0,
            comb_freq: 200.0,
            comb_feedback: 0.5,
            comb_damp: 0.5,
        }
    }
    
    pub fn set_drive(&mut self, val: f32) {
        self.drive = val.max(0.1).min(10.0);
    }
    
    pub fn set_noise_level(&mut self, level: f32) {
        self.noise_level = level.max(0.0).min(1.0);
    }
    
    pub fn set_noise_gate_follow(&mut self, follow: bool) {
        self.noise_gate_follow = follow;
    }
    
    pub fn set_noise_drone(&mut self, enabled: bool) {
        self.noise_drone_enabled = enabled;
    }
    
    pub fn set_fm_level(&mut self, level: f32) {
        self.fm_level = level.max(0.0).min(1.0);
    }
    
    pub fn set_fold_amount(&mut self, amount: f32) {
        self.global_fold = amount.max(1.0).min(10.0);
        for voice in &mut self.voices {
            voice.set_fold_amount(self.global_fold);
        }
    }
    
    pub fn set_bit_depth(&mut self, depth: f32) {
        self.global_bitcrush = depth.max(1.0).min(16.0);
        for voice in &mut self.voices {
            voice.set_bit_depth(self.global_bitcrush);
        }
    }
    
    pub fn set_cutoff(&mut self, freq: f32) {
        self.global_cutoff = freq.max(20.0).min(20000.0);
        for voice in &mut self.voices {
            voice.set_cutoff(self.global_cutoff);
        }
    }
    
    pub fn set_resonance(&mut self, res: f32) {
        self.global_resonance = res.max(0.0).min(1.0);
        for voice in &mut self.voices {
            voice.set_resonance(self.global_resonance);
        }
    }
    
    pub fn set_feedback(&mut self, fb: f32) {
        self.global_feedback = fb.max(0.0).min(0.99);
        for voice in &mut self.voices {
            voice.set_feedback(self.global_feedback);
        }
    }
    
    pub fn set_adsr(&mut self, attack_ms: f32, decay_ms: f32, sustain: f32, release_ms: f32) {
        for voice in &mut self.voices {
            voice.set_adsr(attack_ms, decay_ms, sustain, release_ms);
        }
    }
    
    pub fn set_sub(&mut self, level: f32, detune: f32) {
        self.sub_level = level.max(0.0).min(1.0);
        self.sub_detune = detune.max(-100.0).min(100.0);
    }
    
    pub fn set_saturation(&mut self, drive: f32, mix: f32) {
        self.sat_drive = drive.max(1.0).min(10.0);
        self.sat_mix = mix.max(0.0).min(1.0);
    }
    
    pub fn set_tilt(&mut self, value: f32) {
        self.tilt = value.max(-1.0).min(1.0);
    }
    
    pub fn set_post_gain(&mut self, gain: f32) {
        self.post_gain = gain.max(0.0).min(4.0);
    }
    
    pub fn set_limiter(&mut self, amount: f32) {
        self.limiter_amount = amount.max(0.0).min(1.0);
        self.limiter_threshold = 0.9 - (amount * 0.4);
    }
    
    pub fn set_lfo(&mut self, rate_hz: f32, depth: f32, shape: i32) {
        self.lfo_rate_hz = rate_hz.max(0.05).min(40.0);
        self.lfo_depth = depth.max(0.0).min(1.0);
        self.lfo_shape = shape.max(0).min(2);
        
        for voice in &mut self.voices {
            voice.lfo.set_rate(self.lfo_rate_hz);
        }
    }
    
    pub fn set_sample_hold(&mut self, rate_hz: f32, depth: f32, slew_ms: f32) {
        self.sh_rate_hz = rate_hz.max(1.0).min(200.0);
        self.sh_depth = depth.max(0.0).min(1.0);
        self.sh_slew_ms = slew_ms.max(0.0).min(50.0);
        
        for voice in &mut self.voices {
            voice.sample_hold.set_rate(self.sh_rate_hz);
            voice.sample_hold.set_slew(self.sh_slew_ms);
        }
    }
    
    pub fn set_jitter(&mut self, amount: f32, band_hz: f32) {
        self.jitter_amount = amount.max(0.0).min(1.0);
        self.jitter_band_hz = band_hz.max(1.0).min(100.0);
    }
    
    pub fn set_chorus(&mut self, mix: f32, rate_hz: f32, depth_ms: f32, feedback: f32) {
        self.chorus_mix = mix.max(0.0).min(1.0);
        self.chorus_rate_hz = rate_hz.max(0.1).min(10.0);
        self.chorus_depth_ms = depth_ms.max(1.0).min(50.0);
        self.chorus_feedback = feedback.max(0.0).min(0.99);
    }
    
    pub fn set_spasm(&mut self, value: f32) {
        self.spasm = value.max(0.0).min(1.0);
        
        let spasm_lfo = self.spasm * 0.5;
        let spasm_sh = self.spasm * 0.3;
        let spasm_jitter = self.spasm * 0.2;
        
        self.lfo_depth = spasm_lfo;
        self.sh_depth = spasm_sh;
        self.jitter_amount = spasm_jitter;
    }
    
    pub fn set_mod_routing(&mut self, lfo_cutoff: f32, lfo_fold: f32, sh_cutoff: f32, 
                          sh_fold: f32, sh_bit: f32, jitter_pitch: f32) {
        self.lfo_to_cutoff = lfo_cutoff.max(0.0).min(1.0);
        self.lfo_to_fold = lfo_fold.max(0.0).min(1.0);
        self.sh_to_cutoff = sh_cutoff.max(0.0).min(1.0);
        self.sh_to_fold = sh_fold.max(0.0).min(1.0);
        self.sh_to_bit = sh_bit.max(0.0).min(1.0);
        self.jitter_to_pitch = jitter_pitch.max(0.0).min(1.0);
    }
    
    pub fn note_on(&mut self, note_id: i32, frequency: f32, mod_index: f32) {
        for voice in &mut self.voices {
            if !voice.is_active() {
                voice.note_on(note_id, frequency, mod_index);
                voice.set_fold_amount(self.global_fold);
                voice.set_bit_depth(self.global_bitcrush);
                voice.set_cutoff(self.global_cutoff);
                voice.set_resonance(self.global_resonance);
                voice.set_feedback(self.global_feedback);
                voice.set_sync_amount(self.sync_amount);
                voice.set_ring_ratio(self.ring_ratio);
                voice.set_ring_mix(self.ring_mix);
                voice.set_comb_mix(self.comb_mix);
                voice.set_comb_freq(self.comb_freq);
                voice.set_comb_feedback(self.comb_feedback);
                voice.set_comb_damp(self.comb_damp);
                voice.set_wavetable_position(self.wavetable_position);
                return;
            }
        }
        
        self.voices[0].note_on(note_id, frequency, mod_index);
        self.voices[0].set_fold_amount(self.global_fold);
        self.voices[0].set_bit_depth(self.global_bitcrush);
        self.voices[0].set_cutoff(self.global_cutoff);
        self.voices[0].set_resonance(self.global_resonance);
        self.voices[0].set_feedback(self.global_feedback);
        self.voices[0].set_sync_amount(self.sync_amount);
        self.voices[0].set_ring_ratio(self.ring_ratio);
        self.voices[0].set_ring_mix(self.ring_mix);
        self.voices[0].set_comb_mix(self.comb_mix);
        self.voices[0].set_comb_freq(self.comb_freq);
        self.voices[0].set_comb_feedback(self.comb_feedback);
        self.voices[0].set_comb_damp(self.comb_damp);
        self.voices[0].set_wavetable_position(self.wavetable_position);
    }
    
    pub fn note_off(&mut self, note_id: i32) {
        for voice in &mut self.voices {
            if voice.get_note_id() == note_id {
                voice.note_off(note_id);
                return;
            }
        }
    }
    
    pub fn all_notes_off(&mut self) {
        for voice in &mut self.voices {
            if voice.is_active() {
                voice.force_stop();
            }
        }
    }
    
    pub fn panic(&mut self) {
        for voice in &mut self.voices {
            voice.force_stop();
        }
    }
    
    pub fn set_mod_index(&mut self, mod_index: f32) {
        for voice in &mut self.voices {
            if voice.active {
                voice.mod_index_base = mod_index.max(0.0).min(20.0);
            }
        }
    }
    
    fn noise_sample(&mut self) -> f32 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.seed = x;
        
        (x as f32 / std::u32::MAX as f32) * 2.0 - 1.0
    }
    
    pub fn process(&mut self, output: &mut [f32]) {
        let chaos_value = if self.chaos_enabled {
            self.chaos_lfo.process()
        } else {
            0.0
        };
        
        let drift_value = self.spectral_drift.process();
        
        for sample in output.iter_mut() {
            let mut mix = 0.0;
            
            for voice in &mut self.voices {
                if voice.is_active() {
                    let voice_sample = voice.process_sample(
                        self.sub_level,
                        self.sub_detune,
                        self.sat_drive,
                        self.sat_mix,
                        self.tilt,
                        self.lfo_depth,
                        self.lfo_shape,
                        self.sh_depth,
                        self.jitter_amount,
                        self.jitter_band_hz,
                        self.chorus_mix,
                        self.chorus_rate_hz,
                        self.chorus_depth_ms,
                        self.chorus_feedback,
                        self.lfo_to_cutoff,
                        self.lfo_to_fold,
                        self.sh_to_cutoff,
                        self.sh_to_fold,
                        self.sh_to_bit,
                        self.jitter_to_pitch,
                        chaos_value,
                        drift_value,
                        self.synth_type,
                        self.fm_ratio,
                        self.harmonics_count,
                        self.harmonic_rolloff,
                        self.phase_dist_amount,
                        self.vector_x,
                        self.vector_y,
                        self.grain_size,
                        self.grain_density,
                        self.modal_stiffness,
                        self.modal_inharmonicity,
                        self.filter_q,
                        self.filter_damping,
                        self.filter_drive,
                    );
                    mix += voice_sample * self.fm_level;
                }
            }
            
            if self.noise_level > 0.0 {
                let noise = self.noise_sample() * self.noise_level;
                mix += noise;
            }
            
            mix *= self.drive;
            mix = mix.max(-1.0).min(1.0);
            
            mix = self.diffusion.process(mix);
            
            mix *= self.post_gain;
            
            // BUILD 023: Enhanced limiter with soft knee
            if mix.abs() > self.limiter_threshold {
                let excess = mix.abs() - self.limiter_threshold;
                let reduction = excess * self.limiter_amount;
                let soft_limited = reduction.tanh() * (1.0 - self.limiter_threshold);
                mix = mix.signum() * (self.limiter_threshold + soft_limited);
            }
            
            *sample = mix;
        }
    }
    
    pub fn set_chaos_mode(&mut self, mode: usize) {
        let chaos_mode = match mode {
            0 => ChaosMode::Logistic,
            1 => ChaosMode::Lorenz,
            2 => ChaosMode::DoublePendulum,
            _ => ChaosMode::Logistic,
        };
        self.chaos_lfo.set_mode(chaos_mode);
    }
    
    pub fn set_chaos_rate(&mut self, rate: f32) {
        self.chaos_lfo.set_rate(rate);
    }
    
    pub fn set_chaos_enabled(&mut self, enabled: bool) {
        self.chaos_enabled = enabled;
    }
    
    pub fn set_drift_amount(&mut self, amount: f32) {
        self.spectral_drift.set_amount(amount);
    }
    
    pub fn set_drift_speed(&mut self, speed: f32) {
        self.spectral_drift.set_speed(speed);
    }
    
    pub fn set_drift_type(&mut self, drift_type: usize) {
        self.spectral_drift.set_type(drift_type);
    }
    
    pub fn set_diffusion_mix(&mut self, mix: f32) {
        self.diffusion.set_mix(mix);
    }
    
    pub fn set_synth_type(&mut self, synth_type: usize) {
        self.synth_type = synth_type.min(6);
    }
    
    pub fn set_fm_ratio(&mut self, ratio: f32) {
        self.fm_ratio = ratio.max(0.25).min(16.0);
    }
    
    pub fn set_wavetable_position(&mut self, position: f32) {
        self.wavetable_position = position.max(0.0).min(1.0);
        for voice in &mut self.voices {
            if voice.is_active() {
                voice.set_wavetable_position(self.wavetable_position);
            }
        }
    }
    
    pub fn set_wave_morph_speed(&mut self, speed: f32) {
        self.wave_morph_speed = speed.max(0.0).min(10.0);
    }
    
    pub fn set_harmonics_count(&mut self, count: usize) {
        self.harmonics_count = count.min(16).max(1);
    }
    
    pub fn set_harmonic_rolloff(&mut self, rolloff: f32) {
        self.harmonic_rolloff = rolloff.max(0.0).min(3.0);
    }
    
    pub fn set_phase_dist_amount(&mut self, amount: f32) {
        self.phase_dist_amount = amount.max(0.0).min(1.0);
    }
    
    pub fn set_phase_resonance_point(&mut self, point: f32) {
        self.phase_resonance_point = point.max(0.0).min(1.0);
    }
    
    pub fn set_vector_x(&mut self, x: f32) {
        self.vector_x = x.max(0.0).min(1.0);
    }
    
    pub fn set_vector_y(&mut self, y: f32) {
        self.vector_y = y.max(0.0).min(1.0);
    }
    
    pub fn set_grain_size(&mut self, size: f32) {
        self.grain_size = size.max(1.0).min(200.0);
    }
    
    pub fn set_grain_density(&mut self, density: f32) {
        self.grain_density = density.max(0.0).min(1.0);
    }
    
    pub fn set_modal_stiffness(&mut self, stiffness: f32) {
        self.modal_stiffness = stiffness.max(0.0).min(1.0);
    }
    
    pub fn set_modal_inharmonicity(&mut self, inharmonicity: f32) {
        self.modal_inharmonicity = inharmonicity.max(0.0).min(1.0);
    }
    
    pub fn set_filter_q(&mut self, q: f32) {
        self.filter_q = q.max(0.0).min(1.0);
    }
    
    pub fn set_filter_damping(&mut self, damping: f32) {
        self.filter_damping = damping.max(0.0).min(1.0);
    }
    
    pub fn set_filter_drive(&mut self, drive: f32) {
        self.filter_drive = drive.max(0.1).min(10.0);
    }
    
    pub fn set_sync_amount(&mut self, amount: f32) {
        self.sync_amount = amount.max(0.0).min(5.0);
        for voice in &mut self.voices {
            if voice.is_active() {
                voice.set_sync_amount(self.sync_amount);
            }
        }
    }
    
    pub fn set_ring_ratio(&mut self, ratio: f32) {
        self.ring_ratio = ratio.max(0.5).min(4.0);
        for voice in &mut self.voices {
            if voice.is_active() {
                voice.set_ring_ratio(self.ring_ratio);
            }
        }
    }
    
    pub fn set_ring_mix(&mut self, mix: f32) {
        self.ring_mix = mix.max(0.0).min(1.0);
        for voice in &mut self.voices {
            if voice.is_active() {
                voice.set_ring_mix(self.ring_mix);
            }
        }
    }
    
    pub fn set_comb_mix(&mut self, mix: f32) {
        self.comb_mix = mix.max(0.0).min(1.0);
        for voice in &mut self.voices {
            if voice.is_active() {
                voice.set_comb_mix(self.comb_mix);
            }
        }
    }
    
    pub fn set_comb_freq(&mut self, freq: f32) {
        self.comb_freq = freq.max(50.0).min(1000.0);
        for voice in &mut self.voices {
            if voice.is_active() {
                voice.set_comb_freq(self.comb_freq);
            }
        }
    }
    
    pub fn set_comb_feedback(&mut self, feedback: f32) {
        self.comb_feedback = feedback.max(0.0).min(0.99);
        for voice in &mut self.voices {
            if voice.is_active() {
                voice.set_comb_feedback(self.comb_feedback);
            }
        }
    }
    
    pub fn set_comb_damp(&mut self, damp: f32) {
        self.comb_damp = damp.max(0.0).min(1.0);
        for voice in &mut self.voices {
            if voice.is_active() {
                voice.set_comb_damp(self.comb_damp);
            }
        }
    }
}
