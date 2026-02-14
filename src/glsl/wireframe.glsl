//!VAR int selected_button 0

//!VAR int[] button1_txt
//!VAR int button1_len 0
//!VAR int[] button1_val
//!VAR int button1_val_len 0

//!VAR int[] button2_txt
//!VAR int button2_len 0
//!VAR int[] button2_val
//!VAR int button2_val_len 0

//!VAR int[] button3_txt
//!VAR int button3_len 0
//!VAR int[] button3_val
//!VAR int button3_val_len 0

//!VAR int[] button4_txt
//!VAR int button4_len 0
//!VAR int[] button4_val
//!VAR int button4_val_len 0

//!VAR int[] button5_txt
//!VAR int button5_len 0
//!VAR int[] button5_val
//!VAR int button5_val_len 0

//!VAR int[] button6_txt
//!VAR int button6_len 0
//!VAR int[] button6_val
//!VAR int button6_val_len 0

//!VAR int[] button7_txt
//!VAR int button7_len 0
//!VAR int[] button7_val
//!VAR int button7_val_len 0

//!VAR int[] button8_txt
//!VAR int button8_len 0
//!VAR int[] button8_val
//!VAR int button8_val_len 0

//!VAR int[] button9_txt
//!VAR int button9_len 0
//!VAR int[] button9_val
//!VAR int button9_val_len 0

//!VAR int[] button10_txt
//!VAR int button10_len 0
//!VAR int[] button10_val
//!VAR int button10_val_len 0

//!VAR int[] button11_txt
//!VAR int button11_len 0
//!VAR int[] button11_val
//!VAR int button11_val_len 0

//!VAR int[] button12_txt
//!VAR int button12_len 0
//!VAR int[] button12_val
//!VAR int button12_val_len 0

//!VAR int[] button13_txt
//!VAR int button13_len 0
//!VAR int[] button13_val
//!VAR int button13_val_len 0

//!VAR int[] button14_txt
//!VAR int button14_len 0
//!VAR int[] button14_val
//!VAR int button14_val_len 0

//!VAR int[] button15_txt
//!VAR int button15_len 0
//!VAR int[] button15_val
//!VAR int button15_val_len 0

//!VAR int[] button16_txt
//!VAR int button16_len 0
//!VAR int[] button16_val
//!VAR int button16_val_len 0

color = texture(src_tex0, src_coord0);
vec2 uv = src_coord0 * iResolution.xy;

#define OFFSET_X 114.0
#define OFFSET_Y 34.0
#define AREA_W (640 - 2.0 * OFFSET_X)
#define AREA_H (480 - 2.0 * OFFSET_Y)
#define BUTTON_W (AREA_W / 4.0)
#define BUTTON_H (AREA_H / 4.0)
#define FONT_W 8.0
#define FONT_H 16.0

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
    int k = int(uv.y - line1_pos.y) / int(FONT_H + 4.0);
    int m = int(uv.x - line1_pos.x) / int(FONT_W);
    if (uv.y > line1_pos.y && uv.y < line1_pos.y + BUTTON_H && (k == 2 || k == 1) && m < 12) {
        vec2 line_pos = line1_pos + vec2(0.0, float(k) * (FONT_H + 4.0));
        bool draw = false;
        int len = 0;
        switch (i * 4 + j) {
            case 0:
                if (k == 1 && m < button1_len) {
                    draw = font_8x16(uv, line_pos, button1_txt, 0, 11);
                    len = button1_len;
                } else if (k == 2 && m < button1_val_len) {
                    draw = font_8x16(uv, line_pos, button1_val, 0, 11);
                    len = button1_val_len;
                }
                break;
            case 1:
                if (k == 1 && m < button2_len) {
                    draw = font_8x16(uv, line_pos, button2_txt, 0, 11);
                    len = button2_len;
                } else if (k == 2 && m < button2_val_len) {
                    draw = font_8x16(uv, line_pos, button2_val, 0, 11);
                    len = button2_val_len;
                }
                break;
            case 2:
                if (k == 1 && m < button3_len) {
                    draw = font_8x16(uv, line_pos, button3_txt, 0, 11);
                    len = button3_len;
                } else if (k == 2 && m < button3_val_len) {
                    draw = font_8x16(uv, line_pos, button3_val, 0, 11);
                    len = button3_val_len;
                }
                break;
            case 3:
                if (k == 1 && m < button4_len) {
                    draw = font_8x16(uv, line_pos, button4_txt, 0, 11);
                    len = button4_len;
                } else if (k == 2 && m < button4_val_len) {
                    draw = font_8x16(uv, line_pos, button4_val, 0, 11);
                    len = button4_val_len;
                }
                break;
            case 4:
                if (k == 1 && m < button5_len) {
                    draw = font_8x16(uv, line_pos, button5_txt, 0, 11);
                    len = button5_len;
                } else if (k == 2 && m < button5_val_len) {
                    draw = font_8x16(uv, line_pos, button5_val, 0, 11);
                    len = button5_val_len;
                }
                break;
            case 5:
                if (k == 1 && m < button6_len) {
                    draw = font_8x16(uv, line_pos, button6_txt, 0, 11);
                    len = button6_len;
                } else if (k == 2 && m < button6_val_len) {
                    draw = font_8x16(uv, line_pos, button6_val, 0, 11);
                    len = button6_val_len;
                }
                break;
            case 6:
                if (k == 1 && m < button7_len) {
                    draw = font_8x16(uv, line_pos, button7_txt, 0, 11);
                    len = button7_len;
                } else if (k == 2 && m < button7_val_len) {
                    draw = font_8x16(uv, line_pos, button7_val, 0, 11);
                    len = button7_val_len;
                }
                break;
            case 7:
                if (k == 1 && m < button8_len) {
                    draw = font_8x16(uv, line_pos, button8_txt, 0, 11);
                    len = button8_len;
                } else if (k == 2 && m < button8_val_len) {
                    draw = font_8x16(uv, line_pos, button8_val, 0, 11);
                    len = button8_val_len;
                }
                break;
            case 8:
                if (k == 1 && m < button9_len) {
                    draw = font_8x16(uv, line_pos, button9_txt, 0, 11);
                    len = button9_len;
                } else if (k == 2 && m < button9_val_len) {
                    draw = font_8x16(uv, line_pos, button9_val, 0, 11);
                    len = button9_val_len;
                }
                break;
            case 9:
                if (k == 1 && m < button10_len) {
                    draw = font_8x16(uv, line_pos, button10_txt, 0, 11);
                    len = button10_len;
                } else if (k == 2 && m < button10_val_len) {
                    draw = font_8x16(uv, line_pos, button10_val, 0, 11);
                    len = button10_val_len;
                }
                break;
            case 10:
                if (k == 1 && m < button11_len) {
                    draw = font_8x16(uv, line_pos, button11_txt, 0, 11);
                    len = button11_len;
                } else if (k == 2 && m < button11_val_len) {
                    draw = font_8x16(uv, line_pos, button11_val, 0, 11);
                    len = button11_val_len;
                }
                break;
            case 11:
                if (k == 1 && m < button12_len) {
                    draw = font_8x16(uv, line_pos, button12_txt, 0, 11);
                    len = button12_len;
                } else if (k == 2 && m < button12_val_len) {
                    draw = font_8x16(uv, line_pos, button12_val, 0, 11);
                    len = button12_val_len;
                }
                break;
            case 12:
                if (k == 1 && m < button13_len) {
                    draw = font_8x16(uv, line_pos, button13_txt, 0, 11);
                    len = button13_len;
                } else if (k == 2 && m < button13_val_len) {
                    draw = font_8x16(uv, line_pos, button13_val, 0, 11);
                    len = button13_val_len;
                }
                break;
            case 13:
                if (k == 1 && m < button14_len) {
                    draw = font_8x16(uv, line_pos, button14_txt, 0, 11);
                    len = button14_len;
                } else if (k == 2 && m < button14_val_len) {
                    draw = font_8x16(uv, line_pos, button14_val, 0, 11);
                    len = button14_val_len;
                }
                break;
            case 14:
                if (k == 1 && m < button15_len) {
                    draw = font_8x16(uv, line_pos, button15_txt, 0, 11);
                    len = button15_len;
                } else if (k == 2 && m < button15_val_len) {
                    draw = font_8x16(uv, line_pos, button15_val, 0, 11);
                    len = button15_val_len;
                }
                break;
            case 15:
                if (k == 1 && m < button16_len) {
                    draw = font_8x16(uv, line_pos, button16_txt, 0, 11);
                    len = button16_len;
                } else if (k == 2 && m < button16_val_len) {
                    draw = font_8x16(uv, line_pos, button16_val, 0, 11);
                    len = button16_val_len;
                }
                break;
            default:
                draw = false;
        }

        if ((k == 1 || k == 2) && m < len) {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        }
        
        if (draw) {
            color = vec4(0.7, 0.7, 0.7, 1.0);
        }
    }   
}


if (uv.x > OFFSET_X && 
    uv.x < OFFSET_X + AREA_W && 
    uv.y > OFFSET_Y + AREA_H &&
    uv.y < OFFSET_Y + AREA_H + FONT_H) {
    int txt[3];
    int len = 3;
    txt[0] = selected_button / 100 + 48;
    txt[1] = (selected_button / 10) % 10 + 48;
    txt[2] = selected_button % 10 + 48;
    if (font_8x16(uv, vec2(OFFSET_X, OFFSET_Y + AREA_H), txt, 0, len)) {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }
}