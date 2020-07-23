cd godot_bind
if cargo build --release; then
    rm ../godot_project/gdnative/libsscg_gd.so
    cp -v target/release/libsscg_gd.so ../godot_project/gdnative/
    cd ../godot_project/
    ../Godot_v3.2.2-stable_x11.64 project.godot
fi
cd ..
