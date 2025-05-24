//!VAR uint overlay_kind 0
//!VAR uint scanline_kind 0

// scanlines

if (scanline_kind != 0) {
    vec4 line = vec4(0.0, 0.0, 0.0, 1.0);
    float intensity = fract(src_coord0.y * 100000);
    line.rgb = vec3(intensity * 0.1, intensity * 0.1, intensity  * 0.1);

    color = blend_by_mode(
        texture(src_tex0, src_coord0),
        line,
        scanline_kind
    );

} else {
    color = texture(src_tex0, src_coord0);
}

// overlay grit
color = blend_by_mode(
    color,
    texture(src_tex1, src_coord1),
    overlay_kind
);