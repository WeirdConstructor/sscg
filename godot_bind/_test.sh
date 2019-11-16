if cargo build; then
#;    mv ../godot_project/gdnative/libsscg_gd.so ../godot_project/gdnative/libsscg_gd.so.0
    rm ../godot_project/gdnative/libsscg_gd.so
    cp -v target/debug/libsscg_gd.so ../godot_project/gdnative/
    cd ../godot_project/
#;    ../godot.x11.tools.64 project.godot
    ../Godot_v3.1.1-stable_x11.64 project.godot
    cd ../godot_bind/
fi
