#include "utils.glsl"
#include "font_8x8.glsl"
#include "patch_warp_px.glsl"

//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0
//!VAR vec2 iResolution2 1.0 1.0
//!VAR vec2 iResolution3 1.0 1.0
//!VAR vec2 iResolution4 1.0 1.0
//!VAR vec2 iResolution5 1.0 1.0
//!VAR vec2 iResolution6 1.0 1.0
//!VAR vec2 iResolution7 1.0 1.0
//!VAR vec2 iResolution8 1.0 1.0
//!VAR vec2 iResolution9 1.0 1.0


//!VAR uint[] maze_txt
//!VAR uint[] maze_starts 
//!VAR uint[] maze_lens

#define TILE_SIZE vec2(32.0,32.0)

#define get_sample(n, uv) texture(src_tex##n, (uv).xy *  iResolution.xy / iResolution##n.xy)

void pass0(out vec4 color) {
    vec2 coord = src_uv.xy * iResolution.xy;
    vec2 warped_coord = patch_warp_px(coord, TILE_SIZE, 0.2, iResolution.xy, iTime);
    vec2 warped_uv = warped_coord.xy/iResolution.xy;
    // color = vec4(hsv2rgb(vec3(src_uv.x, 0.5, warped_uv.y)), 1.0);
    color = vec4(vec3(0.0), 1.0);
    vec2 tile = warped_uv.xy * iResolution.xy / TILE_SIZE;
    warped_uv += vec2(sin(iTime), cos(iTime*0.9))*TILE_SIZE*4.0/iResolution.xy;
    uint row_len = maze_lens[0];
    vec4 maze_color = vec4(vec3(0.0), 1.0);
    switch(maze_txt[row_len * uint(tile.y) + uint(tile.x)]) {
        case 0x30:
            maze_color = get_sample(0, warped_uv);
            break;
        case 0x31:
            maze_color = get_sample(1, warped_uv);
            break;
        case 0x32:
            maze_color = get_sample(2, warped_uv);
            break;
        case 0x33:
            maze_color = get_sample(3, warped_uv);
            break;
        case 0x34:
            maze_color = get_sample(4, warped_uv);
            break;
        case 0x35:
            maze_color = get_sample(5, warped_uv);
            break;
        case 0x36:
            maze_color = get_sample(6, warped_uv);
            break;
        case 0x37:
            maze_color = get_sample(7, warped_uv);
            break;
        case 0x38:
            maze_color = get_sample(8, warped_uv);
            break;
        case 0x39:
            maze_color = get_sample(9, warped_uv);
            break;
        default:
            maze_color = color;
            break;
    }
    if (maze_color.r > 0.5) { 
        color = maze_color;
    }
}