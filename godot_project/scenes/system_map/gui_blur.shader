shader_type canvas_item;


void fragment() {
	vec4 screen_blur = texture(SCREEN_TEXTURE, SCREEN_UV, 2.0);
	vec4 ui          = texture(TEXTURE, UV);
	if (ui.a > 0.1) {
		COLOR = mix(ui, screen_blur, 0.3);
	} else {
		COLOR = ui;
	}
}