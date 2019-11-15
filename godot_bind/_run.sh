if cargo build; then
#;    mv ../godot_project/gdnative/libsscg_gd.so ../godot_project/gdnative/libsscg_gd.so.0
    rm ../godot_project/gdnative/libsscg_gd.so
    cp -v target/debug/libsscg_gd.so ../godot_project/gdnative/
    cd ../godot_project/
    ../godot.x11.tools.64
    cd ../godot_bind/
fi
