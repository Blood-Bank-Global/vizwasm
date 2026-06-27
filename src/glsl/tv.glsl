#include "utils.glsl"
#include "patch_halftone.glsl"
//!VAR float user0 0.0

void pass0(out vec4 color) {
    float scale = mix(1.0, 0.1, user0);
    color = texture(src_tex0, (src_uv - vec2(0.5)) * scale + vec2(0.5));
    if (distance(color.rgb, vec3(0.0)) < 0.5) {
        color = vec4(0.0, 0.0, 0.0, 0.0);
    } else {

        color.a = 1.0;
    }

    vec3 hsv = rgb2hsv(color.rgb);

    if (hsv[2] > 0.6) {
        hsv[0] = mod(hsv[0] + fract(iTime), 1.0);
    }

    if (hsv[2] > 0.9 && hsv[1] < 0.1) {
        hsv[1] = 1.0;
    }

    color.rgb = hsv2rgb(hsv);




}

#define BPM 40
#define MAX 6.0
void pass1(out vec4 color) {
    float t = BPM / 60.0 * 4.0;
    float beat = iTime * t;
    uint b = uint(beat);
    float p = mod(beat, t)/t;

    uint select = uint(MAX * abs(randf(b)));
    vec4 c1, c2, c3;
    vec2 offset, pos;
    switch (select) {
        case 0:
            color = texture(pass_tex0, src_uv);
            break;
        case 1:
            patch_ordered_dither4x4(
                color,
                src_uv,
                iResolution.xy,
                pass_tex0
            );
            break;
        case 2:
            patch_halftone45(
                color, 
                src_uv, 
                iResolution.xy,
                pass_tex0, 
                16.0/iResolution.x, 
                25.0);
            c2 = texture(pass_tex0, src_uv);
            color.rgb = color.rgb * c2.rgb;
            break;
        case 3:
            c1 = texture(pass_tex0, src_uv);
            offset = vec2(randf(b ^ 0xBABE), randf(b ^ 0xCAFE));
            pos = coord_wrap(src_uv.xy + offset, true, true);
            c2 = texture(src_tex1, pos);
            color = blend_by_mode(c1, c2, BLEND_AND);
            color.a = 1.0;
            break;
        case 4:
            c1 = texture(pass_tex0, src_uv);
            offset = vec2(sin(fract(iTime) * M_PI * 2.0), 0.0);
            pos = coord_wrap(src_uv.xy * 0.5 + offset, true, true);
            c2 = texture(src_tex3, pos);
            color = blend_by_mode(c1, c2, BLEND_AND);
            color.a = 1.0;
            break;
        case 5:
            patch_wave_dither(
                color,
                pass_tex0,
                src_uv,
                iResolution.xy,
                iTime/4.0,
                true,
                500.0,
                0.1,
                0.1,
                0.25,
                false,
                0x7
            );
            color.a = 1.0;
            c1 = texture(pass_tex0, src_uv);
            color.rgb *= c1.rgb;
            break;
    }
}