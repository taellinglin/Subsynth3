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
    // Drum sounds
    #[name = "Kick"]
    DrumKick,
    #[name = "Snare"]
    DrumSnare,
    #[name = "Tom High"]
    DrumTomHigh,
    #[name = "Tom Mid"]
    DrumTomMid,
    #[name = "Tom Low"]
    DrumTomLow,
    #[name = "Hat Closed"]
    DrumHatClosed,
    #[name = "Hat Open"]
    DrumHatOpen,
    #[name = "Hat Pedal"]
    DrumHatPedal,
    #[name = "Clap"]
    DrumClap,
    #[name = "Rim"]
    DrumRim,
}

pub fn generate_waveform(waveform: Waveform, phase: f32) -> f32 {
    match waveform {
        Waveform::Sine => ((phase % 1.0) * 2.0 * std::f32::consts::PI).sin(),
        Waveform::Triangle => (2.0 * (phase - 0.5)).abs() * 2.0 - 1.0,
        Waveform::Sawtooth => 1.0 - phase * 2.0,
        Waveform::Square => {
            if phase < 0.5 {
                1.0
            } else {
                -1.0
            }
        }
        Waveform::Pulse => {
            if phase < 0.25 || phase >= 0.75 {
                1.0
            } else {
                -1.0
            }
        }
        Waveform::Noise => rand::random::<f32>() * 2.0 - 1.0,
        
        // Drum sounds - these are designed to work with envelopes
        Waveform::DrumKick => {
            // Kick: Sine wave that works best with pitch envelope down from 200Hz to 40Hz
            // The envelope should handle the pitch drop
            ((phase % 1.0) * 2.0 * std::f32::consts::PI).sin()
        }
        Waveform::DrumSnare => {
            // Snare: Mix of noise (70%) and low tone (30%)
            let noise = rand::random::<f32>() * 2.0 - 1.0;
            let tone = ((phase % 1.0) * 2.0 * std::f32::consts::PI * 180.0).sin();
            noise * 0.7 + tone * 0.3
        }
        Waveform::DrumTomHigh => {
            // High tom: Sine with some noise, tuned around 220Hz
            let tone = ((phase % 1.0) * 2.0 * std::f32::consts::PI).sin();
            let noise = (rand::random::<f32>() * 2.0 - 1.0) * 0.15;
            tone * 0.85 + noise
        }
        Waveform::DrumTomMid => {
            // Mid tom: Sine with some noise, tuned around 150Hz
            let tone = ((phase % 1.0) * 2.0 * std::f32::consts::PI).sin();
            let noise = (rand::random::<f32>() * 2.0 - 1.0) * 0.18;
            tone * 0.82 + noise
        }
        Waveform::DrumTomLow => {
            // Low tom: Sine with some noise, tuned around 100Hz
            let tone = ((phase % 1.0) * 2.0 * std::f32::consts::PI).sin();
            let noise = (rand::random::<f32>() * 2.0 - 1.0) * 0.2;
            tone * 0.8 + noise
        }
        Waveform::DrumHatClosed => {
            // Closed hi-hat: High-frequency filtered noise
            let noise = rand::random::<f32>() * 2.0 - 1.0;
            // Bandpass-ish effect by mixing multiple noise sources
            let noise2 = rand::random::<f32>() * 2.0 - 1.0;
            (noise + noise2 * 0.7) * 0.6
        }
        Waveform::DrumHatOpen => {
            // Open hi-hat: Similar to closed but with more brightness
            let noise = rand::random::<f32>() * 2.0 - 1.0;
            let noise2 = rand::random::<f32>() * 2.0 - 1.0;
            let noise3 = rand::random::<f32>() * 2.0 - 1.0;
            (noise + noise2 * 0.8 + noise3 * 0.4) * 0.5
        }
        Waveform::DrumHatPedal => {
            // Pedal hi-hat: Short, dampened version
            let noise = rand::random::<f32>() * 2.0 - 1.0;
            noise * 0.7
        }
        Waveform::DrumClap => {
            // Clap: Burst of noise with some filtering
            let noise = rand::random::<f32>() * 2.0 - 1.0;
            let noise2 = rand::random::<f32>() * 2.0 - 1.0;
            // Multiple layers for clap effect
            (noise * 0.6 + noise2 * 0.4)
        }
        Waveform::DrumRim => {
            // Rim shot: Very short, sharp noise burst with metallic quality
            let noise = rand::random::<f32>() * 2.0 - 1.0;
            let click = if phase < 0.01 { 1.0 } else { 0.0 };
            noise * 0.4 + click * 0.6
        }
    }
}
