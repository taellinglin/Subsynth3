# SubSynth

[![Automated builds](https://github.com/robbert-vdh/nih-plug/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/robbert-vdh/nih-plug/actions/workflows/build.yml?query=branch%3Amaster)
[![Tests](https://github.com/robbert-vdh/nih-plug/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/robbert-vdh/nih-plug/actions/workflows/test.yml?query=branch%3Amaster)
[![Docs](https://github.com/robbert-vdh/nih-plug/actions/workflows/docs.yml/badge.svg?branch=master)](https://nih-plug.robbertvanderhelm.nl/)

# Overview

SubSynth is a subtractive synthesizer implemented as a VST3/CLAP plugin. It leverages ADSR envelopes and different waveform types to produce a variety of sounds. It's perfect for electronic music and sound design, providing a range of expressive control parameters for detailed sound sculpting.
Building

SubSynth is written in Rust and built with Cargo. Before building, make sure you have the latest Rust compiler and Cargo package manager installed. You can install them from the official Rust website.

Once you've set up Rust and Cargo, clone the SubSynth repository and navigate to its directory:

```bash
git clone https://github.com/taellinglin/Subsynth3.git
cd Subsynth3
```
You can then build SubSynth using:
```bash
cargo xtask bundle subsynth --release
```
This will create a release build of the synthesizer.


## Parameters

SubSynth provides a variety of parameters for you to shape the sound output:

- **Gain**: Controls the overall volume of the synthesizer.
  
- **Attack**: Sets the time it takes for a note to reach its peak level after being triggered.
  
- **Release**: Determines the time it takes for a note to decay to silence after being released.
  
- **Waveform**: Lets you select the type of waveform (sine, square, sawtooth, and triangle) used for sound generation.
  
- **Decay**: Defines the time it takes for the sound to transition from the peak level to the sustain level.
  
- **Sustain**: Defines the level of the sound during the main part of its duration.
  
- **Filter Type**: Sets the type of filter (none, low-pass, high-pass, band-pass) applied to the audio signal.
  
- **Filter Cutoff**: Defines the frequency at which the filter begins to take effect.
  
- **Filter Resonance**: Amplifies frequencies near the filter cutoff point.
  
- **Filter Cut Attack/Decay/Sustain/Release**: These parameters control the envelope of the filter cutoff. They determine how quickly the filter opens and closes, allowing you to shape the tonal character of the sound.
  
- **Filter Res Attack/Decay/Sustain/Release**: These parameters control the envelope of the filter resonance. They allow you to dynamically control the resonant peak of the filter over the duration of the note.
