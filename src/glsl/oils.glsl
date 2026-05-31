#include "utils.glsl"
#include "patch_warp_px.glsl"
#include "patch_pixelate.glsl"
#include "patch_halftone.glsl"
//!VAR float cc_iac_driver_bus_1_0_0 0.0

#define BPM 92.0
#define VISIBLE (mat4x2(0.0,0.0,1.0,0.0,0.0,1.0,1.0,1.0))
#define SPEED (5.0)

void feedback_swirl(out vec4 color_out, sampler2D prev, vec4 color_in) {
    vec2 coord = src_uv * iResolution.xy;
    vec2 warped = patch_warp_px(coord, vec2(30.0), 0.1, iResolution.xy, iTime*2.0);
    if (distance(warped.y, iResolution.y) < SPEED * 2.0) {
        color_out =  color_in;
        return;
    }

    warped += vec2(0.0, SPEED);
    warped /= iResolution.xy;
    warped = coord_smear(warped);
    color_out = texture(prev, warped);
    color_out.a = 1.0;

}

void pass0(out vec4 color) {
    vec4 color_in = vec4(
            hsv2rgb(
                vec3(
                    0.8,
                    (sin(fract(iTime/2.0) * 2.0 * M_PI) + 1.0)/2.0,
                    (sin(fract(iTime/2.0) * 4.0 * M_PI) + 1.0) / 2.0 * 0.6 + 0.4)
            ),
            1.0);
    feedback_swirl(color, pass_tex0, color_in);
}

void pass1(out vec4 color) {
    color = patch_pixelate(src_uv * iResolution.xy, vec2(4.0), pass_tex0, iResolution.xy);
}

void pass2(out vec4 color) {

    float hue = sin(fract(iTime) * 2.0 * M_PI);
    if (hue < 0.0) { 
        hue = abs(hue) * 0.1 + 0.25;
    } else {
        hue = hue * 0.1 + 0.3;
    }

    vec4 color_in = vec4(
            hsv2rgb(
                vec3(
                    hue,
                    1.0,
                    (sin(fract(iTime/2.0) * 2.0 * M_PI) + 1.0) / 2.0 * 0.3 + 0.31)
                ),
            1.0);
    feedback_swirl(color, pass_tex2, color_in);
}

void pass3(out vec4 color) {
    color = patch_pixelate(src_uv * iResolution.xy, vec2(4.0), pass_tex2, iResolution.xy);
}

void pass4(out vec4 color) {
    vec2 uv = mod(src_uv - vec2(0.0,fract(iTime/4.0)), 1.0);
    vec2 coord = uv * iResolution.xy;
    vec2 block = floor(coord / 10.0);
    if (distance(uv.x, 0.5) < 0.025) {
        if ((uint(block.y) & 1u) == 0u )  {
            color = vec4(hsv2rgb(vec3(0.1, 0.0, 0.1 + 0.1 * sin(iTime*10.0 + M_PI/2.0))), 1.0);
        } else {
            color = vec4(hsv2rgb(vec3(0.05, 0.0, 0.1 + 0.1 * sin(iTime*10.0))), 1.0);
        }
    } else {
        color = vec4(0.0, 0.0, 0.0, 0.0);
    }
}

void pass5(out vec4 color) {
    mat4x2 corners = mat4x2(
        vec2(-4000.0, 0.0) / iResolution.xy,
        vec2(iResolution.x + 4000.0, 0.0) / iResolution.xy,
        vec2(0.0, 100.0) / iResolution.xy,
        vec2(iResolution.x, 100.0) / iResolution.xy
    );

    vec2 uv = skew3(src_uv, corners);
    if (pointInRhombus(uv, VISIBLE)) {
        color = texture(pass_tex1, uv);
        color.a = 1.0;
        return;
    }
    
    corners = mat4x2(
        vec2(0.0, 300.0) / iResolution.xy,
        vec2(iResolution.x, 300.0) / iResolution.xy,
        vec2(-4000.0, iResolution.y) / iResolution.xy,
        vec2(iResolution.x + 4000.0, iResolution.y) / iResolution.xy
    );

    uv = skew3(src_uv, corners);
    if (pointInRhombus(uv, VISIBLE)) {
        vec4 over = texture(pass_tex4, uv);
        vec4 under = texture(pass_tex3, -uv);
        color = blend_by_mode(under, over, BLEND_ALPHA);
        return;
    }
    
    float beat_sec = BPM / 60.0  * 4.0;
    uint seq = uint(iTime * beat_sec);
    if (randf(seq) < 0.0) {
        vec2 uv = (src_uv - vec2(0.5)) * .6 + vec2(0.5);
        if (randf(seq + 1u) < 0.0) {
            uv = vec2(-uv.x, uv.y);
        }
        color = texture(src_tex2, uv);
    } else {
        vec2 offset = vec2(
            (randf(seq + 1u) - 0.5) * 0.1,
            (randf(seq + 2u) - 0.5) * 0.1
        );
        offset.y += fract(iTime/5.0);
        if (randf(seq + 2u) < 0.0) {
            color = texture(src_tex1, (src_uv + offset));
        } else {
            color = texture(src_tex0, (src_uv + offset)/4.0);
        }
    }
    if (distance(color.rgb, vec3(0.0)) < 0.1) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        vec3 hsv = rgb2hsv(color.rgb);
        hsv[0] = abs(randf(seq));
        hsv[1] = 1.0;
        color = vec4(hsv2rgb(hsv), 1.0);
    }
    color.a = 1.0/3.0;
    vec4 old = texture(pass_tex5, src_uv);
    color = blend_by_mode(old, color, BLEND_ALPHA);
}

void pass6(out vec4 color) {
    patch_ordered_dither4x4(color, src_uv, iResolution.xy, pass_tex5);
}