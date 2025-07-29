# `last-watched`

Easily keep track of what episodes you've watched from ~~pirated TV shows~~ your collection of ripped DVDs.
This system is implemented as a shell extension on Windows and a [Dolphin](https://apps.kde.org/dolphin) plugin on Linux.
It shows overlays on watched video file icons and uses scripts loaded by MPV to mark a video as watched.

![GrF7AYYsFf](https://github.com/user-attachments/assets/85217e19-cfd2-4af8-b2c6-b5e3b9859394)

## Installation

Depending on what media player you use the plugin installation will differ, all instructions can be found [here](plugins).
Currently only MPV is supported.

### Linux (Dolphin)

Download `lastwatchedplugin.so` from the latest release and copy it to `/usr/lib64/qt6/plugins/dolphin/vcs/`.
To show the check icon in file icons, the version control system plugin interface is used.
To compile from source look into [kde-builder](https://kde-builder.kde.org/en/).

### Windows

Download and extract the zip from the latest release and put it in a permanent spot because once the shell extension is registered, it will check that directory for the needed dll, exe, and ico files.
Now to register the extension, open an administrator command prompt in that folder and run `regsvr32 last_watched.dll`, if you ever want to remove the extension in the future, instead run `regsvr32 /u last_watched.dll`.
To get to see the changed take effect, try restarting Windows Explorer with Task Manager or just restart your system.

## How it Works

When you play a video file, the plugin for your video player will add the video's file name to a hidden `.watched` file in the same directory, creating it if it doesn't exist.
Then, when the shell / explorer comes across a video file (mp4, mkv, m4v, avi, webm, flv, mov, wmv) the last-watched plugin will check for the watched sidecar file and add the icon if needed.
