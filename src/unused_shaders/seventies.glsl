#include "utils.glsl"
#include "font_8x16.glsl"

//!VAR float cc_iac_driver_bus_1_1_1 0.0

void pass0(out vec4 color) {
    color = texture(src_tex0, src_uv);

    if (cc_iac_driver_bus_1_1_1 > 10.0) {
        color.rgb = vec3(1.0) - color.rgb;
    }    

    vec2 title_uv = src_uv.xy + vec2(0.025 * sin(fract(iTime + src_uv.y*5.0) * 2.0 * M_PI), sin(fract(iTime*2.0) * 2.0 * M_PI)*0.01);
    vec4 title_color = texture(src_tex1, title_uv);
    if (rgb2hsv(title_color.rgb).z > 0.9) {
        color = title_color;
        color.a = 1.0;
    }

}