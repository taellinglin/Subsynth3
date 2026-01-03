# SubSynth

[![Automated builds](https://github.com/taellinglin/Subsynth3/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/taellinglin/Subsynth3/actions/workflows/build.yml?query=branch%3Amaster)
[![Tests](https://github.com/taellinglin/Subsynth3/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/taellinglin/Subsynth3/actions/workflows/test.yml?query=branch%3Amaster)
[![Documentation](https://github.com/taellinglin/Subsynth3/actions/workflows/docs.yml/badge.svg?branch=master)](https://github.com/taellinglin/Subsynth3/actions/workflows/docs.yml?query=branch%3Amaster)

SubSynth is a powerful subtractive synthesizer implemented as a VST3/CLAP plugin, built with [NIH-plug](https://github.com/robbert-vdh/nih-plug). It leverages ADSR envelopes, multiple waveform types, and comprehensive filter controls to produce a wide variety of sounds perfect for electronic music and sound design.

![SubSynth Interface](https://github.com/taellinglin/Subsynth3/assets/82527149/9138dc3e-4969-473d-948a-b780191ff09b)

🎵 [Listen to demo track](https://soundcloud.com/taellinglin/8kwealj94t22)

## Features

- **Multiple Waveform Types**: Sine, square, sawtooth, and triangle waves
- **Full ADSR Envelope Control**: Attack, Decay, Sustain, Release for amplitude
- **Comprehensive Filter Section**: 
  - Filter types: None, Low-pass, High-pass, Band-pass
  - Independent ADSR envelopes for cutoff and resonance
- **VST3 and CLAP Support**: Works in all major DAWs
- **Cross-platform**: Windows, macOS, and Linux support

## Building from Source

SubSynth is written in Rust and uses Cargo for building. You'll need:
- [Rust](https://rustup.rs/) (latest stable or nightly)
- Platform-specific dependencies (see below)

### Platform Dependencies

**Linux (Ubuntu/Debian)**:
```bash
sudo apt-get install -y libasound2-dev libgl-dev libjack-dev \
  libx11-xcb-dev libxcb1-dev libxcb-dri2-0-dev libxcb-icccm4-dev \
  libxcursor-dev libxkbcommon-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

**macOS**: Xcode Command Line Tools
```bash
xcode-select --install
```

**Windows**: No additional dependencies required

### Building the Plugin

```bash
# Clone the repository
git clone https://github.com/taellinglin/Subsynth3.git
cd Subsynth3

# Build release version
cargo xtask bundle subsynth --release
```

The compiled plugins will be in `target/bundled/`:
- `SubSynth.vst3` (VST3 plugin)
- `SubSynth.clap` (CLAP plugin)

## Installation

### Windows
Copy the `.vst3` directory to:
```
C:\Program Files\Common Files\VST3\
```

Copy the `.clap` file to:
```
C:\Program Files\Common Files\CLAP\
```

### macOS
Copy the `.vst3` bundle to:
```
~/Library/Audio/Plug-Ins/VST3
```

Copy the `.clap` bundle to:
```
~/Library/Audio/Plug-Ins/CLAP
```

**Note**: You may need to disable Gatekeeper for these plugins. See [disable-gatekeeper.github.io](https://disable-gatekeeper.github.io/) for instructions.

### Linux
Copy the `.vst3` directory to:
```
~/.vst3
```

Copy the `.clap` file to:
```
~/.clap
```

## Parameters

### Amplitude Envelope
- **Gain**: Master output volume control
- **Attack**: Time for note to reach peak level after trigger
- **Decay**: Time to transition from peak to sustain level
- **Sustain**: Level maintained during note hold
- **Release**: Time to decay to silence after note release

### Oscillator
- **Waveform**: Select from sine, square, sawtooth, or triangle waves

### Filter Section
- **Filter Type**: Choose between none, low-pass, high-pass, or band-pass
- **Filter Cutoff**: Frequency where filter takes effect
- **Filter Resonance**: Emphasis of frequencies near cutoff point

### Filter Cutoff Envelope
- **Attack**: How quickly the filter opens
- **Decay**: Time from peak to sustain cutoff
- **Sustain**: Sustained cutoff frequency level
- **Release**: How quickly the filter closes after release

### Filter Resonance Envelope
- **Attack**: Speed of resonance increase
- **Decay**: Time from peak to sustain resonance
- **Sustain**: Sustained resonance level
- **Release**: Speed of resonance decrease

## Development

### Running Tests
```bash
cargo test --workspace --features "simd,standalone,zstd"
```

### Building Documentation
```bash
cargo doc --features docs,simd,standalone,zstd --no-deps --open
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

See [LICENSE](LICENSE) file for details.

## Credits

Built with [NIH-plug](https://github.com/robbert-vdh/nih-plug) by Robbert van der Helm.


