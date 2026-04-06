#include "utils.glsl"
#include "patch_warp_px.glsl"

//!VAR float cc_iac_driver_bus_1_1_0 0.0
//!VAR float cc_iac_driver_bus_1_1_1 0.0
//!VAR float cc_iac_driver_bus_1_1_2 0.0

void pass0(out vec4 color) {

    vec2 src_coord = src_uv.xy * iResolution.xy;
    src_coord = patch_warp_px(
        src_coord,
        vec2(128.0), 
        cc_iac_driver_bus_1_1_1 / 127.0,
        iResolution.xy,
        iTime
    );

    vec2 warped_uv = src_coord / iResolution.xy;
    if (cc_iac_driver_bus_1_1_0 > 0.5) {
        color = texture(src_tex1, warped_uv);

        if (distance(color.rgb, vec3(1.0)) < 0.9) {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        } else {
            color = vec4(0.0);
        }

    } else {

        color = texture(src_tex0, warped_uv);

        if (distance(color.rgb, vec3(0.0)) < 0.5) {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        } else {
            color.a = 0.3;
        }

        vec3 hsv = rgb2hsv(color.rgb);
        hsv.z = round(hsv.z * 4.0) / 4.0;

        color.rgb = hsv2rgb(hsv);
    }

}
