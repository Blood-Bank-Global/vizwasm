#include "font_8x16.glsl"
#include "utils.glsl"

//!VAR int selected_button 0
//!VAR uint[] selected_button_str
//!VAR uint[] frame_info

//!VAR uint[] label_data
//!VAR uint[] label_starts
//!VAR uint[] label_lens

//!VAR uint[] value_data
//!VAR uint[] value_starts
//!VAR uint[] value_lens

#define ACTIVE_NAME_IDX 16
#define DISPLAY_NAME_IDX 17
#define SCAN_NAME_IDX 18

void main_frag(out vec4 color) {
    color = texture(src_tex0, src_coord0);
    vec2 uv = src_coord0 * iResolution.xy;

    #define OFFSET_X 114.0
    #define OFFSET_Y 34.0
    #define AREA_W (640 - 2.0 * OFFSET_X)
    #define AREA_H (480 - 2.0 * OFFSET_Y)
    #define BUTTON_W (AREA_W / 4.0)
    #define BUTTON_H (AREA_H / 4.0)
    #define FONT_W font_8x16_width
    #define FONT_H font_8x16_height

    #define blank(uv,pos,len) (str_bounds((uv),(pos),float(FONT_W),float(FONT_H),(len)))

    if (uv.x > OFFSET_X && uv.x < (640.0 - OFFSET_X) && uv.y > OFFSET_Y && uv.y < (480.0 - OFFSET_Y)) {  
        int i = int((uv.y - OFFSET_Y)/BUTTON_H);
        int j = int((uv.x - OFFSET_X)/BUTTON_W);

        if (distance(color.rgb, vec3(0.0, 0.0, 0.0)) > 0.1) {
            vec3 hsv = rgb2hsv(color.rgb);
            hsv[0] = mod(1.0 - 1.0/16.0 * float(i * 4 + j), 1.0);
            hsv[1] = 1.0;
            if (selected_button == i * 4 + j) {
                hsv[2] = 0.75;
            }  else {
                hsv[2] = 0.45;
            }
            color.rgb = hsv2rgb(hsv);
        
        }

        vec2 line1_pos = vec2(
            OFFSET_X + (float(j) * BUTTON_W),
            OFFSET_Y + (float(i) * BUTTON_H)
        );
        int k = int(uv.y - line1_pos.y) / int(FONT_H);
        int m = int(uv.x - line1_pos.x) / int(FONT_W);
        bool draw = false;
        //fill in a solid blank area
        if (blank(uv, line1_pos + vec2(4.0, FONT_H * 2.0), min(12, label_lens[j + i * 4])) 
            || blank(uv, line1_pos + vec2(4.0, FONT_H * 3.0), min(12, value_lens[j + i * 4]))) {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        }
        //render the font
        if (font_8x16(uv, line1_pos + vec2(4.0, FONT_H * 2.0), label_data, label_starts[j + i * 4], label_lens[j + i * 4]) 
            || font_8x16(uv, line1_pos + vec2(4.0, FONT_H * 3.0), value_data, value_starts[j + i * 4], value_lens[j + i * 4])) {
            draw = true;
        }
        // add a 4px padding inside the button
        if (uv.x < line1_pos.x + 4.0 || uv.x > line1_pos.x + BUTTON_W - 4.0 || uv.y < line1_pos.y + 4.0 || uv.y > line1_pos.y + BUTTON_H - 4.0) {
            draw = false;
        }
        if (draw) {
            color = vec4(0.7, 0.7, 0.7, 1.0);
        }

    }

    #define ACTIVE_LABEL_POS vec2(OFFSET_X + AREA_W + FONT_W, OFFSET_Y + BUTTON_H)
    #define ACTIVE_VALUE_POS vec2(OFFSET_X + AREA_W + FONT_W, OFFSET_Y + BUTTON_H + FONT_H)
    #define DISPLAY_LABEL_POS vec2(OFFSET_X + AREA_W + FONT_W, OFFSET_Y + BUTTON_H * 2.0)
    #define DISPLAY_VALUE_POS vec2(OFFSET_X + AREA_W + FONT_W, OFFSET_Y + BUTTON_H * 2.0 + FONT_H)
    #define SCAN_LABEL_POS vec2(FONT_W * 3.0, OFFSET_Y + BUTTON_H + FONT_H * 0.0)
    #define SCAN_VALUE_POS vec2(FONT_W * 3.0, OFFSET_Y + BUTTON_H + FONT_H * 1.0)

    if (blank(uv, ACTIVE_LABEL_POS, label_lens[ACTIVE_NAME_IDX]) \
        || blank(uv, DISPLAY_LABEL_POS, label_lens[DISPLAY_NAME_IDX]) \
        ||blank(uv, ACTIVE_VALUE_POS, value_lens[ACTIVE_NAME_IDX]) \
        || blank(uv, DISPLAY_VALUE_POS, value_lens[DISPLAY_NAME_IDX]) \
        || blank(uv, SCAN_LABEL_POS, min(label_lens[SCAN_NAME_IDX], 10)) \
        || blank(uv, SCAN_VALUE_POS, min(value_lens[SCAN_NAME_IDX], 10))) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }

    if (font_8x16(uv, ACTIVE_LABEL_POS, label_data, label_starts[ACTIVE_NAME_IDX], label_lens[ACTIVE_NAME_IDX]) \
        || font_8x16(uv, ACTIVE_VALUE_POS, value_data, value_starts[ACTIVE_NAME_IDX], value_lens[ACTIVE_NAME_IDX]) \
        || font_8x16(uv, DISPLAY_LABEL_POS, label_data, label_starts[DISPLAY_NAME_IDX], label_lens[DISPLAY_NAME_IDX]) \
        || font_8x16(uv, DISPLAY_VALUE_POS, value_data, value_starts[DISPLAY_NAME_IDX], value_lens[DISPLAY_NAME_IDX]) \
        || font_8x16(uv, SCAN_LABEL_POS, label_data, label_starts[SCAN_NAME_IDX], min(label_lens[SCAN_NAME_IDX], 10)) \
        || font_8x16(uv, SCAN_VALUE_POS, value_data, value_starts[SCAN_NAME_IDX], min(value_lens[SCAN_NAME_IDX], 10))) {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }

    if (blank(uv, vec2(OFFSET_X, OFFSET_Y + AREA_H), selected_button_str.length()) ) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }
    if (font_8x16(uv, vec2(OFFSET_X, OFFSET_Y + AREA_H), selected_button_str, 0, selected_button_str.length())) {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }
}