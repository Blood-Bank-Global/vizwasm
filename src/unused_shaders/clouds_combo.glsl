//UPPER DRAGON ALPHA
//!VAR float cc_iac_driver_bus_1_0_1 0.0
//UPPER DRAGON HUE
//!VAR float cc_iac_driver_bus_1_0_2 0.0
//LOWER DRAGON ALPHA
//!VAR float cc_iac_driver_bus_1_0_3 0.0
//LOWER DRAGON HUE
//!VAR float cc_iac_driver_bus_1_0_4 0.0

//TAP LOW
//!VAR float cc_iac_driver_bus_1_0_5 0.0
//TAP HIGH
//!VAR float cc_iac_driver_bus_1_0_6 0.0
//BELL
//!VAR float cc_iac_driver_bus_1_0_7 0.0

vec2 base_coord = src_coord0;
if (cc_iac_driver_bus_1_0_7 > 5.0) {
    float angle = sin(dot(base_coord.xy, vec2(2.9898,3.233)) + iTime * 7.0) * 3.14159;
    float radius = length(base_coord - 0.5) * cc_iac_driver_bus_1_0_7 / 16.0;
    vec2 offset = vec2(cos(angle), sin(angle)) * radius * 0.1;
    base_coord = base_coord + offset;
}

if (cc_iac_driver_bus_1_0_5 > 5.0 || cc_iac_driver_bus_1_0_6 > 5.0) {
    base_coord.x += 0.025 * (0.0 - step(0.5, fract(base_coord.y / 0.025)));
}

color = vec4(handle_edge(src_tex0, base_coord, EDGE_MODE_MIRROR), 1.0);

vec4 upper = texture(src_tex1, src_coord1);
vec3 upper_hsv = rgb2hsv(upper.rgb);
upper.rgb = hsv2rgb(vec3(mod(upper_hsv.x + cc_iac_driver_bus_1_0_2/127.0, 1.0), 0.5, upper_hsv.z));
upper.a = min(upper_hsv.z, cc_iac_driver_bus_1_0_1/127.0);

color = blend_by_mode(color, upper, BLEND_ALPHA); 

vec4 lower = texture(src_tex2, src_coord2);
vec3 lower_hsv = rgb2hsv(lower.rgb);
lower.rgb = hsv2rgb(vec3(mod(lower_hsv.x + cc_iac_driver_bus_1_0_4/127.0, 1.0), 0.5, lower_hsv.z));
lower.a = min(lower_hsv.z, cc_iac_driver_bus_1_0_3/127.0);

color = blend_by_mode(color, lower, BLEND_ALPHA);

