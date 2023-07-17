use enum_iterator::Sequence;
use nih_plug::params::enums::Enum;
use std::f32::consts::PI;

use crate::envelope::*;

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
    sample_rate: f32,
    prev_input: f32,
    prev_output: f32,
}

impl HighpassFilter {
    pub fn new(
        cutoff: f32,
        resonance: f32,
        sample_rate: f32,
    ) -> Self {
        HighpassFilter {
            cutoff,
            resonance,
            sample_rate,
            prev_input: 0.0,
            prev_output: 0.0,
        }
    }
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
    }

    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance;
    }
}

impl Filter for HighpassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff;
        let resonance = self.resonance;
        let c = 1.0 / (2.0 * std::f32::consts::PI * cutoff / self.sample_rate);
        let r = 1.0 - resonance;
        self.prev_output = c * (input - self.prev_input + r * self.prev_output);
        self.prev_input = input;
        self.prev_output
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

pub struct BandpassFilter {
    cutoff: f32,
    resonance: f32,
    sample_rate: f32,
    prev_input: f32,
    prev_output: f32,
}

impl BandpassFilter {
    pub fn new(
        cutoff: f32,
        resonance: f32,
        sample_rate: f32,
    ) -> Self {
        BandpassFilter {
            cutoff,
            resonance,
            sample_rate,
            prev_input: 0.0,
            prev_output: 0.0,
        }
    }
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
    }

    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance;
    }
}
impl Filter for BandpassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff;
        let resonance = self.resonance;
        let c = 1.0 / (2.0 * std::f32::consts::PI * cutoff / self.sample_rate);
        let r = 1.0 - resonance;
        self.prev_output = c * (input - self.prev_output) + r * self.prev_output;
        self.prev_input = input;
        self.prev_output
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

pub struct LowpassFilter {
    cutoff: f32,
    resonance: f32,
    sample_rate: f32,
    prev_output: f32,
}

impl LowpassFilter {
    pub fn new(
        cutoff: f32,
        resonance: f32,
        sample_rate: f32,
    ) -> Self {
        LowpassFilter {
            cutoff,
            resonance,
            sample_rate,
            prev_output: 0.0,
        }
    }
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
    }

    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance;
    }
}

impl Filter for LowpassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff;
        let resonance = self.resonance;
        let c = 1.0 / (2.0 * std::f32::consts::PI * cutoff / self.sample_rate);
        let r = resonance;
        self.prev_output = c * input + r * self.prev_output;
        self.prev_output
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

pub struct NotchFilter {
    cutoff: f32,
    bandwidth: f32,
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
    pub fn new(
        cutoff: f32,
        bandwidth: f32,
        sample_rate: f32,
    ) -> Self {
        let mut filter = NotchFilter {
            cutoff,
            bandwidth,
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
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
    }

    pub fn set_resonance(&mut self, resonance: f32) {
        self.bandwidth = resonance;
    }
}

impl Filter for NotchFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff;
        let bandwidth = self.bandwidth;

        if cutoff != self.cutoff || bandwidth != self.bandwidth {
            self.cutoff = cutoff;
            self.bandwidth = bandwidth;
            self.calculate_coefficients();
        }

        // apply filter
        let output = self.a0 * input + self.a1 * self.buf0 + self.a2 * self.buf1
            - self.b1 * self.buf0
            - self.b2 * self.buf1;
        self.buf1 = self.buf0;
        self.buf0 = output;
        output
    }
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.calculate_coefficients();
    }
}

pub struct StatevariableFilter {
    cutoff: f32,
    resonance: f32,
    sample_rate: f32,
    prev_input: f32,
    lowpass_output: f32,
    highpass_output: f32,
    bandpass_output: f32,
}

impl StatevariableFilter {
    pub fn new(
        cutoff: f32,
        resonance: f32,
        sample_rate: f32,
    ) -> Self {
        StatevariableFilter {
            cutoff,
            resonance,
            sample_rate,
            prev_input: 0.0,
            lowpass_output: 0.0,
            highpass_output: 0.0,
            bandpass_output: 0.0,
        }
    }
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
    }

    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance;
    }
}

impl Filter for StatevariableFilter {
    fn process(&mut self, input: f32) -> f32 {
        let cutoff = self.cutoff;
        let resonance = self.resonance;

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
    sample_rate: f32,
}

impl NoneFilter {
    pub fn new(
        cutoff: f32,
        resonance: f32,
        sample_rate: f32,
    ) -> Self {
        NoneFilter {
            cutoff,
            resonance,
            sample_rate,
        }
    }
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
    }

    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance;
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
            r: 0.995, // The closer this value to 1.0, the lower the cutoff frequency
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        self.y1 = input - self.x1 + self.r * self.y1;
        self.x1 = input;
        self.y1
    }
}

pub fn generate_filter(
    filter_type: FilterType,
    cutoff: f32,
    resonance: f32,
    filter_cut_envelope: &mut ADSREnvelope,
    filter_res_envelope: &mut ADSREnvelope,
    input: f32,
    sample_rate: f32,
) -> f32 {
    let filter_cut = filter_cut_envelope.get_value() * cutoff;
    let filter_res = filter_res_envelope.get_value() * resonance;
    
    match filter_type {
        FilterType::None => input,
        FilterType::Lowpass => {
            let mut filter = LowpassFilter::new(cutoff, resonance, sample_rate);
            filter.set_cutoff(filter_cut);
            filter.set_resonance(filter_res);
            filter.process(input)
        }
        FilterType::Highpass => {
            let mut filter = HighpassFilter::new(cutoff, resonance, sample_rate);
            filter.set_cutoff(filter_cut);
            filter.set_resonance(filter_res);
            filter.process(input)
        }
        FilterType::Bandpass => {
            let mut filter = BandpassFilter::new(cutoff, resonance, sample_rate);
            filter.set_cutoff(filter_cut);
            filter.set_resonance(filter_res);
            filter.process(input)
        }
        FilterType::Notch => {
            let mut filter = NotchFilter::new(cutoff, resonance, sample_rate);
            filter.set_cutoff(filter_cut);
            filter.set_resonance(filter_res);
            filter.process(input)
        }
        FilterType::Statevariable => {
            let mut filter = StatevariableFilter::new(cutoff, resonance, sample_rate);
            filter.set_cutoff(filter_cut);
            filter.set_resonance(filter_res);
            filter.process(input)
        }
    }
}

