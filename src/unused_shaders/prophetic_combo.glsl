
//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0
//!VAR vec2 iResolution2 1.0 1.0
//!VAR vec2 iResolution3 1.0 1.0

//!VAR int usr_var 0;

//!VAR float cc_iac_driver_bus_1_0_0 0.0
//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0

#define ZOL_TEX src_tex0
#define ZOL_RES iResolution0
#define ZOL_COORDS src_coord0
#define CARD_TEX src_tex1
#define CARD_RES iResolution1
#define CARD_COORDS src_coord1
#define MAKE_TEX src_tex2
#define MAKE_RES iResolution2
#define MAKE_COORDS src_coord2

#define CC_CLAP cc_iac_driver_bus_1_0_1
#define CC_CRASH cc_iac_driver_bus_1_0_2


color = texture(ZOL_TEX, ZOL_COORDS);

vec4 make_color = texture(MAKE_TEX, MAKE_COORDS);
if (distance(make_color.rgb, vec3(0,0,0)) > 10 * EPSILON && CC_CRASH > 5.0) {
    make_color.a = dot(make_color.rgb, vec3(0.299, 0.587, 0.114));
} else {
    make_color.a = 0.0;
}

color = blend_by_mode(color, make_color, BLEND_ALPHA);

vec4 card_color = texture(CARD_TEX, CARD_COORDS);
if (distance(card_color.rgb, vec3(0,0,0)) > 10 * EPSILON && CC_CLAP > 5.0) {
    card_color.a = dot(card_color.rgb, vec3(0.299, 0.587, 0.114));
} else {
    card_color.a = 0.0;
}
color = blend_by_mode(color, card_color, BLEND_ALPHA);