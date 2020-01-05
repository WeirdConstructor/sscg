shader_type spatial;

render_mode diffuse_burley;
//render_mode depth_draw_always;

void fragment() {
	float xv = 0.0;
	float yv = 0.0;

	
	float depth = clamp((FRAGCOORD.z / FRAGCOORD.w) / 64.0, 0.0, 1.0);
	float idepth = 1.0 - depth;
	float bias = 0.2;
	float sdepth =
		clamp(((depth + bias) / (1.0 + bias)), bias / (1.0 + bias), 1.0);

	float width = 0.2 * sdepth;

	if (UV.x <= width)         { xv = 1.0 * (1.0 - UV.x / width); }
	if (UV.y <= width)         { yv = 1.0 * (1.0 - UV.y / width); }
	if (UV.x >= (1.0 - width)) { xv = 1.0 * (1.0 - (1.0 - UV.x) / width); }
	if (UV.y >= (1.0 - width)) { yv = 1.0 * (1.0 - (1.0 - UV.y) / width); }
	float line_val = max(clamp(xv, 0.0, 1.0), clamp(yv, 0.0, 1.0));
	
	vec3 base_color = COLOR.rgb * (1.0 - length(vec2(0.5, 0.5) - UV));
	vec3 emit_base_color = vec3(0, line_val, line_val);
	if (line_val > 1.00001) {
		emit_base_color = vec3(1.0, 0.0, 1.0);
	}
	vec3 emit_color = mix(emit_base_color, base_color, depth);
	ALBEDO = mix(base_color,	emit_color,line_val);
	EMISSION = emit_base_color * 4.0  * (idepth * idepth) * line_val;
}