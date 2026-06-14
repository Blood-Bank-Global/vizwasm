#include "utils.glsl"
#include "patch_blob_px.glsl"
#include "font_8x16.glsl"

#define BPM 150.0

void pass0(out vec4 color) {
    float t = BPM / 60.0;
    float p1 = randf(uint(iTime * t));

    t = BPM / 60.0;
    float beat = iTime * t / 4.0; // 4 beats per measure
    uint b = uint(beat);
    float f = fract(beat);
    vec2 pts[] = vec2[](
        vec2(randf((b - 1u) ^ 0xBABE0000u), randf((b - 1u) ^ 0xCAFE0000u)),
        vec2(randf((b + 0u) ^ 0xBABE0000u), randf((b + 0u) ^ 0xCAFE0000u)),
        vec2(randf((b + 1u) ^ 0xBABE0000u), randf((b + 1u) ^ 0xCAFE0000u)),
        vec2(randf((b + 2u) ^ 0xBABE0000u), randf((b + 2u) ^ 0xCAFE0000u))
    );
    float x = bicubic_mix(pts[0].x, pts[1].x, pts[2].x, pts[3].x, f);
    float y = bicubic_mix(pts[0].y, pts[1].y, pts[2].y, pts[3].y, f);
    vec2 interp = vec2(x, y);

    vec4 c1 = texture(src_tex1, interp + src_uv);
    color = c1;
    

    if (distance(color.rgb, vec3(0.0)) < 0.01) {
        beat = iTime * t * 8; // 64th notes
        b = uint(beat);
        f = fract(beat);

        if (randf(uint(b ^ 0xDEAD0000u)) > 0.0) {
            color = texture(src_tex0, vec2(p1) + src_uv);
        }
    }

    // vec4 cout = patch_blob_px(
    //     (src_uv.xy) * iResolution.xy,
    //     iResolution.xy,
    //     texture(src_tex0, src_uv + vec2(p1)),
    //     texture(src_tex1, src_uv + interp),
    //     vec2(0.5) * iResolution.xy ,
    //     400.0,
    //     iTime
    // );'

}