use nih_plug::params::enums::{Enum, EnumParam};
use enum_iterator::Sequence;

pub trait Envelope {
    fn get_value(&mut self, dt: f32) -> f32;
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
    
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        ADSREnvelope {
            attack,
            decay,
            sustain,
            release,
            state: ADSREnvelopeState::Attack,
            time: 0.0,
        }
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
    fn get_value(&mut self, dt: f32) -> f32 {
        self.time += dt;
        // Check if the envelope has completed and move to the next stage
        if self.state != ADSREnvelopeState::Idle && self.time >= self.release {
            self.state = ADSREnvelopeState::Idle;
            self.time = 0.0;
        }
        let value = match self.state {
            ADSREnvelopeState::Idle => 0.0,
            ADSREnvelopeState::Attack => {
                let attack_value = self.time / self.attack;
                if self.time >= self.attack {
                    self.state = ADSREnvelopeState::Decay;
                    self.time = 0.0;
                    1.0
                } else {
                    attack_value
                }
            }
            ADSREnvelopeState::Decay => {
                let decay_value = 1.0 - (1.0 - self.sustain) * (self.time / self.decay);
                if self.time >= self.decay {
                    self.state = ADSREnvelopeState::Sustain;
                    self.time = 0.0;
                    self.sustain
                } else {
                    decay_value
                }
            }
            ADSREnvelopeState::Sustain => self.sustain,
            ADSREnvelopeState::Release => {
                let release_value = self.sustain * (1.0 - (self.time / self.release));
                if self.time >= self.release {
                    self.state = ADSREnvelopeState::Idle;
                    self.time = 0.0;
                    0.0
                } else {
                    release_value
                }
            }
        };

        // Check if the envelope has completed and move to the next stage
        if self.state != ADSREnvelopeState::Idle && self.time >= self.release {
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

pub struct FilterFactory;

impl FilterFactory {
    pub fn create_filter(
        filter_type: FilterType,
        cutoff: f32,
        cutoff_envelope: ADSREnvelope,
        resonance: f32,
        resonance_envelope: ADSREnvelope,
        sample_rate: f32,
    ) -> Box<dyn Filter> {
        match filter_type {
            FilterType::Lowpass => Box::new(LowpassFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
            FilterType::Bandpass => Box::new(BandpassFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
            FilterType::Highpass => Box::new(HighpassFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
            FilterType::Notch => Box::new(NotchFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
            FilterType::Statevariable => Box::new(StatevariableFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
        }
    }
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

impl  HighpassFilter {
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
        let cutoff = self.cutoff * self.cutoff_envelope.get_value(self.cutoff_envelope.time);
        let resonance = self.resonance * self.resonance_envelope.get_value(self.resonance_envelope.time);
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
        let cutoff = self.cutoff * self.cutoff_envelope.get_value(self.cutoff_envelope.time);
        let resonance = self.resonance * self.resonance_envelope.get_value(self.resonance_envelope.time);
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
        let cutoff = self.cutoff * self.cutoff_envelope.get_value(self.cutoff_envelope.time);
        let resonance = self.resonance * self.resonance_envelope.get_value(self.resonance_envelope.time);

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
    resonance: f32,
    cutoff_envelope: ADSREnvelope,
    resonance_envelope: ADSREnvelope,
    sample_rate: f32,
    prev_input: f32,
    prev_output: f32,
}

impl NotchFilter {
    pub fn new(cutoff: f32, cutoff_envelope: ADSREnvelope, resonance: f32, resonance_envelope: ADSREnvelope, sample_rate: f32) -> Self {
        NotchFilter {
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

impl Filter for NotchFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff * self.cutoff_envelope.get_value(self.cutoff_envelope.time);
        let resonance = self.resonance * self.resonance_envelope.get_value(self.resonance_envelope.time);
        let c = 1.0 / (2.0 * std::f32::consts::PI * cutoff / self.sample_rate);
        let r = resonance;
        let output = (input - self.prev_output) + r * (self.prev_input - self.prev_output);
        self.prev_input = input;
        self.prev_output = output;
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
        let cutoff = self.cutoff * self.cutoff_envelope.get_value(self.cutoff_envelope.time);
        let resonance = self.resonance * self.resonance_envelope.get_value(self.resonance_envelope.time);

        let f = cutoff / self.sample_rate;
        let k = 2.0 * (1.0 - resonance);
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


pub fn generate_filter(
    filter_type: FilterType,
    cutoff: f32,
    resonance: f32,
    cutoff_attack: f32,
    cutoff_decay: f32,
    cutoff_sustain: f32,
    cutoff_release: f32,
    resonance_attack: f32,
    resonance_decay: f32,
    resonance_sustain: f32,
    resonance_release: f32,
    generated_sample: f32,
    sample_rate: f32,
) -> Box<dyn Filter> {
    let cutoff_envelope = ADSREnvelope::new(cutoff_attack, cutoff_decay, cutoff_sustain, cutoff_release);
    let resonance_envelope = ADSREnvelope::new(resonance_attack, resonance_decay, resonance_sustain, resonance_release);

    match filter_type {
        FilterType::Lowpass => Box::new(LowpassFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
        FilterType::Bandpass => Box::new(BandpassFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
        FilterType::Highpass => Box::new(HighpassFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
        FilterType::Notch => Box::new(NotchFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
        FilterType::Statevariable => Box::new(StatevariableFilter::new(cutoff, cutoff_envelope, resonance, resonance_envelope, sample_rate)),
    }
}