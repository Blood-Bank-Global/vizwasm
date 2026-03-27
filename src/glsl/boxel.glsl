#include "patch_pixelate.glsl"
//!VAR vec3 iResolution0 0.0 0.0 0.0
void pass0(out vec4 color) {
    color = texture(src_tex0, src_uv0);
    color.rgb = floor(color.rgb*4)/4.0;
}
void pass1(out vec4 color) {
    color = patch_boxelate(pass_uv0.xy * iResolution.xy, 1, pass_tex0, iResolution.xy);
}