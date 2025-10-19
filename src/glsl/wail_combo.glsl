//!VAR float cc_iac_driver_bus_1_0_3 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0

#define WAIL_SCENE src_tex0
#define WAIL_TEXT src_tex1

color = texture(WAIL_SCENE, src_coord0);
vec4 text_color = texture(WAIL_TEXT, src_coord1);

if (distance(text_color.rgb, vec3(0.0)) > 10.0 * EPSILON) {
    // set text_color rgb to be a linear hue from the velocity in cc_iac_driver_bus_1_0_2
    float luma = dot(text_color.rgb, vec3(0.299, 0.587, 0.114));
    text_color.rgb = hsv2rgb(vec3(clamp(cc_iac_driver_bus_1_0_2 / 127.0, 0.0, 1.0), 1.0, luma));
    text_color.a = clamp(cc_iac_driver_bus_1_0_3 / 127.0, 0.0, 1.0);
    color = blend_by_mode(color, text_color, BLEND_ALPHA);
}
