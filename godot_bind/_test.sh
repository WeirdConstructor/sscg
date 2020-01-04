if cargo build --release; then
    rm ../godot_project/gdnative/libsscg_gd.so
    cp -v target/release/libsscg_gd.so ../godot_project/gdnative/
    cd ../godot_project/
    ../Godot_v3.1.1-stable_x11.64 project.godot
    cd ../godot_bind/
fi
