#include "utils.glsl"
#include "patch_drippy_px.glsl"

#include "font_8x8.glsl"
#include "font_cyber.glsl"

#define REGION_SZ vec2(45.0, 45.0)
#define PIXEL_SZ vec2(8.0, 8.0)
#define CUTOFF_Y 400.0

//!VAR float cc_iac_driver_bus_1_1_0 100.0

void pass0(out vec4 color) {
    patch_drippy_px(
        color, 
        src_tex0,
        src_uv,
        iResolution.xy,
        REGION_SZ,
        PIXEL_SZ,
        cc_iac_driver_bus_1_1_0 / 127.0 * iResolution.y,
        iTime);
}


//!VAR uint[] dialog 0.0 0.0
//!VAR uint[] dialog_starts 0.0 0.0
//!VAR uint[] dialog_lens 0.0 0.0

//!VAR uint[] msg 0.0 0.0
//!VAR uint[] msg_starts 0.0 0.0
//!VAR uint[] msg_lens 0.0 0.0


#define TIME_SLICE 3.0
#define TEXT_WINDOW_START 0.2
#define TEXT_WINDOW_END 2.8

void pass1(out vec4 color) {
    color = texture(pass_tex0, src_uv);
    
    vec2 coord = src_uv.xy * iResolution.xy;

    uint epoc = uint(floor(iTime / TIME_SLICE) * TIME_SLICE);
    float t = mod(iTime , TIME_SLICE);
    vec2 pos = vec2(abs(randf(epoc ^ 823471u)), abs(randf(epoc ^ 234872u)))
        * (iResolution.xy  - vec2(font_8x8_width * float(dialog_lens[0]), font_8x8_height * 24.0))
        + vec2(0.0, font_8x8_height * 4.0);


    float shade = smoothstep(0.0, TEXT_WINDOW_START, t);
    float shade_y = dialog_lens.length() * font_8x8_height * shade;
    if (coord.y <= pos.y + shade_y) {
        if (multiline_bounds(coord, pos, font_8x8_width, font_8x8_height, dialog_starts, dialog_lens)) {
            vec4 overlay = vec4(1.0, 1.0, 1.0, 0.7);
            color = blend_by_mode(color, overlay, BLEND_ALPHA);

        }

        if (multiline_8x8(coord, pos, dialog, dialog_starts, dialog_lens)) {
            color = vec4(0.0, 0.0, 0.0, 0.7);
        }
    }

    float txt_prog = smoothstep(1.0, 2.0, t);
    uint idx = uint(txt_prog * float(msg.length() - 1.0));
    vec2 idx_cr = vec2(-1.0);

    for (int j = 0; j < msg_starts.length(); j++) {
        for (int i = 0; i <= msg_lens[j]; i++) {
            if (msg_starts[j] + i > idx) {
                idx_cr = vec2(float(i), float(j));
                break;
            }
        }
        if (idx_cr != vec2(-1.0)) {
            break;
        }
    }
    pos += vec2(font_8x8_width, font_8x8_height);
    vec2 coord_cr = floor((coord.xy - pos) / vec2(font_cyber_width, font_cyber_height));
    if (txt_prog > 0.0 && (coord_cr.y < idx_cr.y || (coord_cr.y == idx_cr.y && coord_cr.x <= idx_cr.x))) {
        if (multiline_cyber(coord, pos, msg, msg_starts, msg_lens)) {
            color = vec4(0.0, 0.0, 0.0, 0.7);
        }
    }
}