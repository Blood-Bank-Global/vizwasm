//!VAR int usr_var 0

float scale = 300.0;
float usr_p = (scale - mod(iTime, scale))/scale;
vec2 sun_center = vec2(0.5, (0.2 + 0.6 * usr_p)) * iResolution.xy;
vec2 pt = src_coord0 * iResolution.xy;

float radius = 100.0 + usr_p * 300.0;
if (distance(pt, sun_center) < radius) {
    color = vec4(1.0, 1.0, 1.0, 1.0);
} else {
    color = vec4(0.0, 0.0, 0.0, 1.0);
}

if (sun_center.y >= pt.y && (sin(2 * fract(src_coord0.y + mod(frame,720.0)/720.0) * ((400.0 - radius)/radius * 5 + 5) * M_PI * 2.0) + 1) / 2.0 * (pt.y/(sun_center.y - 0.2 * iResolution.y)) > 0.8) {
    color = vec4(0.0, 0.0, 0.0, 1.0);
}

if (src_coord0.y > 0.6) {
    color = vec4(0.0, 0.0, 0.0, 1.0);
}