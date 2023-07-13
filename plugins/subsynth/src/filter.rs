use nih_plug::params::enums::{Enum};
use enum_iterator::Sequence;
use std::f32::consts::PI;
pub trait Envelope {
    fn get_value(&mut self) -> f32;
    fn trigger(&mut self);
    fn release(&mut self);
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ADSREnvelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    state: ADSREnvelopeState,
    time: f32,
    delta_time_per_sample: f32, // add this new field
    sample_rate: f32,
    velocity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum ADSREnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}


impl ADSREnvelope {
    
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32, sample_rate: f32, velocity: f32) -> Self {
        ADSREnvelope {
            attack,
            decay,
            sustain,
            release,
            state: ADSREnvelopeState::Attack,
            time: 0.0,
            sample_rate,
            delta_time_per_sample: 1.0 / sample_rate, // calculate the delta_time_per_sample
            velocity,
        }
    }
    pub fn set_velocity(&mut self, velocity: f32) {
        self.velocity = velocity;

        // Adjust envelope parameters based on velocity
        // Example: Modify attack and release times based on velocity
        self.attack *= velocity;
        self.release *= velocity;
        self.decay *= velocity;
        self.sustain *= velocity;

        // Additional adjustments based on velocity if needed
    }
    pub fn get_time(&mut self) -> f32{
        self.time
    }
    pub fn set_attack(&mut self, attack: f32) {
        self.attack = attack;
    }

    pub fn set_decay(&mut self, decay: f32) {
        self.decay = decay;
    }

    pub fn set_sustain(&mut self, sustain: f32) {
        self.sustain = sustain;
    }

    pub fn set_release(&mut self, release: f32) {
        self.release = release;
    }

    pub fn get_state(&self) -> ADSREnvelopeState {
        self.state
    }

    pub fn previous_value(&self) -> f32 {
        match self.state {
            ADSREnvelopeState::Idle => 0.0,
            ADSREnvelopeState::Attack => self.time / self.attack,
            ADSREnvelopeState::Decay => 1.0 - (1.0 - self.sustain) * (self.time / self.decay),
            ADSREnvelopeState::Sustain => self.sustain,
            ADSREnvelopeState::Release => self.sustain * (1.0 - (self.time / self.release)),
        }
    }
    
}

impl Envelope for ADSREnvelope {
    fn get_value(&mut self) -> f32 {
        self.time += self.delta_time_per_sample;

        // Adjust envelope parameters based on velocity sensitivity
        let velocity_sensitive_attack = self.attack / self.velocity;
        let velocity_sensitive_decay = self.decay / self.velocity;
        let velocity_sensitive_release = self.release / self.velocity;

        // Check if the envelope has completed and move to the next stage
        if self.state != ADSREnvelopeState::Idle && self.time >= velocity_sensitive_release {
            self.state = ADSREnvelopeState::Idle;
            self.time = 0.0;
        }

        let value = match self.state {
            ADSREnvelopeState::Idle => 0.0,
            ADSREnvelopeState::Attack => {
                let attack_value = self.time / velocity_sensitive_attack;
                if self.time >= velocity_sensitive_attack {
                    self.state = ADSREnvelopeState::Decay;
                    self.time = 0.0;
                    1.0
                } else {
                    attack_value
                }
            }
            ADSREnvelopeState::Decay => {
                let decay_value =
                    1.0 - (1.0 - self.sustain) * (self.time / velocity_sensitive_decay);
                if self.time >= velocity_sensitive_decay {
                    self.state = ADSREnvelopeState::Sustain;
                    self.time = 0.0;
                    self.sustain
                } else {
                    decay_value
                }
            }
            ADSREnvelopeState::Sustain => self.sustain,
            ADSREnvelopeState::Release => {
                let release_value =
                    self.sustain * (1.0 - (self.time / velocity_sensitive_release));
                if self.time >= velocity_sensitive_release {
                    self.state = ADSREnvelopeState::Idle;
                    self.time = 0.0;
                    0.0
                } else {
                    release_value
                }
            }
        };

        // Check if the envelope has completed and move to the next stage
        if self.state != ADSREnvelopeState::Idle && self.time >= velocity_sensitive_release {
            self.state = ADSREnvelopeState::Idle;
            self.time = 0.0;
        }

        value
    }

    fn trigger(&mut self) {
        self.state = ADSREnvelopeState::Attack;
        self.time = 0.0;
    }

    fn release(&mut self) {
        self.state = ADSREnvelopeState::Release;
    }
}




#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Sequence)]
pub enum FilterType {
    None,
    Lowpass,
    Bandpass,
    Highpass,
    Notch,
    Statevariable,
}


pub trait Filter: Send {
    fn process(&mut self, input: f32) -> f32;
    fn set_sample_rate(&mut self, sample_rate: f32);
}



pub struct HighpassFilter {
    cutoff: f32,
    resonance: f32,
    cutoff_envelope: ADSREnvelope,
    resonance_envelope: ADSREnvelope,
    sample_rate: f32,
    prev_input: f32,
    prev_output: f32,
}

impl HighpassFilter {
    pub fn new(cutoff: f32, cutoff_envelope: ADSREnvelope, resonance: f32, resonance_envelope: ADSREnvelope, sample_rate: f32) -> Self {
        HighpassFilter {
            cutoff,
            resonance,
            cutoff_envelope,
            resonance_envelope,
            sample_rate,
            prev_input: 0.0,
            prev_output: 0.0,
        }
    }
}

impl Filter for HighpassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff * self.cutoff_envelope.get_value();
        let resonance = self.resonance * self.resonance_envelope.get_value();
        let c = 1.0 / (2.0 * std::f32::consts::PI * cutoff / self.sample_rate);
        let r = 1.0 - resonance;
        let output = c * (input - self.prev_input + r * self.prev_output);
        self.prev_input = input;
        self.prev_output = output;
        output
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

pub struct BandpassFilter {
    cutoff: f32,
    resonance: f32,
    cutoff_envelope: ADSREnvelope,
    resonance_envelope: ADSREnvelope,
    sample_rate: f32,
    prev_input: f32,
    prev_output: f32,
}

impl BandpassFilter {
    pub fn new(cutoff: f32, cutoff_envelope: ADSREnvelope, resonance: f32, resonance_envelope: ADSREnvelope, sample_rate: f32) -> Self {
        BandpassFilter {
            cutoff,
            resonance,
            cutoff_envelope,
            resonance_envelope,
            sample_rate,
            prev_input: 0.0,
            prev_output: 0.0,
        }
    }
}
impl Filter for BandpassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff * self.cutoff_envelope.get_value();
        let resonance = self.resonance * self.resonance_envelope.get_value();
        let c = 1.0 / (2.0 * std::f32::consts::PI * cutoff / self.sample_rate);
        let r = 1.0 - resonance;
        let output = c * (input - self.prev_output) + r * self.prev_output;
        self.prev_input = input;
        self.prev_output = output;
        output
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}




pub struct LowpassFilter {
    cutoff: f32,
    resonance: f32,
    cutoff_envelope: ADSREnvelope,
    resonance_envelope: ADSREnvelope,
    sample_rate: f32,
    prev_output: f32,
}

impl LowpassFilter {
    pub fn new(cutoff: f32, cutoff_envelope: ADSREnvelope, resonance: f32, resonance_envelope: ADSREnvelope, sample_rate: f32) -> Self {
        LowpassFilter {
            cutoff,
            resonance,
            cutoff_envelope,
            resonance_envelope,
            sample_rate,
            prev_output: 0.0,
        }
    }
}

impl Filter for LowpassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff * self.cutoff_envelope.get_value();
        let resonance = self.resonance * self.resonance_envelope.get_value();

        let c = 1.0 / (2.0 * std::f32::consts::PI * cutoff / self.sample_rate);
        let r = resonance;
        let output = c * input + r * self.prev_output;

        self.prev_output = output;
        output
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}




pub struct NotchFilter {
    cutoff: f32,
    bandwidth: f32,
    cutoff_envelope: ADSREnvelope,
    bandwidth_envelope: ADSREnvelope,
    sample_rate: f32,
    buf0: f32,
    buf1: f32,
    a0: f32,
    a1: f32,
    a2: f32,
    b1: f32,
    b2: f32,
}

impl NotchFilter {
    pub fn new(cutoff: f32, cutoff_envelope: ADSREnvelope, bandwidth: f32, bandwidth_envelope: ADSREnvelope, sample_rate: f32) -> Self {
        let mut filter = NotchFilter {
            cutoff,
            bandwidth,
            cutoff_envelope,
            bandwidth_envelope,
            sample_rate,
            buf0: 0.0,
            buf1: 0.0,
            a0: 0.0,
            a1: 0.0,
            a2: 0.0,
            b1: 0.0,
            b2: 0.0,
        };
        filter.calculate_coefficients();
        filter
    }

    pub fn calculate_coefficients(&mut self) {
        let wc = 2.0 * PI * self.cutoff / self.sample_rate; // cutoff frequency in radians
        let bw = 2.0 * PI * self.bandwidth / self.sample_rate; // bandwidth in radians
        let alpha = wc.sin() * (bw / 2.0).sinh().ln() / (2.0 * (3.0 as f32).sqrt().ln()); // bandwidth parameter
        
        self.a0 = 1.0;
        self.a1 = -2.0 * wc.cos();
        self.a2 = 1.0;
        let norm = 1.0 / (1.0 + alpha); // normalization factor
        self.a0 *= norm;
        self.a1 *= norm;
        self.a2 *= norm;
        self.b1 = -2.0 * wc.cos() * norm;
        self.b2 = (1.0 - alpha) * norm;
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.calculate_coefficients();
    }
}

impl Filter for NotchFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff * self.cutoff_envelope.get_value();
        let bandwidth = self.bandwidth * self.bandwidth_envelope.get_value();

        if cutoff != self.cutoff || bandwidth != self.bandwidth {
            self.cutoff = cutoff;
            self.bandwidth = bandwidth;
            self.calculate_coefficients();
        }

        // apply filter
        let output = self.a0 * input + self.a1 * self.buf0 + self.a2 * self.buf1 - self.b1 * self.buf0 - self.b2 * self.buf1;
        self.buf1 = self.buf0;
        self.buf0 = output;
        output
    }
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}


pub struct StatevariableFilter {
    cutoff: f32,
    resonance: f32,
    cutoff_envelope: ADSREnvelope,
    resonance_envelope: ADSREnvelope,
    sample_rate: f32,
    prev_input: f32,
    lowpass_output: f32,
    highpass_output: f32,
    bandpass_output: f32,
}

impl StatevariableFilter {
    pub fn new(
        cutoff: f32,
        cutoff_envelope: ADSREnvelope,
        resonance: f32,
        resonance_envelope: ADSREnvelope,
        sample_rate: f32,
    ) -> Self {
        StatevariableFilter {
            cutoff,
            resonance,
            cutoff_envelope,
            resonance_envelope,
            sample_rate,
            prev_input: 0.0,
            lowpass_output: 0.0,
            highpass_output: 0.0,
            bandpass_output: 0.0,
        }
    }
}

impl Filter for StatevariableFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff * self.cutoff_envelope.get_value();
        let resonance = self.resonance * self.resonance_envelope.get_value();

        let f = cutoff / self.sample_rate;
        let _k = 2.0 * (1.0 - resonance);
        let q = 1.0 / (2.0 * resonance);

        let input_minus_hp = input - self.highpass_output;
        let lp_output = self.lowpass_output + f * self.bandpass_output;
        let hp_output = input_minus_hp - lp_output * q - self.bandpass_output;
        let bp_output = f * hp_output + self.bandpass_output;

        self.prev_input = input;
        self.lowpass_output = lp_output;
        self.highpass_output = hp_output;
        self.bandpass_output = bp_output;

        bp_output
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}
pub struct NoneFilter {
    cutoff: f32,
    resonance: f32,
    cutoff_envelope: ADSREnvelope,
    resonance_envelope: ADSREnvelope,
    sample_rate: f32,
}

impl NoneFilter {
    pub fn new(
        cutoff: f32,
        cutoff_envelope: ADSREnvelope,
        resonance: f32,
        resonance_envelope: ADSREnvelope,
        sample_rate: f32,
    ) -> Self {
        NoneFilter {
            cutoff,
            resonance,
            cutoff_envelope,
            resonance_envelope,
            sample_rate,
        }
    }
}

impl Filter for NoneFilter {
    fn process(&mut self, input: f32) -> f32 {
        // No filtering, simply return the input unchanged
        input
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

pub struct DCBlocker {
    x1: f32,
    y1: f32,
    r: f32,
}

impl DCBlocker {
    pub fn new() -> Self {
        DCBlocker {
            x1: 0.0,
            y1: 0.0,
            r: 0.995,  // The closer this value to 1.0, the lower the cutoff frequency
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let output = input - self.x1 + self.r * self.y1;
        self.x1 = input;
        self.y1 = output;
        output
    }
}

pub fn generate_filter(
    filter_type: FilterType,
    cutoff: f32,
    resonance: f32,
    cutoff_envelope: ADSREnvelope,
    resonance_envelope: ADSREnvelope,
    _generated_sample: f32,
    sample_rate: f32,
) -> Box<dyn Filter> {

    match filter_type {
        FilterType::None => Box::new(NoneFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
        FilterType::Lowpass => Box::new(LowpassFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
        FilterType::Bandpass => Box::new(BandpassFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
        FilterType::Highpass => Box::new(HighpassFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
        FilterType::Notch => Box::new(NotchFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
        FilterType::Statevariable => Box::new(StatevariableFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
    }
}