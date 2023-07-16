use nih_plug::prelude::Enum;

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
    pub fn new(
        attack: f32,
        decay: f32,
        sustain: f32,
        release: f32,
        sample_rate: f32,
        velocity: f32,
    ) -> Self {
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
    pub fn get_time(&mut self) -> f32 {
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
    pub fn advance(&mut self) {
        self.time += self.delta_time_per_sample;

        // Adjust envelope parameters based on velocity sensitivity
        let change = self.time * self.velocity;

        match self.state {
            // Check if the envelope has completed and move to the next stage
            _ if self.state != ADSREnvelopeState::Idle && change >= self.release => {
                self.state = ADSREnvelopeState::Idle;
                self.time = 0.0;
            }
            ADSREnvelopeState::Attack if change >= self.attack => {
                self.state = ADSREnvelopeState::Decay;
                self.time = 0.0;
            }
            ADSREnvelopeState::Decay if change >= self.decay => {
                self.state = ADSREnvelopeState::Sustain;
                self.time = 0.0;
            }
            ADSREnvelopeState::Release if change >= self.release => {
                self.state = ADSREnvelopeState::Idle;
                self.time = 0.0;
            }
            _ => {}
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
                if self.time >= velocity_sensitive_attack {
                    self.state = ADSREnvelopeState::Decay;
                    self.time = 0.0;
                    1.0
                } else {
                    self.time / velocity_sensitive_attack
                }
            }
            ADSREnvelopeState::Decay => {
                if self.time >= velocity_sensitive_decay {
                    self.state = ADSREnvelopeState::Sustain;
                    self.time = 0.0;
                    self.sustain
                } else {
                    1.0 - (1.0 - self.sustain) * (self.time / velocity_sensitive_decay)
                }
            }
            ADSREnvelopeState::Sustain => self.sustain,
            ADSREnvelopeState::Release => {
                if self.time >= velocity_sensitive_release {
                    self.state = ADSREnvelopeState::Idle;
                    self.time = 0.0;
                    0.0
                } else {
                    self.sustain * (1.0 - (self.time / velocity_sensitive_release))
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
