//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0
//!VAR vec2 iResolution2 1.0 1.0
//!VAR vec2 iResolution3 1.0 1.0
//!VAR vec2 iResolution4 1.0 1.0

//!VAR float cc_iac_driver_bus_1_0_0 0.0
//!VAR float note_iac_driver_bus_1_1_36 0.0
//!VAR float note_iac_driver_bus_1_1_37 0.0
//!VAR float note_iac_driver_bus_1_1_38 0.0
//!VAR float note_iac_driver_bus_1_1_39 0.0
//!VAR float note_iac_driver_bus_1_1_40 0.0
//!VAR float note_iac_driver_bus_1_1_41 0.0
//!VAR float note_iac_driver_bus_1_1_42 0.0
//!VAR float note_iac_driver_bus_1_1_43 0.0

#define DICT_TEX src_tex0
#define DICT_COORD src_coord0
#define DICT_RESOLUTION iResolution0

#define SWIRL_TEX src_tex1
#define SWIRL_COORD src_coord1
#define SWIRL_RESOLUTION iResolution1

#define FLOWERS_TEX src_tex2
#define FLOWERS_COORD src_coord2
#define FLOWERS_RESOLUTION iResolution22

#define SCENES_TEX src_tex3
#define SCENES_COORD src_coord3
#define SCENES_RESOLUTION iResolution3


vec2 offset_r = vec2(note_iac_driver_bus_1_1_36/127 * 8, 0.0) + vec2(note_iac_driver_bus_1_1_37/127 * 8, 0.0) + vec2(note_iac_driver_bus_1_1_38/127 * 8, 0.0);
vec2 offset_b = vec2(note_iac_driver_bus_1_1_39/127 * 8, 0.0) + vec2(note_iac_driver_bus_1_1_40/127 * 8, 0.0) + vec2(note_iac_driver_bus_1_1_41/127 * 8, 0.0);
vec2 offset_g = vec2(0.0, note_iac_driver_bus_1_1_42/127 * 8) + vec2(0.0, note_iac_driver_bus_1_1_43/127 * 8);

vec4 scene_color = vec4(0.0, 0.0, 0.0, 1.0);
scene_color.r = texture(SCENES_TEX, SCENES_COORD + vec2(5.0/SCENES_RESOLUTION.x, 0.0) + offset_r).r;
scene_color.g = texture(SCENES_TEX, SCENES_COORD + vec2(5.0/SCENES_RESOLUTION.x, 0.0) + offset_g).g;
scene_color.b = texture(SCENES_TEX, SCENES_COORD + vec2(5.0/SCENES_RESOLUTION.x, 0.0) + offset_b).b;

color = scene_color;

vec4 swirl_color = texture(SWIRL_TEX, SWIRL_COORD);

if (distance(swirl_color.xyz, vec3(0.0)) > EPSILON) {
    color = swirl_color;
}

vec4 dict_color = texture(DICT_TEX, DICT_COORD);
// let hue_shift = (note_iac_driver_bus_1_1_36 - 64.0) / 127.0 * 2.0;
// dict_color.rgb = clamp(dict_color.rgb + hue_shift, 0.0, 1.0);
if (distance(dict_color.xyz, vec3(0.0)) > EPSILON) {
    dict_color.a = cc_iac_driver_bus_1_0_0 / 127.0;
    color = blend_by_mode(color, dict_color, BLEND_ALPHA);
}