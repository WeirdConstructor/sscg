ffmpeg \
  -i opengl-rotating-triangle.mp4 \
  -r 15 \
  -vf scale=512:-1 \
  -ss 00:00:03 -to 00:00:06 \
  opengl-rotating-triangle.gif
