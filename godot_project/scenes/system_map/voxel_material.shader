shader_type spatial;

render_mode diffuse_burley;

void fragment() {
	float xv = 0.0;
	float yv = 0.0;
	
	// Depth fades out until z > 64
	float depth = clamp((FRAGCOORD.z / FRAGCOORD.w) / 64.0, 0.0, 1.0);
	float idepth = 1.0 - depth;
	float bias = 0.2;
	// Some biased and clamped depth value, so we don't get 0.0 value when
	// we are just in front of a voxel.
	float depth_biased =
		clamp(((depth + bias) / (1.0 + bias)), bias / (1.0 + bias), 1.0);

	float width = 0.2 * depth_biased;

	// Calculating the fragment of the wire line from the UV:
	if (UV.x <= width)         { xv = 1.0 * (1.0 - UV.x / width); }
	if (UV.y <= width)         { yv = 1.0 * (1.0 - UV.y / width); }
	if (UV.x >= (1.0 - width)) { xv = 1.0 * (1.0 - (1.0 - UV.x) / width); }
	if (UV.y >= (1.0 - width)) { yv = 1.0 * (1.0 - (1.0 - UV.y) / width); }
	float line_val = max(clamp(xv, 0.0, 1.0), clamp(yv, 0.0, 1.0));
	
	// Base color of the voxel with some darkness gradiant so it doesn't look so flat:
	vec3 base_color = COLOR.rgb * (1.0 - length(vec2(0.5, 0.5) - UV));
	
	// Color of the line fragment
	vec3 emit_base_color = vec3(0, line_val, line_val);
	if (line_val > 1.00001) {
		emit_base_color = vec3(1.0, 0.0, 1.0);
	}
	// Fade out the line color to the base color with more z distance
	vec3 emit_color = mix(emit_base_color, base_color, depth);
	
	ALBEDO = mix(base_color, emit_color,line_val);
	// 4.0 to make the glow more glaring:
	EMISSION = emit_base_color * 4.0 * (idepth * idepth) * line_val;
}