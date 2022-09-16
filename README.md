A simple CLI tool to play audio clips on Axis network speakers using their API ("VAPIX").  Whipped up as a quick demo that audio playback customization for Axis network speakers (or cameras) is not only possible but very simple.  Axis playback options through the interface are very limited.

Run to initialize settings.json in local folder.  You will be prompted for speaker settings.  Then you can select the speaker to play a clip at a specified volume.  You must know the file name of the clip (for example, logo.mp3).

Use of HTTP and digest authentication is hardcoded.  Passwords are not displayed or saved safely.  Hide them on input and hash them using Argon2 if used beyond testing.