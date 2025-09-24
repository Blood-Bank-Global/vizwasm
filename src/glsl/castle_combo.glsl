// Drum cc
//!VAR float cc_iac_driver_bus_1_0_0 0.0

// C1 base drum
//!VAR float note_iac_driver_bus_1_0_36 0.0
//!VAR uint note_iac_driver_bus_1_0_36_on 0

// C1# tom low
//!VAR float note_iac_driver_bus_1_0_37 0.0
//!VAR uint note_iac_driver_bus_1_0_37_on 0


// D1 HH
//!VAR float note_iac_driver_bus_1_0_38 0.0
//!VAR uint note_iac_driver_bus_1_0_38_on 0

// D1# HH 3
//!VAR float note_iac_driver_bus_1_0_39 0.0
//!VAR uint note_iac_driver_bus_1_0_39_on 0

// E1 Bd 2
//!VAR float note_iac_driver_bus_1_0_40 0.0
//!VAR uint note_iac_driver_bus_1_0_40_on 0

// F1 Sd
//!VAR float note_iac_driver_bus_1_0_41 0.0
//!VAR uint note_iac_driver_bus_1_0_41_on 0

// F1# Doom Tam
//!VAR float note_iac_driver_bus_1_0_42 0.0
//!VAR uint note_iac_driver_bus_1_0_42_on 0

// G1 flash tam
//!VAR float note_iac_driver_bus_1_0_43 0.0
//!VAR uint note_iac_driver_bus_1_0_43_on 0


// Droplet
//!VAR float cc_iac_driver_bus_1_1_0 0.0

// Chant
//!VAR float cc_iac_driver_bus_1_2_0 0.0

// use noise to transform the image coordinate to make a swirl effect
// noise function not implemented, so using a simple sin/cos function instead
// could be replaced with a better noise function later
vec2 base_coord = src_coord0;
if (cc_iac_driver_bus_1_1_0 > 0.0) {
    float angle = sin(dot(base_coord.xy, vec2(2.9898,3.233)) + iTime * 7.0) * 3.14159;
    float radius = length(base_coord - 0.5) * cc_iac_driver_bus_1_1_0 / 64.0;
    vec2 offset = vec2(cos(angle), sin(angle)) * radius * 0.1;
    base_coord = base_coord + offset;
}

color = vec4(handle_edge(src_tex0, base_coord.xy, EDGE_MODE_MIRROR), 1.0);
if (note_iac_driver_bus_1_0_37_on > 0u) {
    color.r = handle_edge(src_tex0, vec2(base_coord.x + 1.0/640.0 * (note_iac_driver_bus_1_0_37/127.0 * 20), base_coord.y), EDGE_MODE_SMEAR).r;
}

if (note_iac_driver_bus_1_0_40_on > 0u || note_iac_driver_bus_1_0_36_on > 0u) {
    float val = 1.0/640.0 * (max(note_iac_driver_bus_1_0_40, note_iac_driver_bus_1_0_36)/127.0 * 20);
    color.b = handle_edge(src_tex0, vec2(base_coord.x, base_coord.y - val), EDGE_MODE_SMEAR).b;
}

if (note_iac_driver_bus_1_0_42_on > 0u) {
    float tmp = color.g;
    color.g = color.b;
    color.b = tmp;
}

if (note_iac_driver_bus_1_0_43_on > 0u) {
    float tmp = color.b;
    color.b = color.r;
    color.r = tmp;
}

vec4 other = texture(src_tex1, src_coord1.xy);


float intensity = clamp(cc_iac_driver_bus_1_2_0 / 127.0, 0.0, 1.0);
other.a = min(distance(other.rgb, vec3(0,0,0))/2.0, intensity);
other.g = intensity;
other.b = intensity;

color = blend_by_mode(color, other, BLEND_ALPHA);