//!VAR vec3 iResolution0 0.0 0.0 0.0
#include "utils.glsl"
void pass0(out vec4 color) {
    vec2 uv = scale_uv(src_uv, iResolution0.xy, iResolution.xy) * 0.7 + vec2(0.15);

    if (uv.y < 0.0 || uv.y > 1.0) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        color = texture(src_tex0, uv);
    }

    if (beat4x4(0xAAAA, 120, iTime)) {
        color.r += 0.3;
    }

    if (beat4x4(0x8888, 120, iTime)) {
        color = blend_by_mode(color, texture(src_tex1, src_uv), BLEND_ADDITION);
    }
}