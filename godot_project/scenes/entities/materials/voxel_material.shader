shader_type spatial;

render_mode diffuse_burley;
//render_mode unshaded;

void fragment() {
	float xv = 0.0;
	float yv = 0.0;
	
	// Depth fades out until z > 64
	float depth = clamp((FRAGCOORD.z / FRAGCOORD.w) / 64.0, 0.0, 1.0);
	float idepth = 1.0 - depth;
	float bias = 0.1;
	// Some biased and clamped depth value, so we don't get 0.0 value when
	// we are just in front of a voxel.
	float depth_biased =
		clamp(((depth + bias) / (1.0 + bias)), bias / (1.0 + bias), 1.0);

	vec2 width =
	   max(fwidth(UV),
	       vec2((0.2 * depth_biased) / UV2.x,
		        (0.2 * depth_biased) / UV2.y));

	// Calculating the fragment of the wire line from the UV:
	if (UV.x <= width.x)         { xv = 1.0 * (1.0 - UV.x / width.x); }
	if (UV.y <= width.y)         { yv = 1.0 * (1.0 - UV.y / width.y); }
	if (UV.x >= (1.0 - width.x)) { xv = 1.0 * (1.0 - (1.0 - UV.x) / width.x); }
	if (UV.y >= (1.0 - width.y)) { yv = 1.0 * (1.0 - (1.0 - UV.y) / width.y); }
	float line_val = max(clamp(xv, 0.0, 1.0), clamp(yv, 0.0, 1.0));
	
	// Base color of the voxel with some darkness gradiant so it doesn't look so flat:
	vec3 base_color = COLOR.rgb * clamp(1.0 - pow(length(vec2(0.5, 0.5) - UV), 2), 0.0, 1.0);
	
	// Color of the line fragment
	vec3 emit_base_color = vec3(0, line_val, line_val);
	if (line_val > 1.00001) {
		emit_base_color = vec3(1.0, 0.0, 1.0);
	}
	// Fade out the line color to the base color with more z distance
	vec3 emit_color = mix(emit_base_color, base_color, depth);
	
	ALBEDO = mix(base_color, emit_color, line_val);
	// 4.0 to make the glow more glaring:
	EMISSION = emit_base_color * 2.0 * (idepth * idepth * idepth) * line_val;
}