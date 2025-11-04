
//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0

//!VAR int usr_var 0;

//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0

#define CARD_TEX src_tex0
#define CARD_RES iResolution0
#define CARD_COORDS src_coord0

#define FG_TEX src_tex1
#define FG_RES iResolution1
#define FG_COORDS src_coord1

#define CC_CLAP cc_iac_driver_bus_1_0_1
#define CC_CRASH cc_iac_driver_bus_1_0_2


color = texture(CARD_TEX, CARD_COORDS);

vec4 scene_color = texture(FG_TEX, FG_COORDS);
if (CC_CLAP > 5.0 && dot(scene_color.rgb, vec3(0.299, 0.587, 0.114)) > 2 * EPSILON) {
    scene_color.a = CC_CLAP/127.0;
} else {
    scene_color.a = 0.0;
}

color = blend_by_mode(color, scene_color, BLEND_ALPHA);
