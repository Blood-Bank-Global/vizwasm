#define SCENE_TEX src_tex0
#define SCENE_COORD src_coord0
#define TITLE_TEX src_tex1
#define TITLE_COORD src_coord1

//!VAR float cc_iac_driver_bus_1_0_0 0.0
//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0
#define CC_ZERO cc_iac_driver_bus_1_0_0
#define CC_ONE cc_iac_driver_bus_1_0_1
#define CC_TWO cc_iac_driver_bus_1_0_2

color = texture(SCENE_TEX, SCENE_COORD);

// Overlay the title texture with alpha blending
vec4 title_color = texture(TITLE_TEX, TITLE_COORD);
if (distance(title_color.rgb, vec3(0.0)) > 5 * EPSILON && CC_ONE > 10) {
    float luma = dot(title_color.rgb, vec3(0.3333));
    title_color.a = luma * CC_ONE/127.0;
    title_color.rgb = hsv2rgb(vec3(CC_TWO / 127.0, 1.0, luma)) ;
    color = blend_by_mode(color, title_color, BLEND_ALPHA);
}

