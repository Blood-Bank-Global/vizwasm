#include "utils.glsl"
#include "patch_warp_px.glsl"
#include "patch_pixelate.glsl"
//!VAR float cc_iac_driver_bus_1_0_0 0.0

void pass0(out vec4 color) {
    vec2 d = src_uv - vec2(0.5);
    vec2 uv = src_uv + vec2(10.0 * d.x, 10.0 * d.y) / iResolution.xy;
    vec2 warped = patch_warp_px(uv * iResolution.xy, vec2(30.0), 1.1 - abs(d.x), iResolution.xy, iTime*2.0) / iResolution.xy;

    color = texture(pass_tex2, warped);
    color.a = 1.0;
}

void pass1(out vec4 color) {

    float r = 50.0;

    vec2 warped = patch_warp_px(src_uv * iResolution.xy, vec2(r), 4.0, iResolution.xy, iTime*2.0) / iResolution.xy;
    vec2 w_coord = warped * iResolution.xy;
    mat4x2 corners = mat4x2(
        vec2(0.0, 0.0),
        vec2(iResolution.x, 0.0),
        vec2(0.0, iResolution.y),
        vec2(iResolution.x, iResolution.y)
    );

    bool v[] = bool[](
        distance(w_coord.x, corners[0].x) < r,
        distance(w_coord.x, corners[1].x) < r,
        distance(w_coord.x, corners[2].x) < r,
        distance(w_coord.x, corners[3].x) < r
    );

    if (v[0] || v[1] || v[2] || v[3]) {
        color = vec4(
            hsv2rgb(
                vec3(
                    sin(fract(iTime) * 2.0 * M_PI) * 0.1,
                    1.0,
                    (sin(fract(iTime/2.0) * 2.0 * M_PI) + 1.0) / 2.0 * 0.7)
            ),
            1.0);
    } else {
        color = vec4(0.0, 0.0, 0.0, 0.0);   
    }
}

void pass2(out vec4 color) {
    float scale = clamp(distance(src_uv, vec2(0.5)) * 4.0, 0.1, 1.0);
    vec4 under = texture(pass_tex0, src_uv  / scale);
    vec4 over = texture(pass_tex1, src_uv);
    color = blend_by_mode(under, over, BLEND_ALPHA);
}

#define BPM 84.08
void pass3(out vec4 color) {
    color  = texture(pass_tex2, src_uv);
    vec2 coord = src_uv * iResolution.xy;

    float bars_per_sec = BPM / 60.0 * 4.0;
    int i_max = 4;
    for (int i = 0; i < i_max; i++) {
        vec2 scale = vec2((8.0 - 8.0 * fract(iTime / bars_per_sec + bars_per_sec/i_max * float(i))));
        if (scale.x <= EPSILON || scale.y <= EPSILON) continue;
        vec2 res = iResolution.xy / scale;
        vec2 offset = (iResolution.xy - res)/2.0;
        mat4x2 center = mat4x2(
            offset,
            offset + vec2(res.x, 0.0),
            offset + vec2(0.0, res.y),
            offset + res
        );
        if (!pointInRhombus(coord, center)) continue;
        if (distance(coord.y, center[3].y) <= 1.0) continue;

        vec2 uv = (src_uv - vec2(0.5, 0.5)) * scale - vec2(0.5, 0.5);
        vec4 over = texture(src_tex0, uv);
        if (distance(over.rgb, vec3(0.0)) <= 0.01) {
            over.a = 0.0;
        }
        color = blend_by_mode(color, over, BLEND_ALPHA);
    }

    //knight
    if (cc_iac_driver_bus_1_0_0 >= 10.0) {
        uint seed = uint(iTime / (BPM / 60.0 / 16.0));
        vec2 uv = coord_smear(src_uv*1.5 - vec2(0.25, 0.25));
        vec2 tex_coord = uv * iResolution.xy * 1.5;
        if (randf(seed) >= 0.25) {
            vec4 over = texture(src_tex1, uv);
            if (distance(over.rgb, vec3(0.0)) <= 0.01) {
                over.a = 0.0;
            }
            color = blend_by_mode(color, over, BLEND_ALPHA);
        } else if (randf(seed) >= 0.0) {
            vec4 over = patch_pixelate(tex_coord, vec2(16.0), src_tex1, iResolution.xy * 1.5);
            if (distance(over.rgb, vec3(0.0)) <= 0.01) {
                over.a = 0.0;
            }
            color = blend_by_mode(color, over, BLEND_ALPHA);
        } else if (randf(seed) >= -0.25) {
            vec4 over = patch_textelate(tex_coord, 2.0, src_tex1, iResolution.xy * 1.5);
            if (distance(over.rgb, vec3(0.0)) <= 0.01) {
                over.a = 0.0;
            }
            color = blend_by_mode(color, over, BLEND_ALPHA);
        }
    }
}