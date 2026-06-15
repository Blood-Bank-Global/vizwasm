#include "utils.glsl"
#include "patch_blob_px.glsl"
#include "font_8x16.glsl"
#include "patch_halftone.glsl"
#define BPM 150.0

void strobe_text_move_tex(out vec4 color, sampler2D t0, sampler2D t1, vec2 uv, vec2 res, float time) {
    float t = BPM / 60.0;
    float p1 = randf(uint(time * t));

    t = BPM / 60.0;
    float beat = time * t / 4.0; // 4 beats per measure
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

    vec4 c1 = texture(t1, interp + uv);
    color = c1;
    
    if (distance(color.rgb, vec3(0.0)) < 0.01) {
        beat = time * t * 8; // 64th notes
        b = uint(beat);
        f = fract(beat);

        if (randf(uint(b ^ 0xDEAD0000u)) > 0.0) {
            color = texture(t0, (vec2(p1) + uv) / 4.0);
        }
    }

    if (distance(color.rgb, vec3(0.0)) > 0.01) {
        vec3 hsv = rgb2hsv(color.rgb);
        hsv[0] = 30.0/360.0;
        hsv[1] = 1.0;
        color.rgb = hsv2rgb(hsv);
    }
}

void slides(out vec4 color, sampler2D t0, vec2 uv, vec2 res, float time) {
    float t = BPM / 60.0 * 4.0; // whole note;
    float beat = time * t;
    uint b = uint(beat);

    float chunk_sz = 64.0;
    vec2 coord = uv * res;
    vec2 chunk = floor(coord / chunk_sz);

    if ((uint(chunk.x) & 1u) == 0u) {
        color = texture(t0, (src_uv + vec2(randf(b ^ 0xDEAD0000u), fract(time/t))) / 4.0);
        vec3 hsv = rgb2hsv(color.rgb);
        hsv[0] = 300.0/360.0;
        hsv[1] = 1.0;
        color.rgb = hsv2rgb(hsv);
    } else {
        color = texture(t0, (src_uv - vec2(randf(b ^ 0xCAFE0000u), fract(time/t))) / 4.0);
        vec3 hsv = rgb2hsv(color.rgb);
        hsv[0] = 110.0/360.0;
        hsv[1] = 1.0;
        color.rgb = hsv2rgb(hsv);
    }
}

void stars(out vec4 color, sampler2D t0, vec2 uv, vec2 res, float time) {
    float t = BPM / 60.0 * 8.0;
    float remainder = mod(time, t);

    float num = 10.0;
    float chunk_sz = iResolution.y / num;
    vec2 coord = ((uv - vec2(0.5)) * res); // zero center
    vec2 chunk = floor(coord / chunk_sz + 0.5);
    float dist = abs(chunk.y);
    color = texture(t0, coord_mirror((src_uv + vec2(remainder/t, chunk.y) * 2.0 * (dist + 1.0)), true, false)/4.0);
    if (distance(color.rgb, vec3(0.0)) < 0.01) {
        vec3 hsv = rgb2hsv(color.rgb);
        hsv[0] = 230.0/360.0;
        hsv[1] = 1.0;
        hsv[2] = 0.25;
        color.rgb = hsv2rgb(hsv);
    } else {
        vec3 hsv = rgb2hsv(color.rgb);
        hsv[0] = 70.0/360.0;
        hsv[1] = 1.0;
        color.rgb = hsv2rgb(hsv);
    }
}

//!VAR float user0 0.0
//!VAR float user1 0.0
//!VAR float user2 0.0
//!VAR float user3 0.0
//!VAR float user4 0.0
void wave(out vec4 color) {
    float baseFrequency = 400.0;
    float modulationIntensity = 0.5;
    float warpStrength = 0.5; // Controls how aggressively the lines bend
    float bias = 0.9; // -1.0 = more white, 0.0 = 50/50, 1.0 = more black
    patch_wave_dither(
        color,
        src_tex4,
        src_uv,
        iResolution.xy,
        -iTime*10.0, // time, multiplied to increase speed
        true,
        baseFrequency,
        modulationIntensity,
        warpStrength,
        bias,
        false);
}


void pass0(out vec4 color) {
    float t = BPM / 60.0 * 2.0;
    float beat = iTime * t;
    uint b = uint(beat);
    float s = abs(randf(b ^ 0xBABEBABE)) * 3.0;
    switch (uint(s)) {
        case 0u:
            strobe_text_move_tex(color, src_tex0, src_tex1, src_uv, iResolution.xy, iTime);
            break;
        case 1u:
            slides(color, src_tex3, src_uv, iResolution.xy, iTime);
            break;
        case 2u:
            stars(color, src_tex2, src_uv, iResolution.xy, iTime);
            break;
        default:
            color = vec4(0.0);
    }
    wave(color);
}