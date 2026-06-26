#include "utils.glsl"
#include "patch_warp_px.glsl"
//!VAR vec2 iResolution1 1.0 1.0
//!VAR vec2 iResolution2 1.0 1.0

#define frogIt(offset, sz) (mat4x2( \
        (offset), \
        (offset) + vec2((sz).x, 0.0), \
        (offset) + vec2(0.0, (sz).y), \
        (offset) + (sz)) \
)

#define BPM 150
void pass0(out vec4 color) {

    color = texture(src_tex0, src_uv);

    if (distance(color.rgb, vec3(1.0, 0.0, 1.0)) > 0.10) {
        return;
    }


    vec4 mask = texture(src_tex3, src_uv);
    if (distance(mask.rgb, vec3(1.0, 0.0, 1.0)) < 0.20) {
    

        vec2 sz = vec2(135.0,192.0);
        vec2 offset = vec2(0.0, 35.0);
        mat4x2 frogs[] = mat4x2[](
            frogIt(offset, sz),
            frogIt(vec2(iResolution.x - sz.x - offset.x, offset.y), sz),
            frogIt(vec2(offset.x, iResolution.y - sz.y - offset.y), sz),
            frogIt(vec2(iResolution.x - sz.x - offset.x, iResolution.y - sz.y - offset.y), sz)
        );

        vec2 frog_bump = vec2(0.0, 30.0/iResolution1.y);


        float beat = BPM / 60.0 * 4.0;
        float b = floor(iTime * beat);
        float t = mod(iTime, beat)/beat;

        for (int i = 0; i < frogs.length(); i++) {
            mat4x2 frog = frogs[i];
            vec2 warped_uv = src_uv;
            float scale = 1.0;

            if (pointInRhombus(warped_uv * iResolution.xy, frog)) {
                
                uint bucket = uint(3.0 * abs(randf(uint(b) ^ 0x4205 ^ (1337 * i))));
                if (bucket == 0) {
                    warped_uv = patch_warp_px(
                        warped_uv * iResolution.xy,
                        vec2(10.0),
                        0.5,
                        iResolution1.xy /vec2(scale),
                        iTime * 10.0
                    ) / iResolution.xy;

                }

                vec2 center_uv = vec2((frogs[i][1].x - frogs[i][0].x) / 2.0 + frogs[i][0].x,
                                    (frogs[i][3].y - frogs[i][0].y) / 2.0 + frogs[i][0].y)
                                / iResolution.xy;
                vec2 delta = vec2(0.5) - center_uv;
                vec2 uv = ((warped_uv.xy + delta) / vec2(scale));
                color = texture(src_tex1, uv - frog_bump);

                if (bucket == 1) {
                    vec4 c = texture(src_tex5, src_uv*.8);
                    color.rgb = mix(color.rgb, c.rgb, 0.5);
                }

                return;
            }

        }
    }

    mask = texture(src_tex4, src_uv);
    if (distance(mask.rgb, vec3(1.0, 0.0, 1.0)) < 0.20) {
        color = texture(src_tex2, src_uv);
        return;
    }

    bool marble = true;
    if (marble) {
        vec2 warped = patch_warp_px(
            (src_uv + vec2(0.0,fract(iTime/30.0))) * iResolution.xy ,
            vec2(50.0),
            2.75,
            iResolution.xy,
            iTime/10.0
        );
        warped = patch_warp_px(
            warped,
            vec2(30.0),
            1.5,
            iResolution.xy,
            iTime + float(0xBABE)
        );


        int choice1 = int(step(5.0, mod(warped.x, 50.0)));
        int choice2 = int(step(20.0, mod(warped.y, 50.0)));
        if (choice1 == 0) {
            color.rgb = vec3(180.0, 255.0, 0.0) / vec3(255.0);
        } else if (choice2 == 0) {
            color.rgb = vec3(100.0, 200.0, 10.0) / vec3(255.0);
        } else {
            color.rgb = vec3(10.0, 155.0, 30.0) / vec3(255.0);
        }

    }

}