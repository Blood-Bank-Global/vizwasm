//!VAR int usr_var 0

color = vec4(0.0, 0.0, 0.0, 1.0);

vec2 coord = src_coord0 * iResolution.xy;
vec2 center = vec2(0.5, 0.5) * iResolution.xy;

vec2 to_coord = coord - center;
float angle = atan(to_coord.y, to_coord.x);
float pos = abs(M_PI - angle);
float t1 = mod(frame + pos * 40.0, 40.0) / 40.0;
float t2 = mod(frame + pos * 20.0 + 5, 20.0) / 20.0;
float t3 = mod(frame + pos * 60.0 + 50, 60.0) / 60.0;
float thickness = 100.0 + 20.0 * sin(t1 * 2.0 * M_PI) + 20.0 * cos(t2 * 2.0 * M_PI) * sin(t3 * 2.0 * M_PI);
thickness *= (float(abs(usr_var)) / 20.0) + 1.0;

if (distance(coord, center) < thickness) {
    float weight = clamp(distance(coord, center) / thickness, 0.0, 1.0);
    color = vec4(weight, weight, weight, 1.0); // Red
}