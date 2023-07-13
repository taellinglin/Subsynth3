mod waveform;
mod editor;
mod filter;

use nih_plug::prelude::*;
use rand::Rng;
use rand_pcg::Pcg32;
use std::sync::Arc;
use waveform::Waveform;
use waveform::generate_waveform;

use filter::{FilterType, Envelope, ADSREnvelope};

use filter::generate_filter;

use nih_plug_vizia::ViziaState;
use nih_plug::params::enums::EnumParam;

const NUM_VOICES: usize = 16;
const MAX_BLOCK_SIZE: usize = 64;
const GAIN_POLY_MOD_ID: u32 = 0;

struct SubSynth {
    params: Arc<SubSynthParams>,
    prng: Pcg32,
    voices: [Option<Voice>; NUM_VOICES as usize],
    next_voice_index: usize,
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
    velocity: f32,
    velocity_sqrt: f32,
    phase: f32,
    phase_delta: f32,
    releasing: bool,
    amp_envelope: ADSREnvelope,
    voice_gain: Option<(f32, Smoother<f32>)>,
    filter_cut_envelope: ADSREnvelope,
    filter_res_envelope: ADSREnvelope,
    filter: Option<FilterType>,
    pan: f32, // Added pan field
    tuning: f32,       // Add tuning field
    vibrato: f32,      // Add vibrato field
    expression: f32,   // Add expression field
    brightness: f32,   // Add brightness field
}


impl Default for SubSynth {
    fn default() -> Self {
        Self {
            
            params: Arc::new(SubSynthParams::default()),

            prng: Pcg32::new(420, 1337),
            voices: [0; NUM_VOICES as usize].map(|_| None),
            next_internal_voice_id: 0,
            next_voice_index: 0,
        }
    }
}

impl Default for SubSynthParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-36.0),
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
                10.0,
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
                1.0,
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
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            amp_sustain_level: FloatParam::new(
                "Sustain",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_type: EnumParam::new("Filter Type", FilterType::None),
            filter_cut: FloatParam::new(
                "Filter Cutoff",
                10000.0,
                FloatRange::Linear {
                    min: 20.0,
                    max: 192000.0,
                },
            )
            .with_unit(" Hz"),
            filter_res: FloatParam::new(
                "Filter Resonance",
                3.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 10.0,
                },
            )
            .with_unit(" Q"),
            filter_cut_attack_ms: FloatParam::new(
                "Filter Cut Attack",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_cut_decay_ms: FloatParam::new(
                "Filter Cut Decay",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_cut_sustain_ms: FloatParam::new(
                "Filter Cut Sustain",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_cut_release_ms: FloatParam::new(
                "Filter Cut Release",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_res_attack_ms: FloatParam::new(
                "Filter Resonance Attack",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_res_decay_ms: FloatParam::new(
                "Filter Resonance Decay",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_res_sustain_ms: FloatParam::new(
                "Filter Resonance Sustain",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_step_size(0.1)
            .with_unit(" ms"),
            filter_res_release_ms: FloatParam::new(
                "Filter Resonance Release",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
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
        _buffer_config: &BufferConfig,
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
        // NIH-plug has a block-splitting adapter for `Buffer`. While this works great for effect
        // plugins, for polyphonic synths the block size should be `min(MAX_BLOCK_SIZE,
        // num_remaining_samples, next_event_idx - block_start_idx)`. Because blocks also need to be
        // split on note events, it's easier to work with raw audio here and to do the splitting by
        // hand.
        let num_samples = buffer.samples();
        let sample_rate = context.transport().sample_rate;
        let output = buffer.as_slice();

        let mut next_event = context.next_event();
        let mut block_start: usize = 0;
        let mut block_end: usize = MAX_BLOCK_SIZE.min(num_samples);
        while block_start < num_samples {
            // First of all, handle all note events that happen at the start of the block, and cut
            // the block short if another event happens before the end of it. To handle polyphonic
            // modulation for new notes properly, we'll keep track of the next internal note index
            // at the block's start. If we receive polyphonic modulation that matches a voice that
            // has an internal note ID that's great than or equal to this one, then we should start
            // the note's smoother at the new value instead of fading in from the global value.
            let this_sample_internal_voice_id_start = self.next_internal_voice_id;
            'events: loop {
                match next_event {
                    // If the event happens now, then we'll keep processing events
                    Some(event) if (event.timing() as usize) < block_end => {

                        // This synth doesn't support any of the polyphonic expression events. A
                        // real synth plugin, however, will want to support those.
                        match event {
                            NoteEvent::NoteOn {
                                timing,
                                voice_id,
                                channel,
                                note,
                                velocity,
                            } => {
                                let pan: f32 = 0.0;
                                let brightness: f32 = 1.0;
                                let expression: f32 = 1.0;
                                let vibrato: f32 = 0.0;
                                let tuning: f32 = 0.0;
                                let initial_phase: f32 = self.prng.gen();
                                // This starts with the attack portion of the amplitude envelope
                                let amp_envelope = ADSREnvelope::new(
                                    self.params.amp_attack_ms.value(),
                                    self.params.amp_decay_ms.value(),
                                    self.params.amp_sustain_level.value(),
                                    self.params.amp_release_ms.value(),
                                    sample_rate,
                                    velocity,
                                );
                                let cutoff_envelope = ADSREnvelope::new(
                                    self.params.filter_cut_attack_ms.value(),
                                    self.params.filter_cut_decay_ms.value(),
                                    self.params.filter_cut_sustain_ms.value(),
                                    self.params.filter_cut_release_ms.value(),
                                    sample_rate,
                                    velocity,
                                );
                                let resonance_envelope = ADSREnvelope::new(
                                    self.params.filter_res_attack_ms.value(),
                                    self.params.filter_res_decay_ms.value(),
                                    self.params.filter_res_sustain_ms.value(),
                                    self.params.filter_res_release_ms.value(),
                                    sample_rate,
                                    velocity,
                                );
                                let voice = self.start_voice(
                                    context,
                                    timing,
                                    voice_id,
                                    channel,
                                    note,
                                    velocity, // Add velocity parameter
                                    pan,
                                    brightness,
                                    expression, // Add expression parameter
                                    vibrato, // Add vibrato parameter
                                    tuning,
                                );
                                voice.velocity_sqrt = velocity.sqrt();
                                voice.phase = initial_phase;
                                let pitch = util::midi_note_to_freq(note) * (2.0_f32).powf((tuning + voice.tuning) / 12.0);
                                voice.phase_delta = pitch / sample_rate;
                                voice.amp_envelope = amp_envelope;
                                voice.filter_cut_envelope = cutoff_envelope;
                                voice.filter_res_envelope = resonance_envelope;
                                voice.velocity = velocity;

                            }
                            NoteEvent::NoteOff {
                                timing: _,
                                voice_id,
                                channel,
                                note,
                                velocity: _,
                            } => {
                                self.start_release_for_voices(sample_rate, voice_id, channel, note);
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
                                // Polyphonic modulation events are matched to voices using the
                                // voice ID, and to parameters using the poly modulation ID. The
                                // host will probably send a modulation event every N samples. This
                                // will happen before the voice is active, and of course also after
                                // it has been terminated (because the host doesn't know that it
                                // will be). Because of that, we won't print any assertion failures
                                // when we can't find the voice index here.
                                if let Some(voice_idx) = self.get_voice_idx(voice_id) {
                                    let voice = self.voices[voice_idx].as_mut().unwrap();
            
                                    match poly_modulation_id {
                                        GAIN_POLY_MOD_ID => {
                                            // This should either create a smoother for this
                                            // modulated parameter or update the existing one.
                                            // Notice how this uses the parameter's unmodulated
                                            // normalized value in combination with the normalized
                                            // offset to create the target plain value
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
            
                                            // If this `PolyModulation` events happens on the
                                            // same sample as a voice's `NoteOn` event, then it
                                            // should immediately use the modulated value
                                            // instead of slowly fading in
                                            if voice.internal_voice_id >= this_sample_internal_voice_id_start
                                            {
                                                smoother.reset(target_plain_value);
                                            } else {
                                                smoother.set_target(sample_rate, target_plain_value);
                                            }
                                        }
                                        n => nih_debug_assert_failure!(
                                            "Polyphonic modulation sent for unknown poly \
                                            modulation ID {}",
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
                                // Modulation always acts as an offset to the parameter's current
                                // automated value. So if the host sends a new automation value for
                                // a modulated parameter, the modulated values/smoothing targets
                                // need to be updated for all polyphonically modulated voices.
                                for voice in self.voices.iter_mut().filter_map(|v| v.as_mut()) {
                                    match poly_modulation_id {
                                        GAIN_POLY_MOD_ID => {
                                            let (normalized_offset, smoother) = match voice.voice_gain.as_mut() {
                                                Some((o, s)) => (o, s),
                                                // If the voice does not have existing
                                                // polyphonic modulation, then there's nothing
                                                // to do here. The global automation/monophonic
                                                // modulation has already been taken care of by
                                                // the framework.
                                                None => continue,
                                            };
                                            let target_plain_value = self.params.gain.preview_plain(
                                                normalized_value + *normalized_offset,
                                            );
                                            smoother.set_target(sample_rate, target_plain_value);
                                        }
                                        n => nih_debug_assert_failure!(
                                            "Automation event sent for unknown poly modulation ID \
                                            {}",
                                            n
                                        ),
                                    }
                                }
                            }
                            NoteEvent::PolyPressure {
                                timing,
                                voice_id,
                                channel,
                                note,
                                pressure,
                            } => {
                                self.handle_poly_pressure_event(
                                    timing,
                                    voice_id,
                                    channel,
                                    note,
                                    pressure,
                                );
                            }
                            NoteEvent::PolyVolume {
                                timing,
                                voice_id,
                                channel,
                                note,
                                gain,
                            } => {
                                self.handle_poly_volume_event(
                                    timing,
                                    voice_id,
                                    channel,
                                    note,
                                    gain,
                                );
                            }
                            NoteEvent::PolyPan {
                                timing,
                                voice_id,
                                channel,
                                note,
                                pan,
                            } => {
                                self.handle_poly_pan_event(
                                    timing, 
                                    voice_id, 
                                    channel, 
                                    note, 
                                    pan);
                            }
                            NoteEvent::PolyTuning {
                                timing,
                                voice_id,
                                channel,
                                note,
                                tuning,
                            } => {
                                self.handle_poly_tuning_event(
                                    timing,
                                    voice_id, 
                                    channel, 
                                    note, 
                                    tuning);
                            }
                            NoteEvent::PolyVibrato {
                                timing,
                                voice_id,
                                channel,
                                note,
                                vibrato,
                            } => {
                                self.handle_poly_vibrato_event(
                                    timing,
                                    voice_id,
                                    channel,
                                    note,
                                    vibrato,
                                );
                            }
                            // Handle other MIDI events if needed
                            _ => (),
                        };
            
                        next_event = context.next_event();
                    }
                    // If the event happens before the end of the block, then the block should be cut
                    // short so the next block starts at the event
                    Some(event) if (event.timing() as usize) < block_end =>{
                        block_end = event.timing() as usize;
                        break 'events;
                    }
                    _ => break 'events,
                }
            }
            

            // We'll start with silence, and then add the output from the active voices
            output[0][block_start..block_end].fill(0.0);
            output[1][block_start..block_end].fill(0.0);

            // These are the smoothed global parameter values. These are used for voices that do not
            // have polyphonic modulation applied to them. With a plugin as simple as this it would
            // be possible to avoid this completely by simply always copying the smoother into the
            // voice's struct, but that may not be realistic when the plugin has hundreds of
            // parameters. The `voice_*` arrays are scratch arrays that an individual voice can use.
            let block_len = block_end - block_start;
            let mut gain = [0.0; MAX_BLOCK_SIZE];
            let mut voice_gain = [0.0; MAX_BLOCK_SIZE];
            self.params.gain.smoothed.next_block(&mut gain, block_len);

            // TODO: Some form of band limiting
            // TODO: Filter
            for (value_idx, sample_idx) in (block_start..block_end).enumerate() {
                // Get mutable reference to the voice at sample_idx
                for voice in self.voices.iter_mut() {
                    if let Some(voice) = voice {
                        // Depending on whether the voice has polyphonic modulation applied to it,
                        // either the global parameter values are used, or the voice's smoother is used
                        // to generate unique modulated values for that voice
                        let gain = match &voice.voice_gain {
                            Some((_, smoother)) => {
                                smoother.next_block(&mut voice_gain, block_len);
                                &voice_gain
                            }
                            None => &gain,
                        };
            
                        // This is an exponential smoother repurposed as an AR envelope with values between
                        // 0 and 1. When a note off event is received, this envelope will start fading out
                        // again. When it reaches 0, we will terminate the voice.
                        voice.amp_envelope.advance();
            
                        let mut dc_blocker = filter::DCBlocker::new();
                        // Apply filter
                        let filter_type = self.params.filter_type.value();
                        let cutoff = self.params.filter_cut.value();
                        let resonance = self.params.filter_res.value();
                        let waveform = self.params.waveform.value();
            
                        // Calculate panning based on voice's pan value
                        let pan = voice.pan;
                        let left_amp = (1.0 - pan).sqrt() as f32;
                        let right_amp = pan.sqrt() as f32;
            
                        // Generate waveform for voice
                        let generated_sample = generate_waveform(waveform, voice.phase);
            
                        // Apply filters to the generated sample
                        let mut filtered_sample = generate_filter(
                            filter_type,
                            cutoff,
                            resonance,
                            voice.filter_cut_envelope,
                            voice.filter_res_envelope,
                            generated_sample,
                            sample_rate,
                        );
                        filtered_sample.set_sample_rate(sample_rate);
            
                        // Calculate amplitude for voice
                        let amp = voice.velocity_sqrt * gain[value_idx] * voice.amp_envelope.get_value();
            
                        // Apply voice-specific processing
                        let naive_waveform = filtered_sample.process(generated_sample);
                        let corrected_waveform =
                            naive_waveform - SubSynth::poly_blep(voice.phase, voice.phase_delta);
                        let generated_sample = corrected_waveform * amp;
            
                        // Apply panning and process the sample
                        let processed_sample = dc_blocker.process(generated_sample);
                        let processed_left_sample = left_amp * processed_sample;
                        let processed_right_sample = right_amp * processed_sample;
            
                        // Add the processed sample to the output channels
                        output[0][sample_idx] += processed_left_sample;
                        output[1][sample_idx] += processed_right_sample;
            
                        // Update voice phase
                        voice.phase += voice.phase_delta;
                        if voice.phase >= 1.0 {
                            voice.phase -= 1.0;
                        }
                    }
                }
            }
            
            // Terminate voices whose release period has fully ended. This could be done as part of
            // the previous loop but this is simpler.
            for voice in &mut self.voices {
                if let Some(v) = voice {
                    if v.releasing && v.amp_envelope.previous_value() == 0.0 {
                        context.send_event(NoteEvent::VoiceTerminated {
                            timing: block_end as u32,
                            voice_id: Some(v.voice_id),
                            channel: v.channel,
                            note: v.note,
                        });
                        *voice = None;
                    }
                }
            }
            
            

            // And then just keep processing blocks until we've run out of buffer to fill
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
        velocity: f32, // Add velocity parameter
        pan: f32, // Add pan parameter
        brightness: f32, // Add brightness parameter
        expression: f32, // Add expression parameter
        vibrato: f32, // Add vibrato parameter
        tuning: f32,
    ) -> &mut Voice {
        let new_voice = Voice {
            voice_id: voice_id.unwrap_or_else(|| compute_fallback_voice_id(note, channel)),
            internal_voice_id: self.next_internal_voice_id,
            channel,
            note,
            velocity,
            velocity_sqrt: velocity.sqrt(),
            pan,
            brightness,
            expression,
            vibrato,
            tuning,
            phase: 0.0,
            phase_delta: 0.0,
            releasing: false,
            amp_envelope: ADSREnvelope::new(
                self.params.amp_attack_ms.value(),
                self.params.amp_decay_ms.value(),
                self.params.amp_sustain_level.value(),
                self.params.amp_release_ms.value(),
                192000.0,
                velocity,
            ),
            voice_gain: None,
            filter_cut_envelope: ADSREnvelope::new(
                self.params.filter_cut_attack_ms.value(),
                self.params.filter_cut_decay_ms.value(),
                self.params.filter_cut_sustain_ms.value(),
                self.params.filter_cut_release_ms.value(),
                192000.0,
                velocity,
            ),
            filter_res_envelope: ADSREnvelope::new(
                self.params.filter_res_attack_ms.value(),
                self.params.filter_res_decay_ms.value(),
                self.params.filter_res_sustain_ms.value(),
                self.params.filter_res_release_ms.value(),
                192000.0,
                velocity,
            ),
            filter: Some(self.params.filter_type.value()),
        };
    
        self.next_internal_voice_id = self.next_internal_voice_id.wrapping_add(1);
    
        if let Some(free_voice_idx) = self.voices.iter().position(|voice| voice.is_none()) {
            let voice = &mut self.voices[free_voice_idx];
            if voice.is_none() {
                *voice = Some(new_voice);
                // voice.as_mut().unwrap().amp_envelope.trigger();
                // voice.as_mut().unwrap().filter_cut_envelope.trigger();
                // voice.as_mut().unwrap().filter_res_envelope.trigger();
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
        _sample_rate: f32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
    ) {
        for voice in &mut self.voices {
            if let Some(voice) = voice {
                if voice_id == Some(voice.voice_id) || (channel == voice.channel && note == voice.note) {
                    voice.amp_envelope.release();
                    voice.filter_cut_envelope.release();
                    voice.filter_res_envelope.release();
                }
            }
        }
    }
    

    fn handle_poly_pressure_event(
        &mut self,
        _timing: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        pressure: f32,
    ) {
        // Find the voice associated with the given voice_id or create a new voice
        let voice = self.find_or_create_voice(voice_id, channel, note, 0.0, 0.0, 0.0, 0.0, 0.0);
        
        // Adjust the voice parameters based on the pressure value
        let velocity = pressure; // Assuming pressure represents velocity here
    
        // Update the voice's velocity and adjust the amplitude envelope
        voice.velocity = velocity;
        voice.amp_envelope.set_velocity(velocity);
    
        // Other actions based on the pressure value if needed
    
        // Optionally trigger the voice to retrigger the envelope or perform other actions
        //voice.amp_envelope.trigger();
        //voice.filter_cut_envelope.trigger();
        //voice.filter_res_envelope.trigger();
    
        // Other operations specific to handling PolyPressure event
    }
    fn find_voice(&mut self, voice_id: Option<i32>, channel: u8, note: u8) -> Option<&mut Voice> {
        self.voices.iter_mut().find(|voice| {
            let voice_id = voice_id.clone(); // Clone the voice_id for comparison inside the closure
            if let Some(voice) = voice {
                voice.voice_id == voice_id.unwrap_or(voice.voice_id) && voice.channel == channel && voice.note == note
            } else {
                false
            }
        }).map(|voice| voice.as_mut().unwrap())
    }
    
    
    
    
    
    fn compute_fallback_voice_id(note: u8, channel: u8, next_voice_id: i32) -> i32 {
        // Fallback voice ID computation...
        // Modify this function to generate a unique voice ID based on note, channel, and next_voice_id.
        // Example implementation:
        (note as i32) + (channel as i32) + next_voice_id
    }

    fn find_or_create_voice(
        &mut self,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        pan: f32,
        brightness: f32,
        expression: f32,
        tuning: f32,
        vibrato: f32,
    ) -> &mut Voice {
        // Search for an existing voice with the given voice_id
        if let Some(existing_index) = self.voices.iter().position(|voice| {
            voice.as_ref().map(|voice_ref| {
                voice_ref.voice_id == voice_id.unwrap_or(voice_ref.voice_id)
                    && voice_ref.channel == channel
                    && voice_ref.note == note
            }).unwrap_or(false)
        }) {
            return self.voices[existing_index].as_mut().unwrap();
        }

        // If no existing voice found, create a new voice
        let new_voice_id = voice_id.unwrap_or_else(|| {
            // Generate a fallback voice ID
            self.next_voice_index += 1;
            Self::compute_fallback_voice_id(note, channel, self.next_voice_index.try_into().unwrap())
        });

        // If no existing voice found, create a new voice
        let new_voice = Voice {
            voice_id: new_voice_id,
            channel,
            note,
            internal_voice_id: self.next_internal_voice_id,
            velocity: 0.0,
            velocity_sqrt: 0.0,
            phase: 0.0,
            phase_delta: 0.0,
            releasing: false,
            amp_envelope: ADSREnvelope::new(
                self.params.amp_attack_ms.value(),
                self.params.amp_decay_ms.value(),
                self.params.amp_sustain_level.value(),
                self.params.amp_release_ms.value(),
                192000.0,
                1.0,
            ),
            voice_gain: None,
            filter_cut_envelope: ADSREnvelope::new(
                self.params.filter_cut_attack_ms.value(),
                self.params.filter_cut_decay_ms.value(),
                self.params.filter_cut_sustain_ms.value(),
                self.params.filter_cut_release_ms.value(),
                192000.0,
                1.0,
            ),
            filter_res_envelope: ADSREnvelope::new(
                self.params.filter_res_attack_ms.value(),
                self.params.filter_res_decay_ms.value(),
                self.params.filter_res_sustain_ms.value(),
                self.params.filter_res_release_ms.value(),
                192000.0,
                1.0,
            ),
            filter: Some(FilterType::Lowpass),
            pan,
            brightness,
            expression,
            tuning,
            vibrato,
        };

        // Find the next available slot for a new voice
        let mut next_voice_index = self.next_voice_index;
        while self.voices[next_voice_index].is_some() {
            next_voice_index = (next_voice_index + 1) % NUM_VOICES;
            if next_voice_index == self.next_voice_index {
                panic!("No available slots for new voices");
            }
        }

        // Store the new voice in the found slot
        self.voices[next_voice_index] = Some(new_voice);

        // Update the next available slot index
        self.next_voice_index = next_voice_index;

        // Return a mutable reference to the newly created voice
        self.voices[next_voice_index].as_mut().unwrap()
    }
    
    
    
    fn handle_poly_volume_event(
        &mut self,
        _timing: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        gain: f32,
    ) {
        let voice = self.find_or_create_voice(voice_id, channel, note, 0.0, 0.0, 0.0, 0.0, 0.0);
        voice.velocity = gain;
        voice.velocity_sqrt = gain.sqrt();
        voice.amp_envelope.set_velocity(gain); // Set velocity for the amp envelope
    }
    
    fn handle_poly_pan_event(
        &mut self,
        _timing: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        pan: f32,
    ) {
        let voice = self.find_or_create_voice(voice_id, channel, note, pan, 0.0, 0.0, 0.0, 0.0);
        voice.pan = pan;
    }
    
    fn handle_poly_tuning_event(
        &mut self,
        _timing: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        tuning: f32,
    ) {
        let voice = self.find_or_create_voice(voice_id, channel, note, 0.0, 0.0, 0.0, tuning, 0.0);
        voice.tuning = tuning;
    }
    
    fn handle_poly_vibrato_event(
        &mut self,
        _timing: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        vibrato: f32,
    ) {
        let voice = self.find_or_create_voice(voice_id, channel, note, 0.0, 0.0, 0.0, 0.0, vibrato);
        voice.vibrato = vibrato;
    }
    
    fn handle_poly_expression_event(
        &mut self,
        _timing: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        expression: f32,
    ) {
        let voice = self.find_or_create_voice(voice_id, channel, note, 0.0, 0.0, expression, 0.0, 0.0);
        voice.expression = expression;
    }
    
    fn handle_poly_brightness_event(
        &mut self,
        _timing: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        brightness: f32,
    ) {
        let voice = self.find_or_create_voice(voice_id, channel, note, 0.0, brightness, 0.0, 0.0, 0.0);
        voice.brightness = brightness;
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
                    *voice = None;

                    if voice_id.is_some() {
                        return;
                    }
                }
                _ => (),
            }
        }
    }
    pub fn clip(input: f32, limit: f32) -> f32 {
        if input > limit {
            limit
        } else if input < -limit {
            -limit
        } else {
            input
        }
    }
    pub fn poly_blep(t: f32, dt: f32) -> f32 {
        if t < dt {
            let t = t / dt;
            // 2 * (t - t^2/2 - 0.5)
            return t+t - t*t - 1.0;
        }
        else if t > 1.0 - dt {
            let t = (t - 1.0) / dt;
            // 2 * (t^2/2 + t + 0.5)
            return t*t + t+t + 1.0;
        }
        0.0
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
        max_voice_capacity: NUM_VOICES as u32,
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
