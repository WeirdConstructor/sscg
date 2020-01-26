cd godot_bind
@cargo build --release
@copy target\release\sscg_gd.dll ..\godot_project\gdnative\
cd ..\godot_project\
start ..\godot\Godot_v3.1.1-stable_win64.exe
cd ..
