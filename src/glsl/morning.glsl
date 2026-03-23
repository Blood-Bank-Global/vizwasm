#include "utils.glsl"
#include "patch_pixelate.glsl"

void pass0(out vec4 color) {
    vec4 morning = texture(src_tex0, src_coord.xy);
    vec4 coffee;

    float t = fract(iTime/4.0) * M_PI * 2.0;
    float w1 = sin(t*5.0);
    float w2 = sin(t*2.1);
    float w3 = sin(t*0.2);
    float mean = (w1 + w2 + w3) / 3.0;
    if (mean > 0.4) {
        vec2 uv = src_coord.xy * iResolution.xy;
        coffee = patch_textelate(uv, 0.5, src_tex1, iResolution.xy);
    } else {
        coffee = texture(src_tex1, src_coord.xy);
    }

    color = blend_by_mode(morning, coffee, BLEND_ADDITION);
    vec3 hsv = rgb2hsv(color.rgb);
    if (hsv.z < 170.0/256.0) {
        color.a = 40.0/256.0;
    }
}
