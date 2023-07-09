use enum_iterator::Sequence;
use nih_plug::params::enums::Enum;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Sequence)]
pub enum Waveform {
    Sine,
    Triangle,
    Sawtooth,
    Square,
    Pulse,
    Noise,
}

pub fn generate_waveform(waveform: Waveform, phase: f32) -> f32 {
    match waveform {
        Waveform::Sine => sine(phase),
        Waveform::Triangle => (2.0 * (phase - 0.5)).abs() * 2.0 - 1.0,
        Waveform::Sawtooth => 1.0 - phase * 2.0,
        Waveform::Square => if phase < 0.5 { 1.0 } else { -1.0 },
        Waveform::Pulse => if phase < 0.25 || phase >= 0.75 { 1.0 } else { -1.0 },
        Waveform::Noise => rand::random::<f32>() * 2.0 - 1.0,
    }
}

fn sine(phase: f32) -> f32 {
    // Wrap phase to the range [0, 1)
    let wrapped_phase = phase % 1.0;

    // Calculate the corresponding angle in radians
    let angle = wrapped_phase * 2.0 * std::f32::consts::PI;

    // Compute the sine value
    angle.sin()
}