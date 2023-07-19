use std::f32::consts::PI;
use enum_iterator::Sequence;
use nih_plug::params::enums::Enum;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Sequence)]
pub enum OscillatorShape {
    Sine,
    Triangle,
    Sawtooth,
    Square,
}

#[derive(Debug, Clone, Copy)]
pub struct Modulator {
    modulation_rate: f32,
    peak_intensity: f32,
    attack_duration: f32,
    oscillator_shape: OscillatorShape,
    current_time: f32,
    triggered: bool,
}

impl Modulator {
    pub fn new(
        modulation_rate: f32,
        peak_intensity: f32,
        attack_duration: f32,
        oscillator_shape: OscillatorShape,
    ) -> Self {
        Modulator {
            modulation_rate,
            peak_intensity,
            attack_duration,
            oscillator_shape,
            current_time: 0.0,
            triggered: true,
        }
    }

    pub fn trigger(&mut self) {
        self.current_time = 0.0;
        self.triggered = true;
    }

    fn update(&mut self, dt: f32) {
        if self.triggered {
            self.current_time += dt;
            self.current_time = self.current_time.min(self.attack_duration); // Clamp current time to attack duration
            if self.current_time >= self.attack_duration {
                self.triggered = false;
            }
        } else {
            self.current_time += dt; // Continue incrementing the current time even after the attack duration
        }
    }

    pub fn get_modulation(&mut self, sample_rate: f32) -> f32 {
        let dt = 1.0 / sample_rate;
        self.update(dt);
    
        let attack_progress = self.current_time / self.attack_duration;
        let intensity = if attack_progress < 1.0 {
            self.peak_intensity * attack_progress
        } else {
            self.peak_intensity
        };
    
        let modulation = match self.oscillator_shape {
            OscillatorShape::Sine => (2.0 * PI * self.modulation_rate * self.current_time).sin(),
            OscillatorShape::Triangle => (2.0 * self.modulation_rate * self.current_time).fract() * 2.0 - 1.0,
            OscillatorShape::Sawtooth => (2.0 * self.modulation_rate * self.current_time).fract() * 2.0 - 1.0,
            OscillatorShape::Square => {
                if (2.0 * self.modulation_rate * self.current_time).fract() >= 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
        };
    
        modulation * intensity
    }
    
}
