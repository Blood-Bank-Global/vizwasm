// F1 claps
//!VAR float cc_iac_driver_bus_1_2_0 0.0

// C1# snare
//!VAR float cc_iac_driver_bus_1_2_1 0.0

vec4 bottom = vec4(handle_edge(src_tex1, src_coord1.xy, EDGE_MODE_BLANK), 1.0);

vec2 base_coord = src_coord0;
float offset = cc_iac_driver_bus_1_2_0 / 127.0 * (20.0 + 5.0 * sin(base_coord.y * M_PI * 12.0 +iTime*500.0))/ iResolution.x; ;

if (fract(base_coord.y * iResolution.y / 6.0) < 0.5) {
    offset = -offset;
}

base_coord.x += offset;

vec4 top = vec4(handle_edge(src_tex0, base_coord.xy, EDGE_MODE_BLANK), 1.0);

if (
    step(185, base_coord.x * iResolution.x) > 0.0
    && step(iResolution.x - 185, base_coord.x * iResolution.x) < 1.0
    && distance(top.rgb, vec3(0.933, 0.855, 0.533)) > 0.2
) {
    color = top;
} else {
    color = bottom;
}
