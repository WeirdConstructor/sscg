if cargo build --release; then
#;    mv ../godot_project/gdnative/libsscg_gd.so ../godot_project/gdnative/libsscg_gd.so.0
    rm ../godot_project/gdnative/libsscg_gd.so
    cp -v target/release/libsscg_gd.so ../godot_project/gdnative/
    cd ../godot_project/
    #../godot.x11.tools.64
    ../Godot_v3.1.1-stable_x11.64
    cd ../godot_bind/
fi
