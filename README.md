# SubSynth

[![Automated builds](https://github.com/robbert-vdh/nih-plug/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/robbert-vdh/nih-plug/actions/workflows/build.yml?query=branch%3Amaster)
[![Tests](https://github.com/robbert-vdh/nih-plug/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/robbert-vdh/nih-plug/actions/workflows/test.yml?query=branch%3Amaster)
[![Docs](https://github.com/robbert-vdh/nih-plug/actions/workflows/docs.yml/badge.svg?branch=master)](https://nih-plug.robbertvanderhelm.nl/)


SubSynth是一种实现为VST3/CLAP插件的减法合成器。它利用ADSR包络和不同的波形类型产生各种声音。它非常适合电子音乐和声音设计，提供一系列的表现控制参数进行详细的声音塑造。
![image](https://github.com/taellinglin/Subsynth3/assets/82527149/3992ff6e-7c09-4e86-8658-395a7688cafb)
<iframe width="100%" height="300" scrolling="no" frameborder="no" allow="autoplay" src="https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1561371871&color=%23ff5500&auto_play=false&hide_related=false&show_comments=true&show_user=true&show_reposts=false&show_teaser=true&visual=true"></iframe><div style="font-size: 10px; color: #cccccc;line-break: anywhere;word-break: normal;overflow: hidden;white-space: nowrap;text-overflow: ellipsis; font-family: Interstate,Lucida Grande,Lucida Sans Unicode,Lucida Sans,Garuda,Verdana,Tahoma,sans-serif;font-weight: 100;"><a href="https://soundcloud.com/taellinglin" title="灵林【LingLin】" target="_blank" style="color: #cccccc; text-decoration: none;">灵林【LingLin】</a> · <a href="https://soundcloud.com/taellinglin/8kwealj94t22" title="子合成器演示" target="_blank" style="color: #cccccc; text-decoration: none;">子合成器演示</a></div>
构建

SubSynth使用Rust编写并使用Cargo构建。在构建之前，请确保您已安装了最新的Rust编译器和Cargo包管理器。您可以从官方的Rust网站上安装它们。

一旦您安装了Rust和Cargo，就可以克隆SubSynth仓库并导航到其目录：
```bash
git clone https://github.com/taellinglin/Subsynth3.git
cd Subsynth3
```
然后，您可以使用以下命令构建SubSynth：
```bash
cargo xtask bundle subsynth --release
```
这将会创建一个合成器的发布构建。
## 参数

SubSynth为您提供各种参数以形塑声音输出：

- **增益**：控制合成器的整体音量。
  
- **攻击**：设置触发后一个音符达到峰值水平所需的时间。
  
- **释放**：确定音符在被释放后衰减为无声的时间。
  
- **波形**：让您选择用于声音生成的波形类型（正弦，方波，锯齿波和三角波）。
  
- **衰减**：定义声音从峰值水平转变到持续水平的时间。
  
- **持续**：定义声音在其主要部分的持续时间内的水平。
  
- **滤波器类型**：设置应用于音频信号的滤波器类型（无，低通，高通，带通）。
  
- **滤波器截止频率**：定义滤波器开始起作用的频率。
  
- **滤波器谐振**：放大滤波器截止点附近的频率。
  
- **滤波器截止攻击/衰减/持续/释放**：这些参数控制滤波器截止的包络。他们确定滤波器打开和关闭的速度，使您能够塑造声音的音调特性。
  
- **滤波器谐振攻击/衰减/持续/释放**：这些参数控制滤波器谐振的包络。他们使您可以在音符的持续时间内动态地控制滤波器的共振峰。
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


