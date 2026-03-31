#include "utils.glsl"
#include "font_8x8.glsl"
#include "patch_warp_px.glsl"
//!VAR uint[] maze_txt
//!VAR uint[] maze_starts 
//!VAR uint[] maze_lens

#define TILE_SIZE vec2(10.0,10.0)

void pass0(out vec4 color) {
    vec2 coord = src_uv.xy * iResolution.xy;
    vec2 warped_coord = patch_warp_px(coord, TILE_SIZE, 0.1, iResolution.xy, iTime);
    vec2 warped_uv = warped_coord.xy/iResolution.xy;
    color = vec4(hsv2rgb(vec3(warped_uv.x, 0.5, warped_uv.y)), 1.0);
    vec2 tile = warped_uv.xy * iResolution.xy / TILE_SIZE;
    uint row_len = maze_lens[0];
    vec4 maze_color = vec4(vec3(0.0), 1.0);
    switch(maze_txt[row_len * uint(tile.y) + uint(tile.x)]) {
        case 0x30:
            maze_color = texture(src_tex0, warped_uv);
            break;
        case 0x31:
            maze_color = texture(src_tex1, warped_uv);
            break;
        case 0x32:
            maze_color = texture(src_tex2, warped_uv);
            break;
        case 0x33:
            maze_color = texture(src_tex3, warped_uv);
            break;
        default:
            maze_color = color;
            break;
    }
    if (maze_color.r > 0.5) { 
        color = maze_color;
    }
}