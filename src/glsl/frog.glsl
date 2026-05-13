#include "utils.glsl"
#include "patch_halftone.glsl"
//!VAR vec2 iResolution0 0.0 0.0
#define NEIGHBORHOOD_SIZE 8.0
void pass0(out vec4 color) {
    float scale = iResolution.y / iResolution0.y;
    float x_offset = (iResolution.x - iResolution0.x * scale) * 0.5;
    vec2 coord0 = src_uv.xy * iResolution.xy - vec2(x_offset, 0.0);
    vec2 uv0 = coord0 / (scale * iResolution0.xy);
    color = vec4(vec3(rgb2hsv(texture(src_tex0, uv0).rgb).z), 1.0);
}

void pass1(out vec4 color) {
    float scale = iResolution.y / iResolution0.y;
    float x_offset = (iResolution.x - iResolution0.x * scale) * 0.5;
    vec2 coord0 = src_uv.xy * iResolution.xy - vec2(x_offset, 0.0);
    vec2 uv0 = coord0 / (scale * iResolution0.xy);
    
    float p = abs(randf(uint(floor(iTime*10.0))^0xdeadbeef));
    if (p < 0.1) {
            patch_ordered_dither8x8(color, src_uv, iResolution.xy, pass_tex0);
            patch_halftone45(color, src_uv, iResolution.xy, pass_tex0, 15.0, 27.5);
            // color.gb = vec2(0.0);
            color.a = 1.0;
    } else if (p < 0.75) {
        vec2 uv2 = uv0;
        if (p < 0.2) {
            uv2*=2.0;
        }
        color = texture(src_tex0, coord_mirror(uv2, true, true));
    } else {
        color = texture(src_tex1, (src_uv + vec2(0.0, fract(iTime/2.0))) * 0.5);
        if (distance(color.rgb, vec3(0.0)) < 0.1) {
            color = vec4(0.0);
        } else {
            color.a = 0.25;
        }
    }
    // color = texture(src_tex0, src_uv);


}