color = texture(src_tex1, src_coord1);
if (distance(color.rgb, vec3(0.0)) < 0.9) {
    color = blend_by_mode(texture(src_tex0, src_coord0), color, BLEND_SCREEN);
} else if (distance(color.rgb, vec3(0.0)) < 0.01) {
    color = texture(src_tex0, src_coord0);
}
