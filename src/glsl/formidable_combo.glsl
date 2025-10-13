
//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0
//!VAR vec2 iResolution2 1.0 1.0

//!VAR int usr_var 0;

//!VAR float cc_iac_driver_bus_1_0_0 0.0
//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0

#define SCENE_TEX src_tex0
#define SCENE_RES iResolution0
#define SCENE_COORDS src_coord0
#define TOP_TEX src_tex1
#define TOP_RES iResolution1
#define TOP_COORDS src_coord1
#define BOTTOM_TEX src_tex2
#define BOTTOM_RES iResolution2
#define BOTTOM_COORDS src_coord2


#define CC_ZERO cc_iac_driver_bus_1_0_0
#define CC_ONE cc_iac_driver_bus_1_0_1
#define CC_TWO cc_iac_driver_bus_1_0_2


color = texture(SCENE_TEX, SCENE_COORDS);

vec4 bottom_color = texture(BOTTOM_TEX, BOTTOM_COORDS);
if (distance(bottom_color.rgb, vec3(0,0,0)) > 10 * EPSILON && CC_ONE > 5.0) {
    bottom_color.a = CC_ONE / 127.0;
} else {
    bottom_color.a = 0.0;
}
color = blend_by_mode(color, bottom_color, BLEND_ALPHA);

vec4 top_color = texture(TOP_TEX, TOP_COORDS + vec2(0.0, 0.15));
if (distance(top_color.rgb, vec3(0,0,0)) > 10 * EPSILON && CC_TWO > 5.0) {
    top_color.a = CC_TWO / 127.0;
} else {
    top_color.a = 0.0;
}
color = blend_by_mode(color, top_color, BLEND_ALPHA);