#include "utils.glsl"
#include "font_8x8.glsl"

//!VAR vec3 iResolution0 1.0 1.0 1.0
//!VAR uint[] cyberpunk_txt
//!VAR uint[] cyberpunk_starts
//!VAR uint[] cyberpunk_lens


#define TILE_SIZE vec2(8.0,8.0)

void pass0(out vec4 color) {
    vec2 uv = src_uv/2.0;
    color = vec4(vec3(0.0), 1.0);

    vec2 tile = floor(uv.xy * iResolution.xy / TILE_SIZE);
    vec2 coord = uv.xy * iResolution.xy;
    vec2 pos = tile * TILE_SIZE;

    vec2 sprite_uv = (uv.xy * iResolution.xy + vec2(0.0, 8.0)) / iResolution0.xy ;
    if (sprite_uv.x <= 1.0 && sprite_uv.y <= 1.0) {
        color = texture(src_tex0, sprite_uv);
        color.a = 1.0;
        uint c[] = uint[](uint(clamp(0x30 + int(tile.x) + int(tile.y * 7.0), 0, 255)));
        if (font_8x8(coord, pos, c, 0, 1)) {
            color = vec4(1.0);
        }
    }
    if (mod(floor(coord.x), floor(TILE_SIZE.x)) == 0.0 || mod(floor(coord.y), floor(TILE_SIZE.y)) == 0.0) {
        color = vec4(1.0);

    }
}