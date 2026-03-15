#include "patch_pixelate.glsl"
#include "utils.glsl"

//!VAR vec3 iResolution0 1.0 1.0 1.0

void pass0(out vec4 color) {
    vec2 uv = src_coord.xy * iResolution.xy;
    float resize = iResolution0.y / iResolution.y;
    float offx = (iResolution0.x / resize - iResolution.x)/2.0;
    uv = uv * resize + vec2(offx, 0.0);
    color = patch_textelate(uv, 2.25, src_tex0, iResolution0.xy);
}

