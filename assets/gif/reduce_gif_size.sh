#!/usr/bin/env sh

rm -f demo_00.gif && ffmpeg -i demo_00.mp4 -vf "fps=24,scale=480:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" -y temp.gif && gifsicle -i temp.gif --optimize=3 --resize-width 480 -o demo_00.gif && rm temp.gif
