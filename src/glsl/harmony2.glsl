//!VAR int usr_var 0

color = vec4(0.0, 0.0, 0.0, 1.0);

vec2 coord = src_coord0 * iResolution.xy;
vec2 center = vec2(0.5, 0.5) * iResolution.xy;

vec2 to_coord = coord - center;

//rot 180 degrees
to_coord = to_coord * mat2x2(
    -1.0, 0.0,
    0.0, -1.0
);

if (to_coord.x < 0.0) {
    to_coord.x = -to_coord.x;
}

float angle = atan(to_coord.y, to_coord.x);

float t1 = abs(0.5 - mod(frame + angle * 100.0, 100.0) / 100.0);
float thickness = 100.0 + 100.0 * t1;
thickness *= clamp(float(usr_var) / 20.0, -0.9, 100.0) + 1.0;

if (distance(coord, center) < thickness) {
    float weight = clamp(distance(coord, center) / thickness, 0.0, 1.0);
    color = vec4(weight, weight, weight, 1.0); // Red
}