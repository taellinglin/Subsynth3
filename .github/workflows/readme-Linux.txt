SubSynth - Installation Instructions for Linux
===============================================

Thank you for downloading SubSynth!

VST3 Plugin Installation:
--------------------------
1. Copy the SubSynth.vst3 DIRECTORY to:
   ~/.vst3

   Full path: /home/YOUR_USERNAME/.vst3

2. If the directory doesn't exist, create it:
   mkdir -p ~/.vst3

3. Copy command:
   cp -r SubSynth.vst3 ~/.vst3/

4. Restart your DAW.

CLAP Plugin Installation:
--------------------------
1. Copy the SubSynth.clap FILE to:
   ~/.clap

   Full path: /home/YOUR_USERNAME/.clap

2. If the directory doesn't exist, create it:
   mkdir -p ~/.clap

3. Copy command:
   cp SubSynth.clap ~/.clap/

4. Restart your DAW.

5. For CLAP support in your DAW, visit:
   https://github.com/free-audio/clap#hosts

Alternative Installation Paths:
--------------------------------
Some Linux distributions and DAWs may look in:
- VST3: /usr/local/lib/vst3/ or /usr/lib/vst3/
- CLAP: /usr/local/lib/clap/ or /usr/lib/clap/

Check your DAW's documentation for preferred plugin paths.

Troubleshooting:
----------------
- Ensure the plugin files have execute permissions:
  chmod +x ~/.vst3/SubSynth.vst3/Contents/x86_64-linux/SubSynth.so
  chmod +x ~/.clap/SubSynth.clap

- If using Flatpak/Snap DAWs, you may need to grant file system access:
  flatpak override --user --filesystem=~/.vst3 YOUR_DAW_ID
  flatpak override --user --filesystem=~/.clap YOUR_DAW_ID

- Some DAWs require a plugin rescan after installation

For more information and updates:
https://github.com/taellinglin/Subsynth3

