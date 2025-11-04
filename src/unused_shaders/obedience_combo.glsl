
//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0
//!VAR vec2 iResolution2 1.0 1.0

//!VAR int usr_var 0;

//!VAR float cc_iac_driver_bus_1_0_0 0.0
//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0

#define SCHOOL_TEX src_tex0
#define SCHOOL_RES iResolution0
#define SCHOOL_COORDS src_coord0
#define DARK_TEX src_tex1
#define DARK_RES iResolution1
#define DARK_COORDS src_coord1


#define CC_ZERO cc_iac_driver_bus_1_0_0
#define CC_ONE cc_iac_driver_bus_1_0_1
#define CC_TWO cc_iac_driver_bus_1_0_2


color = texture(SCHOOL_TEX, SCHOOL_COORDS);

vec4 top_color = texture(DARK_TEX, DARK_COORDS + vec2(0.0, 0.0));
if (distance(top_color.rgb, vec3(0,0,0)) > 10 * EPSILON && CC_ONE > 5.0) {
    top_color.a = CC_ONE / 127.0;
} else {
    top_color.a = 0.0;
}
color = blend_by_mode(color, top_color, BLEND_ALPHA);