#include "utils.glsl"
#include "font_8x16.glsl"
#define FONT_W (font_8x16_width)
#define FONT_H (font_8x16_height)

void pass0(out vec4 color) {
    color = texture(src_tex0, src_uv0);
    vec2 coord = src_uv.xy * iResolution.xy;
    vec2 pos = floor((coord) / vec2(FONT_W, FONT_H)) * vec2(FONT_W, FONT_H);
    uint c[1] = uint[1](uint(randf(((uint(pos.x) * 11) ^ (uint(pos.y) * 13) ^ (uint(floor(iTime*2.0) * 7)))) * 255.0));
    bool white_space = c[0] == 0u ||
        c[0] == 32u || 
        (c[0] >= 9 && c[0] <= 13) ||
         c[0] == 255;

    if (!white_space) {
        color.rgb = vec3(1.0) - color.rgb;
        if (font_8x16(coord, pos, c, 0, 1)) {
            color = vec4(0.0, 0.0,0.0, 1.0);
        } 
    }    
}