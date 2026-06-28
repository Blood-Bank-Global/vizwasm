#include "utils.glsl"
#include "patch_warp_px.glsl"
#include "patch_halftone.glsl"
//!VAR vec2 iResolution1 1.0 1.0
//!VAR vec2 iResolution2 1.0 1.0
//!VAR float user2 0.0
#define frogIt(offset, sz) (mat4x2( \
        (offset), \
        (offset) + vec2((sz).x, 0.0), \
        (offset) + vec2(0.0, (sz).y), \
        (offset) + (sz)) \
)

#define BPM 150
void pass0(out vec4 color) {

    float beat = BPM / 60.0;
    float b = floor(iTime * beat);
    color = texture(src_tex0, src_uv);

    if (distance(color.rgb, vec3(1.0, 0.0, 1.0)) > 0.7) {
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

        vec2 frog_bump = vec2(0.0, 35.0/iResolution1.y);



        for (int i = 0; i < frogs.length(); i++) {
            mat4x2 frog = frogs[i];
            vec2 warped_uv = src_uv;
            float scale = 1.0;

            if (pointInRhombus(warped_uv * iResolution.xy, frog)) {

                vec2 center_uv = vec2((frogs[i][1].x - frogs[i][0].x) / 2.0 + frogs[i][0].x,
                                    (frogs[i][3].y - frogs[i][0].y) / 2.0 + frogs[i][0].y)
                                / iResolution.xy;
                vec2 delta = vec2(0.5) - center_uv;
                vec2 uv = ((warped_uv.xy + delta) / vec2(scale));


                vec4 c1 = texture(src_tex1, uv - frog_bump);

                vec3 hsv = rgb2hsv(c1.rgb);
                float p1 = mod(iTime, 1.0/(beat/4.0)) / (1.0/(beat/4.0));
                hsv[2] = mod(hsv[2] + p1, 1.0);
                float p2 = mod(iTime, 1.0/beat) / (1.0/beat);
                // hsv[0] = mod(hsv[0] + step(0.5, p2) * 0.5, 1.0);
                c1.rgb = hsv2rgb(hsv);
                vec4 c2;
                patch_wave_dither(
                        c2,
                        src_tex1,
                        uv - frog_bump,
                        iResolution.xy,
                        iTime * 100.0,
                        true,
                        640.0,
                        0.025,
                        0.0,
                        0.7,
                        false,
                        0x7u
                );
                color = blend_by_mode(c1, c2, BLEND_DIFFERENCE);
                return;
            }

        }
    }

    mask = texture(src_tex4, src_uv);
    if (distance(mask.rgb, vec3(1.0, 0.0, 1.0)) < 0.5) {
        if (true /* randf(uint(b) ^ 0x4205) < 0.0 */) {
            patch_ink(color, src_tex2, src_uv, iResolution.xy * 16.0, vec2(32.0), user2, 22.5, 0xCAFE);
            color = vec4(vec3(0.5, 1.0, 0.0) - vec3(color.g * 0.5, color.g, 0.0), 1.0);
            if (color.g < 0.5) {
                color.a = 0.0;
            }
        } else {
            color = texture(src_tex2, src_uv);
        }
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
        // if (choice1 == 0) {
        //     color.rgb = vec3(180.0, 255.0, 0.0) / vec3(255.0);
        // } else if (choice2 == 0) {
        //     color.rgb = vec3(100.0, 200.0, 10.0) / vec3(255.0);
        // } else {
        //     color.rgb = vec3(10.0, 155.0, 30.0) / vec3(255.0);
        // }

        if (choice1 == 0) {
            color.rgb = vec3(180.0, 200.0, 100.0) / vec3(255.0);
        } else if (choice2 == 0) {
            color.rgb = vec3(100.0, 50.0, 50.0) / vec3(255.0);
        } else {
            color.rgb = vec3(30.0, 30.0, 30.0) / vec3(255.0);
        }

    }

}