#include "utils.glsl"
#include "font_8x16.glsl"
#define FONT_W (font_8x16_width)
#define FONT_H (font_8x16_height)

void pass0(out vec4 color) {
    color = texture(src_tex0, src_coord0);
    vec2 uv = src_coord.xy * iResolution.xy;
    vec2 pos = floor((uv) / vec2(FONT_W, FONT_H)) * vec2(FONT_W, FONT_H);
    uint c[1] = uint[1](uint(randf(((uint(pos.x) * 11) ^ (uint(pos.y) * 13) ^ (uint(floor(iTime) * 7)))) * 255.0));
    
    if (font_8x16(uv, pos, c, 0, 1)) {
        color = vec4(1.0);
    }     
}