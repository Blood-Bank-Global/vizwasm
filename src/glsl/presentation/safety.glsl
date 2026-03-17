#include "utils.glsl"
#include "font_8x16.glsl"
#include "patch_blob_px.glsl"
#include "patch_pixelate.glsl"
#define FONT_W (font_8x16_width)
#define FONT_H (font_8x16_height)

void pass0(out vec4 color) {
    color = texture(src_tex0, src_coord0);
    vec2 uv = src_coord.xy * iResolution.xy;
    vec2 pos = floor((uv) / vec2(FONT_W, FONT_H)) * vec2(FONT_W, FONT_H);
    uint c[1] = uint[1](uint(mod(pcg(((uint(pos.x) * 11) ^ (uint(pos.y) * 13) ^ (uint(floor(iTime*1000) * 7)))), 255)));
    vec4 blob_color = vec4(1.0);

    

    int count = 0;
    for (int i = 0; i < 2; i++) {
        for (int j = 0; j < 2; j++) {
            if (patch_blob_px(
                pos.xy + vec2(i * FONT_W, j * FONT_H), 
                iResolution.xy,
                vec4(0.0),
                blob_color,
                iResolution.xy * 0.5,
                300.0, 
                iTime
            ).a > 0.99) {
                count++;
            }
        }
    }

    if (count == 4) {
        color = patch_textelate_arcade(uv, 1.0, src_tex1, iResolution.xy);
        return;
    } else if (count > 0) {
        c[0] = 0x78;
    }

    if (font_8x16(uv, pos, c, 0, 1)) {
        color = vec4(1.0);
    }
}