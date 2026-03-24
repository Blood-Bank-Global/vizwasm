#include "utils.glsl"
#include "font_8x16.glsl"
#define FONT_W (font_8x16_width)
#define FONT_H (font_8x16_height)
#include "patch_blob_px.glsl"
//!VAR float cc_iac_driver_bus_1_0_0 0.0

void pass0(out vec4 color) {
    color = texture(src_tex0, src_coord0);
    vec2 uv = src_coord.xy * iResolution.xy;
    vec2 pos = floor((uv) / vec2(FONT_W, FONT_H)) * vec2(FONT_W, FONT_H);
    uint c[1] = uint[1](uint(randf(((uint(pos.x) * 11) ^ (uint(pos.y) * 13) ^ (uint(floor(iTime*10) * 7)))) * 255.0));
    
    vec2 corners[] = vec2[](pos, pos + vec2(FONT_W, 0.0), pos + vec2(0.0, FONT_H), pos + vec2(FONT_W, FONT_H));
    int count = 0;
    for (int i = 0; i < 4; i++) {
        vec4 blob = patch_blob_px(
            corners[i],
            iResolution.xy,
            vec4(0.0),
            vec4(1.0),
            iResolution.xy * 0.5,
            iResolution.x * 0.5, //* cc_iac_driver_bus_1_0_0/127.0,
            iTime
        );
        if (blob.r > 0.0) {
            count++;
        }
    }
    if (count >= 4) {
        color = texture(src_tex0, src_coord0);
        return;
    } else if (count > 0) {
        c[0] = 0xB0;
    }

    bool white_space = c[0] == 0u || c[0] == 32u || (c[0] >= 9 && c[0] <= 13) || c[0] == 255;
    
    if (!white_space && str_bounds(uv, pos, FONT_W, FONT_H,1)) {
        color.rgb = vec3(1.0) - color.rgb;
    }
    if (font_8x16(uv, pos, c, 0, 1)) {
        color = vec4(1.0);
    }     
}