#!/usr/bin/env sh

printf '\n\nPress Q to stop recording\n\n'
ffmpeg -video_size 640x480 -framerate 24 -f x11grab -i :0.0+0,0 demo_00.mp4
