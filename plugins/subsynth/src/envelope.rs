pub trait Envelope {
    fn get_value(&mut self, dt: f32) -> f32;
    fn trigger(&mut self);
    fn release(&mut self);
}

pub struct ADSREnvelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    state: ADSREnvelopeState,
    time: f32,
}

enum ADSREnvelopeState {
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
            state: ADSREnvelopeState::Idle,
            time: 0.0,
        }
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
}

impl Envelope for ADSREnvelope {
    fn get_value(&mut self, dt: f32) -> f32 {
        self.time += dt;
        match self.state {
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
        }
    }

    fn trigger(&mut self) {
        self.state = ADSREnvelopeState::Attack;
        self.time = 0.0;
    }

    fn release(&mut self) {
        self.state = ADSREnvelopeState::Release;
    }
}
