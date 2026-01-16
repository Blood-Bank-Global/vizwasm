color = texture(src_tex0, src_coord0);

vec2 uv = src_coord0.xy * iResolution.xy;
vec2 center = iResolution.xy * vec2(0.5,0.5);
if (distance(uv, center) < 50.0) {
    color.rgb = vec3(1.0, 1.0, 1.0);
}

