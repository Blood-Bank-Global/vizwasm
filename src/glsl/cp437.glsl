#include "utils.glsl"
#include "font_8x16.glsl"
#define FONT_W (font_8x16_width)
#define FONT_H (font_8x16_height)

void pass0(out vec4 color) {
    vec2 coord = src_uv.xy * iResolution.xy;
    vec2 rc = floor(coord / vec2(FONT_W, FONT_H));
    float idx = (rc.x - 1) + (rc.y - 1) * 16.0;
    vec2 pos = rc * vec2(FONT_W, FONT_H);

    color = vec4(0.0, 0.0, 0.0, 1.0);
    if (rc.x >= 17.0 || rc.y >= 17.0) {
        return;
    }

    vec4 letter_color = vec4(1.0);
    uint c[1] = uint[1](uint(idx));
    if (uint(rc.x) == 0) {
        letter_color = vec4(0.0, 0.0, 0.0, 1.0);
        color = vec4(1.0);
         if (uint(rc.y) >= 10u) {
            c[0] = uint(rc.y) - 11 + 0x41;
        } else {
            c[0] = uint(rc.y) - 1 + 0x30;
        }
    } else if (uint(rc.y) == 0) {
        letter_color = vec4(0.0, 0.0, 0.0, 1.0);
        color = vec4(1.0);
         if (uint(rc.x) >= 10u) {
            c[0] = uint(rc.x) - 11 + 0x41;
        } else {
            c[0] = uint(rc.x) - 1 + 0x30;
        }
    }
    
    if (font_8x16(coord, pos, c, 0, 1)) {
        color = letter_color;
    }
}