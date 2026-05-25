#include "utils.glsl"
#include "patch_warp_px.glsl"

#define COLOR1 vec3(1.0, 0.0, 1.0)
#define COLOR2 vec3(0.0, 1.0, 1.0)

void pass0(out vec4 color) {
    vec2 uv = src_uv - vec2(0.5, 0.5);
    uv *= 0.97;
    uv -= vec2(0.5, 0.5);
    color = texture(pass_tex2, uv);

    if (fract(iTime*20.0) < 0.5) {
        vec4 flip = texture(pass_tex1, uv);
        if (distance(flip.rgb, vec3(0.0)) >= 0.1) {
            if (distance(flip.rgb, COLOR1) < 0.1) {
                color = vec4(COLOR2, 1.0);
            } else {
                color = vec4(COLOR1, 1.0);
            }
        }
    }
    color.a = 1.0;
}

void pass1(out vec4 color) {

    vec2 warped = patch_warp_px(src_uv * iResolution.xy, vec2(50.0), 1.0, iResolution.xy, iTime*2.0) / iResolution.xy;

    if (distance(warped * iResolution.xy, vec2(0.5, 0.5) * iResolution.xy) < 50.0) {
        // color = vec4(0.0, 0.0, 1.0, 1.0);
        color = vec4(COLOR1, 1.0);
    } else {
        color = vec4(0.0, 0.0, 0.0, 0.0);   
    }
}

void pass2(out vec4 color) {
    vec4 under = texture(pass_tex0, src_uv);
    vec4 over = texture(pass_tex1, src_uv);
    color = blend_by_mode(under, over, BLEND_ALPHA);
}