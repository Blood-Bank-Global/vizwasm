vec2 sun_center = vec2(0.5, 0.3) * iResolution.xy;
vec2 pt = src_coord0 * iResolution.xy;

if (distance(pt, sun_center) < 130.0) {
    color = vec4(1.0, 1.0, 1.0, 1.0);
} else {
    color = vec4(0.0, 0.0, 0.0, 1.0);
}

if ((sin(2 * fract(src_coord0.y + mod(frame,360.0)/360.0) * 10 * M_PI * 2.0) + 1) / 2.0 * (src_coord0.y+0.7) >= 0.75) {
    color = vec4(0.0, 0.0, 0.0, 1.0);
}