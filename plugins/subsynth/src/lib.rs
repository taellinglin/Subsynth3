mod waveform;
mod editor;
mod filter;

use nih_plug::prelude::*;
use rand::Rng;
use rand_pcg::Pcg32;
use std::sync::Arc;
use waveform::Waveform;
use waveform::generate_waveform;
use filter::{NotchFilter, BandpassFilter, HighpassFilter, LowpassFilter, StatevariableFilter};
use filter::{Filter, FilterType, FilterFactory, Envelope, ADSREnvelope, ADSREnvelopeState};

use filter::generate_filter;

use nih_plug_vizia::ViziaState;
use nih_plug::params::enums::EnumParam;

const NUM_VOICES: u32 = 16;
const MAX_BLOCK_SIZE: usize = 64;
const GAIN_POLY_MOD_ID: u32 = 0;

struct SubSynth {
    params: Arc<SubSynthParams>,
    prng: Pcg32,
    voices: [Option<Voice>; NUM_VOICES as usize],
    next_internal_voice_id: u64,
    
}

#[derive(Params)]
struct SubSynthParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,
    #[id = "gain"]
    gain: FloatParam,
    #[id = "amp_atk"]
    amp_attack_ms: FloatParam,
    #[id = "amp_rel"]
    amp_release_ms: FloatParam,
    #[id = "waveform"]
    waveform: EnumParam<Waveform>,

    // New parameters for ADSR envelope
    #[id = "amp_dec"]
    amp_decay_ms: FloatParam,
    #[id = "amp_sus"]
    amp_sustain_level: FloatParam,
    #[id = "filter_cut_atk"]
    filter_cut_attack_ms: FloatParam,
    #[id = "filter_cut_dec"]
    filter_cut_decay_ms: FloatParam,
    #[id = "filter_cut_sus"]
    filter_cut_sustain_ms: FloatParam,
    #[id = "filter_cut_rel"]
    filter_cut_release_ms: FloatParam,
    #[id = "filter_res_atk"]
    filter_res_attack_ms: FloatParam,
    #[id = "filter_res_dec"]
    filter_res_decay_ms: FloatParam,
    #[id = "filter_res_sus"]
    filter_res_sustain_ms: FloatParam,
    #[id = "filter_res_rel"]
    filter_res_release_ms: FloatParam,
    #[id = "filter_type"]
    filter_type: EnumParam<FilterType>,
    #[id = "filter_cut"]
    filter_cut: FloatParam,
    #[id = "filter_res"]
    filter_res: FloatParam,
}

#[derive(Debug, Clone)]
struct Voice {
    voice_id: i32,
    channel: u8,
    note: u8,
    internal_voice_id: u64,
    velocity_sqrt: f32,
    phase: f32,
    phase_delta: f32,
    releasing: bool,
    amp_envelope: ADSREnvelope,
    voice_gain: Option<(f32, Smoother<f32>)>,
    filter_cut_envelope: Smoother<f32>,
    filter_res_envelope: Smoother<f32>,
    filter: Option<FilterType>,

}


impl Default for SubSynth {
    fn default() -> Self {
        Self {
            
            params: Arc::new(SubSynthParams::default()),

            prng: Pcg32::new(420, 1337),
            voices: [0; NUM_VOICES as usize].map(|_| None),
            next_internal_voice_id: 0,
        }
    }
}

impl Default for SubSynthParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-12.0),
                FloatRange::Linear {
                    min: util::db_to_gain(-36.0),
                    max: util::db_to_gain(0.0),
                },
            )
            .with_poly_modulation_id(GAIN_POLY_MOD_ID)
            .with_smoother(SmoothingStyle::Logarithmic(5.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            amp_attack_ms: FloatParam::new(
                "Attack",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            amp_release_ms: FloatParam::new(
                "Release",
                0.25,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            waveform: EnumParam::new("Waveform", Waveform::Sine),
            amp_decay_ms: FloatParam::new(
                "Decay",
                2000.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            amp_sustain_level: FloatParam::new(
                "Sustain",
                1000.0,
                FloatRange::Skewed {
                    min: -1000.0,
                    max: 1000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_type: EnumParam::new("Filter Type", FilterType::Lowpass),
            filter_cut: FloatParam::new(
                "Filter Cutoff",
                10000.0,
                FloatRange::Linear {
                    min: 20.0,
                    max: 20000.0,
                },
            )
            .with_unit(" Hz"),
            filter_res: FloatParam::new(
                "Filter Resonance",
                0.0,
                FloatRange::Linear {
                    min: -1000.0,
                    max: 1000.0,
                },
            )
            .with_unit(" Q"),
            filter_cut_attack_ms: FloatParam::new(
                "Filter Cut Attack",
                200.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_cut_decay_ms: FloatParam::new(
                "Filter Cut Decay",
                2000.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_cut_sustain_ms: FloatParam::new(
                "Filter Cut Sustain",
                1000.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_cut_release_ms: FloatParam::new(
                "Filter Cut Release",
                1000.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_res_attack_ms: FloatParam::new(
                "Filter Resonance Attack",
                2000.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_res_decay_ms: FloatParam::new(
                "Filter Resonance Decay",
                2000.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_res_sustain_ms: FloatParam::new(
                "Filter Resonance Sustain",
                1000.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_res_release_ms: FloatParam::new(
                "Filter Resonance Decay",
                200.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
        }
    }
}



impl Plugin for SubSynth {
    const NAME: &'static str = "SubSynthBeta";
    const VENDOR: &'static str = "LingYue Synth";
    const URL: &'static str = "https://taellinglin.art";
    const EMAIL: &'static str = "taellinglin@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }
    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB

        true
    }

    fn reset(&mut self) {
        self.prng = Pcg32::new(420, 1337);

        self.voices.fill(None);
        self.next_internal_voice_id = 0;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let num_samples = buffer.samples();
        let sample_rate = context.transport().sample_rate;
        let output = buffer.as_slice();
    
        let mut next_event = context.next_event();
        let mut block_start: usize = 0;
        let mut block_end: usize = MAX_BLOCK_SIZE.min(num_samples);
    
        while block_start < num_samples {
            let this_sample_internal_voice_id_start = self.next_internal_voice_id;
    
            'events: loop {
                match next_event {
                    Some(event) if (event.timing() as usize) <= block_start => {
                        match event {
                            NoteEvent::NoteOn {
                                timing,
                                voice_id,
                                channel,
                                note,
                                velocity,
                            } => {
                                let initial_phase: f32 = self.prng.gen();
                                let attack_time = self.params.amp_attack_ms.value() * 1000.0; // Convert attack time to seconds
                                let release_time = self.params.amp_release_ms.value() * 1000.0; // Convert release time to seconds
                                let decay_time = self.params.amp_decay_ms.value() * 1000.0; // Convert decay time to seconds
                                let sustain_level = self.params.amp_sustain_level.value() * 1000.0; // Sustain level (between 0.0 and 1.0)

                                let amp_envelope = ADSREnvelope::new(attack_time, decay_time, sustain_level, release_time);
                                //amp_envelope.trigger();

                                let voice = self.start_voice(context, timing, voice_id, channel, note);
                                voice.velocity_sqrt = velocity.sqrt();
                                voice.phase = initial_phase;
                                voice.phase_delta = util::midi_note_to_freq(note) / sample_rate;

                                voice.amp_envelope = amp_envelope;

                            }
                            NoteEvent::NoteOff {
                                timing: _,
                                voice_id,
                                channel,
                                note,
                                velocity: _,
                            } => {
                                self.start_release_for_voices(sample_rate, voice_id, channel, note)
                            }
                            NoteEvent::Choke {
                                timing,
                                voice_id,
                                channel,
                                note,
                            } => {
                                self.choke_voices(context, timing, voice_id, channel, note);
                            }
                            NoteEvent::PolyModulation {
                                timing: _,
                                voice_id,
                                poly_modulation_id,
                                normalized_offset,
                            } => {
                                if let Some(voice_idx) = self.get_voice_idx(voice_id) {
                                    let voice = self.voices[voice_idx].as_mut().unwrap();
    
                                    match poly_modulation_id {
                                        GAIN_POLY_MOD_ID => {
                                            let target_plain_value = self
                                                .params
                                                .gain
                                                .preview_modulated(normalized_offset);
                                            let (_, smoother) = voice.voice_gain.get_or_insert_with(|| {
                                                (
                                                    normalized_offset,
                                                    self.params.gain.smoothed.clone(),
                                                )
                                            });
                                            if voice.internal_voice_id >= this_sample_internal_voice_id_start {
                                                smoother.reset(target_plain_value);
                                            } else {
                                                smoother.set_target(sample_rate, target_plain_value);
                                            }
                                        }
                                        n => nih_debug_assert_failure!(
                                            "Polyphonic modulation sent for unknown poly modulation ID {}",
                                            n
                                        ),
                                    }
                                }
                            }
                            NoteEvent::MonoAutomation {
                                timing: _,
                                poly_modulation_id,
                                normalized_value,
                            } => {
                                for voice in self.voices.iter_mut().filter_map(|v| v.as_mut()) {
                                    match poly_modulation_id {
                                        GAIN_POLY_MOD_ID => {
                                            let (normalized_offset, smoother) =
                                                match voice.voice_gain.as_mut() {
                                                    Some((o, s)) => (o, s),
                                                    None => continue,
                                                };
                                            let target_plain_value =
                                                self.params.gain.preview_plain(
                                                    normalized_value + *normalized_offset,
                                                );
                                            smoother.set_target(sample_rate, target_plain_value);
                                        }
                                        n => nih_debug_assert_failure!(
                                            "Automation event sent for unknown poly modulation ID {}",
                                            n
                                        ),
                                    }
                                }
                            }
                            _ => (),
                        };
    
                        next_event = context.next_event();
                    }
                    Some(event) if (event.timing() as usize) < block_end => {
                        block_end = event.timing() as usize;
                        break 'events;
                    }
                    _ => break 'events,
                }
            }
    
            // Clear output buffer
            output[0][block_start..block_end].fill(0.0);
            output[1][block_start..block_end].fill(0.0);
    
            let block_len = block_end - block_start;
            let mut gain = [0.0; MAX_BLOCK_SIZE];
            let mut voice_gain = [0.0; MAX_BLOCK_SIZE];
            self.params.gain.smoothed.next_block(&mut gain, block_len);
    
            // Process voices
            for voice in self.voices.iter_mut().filter_map(|v| v.as_mut()) {
                let gain = match &voice.voice_gain {
                    Some((_, smoother)) => {
                        smoother.next_block(&mut voice_gain, block_len);
                        &voice_gain
                    }
                    None => &gain,
                };
                    if let ADSREnvelopeState::Idle = voice.amp_envelope.get_state() {
                        if voice.amp_envelope.get_value(0.0) == 0.0 {
                            context.send_event(NoteEvent::VoiceTerminated {
                                timing: block_end as u32,
                                voice_id: Some(voice.voice_id),
                                channel: voice.channel,
                                note: voice.note,
                            });
                            //self.voices[voice_idx] = None;
                        }
                    }
                    
                    for (value_idx, sample_idx) in (block_start..block_end).enumerate() {
                        let envelope_time = voice.amp_envelope.get_time();
                        let amp = voice.velocity_sqrt * gain[value_idx] * voice.amp_envelope.get_value(envelope_time);
                        //voice.amp_envelope.trigger();
                    
                        // Generate waveform
                        let waveform = self.params.waveform.value();
                        let mut generated_sample = generate_waveform(waveform, voice.phase);
                    
                        // Apply filter
                        let filter_type = self.params.filter_type.value();
                        let cutoff = self.params.filter_cut.value();
                        let resonance = self.params.filter_res.value();
                        let cutoff_attack = self.params.filter_cut_attack_ms.value();
                        let cutoff_decay = self.params.filter_cut_decay_ms.value();
                        let cutoff_sustain = self.params.filter_cut_sustain_ms.value();
                        let cutoff_release = self.params.filter_cut_release_ms.value();
                        let resonance_attack = self.params.filter_res_attack_ms.value();
                        let resonance_decay = self.params.filter_res_decay_ms.value();
                        let resonance_sustain = self.params.filter_res_sustain_ms.value();
                        let resonance_release = self.params.filter_res_release_ms.value();
                    
                        let mut filtered_sample = generate_filter(
                            filter_type,
                            cutoff,
                            resonance,
                            cutoff_attack,
                            cutoff_decay,
                            cutoff_sustain,
                            cutoff_release,
                            resonance_attack,
                            resonance_decay,
                            resonance_sustain,
                            resonance_release,
                            generated_sample,
                            sample_rate,
                        );
                        filtered_sample.set_sample_rate(sample_rate);
                    
                        // Apply envelope to each sample of the waveform
                        for _ in 0..block_len {
                            let processed_sample = filtered_sample.process(amp * generated_sample);
                    
                            output[0][sample_idx] += processed_sample;
                            output[1][sample_idx] += processed_sample;
                    
                            //generated_sample = generated_sample * amp;
                            voice.phase += voice.phase_delta;
                            if voice.phase >= 1.0 {
                                voice.phase -= 1.0;
                            }
                        }
                    }
                    
            }

            // Process voice termination
            let mut terminated_voices: Vec<usize> = vec![]; // Track the voices to terminate
            for (voice_idx, voice) in self.voices.iter_mut().enumerate() {
                if let Some(voice) = voice {
                    if voice.amp_envelope.get_state() == ADSREnvelopeState::Idle {
                        // Voice has reached the idle state
                        terminated_voices.push(voice_idx);
                    }
                }
            }

            // Terminate the voices outside the loop to avoid modifying the vector while iterating
            for voice_idx in terminated_voices {
                if let Some(voice) = self.voices[voice_idx].take() {
                    context.send_event(NoteEvent::VoiceTerminated {
                        timing: block_end as u32,
                        voice_id: Some(voice.voice_id),
                        channel: voice.channel,
                        note: voice.note,
                    });
                }
            }




            block_start = block_end;
            block_end = (block_start + MAX_BLOCK_SIZE).min(num_samples);
        }

        ProcessStatus::Normal
    }
}    
impl SubSynth {
    fn get_voice_idx(&mut self, voice_id: i32) -> Option<usize> {
        self.voices
            .iter_mut()
            .position(|voice| matches!(voice, Some(voice) if voice.voice_id == voice_id))
    }

    fn start_voice(
        &mut self,
        context: &mut impl ProcessContext<Self>,
        sample_offset: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
    ) -> &mut Voice {
        let new_voice = Voice {
            voice_id: voice_id.unwrap_or_else(|| compute_fallback_voice_id(note, channel)),
            internal_voice_id: self.next_internal_voice_id,
            channel,
            note,
            velocity_sqrt: 1.0,
    
            phase: 0.0,
            phase_delta: 0.0,
            releasing: false,
            amp_envelope: ADSREnvelope::new(
                self.params.amp_attack_ms.value(),
                self.params.amp_decay_ms.value(),
                self.params.amp_sustain_level.value(),
                self.params.amp_release_ms.value(),
            ),
            voice_gain: None,
            filter_cut_envelope: Smoother::new(SmoothingStyle::Linear(0.0)),
            filter_res_envelope: Smoother::new(SmoothingStyle::Linear(0.0)),
    
            filter: Some(self.params.filter_type.value()),
        };
    
        self.next_internal_voice_id = self.next_internal_voice_id.wrapping_add(1);
    
        if let Some(free_voice_idx) = self.voices.iter().position(|voice| voice.is_none()) {
            let voice = &mut self.voices[free_voice_idx];
            if voice.is_none() {
                *voice = Some(new_voice);
                voice.as_mut().unwrap().amp_envelope.trigger();
            }
            voice.as_mut().unwrap()
        } else {
            let oldest_voice = self
                .voices
                .iter_mut()
                .min_by_key(|voice| voice.as_ref().unwrap().internal_voice_id)
                .unwrap();
            let oldest_voice = oldest_voice.as_mut().unwrap();
    
            context.send_event(NoteEvent::VoiceTerminated {
                timing: sample_offset,
                voice_id: Some(oldest_voice.voice_id),
                channel: oldest_voice.channel,
                note: oldest_voice.note,
            });
    
            *oldest_voice = new_voice;
            oldest_voice
        }
    }
    
    
    

    fn start_release_for_voices(
        &mut self,
        sample_rate: f32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
    ) {
        for voice in self.voices.iter_mut() {
            if let Some(voice) = voice {
                if voice_id == Some(voice.voice_id) || (channel == voice.channel && note == voice.note) {
                    voice.amp_envelope.release();
                }
            }
        }
    }
    

    
    

    fn choke_voices(
        &mut self,
        context: &mut impl ProcessContext<Self>,
        sample_offset: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
    ) {
        for voice in self.voices.iter_mut() {
            match voice {
                Some(Voice {
                    voice_id: candidate_voice_id,
                    channel: candidate_channel,
                    note: candidate_note,
                    ..
                }) if voice_id == Some(*candidate_voice_id)
                    || (channel == *candidate_channel && note == *candidate_note) =>
                {
                    context.send_event(NoteEvent::VoiceTerminated {
                        timing: sample_offset,
                        voice_id: Some(*candidate_voice_id),
                        channel,
                        note,
                    });
                    //*voice = None;

                    if voice_id.is_some() {
                        return;
                    }
                }
                _ => (),
            }
        }
    }
    fn waveform(&self) -> Waveform {
        self.params.waveform.value()
    }
    
}

const fn compute_fallback_voice_id(note: u8, channel: u8) -> i32 {
    note as i32 | ((channel as i32) << 16)
}

impl ClapPlugin for SubSynth {
    const CLAP_ID: &'static str = "art.taellinglin";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A Polyphonic Subtractive Synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
    ];

    const CLAP_POLY_MODULATION_CONFIG: Option<PolyModulationConfig> = Some(PolyModulationConfig {
        max_voice_capacity: NUM_VOICES,
        supports_overlapping_voices: true,
    });
}

impl Vst3Plugin for SubSynth {
    const VST3_CLASS_ID: [u8; 16] = *b"SubSynthLing0Lin";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Stereo,
    ];
}


nih_export_clap!(SubSynth);
nih_export_vst3!(SubSynth);
