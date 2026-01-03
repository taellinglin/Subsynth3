SubSynth - Installation Instructions for macOS
===============================================

Thank you for downloading SubSynth!

IMPORTANT - Gatekeeper Notice:
-------------------------------
macOS may block unsigned plugins. You have two options:

1. Disable Gatekeeper (recommended for music production):
   Visit: https://disable-gatekeeper.github.io/

2. Allow the plugin manually:
   - Right-click the plugin bundle and select "Open"
   - Click "Open" in the security dialog
   - Or go to System Preferences > Security & Privacy and click "Open Anyway"

VST3 Plugin Installation:
--------------------------
1. Copy the SubSynth.vst3 BUNDLE to:
   ~/Library/Audio/Plug-Ins/VST3

   Full path: /Users/YOUR_USERNAME/Library/Audio/Plug-Ins/VST3

2. If the directory doesn't exist, create it:
   mkdir -p ~/Library/Audio/Plug-Ins/VST3

3. Restart your DAW.

CLAP Plugin Installation:
--------------------------
1. Copy the SubSynth.clap BUNDLE to:
   ~/Library/Audio/Plug-Ins/CLAP

2. If the directory doesn't exist, create it:
   mkdir -p ~/Library/Audio/Plug-Ins/CLAP

3. Restart your DAW.

4. For CLAP support in your DAW, visit:
   https://github.com/free-audio/clap#hosts

Troubleshooting:
----------------
- The ~/Library folder is hidden by default. To access it:
  Hold Option/Alt and click "Go" in Finder menu
- Make sure the entire .vst3 or .clap bundle is copied, not just its contents
- Some DAWs require a plugin rescan after installation

For more information and updates:
https://github.com/taellinglin/Subsynth3

