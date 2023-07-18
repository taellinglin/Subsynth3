use nih_plug::prelude::Enum;

pub trait Envelope {
    fn get_value(&mut self) -> f32;
    fn trigger(&mut self);
    fn release(&mut self);
    fn set_envelope_stage(&mut self, stage: ADSREnvelopeState);
    fn get_envelope_stage(&self) -> ADSREnvelopeState;
    fn set_scale(&mut self, envelope_levels: f32);
}
<<<<<<< HEAD

=======
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ADSREnvelope {
    attack: f32,
    hold: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    state: ADSREnvelopeState,
    time: f32,
<<<<<<< HEAD
    delta_time_per_sample: f32,
    sample_rate: f32,
    velocity: f32,
    is_sustained: bool,
    scale: f32,
=======
    delta_time_per_sample: f32, // add this new field
    sample_rate: f32,
    velocity: f32,
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
}

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum ADSREnvelopeState {
    Idle,
    Attack,
    Hold,
    Decay,
    Sustain,
    Release,
}

impl ADSREnvelope {
    pub fn new(
        attack: f32,
<<<<<<< HEAD
        hold: f32,
=======
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
        decay: f32,
        sustain: f32,
        release: f32,
        sample_rate: f32,
        velocity: f32,
    ) -> Self {
        ADSREnvelope {
            attack,
            hold,
            decay,
            sustain,
            release,
            state: ADSREnvelopeState::Attack,
            time: 0.0,
            sample_rate,
<<<<<<< HEAD
            delta_time_per_sample: 1.0 / sample_rate,
            velocity,
            is_sustained: false,
            scale: 1.0,
=======
            delta_time_per_sample: 1.0 / sample_rate, // calculate the delta_time_per_sample
            velocity,
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
        }
    }
    pub fn set_velocity(&mut self, velocity: f32) {
        self.velocity = velocity;

<<<<<<< HEAD
    pub fn set_velocity(&mut self, velocity: f32) {
        self.velocity = velocity;

=======
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
        // Adjust envelope parameters based on velocity
        // Example: Modify attack and release times based on velocity
        self.attack *= velocity;
        self.release *= velocity;
        self.decay *= velocity;
        self.sustain *= velocity;

        // Additional adjustments based on velocity if needed
    }
<<<<<<< HEAD

    pub fn get_time(&mut self) -> f32 {
        self.time
    }

=======
    pub fn get_time(&mut self) -> f32 {
        self.time
    }
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
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
<<<<<<< HEAD
            ADSREnvelopeState::Hold => self.sustain,
=======
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
            ADSREnvelopeState::Decay => 1.0 - (1.0 - self.sustain) * (self.time / self.decay),
            ADSREnvelopeState::Sustain => self.sustain,
            ADSREnvelopeState::Release => self.sustain * (1.0 - (self.time / self.release)),
        }
    }
<<<<<<< HEAD

=======
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
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
<<<<<<< HEAD
                self.state = ADSREnvelopeState::Hold;
                self.time = 0.0;
            }
            ADSREnvelopeState::Hold if change >= self.attack + self.hold => {
                self.state = ADSREnvelopeState::Decay;
                self.time = 0.0;
            }
            ADSREnvelopeState::Decay if change >= self.attack + self.hold + self.decay => {
                self.state = ADSREnvelopeState::Sustain;
                self.time = 0.0;
            }
            ADSREnvelopeState::Sustain if change >= self.attack + self.hold + self.decay + self.sustain => {
                self.state = ADSREnvelopeState::Release;
=======
                self.state = ADSREnvelopeState::Decay;
                self.time = 0.0;
            }
            ADSREnvelopeState::Decay if change >= self.decay => {
                self.state = ADSREnvelopeState::Sustain;
                self.time = 0.0;
            }
            ADSREnvelopeState::Release if change >= self.release => {
                self.state = ADSREnvelopeState::Idle;
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
                self.time = 0.0;
            }
            _ => {}
        }
    }
<<<<<<< HEAD

    pub fn get_attack(&self) -> f32 {
        self.attack
    }

    pub fn get_decay(&self) -> f32 {
        self.decay
    }

    pub fn get_sustain(&self) -> f32 {
        self.sustain
    }

    pub fn get_release(&self) -> f32 {
        self.release
    }

    pub fn get_envelope_stage(&self) -> ADSREnvelopeState {
        self.state
    }

    // Setter for envelope stage
    pub fn set_envelope_stage(&mut self, stage: ADSREnvelopeState) {
        self.state = stage;
    }
    pub fn set_scale(&mut self, envelope_levels: f32) {
        self.scale = envelope_levels;
        // Additional scaling for other envelope parameters if needed
        self.attack *= self.scale;
        self.decay *= self.scale;
        self.sustain *= self.scale;
        self.release *= self.scale;
    }
    pub fn set_hold(&mut self, hold: f32) {
        self.hold = hold;
    }
=======
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
}

impl Envelope for ADSREnvelope {
    fn get_value(&mut self) -> f32 {
<<<<<<< HEAD
        match self.state {
            ADSREnvelopeState::Idle => 0.0,
            ADSREnvelopeState::Attack => {
                if self.time >= self.attack {
                    self.state = ADSREnvelopeState::Hold;
=======
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
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
                    self.time = 0.0;
                    self.previous_value()
                } else {
<<<<<<< HEAD
                    self.time / self.attack
=======
                    self.time / velocity_sensitive_attack
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
                }
            }
            ADSREnvelopeState::Hold => {
                if self.time >= self.hold {
                    self.state = ADSREnvelopeState::Decay;
                    self.time = 0.0;
                }
                self.previous_value()
            }
            ADSREnvelopeState::Decay => {
<<<<<<< HEAD
                if self.time >= self.decay {
=======
                if self.time >= velocity_sensitive_decay {
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
                    self.state = ADSREnvelopeState::Sustain;
                    self.time = 0.0;
                    self.previous_value()
                } else {
<<<<<<< HEAD
                    1.0 - (1.0 - self.sustain) * (self.time / self.decay)
=======
                    1.0 - (1.0 - self.sustain) * (self.time / velocity_sensitive_decay)
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
                }
            }
            ADSREnvelopeState::Sustain => {
                if !self.is_sustained {
                    self.state = ADSREnvelopeState::Release;
                    self.time = 0.0;
                }
                self.sustain
            }
            ADSREnvelopeState::Release => {
<<<<<<< HEAD
                if self.time >= self.release {
=======
                if self.time >= velocity_sensitive_release {
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
                    self.state = ADSREnvelopeState::Idle;
                    self.time = 0.0;
                    0.0
                } else {
<<<<<<< HEAD
                    self.sustain * (1.0 - (self.time / self.release))
=======
                    self.sustain * (1.0 - (self.time / velocity_sensitive_release))
>>>>>>> 3482718249a9c468c4ffc5180848ebfa78e2283d
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
        self.is_sustained = false;
    }

    fn release(&mut self) {
        self.state = ADSREnvelopeState::Release;
        self.time = 0.0;
        self.is_sustained = false;
    }

    fn get_envelope_stage(&self) -> ADSREnvelopeState {
        self.state
    }

    fn set_envelope_stage(&mut self, stage: ADSREnvelopeState) {
        self.state = stage;
    }
    fn set_scale(&mut self, envelope_levels: f32) {
        self.set_scale(envelope_levels);
    }
}
