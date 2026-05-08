#include "utils.glsl"

//!VAR float cc_iac_driver_bus_1_1_0 64.0
//!VAR float cc_iac_driver_bus_1_1_1 0.0

#define DIST_SZ vec2(100.0, 100.0)

void pass0(out vec4 color) {
    color = vec4(0.0);

    mat4x2 rect = mat4x2(vec2(0.4, 0.4), vec2(0.6, 0.4), vec2(0.4, 0.6), vec2(0.6, 0.6));
    mat4x2 ctl_pts[] = mat4x2[](
        mat4x2(
            vec2(0.0, 0.0),
            rect[0],
            vec2(0.0, 1.0),
            rect[2]
        ),
        mat4x2(
            rect[1],
            vec2(1.0, 0.0),
            rect[3],
            vec2(1.0, 1.0)
        )
    );


    mat4x2 floor = mat4x2(
        vec2(0.4, 0.6),
        vec2(0.6, 0.6),
        vec2(0.0, 1.0),
        vec2(1.0, 1.0)
    );

    if (pointInRhombus(src_uv, rect)) {
      color = vec4(0.0, 0.0, 0.0, 1.0);
    } if (pointInRhombus(src_uv, floor)) {
        vec2 scrolled = vec2(0.0, fract(iTime/80.0));
        vec2 skewed = skew3(src_uv, floor);
        vec2 mirrored = coord_mirror(fract(skewed * .125 - scrolled), true, true);
        color = texture(src_tex0, mirrored);
    } else {
        float scroll_dir[] = float[](1.0, -1.0);
        for (int i = 0; i < ctl_pts.length(); i++) {
            if (pointInRhombus(src_uv, ctl_pts[i])) {
                vec2 scrolled = vec2(fract(iTime/10.0), 0.0) * scroll_dir[i];
                vec2 skewed = skew3(src_uv, ctl_pts[i]);
                vec2 mirrored = coord_mirror(fract(skewed + scrolled), true, true);
                vec3 hsv = rgb2hsv(texture(src_tex1, mirrored * 0.25).rgb);
                if (hsv.z > 0.5) {
                    color = vec4(vec3(cc_iac_driver_bus_1_1_0/127.0),1.0);
                } else {
                    color = vec4(0.0, 0.0, 0.0, cc_iac_driver_bus_1_1_1/127.0);
                }
            }
        }
    }


}