#include "utils.glsl"
#include "patch_halftone.glsl"
#define NEIGHBORHOOD_SIZE 8.0
void pass0(out vec4 color) {
    color = vec4(vec3(rgb2hsv(texture(src_tex0, src_uv).rgb).z), 1.0);
}

void pass1(out vec4 color) {
    float p1 = abs(randf(uint(floor(iTime*20.0))));
    float p2 = abs(randf(uint(floor(iTime*10.0))^0xdeadbeef));
    float p3 = abs(randf(uint(floor(iTime*5.0))^0xabcdef01));
    if (p3 < 0.05) {
            patch_ordered_dither8x8(color, src_uv, iResolution.xy, pass_tex0);
            patch_halftone45(color, src_uv, iResolution.xy, pass_tex0, 15.0, 27.5);
            // color.gb = vec2(0.0);
            color.a = 1.0;
    } else if (p1 < 0.75) {
        vec2 uv2 = src_uv;
        if (p2 < 0.1) {
            uv2*=2.0;
        }
        color = texture(src_tex0, coord_mirror(uv2, true, true));
    } else {
        color = texture(src_tex1, (src_uv + vec2(fract(iTime))) * 0.2);
        if (distance(color.rgb, vec3(0.0)) < 0.1) {
            color = vec4(0.0);
        } else {
            color.a = 0.5;
        }
    }
    // color = texture(src_tex0, src_uv);


}