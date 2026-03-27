#include "utils.glsl"
#include "font_arcade.glsl"
#define FONT_SCALE (1.5)
#define FONT_W (font_arcade_width * FONT_SCALE)
#define FONT_H (font_arcade_height * FONT_SCALE)
#include "patch_edge_detect.glsl"
#include "patch_pixelate.glsl"

//!VAR vec3 iResolution0 0.0 0.0 0.0 
//!VAR float cc_iac_driver_bus_1_0_0 0.0
void pass0(out vec4 color) {
    vec2 coord = src_uv.xy * iResolution.xy;

    float resize = iResolution0.y / iResolution.y;
    float offx = (iResolution0.x / resize - iResolution.x)/2.0;
    color = texture(src_tex0, (coord / iResolution0.xy) * resize + vec2(offx, 0.0) / iResolution.xy);
}   

void pass1(out vec4 color) {
    color = vec4(0.0, 0.0, 0.0, 1.0);
    if (patch_edge_detect(pass_uv0, pass_tex0, iResolution.xy, 50)) {
        color = vec4(1.0);
    }
}

void pass2(out vec4 color) {
    color = texture(pass_tex1, pass_uv1);
    vec2 coord = pass_uv1.xy * iResolution.xy;
    vec2 pos = floor((coord) / vec2(FONT_W, FONT_H)) * vec2(FONT_W, FONT_H);
    color = vec4(0.0, 0.0, 0.0, 1.0);
    uint count = 0;
    for (int i = 0; i < FONT_W; i++) {
        for (int j = 0; j < FONT_H; j++) {
            vec2 sample_pos = pos + vec2(i, j);
            if (texture(pass_tex1, sample_pos / iResolution.xy).r > 0.0) {
                count++;
            }
        }
    }
    if (count > 120) {
        color = vec4(1.0);
    }
}

void pass3(out vec4 color) {
    color = texture(pass_tex0, pass_uv0);
    vec3 orig = color.rgb;
    vec2 coord = pass_uv0.xy * iResolution.xy;
    vec2 pos = floor((coord) / vec2(FONT_W, FONT_H)) * vec2(FONT_W, FONT_H);

    bool bypass = randf(uint(iTime * 3) ^ uint(pos.x * 117) ^ uint(pos.y * 311)) > (127.0 - float(cc_iac_driver_bus_1_0_0))/127.0;
    if (bypass || texture(pass_tex2, pos / iResolution.xy).r > 0.0) {
        vec4 flip = patch_textelate_arcade(coord, FONT_SCALE, pass_tex0, iResolution.xy);
        if (flip.r > 0.1) {
            color.rgb = vec3(1.0, 1.0, 1.0) - orig.rgb;
        }
    } 
}