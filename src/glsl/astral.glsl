#include "utils.glsl"
#include "patch_warp_px.glsl"

void pass0(out vec4 color) {
    mat4x2 corners = mat4x2(
        vec2(-iResolution.x, 0.0),
        vec2(2 * iResolution.x, 0.0),
        vec2(0.0, iResolution.y/3),
        vec2(iResolution.x, iResolution.y/3)
    ) / mat4x2(iResolution.xy,iResolution.xy,iResolution.xy,iResolution.xy);


    if (src_uv.y <= 0.125) {
        corners = mat4x2(
            vec2(-2, 0.0),
            vec2(2, 0.0),
            vec2(0.0, 0.125),
            vec2(1.0, 0.125)
        );
        vec2 uv2 = skew3(src_uv, corners);
        uv2 += vec2(0.0, fract(iTime/2.0));
        uv2 = patch_warp_px(uv2 * iResolution.xy, vec2(10.0, 10.0), 0.2, iResolution.xy, iTime*4.0) / iResolution.xy;
        color = texture(src_tex0, uv2);
    } else {
        color = vec4(0.0, 0.0, 0.0, 0.0);
    }
}