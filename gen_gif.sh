#!/bin/sh

# See https://blog.pkh.me/p/21-high-quality-gif-with-ffmpeg.html

palette="/tmp/palette.png"
filters="fps=25,scale=360:-1:flags=lanczos"

input="test_scenes/chess_queen_material_animation.mp4"
output="test_scenes/chess_queen_material_animation_palette.gif"

#ffmpeg -v warning -i $input -vf "$filters,palettegen" -y $palette
#ffmpeg -v warning -i $input -i $palette -lavfi "$filters [x]; [x][1:v] paletteuse" -y $output

#ffmpeg -y -f image2 -i "test_scenes/chess_queen_material_animation_frames/chess_queen_material_animation_%04d.png" -vf "$filters,palettegen" $palette
#ffmpeg -y -f image2 -i "test_scenes/chess_queen_material_animation_frames/chess_queen_material_animation_%04d.png" -i $palette -lavfi "$filters [x]; [x][1:v] paletteuse" -y $output

# name="utah_teapot_animated"
# name="chess_queen_material_animation"
name="satisfying-conveyor"
input="test_scenes/${name}_frames/${name}_%04d.png"
output="test_scenes/${name}.gif"

ffmpeg -y -f image2 -i $input -vf "$filters,palettegen" $palette
ffmpeg -y -f image2 -i $input -i $palette -lavfi "$filters [x]; [x][1:v] paletteuse" -y $output
