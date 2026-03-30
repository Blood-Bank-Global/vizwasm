#include "utils.glsl"
#include "font_8x8.glsl"
#include "patch_rototrans.glsl"

//!VAR uint[] dino_frame
//!VAR uint[] dino_frame_starts
//!VAR uint[] dino_frame_lens

//!STR unix "IBM AIX Version 4  "
//!STR date "Last login: Sat Jun 7 10:12:05  "
//!STR know "Do you know this?  "
//!STR word "Did youd say the magic word?  "
//!STR file "Please select file.  "

#define SCALE  ((iResolution.x / 30.0) / (font_8x8_width))
#define FONT_W (font_8x8_width) * SCALE
#define FONT_H (font_8x8_height) * SCALE
#define LINE_CHARS (floor(iResolution.x / (FONT_W)))
#define LINES (floor(iResolution.y / (FONT_H)))

#define PIP_SIZE vec2(6 * FONT_W, 4 * FONT_H)
#define PIP_OFF vec2((LINE_CHARS * FONT_W) - PIP_SIZE.x, ((LINES - 1.0) * FONT_H) - PIP_SIZE.y)
#define BORDER_SIZE 3.0

void pass0(out vec4 color) {
    color = texture(src_tex0, src_uv0);

    vec2 coord = src_uv.xy * iResolution.xy;

    mat4x2 pip = mat4x2(
        PIP_OFF,
        PIP_OFF + vec2(PIP_SIZE.x, 0),
        PIP_OFF + vec2(0, PIP_SIZE.y),
        PIP_OFF + PIP_SIZE
    );

    if (pointInRhombus(coord, pip)) {
            vec2 pip_uv = (coord - PIP_OFF) / PIP_SIZE;
            color = patch_rototrans(
                pip_uv, 
                src_tex1, 
                src_tex2, 
                src_tex3,
                0.0, 
                0.1, 
                0.1, 
                0.5, 
                EDGE_MODE_WRAP
            );

    } else {
        vec2 rc = floor(coord/vec2(FONT_W, FONT_H));
        vec2 pos = rc * vec2(FONT_W, FONT_H);

        uint c = dino_frame[dino_frame_starts[uint(rc.y)] + uint(rc.x)];
        if (!is_cp437_space(c)) {
            color = vec4(vec3(0.0), 1.0);
        }

        if (multiline_8x8(coord/SCALE, vec2(0,0), dino_frame, dino_frame_starts, dino_frame_lens)) {
            color = vec4(vec3(1.0), 1.0);
        }
    }

    vec2 text_coord = coord/2.0;
    
    uint msg_select = uint(mod(iTime/2.0, M_PI/2.0 * 5.0) / (M_PI/2.0));
    float msg_fract = sin(mod(iTime/2.0, M_PI/2.0));
    vec2 msg_pos = vec2(font_8x8_width * 3, font_8x8_height * 3);

    uint selected_len = 0;
    switch (msg_select) {
        case 0u: selected_len = uint(float(unix.length()) * msg_fract) + 1; break;
        case 1u: selected_len = uint(float(date.length()) * msg_fract) + 1; break;
        case 2u: selected_len = uint(float(know.length()) * msg_fract) + 1; break;
        case 3u: selected_len = uint(float(word.length()) * msg_fract) + 1; break;
        case 4u: selected_len = uint(float(file.length()) * msg_fract) + 1; break;
    }

    if (str_bounds(text_coord, msg_pos, font_8x8_width, font_8x8_height, selected_len)) {
        color = vec4(vec3(0.0), 1.0);
    }
    if (msg_select == 0u && font_8x8(text_coord, msg_pos, unix, 0, selected_len)
        || msg_select == 1u && font_8x8(text_coord, msg_pos, date, 0, selected_len)
        || msg_select == 2u && font_8x8(text_coord, msg_pos, know, 0, selected_len)
        || msg_select == 3u && font_8x8(text_coord, msg_pos, word, 0, selected_len)
        || msg_select == 4u && font_8x8(text_coord, msg_pos, file, 0, selected_len)) {
        color = vec4(vec3(1.0), 1.0);
    }

    vec2 cursor_pos = msg_pos + vec2(selected_len * font_8x8_width, 0);
    uint cursor[] = uint[](0xDB);
    if (fract(iTime) >= 0.5 && font_8x8(text_coord, cursor_pos, cursor, 0, 1)) {
        color = vec4(vec3(1.0), 1.0);
    }
}