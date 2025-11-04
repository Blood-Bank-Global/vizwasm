//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0
//!VAR vec2 iResolution2 1.0 1.0
//!VAR vec2 iResolution3 1.0 1.0
//!VAR vec2 iResolution4 1.0 1.0

//!VAR int usr_var 0;

//!VAR float cc_iac_driver_bus_1_0_0 0.0
//!VAR float cc_iac_driver_bus_1_1_0 0.0
//!VAR float cc_iac_driver_bus_1_2_0 0.0
//!VAR float note_iac_driver_bus_1_1_36 0.0
//!VAR float note_iac_driver_bus_1_1_37 0.0
//!VAR float note_iac_driver_bus_1_1_38 0.0
//!VAR float note_iac_driver_bus_1_1_39 0.0
//!VAR float note_iac_driver_bus_1_1_40 0.0
//!VAR float note_iac_driver_bus_1_1_41 0.0
//!VAR float note_iac_driver_bus_1_1_42 0.0
//!VAR float note_iac_driver_bus_1_1_43 0.0

#define TOXIC_BG_TEX src_tex0
#define TOXIC_BG_COORD src_coord0
#define TOXIC_BG_RESOLUTION iResolution0
#define TOXIC_PLANT_TEX src_tex1
#define TOXIC_PLANT_COORD src_coord1
#define TOXIC_PLANT_RESOLUTION iResolution1
#define TOXIC_BOSS_TEX src_tex2
#define TOXIC_BOSS_COORD src_coord2
#define TOXIC_BOSS_RESOLUTION iResolution2
#define TOXIC_CANS_TEX src_tex3
#define TOXIC_CANS_COORD src_coord3
#define TOXIC_CANS_RESOLUTION iResolution3
#define TOXIC_DOOR_TEX src_tex4
#define TOXIC_DOOR_COORD src_coord4
#define TOXIC_DOOR_RESOLUTION iResolution4
#define TOXIC_DUNK_TEX src_tex5
#define TOXIC_DUNK_COORD src_coord5
#define TOXIC_DUNK_RESOLUTION iResolution5
#define TOXIC_MOP_RESOLUTION iResolution6
#define TOXIC_MOP_TEX src_tex6
#define TOXIC_MOP_COORD src_coord6
#define TOXIC_MOP_RESOLUTION iResolution6
#define TOXIC_STATIC_TEX src_tex7
#define TOXIC_STATIC_COORD src_coord7
#define TOXIC_STATIC_RESOLUTION iResolution7

vec4 bg_color = texture(TOXIC_BG_TEX, TOXIC_BG_COORD);
vec4 plant_color = texture(TOXIC_PLANT_TEX, TOXIC_PLANT_COORD);
vec4 matte = plant_color;
if (cc_iac_driver_bus_1_2_0 > 0.0) {
    bg_color.a = (cc_iac_driver_bus_1_2_0 + 10.0) / 127.0;
    matte = blend_by_mode(plant_color, bg_color, BLEND_ALPHA);
}

vec4 beat_color;
vec2 beat_coord = TOXIC_BOSS_COORD; // doesn't matter, all same size
if (cc_iac_driver_bus_1_1_0 > 0.0) {
    float angle = sin(dot(beat_coord.xy, vec2(2.9898,3.233)) + iTime * 7.0) * 3.14159;
    float radius = length(beat_coord - 0.5) * cc_iac_driver_bus_1_1_0 / 64.0;
    vec2 offset = vec2(cos(angle), sin(angle)) * radius * 0.1;
    beat_coord += offset;
}
switch (usr_var) {
    case 0: {
        beat_color = texture(TOXIC_BOSS_TEX, beat_coord);
        break;
    }
    case 1: {
        beat_color = texture(TOXIC_DUNK_TEX, beat_coord);
        break;
    }
    case 2: {
        beat_color = texture(TOXIC_MOP_TEX, beat_coord);
        break;
    }
    case 3: {
        beat_color = texture(TOXIC_CANS_TEX, beat_coord);
        break;
    }
    case 4: {
        beat_color = texture(TOXIC_DOOR_TEX, beat_coord);
        break;
    }
    case 5: {
        beat_color = vec4(0.0);
        break;
    }
    default: {
        beat_color = vec4(0.5, 0.0, 0.0, 1.0);
        break;
    }
}

float beat_luma = dot(beat_color.rgb, vec3(0.299, 0.587, 0.114));
beat_color.a  = clamp(beat_luma * 4.0, 0.0, 1.0);

color = blend_by_mode(matte, beat_color, BLEND_ALPHA);
