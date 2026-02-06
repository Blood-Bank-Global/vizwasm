//!VAR int selected_button 0

//!VAR int[] button1_txt 128
//!VAR int button1_len 0
//!VAR int[] button1_val 128
//!VAR int button1_val_len 0

//!VAR int[] button2_txt 128
//!VAR int button2_len 0
//!VAR int[] button2_val 128
//!VAR int button2_val_len 0

//!VAR int[] button3_txt 128
//!VAR int button3_len 0
//!VAR int[] button3_val 128
//!VAR int button3_val_len 0

//!VAR int[] button4_txt 128
//!VAR int button4_len 0
//!VAR int[] button4_val 128
//!VAR int button4_val_len 0

//!VAR int[] button5_txt 128
//!VAR int button5_len 0
//!VAR int[] button5_val 128
//!VAR int button5_val_len 0

//!VAR int[] button6_txt 128
//!VAR int button6_len 0
//!VAR int[] button6_val 128
//!VAR int button6_val_len 0

//!VAR int[] button7_txt 128
//!VAR int button7_len 0
//!VAR int[] button7_val 128
//!VAR int button7_val_len 0

//!VAR int[] button8_txt 128
//!VAR int button8_len 0
//!VAR int[] button8_val 128
//!VAR int button8_val_len 0

//!VAR int[] button9_txt 128
//!VAR int button9_len 0
//!VAR int[] button9_val 128
//!VAR int button9_val_len 0

//!VAR int[] button10_txt 128
//!VAR int button10_len 0
//!VAR int[] button10_val 128
//!VAR int button10_val_len 0

//!VAR int[] button11_txt 128
//!VAR int button11_len 0
//!VAR int[] button11_val 128
//!VAR int button11_val_len 0

//!VAR int[] button12_txt 128
//!VAR int button12_len 0
//!VAR int[] button12_val 128
//!VAR int button12_val_len 0

//!VAR int[] button13_txt 128
//!VAR int button13_len 0
//!VAR int[] button13_val 128
//!VAR int button13_val_len 0

//!VAR int[] button14_txt 128
//!VAR int button14_len 0
//!VAR int[] button14_val 128
//!VAR int button14_val_len 0

//!VAR int[] button15_txt 128
//!VAR int button15_len 0
//!VAR int[] button15_val 128
//!VAR int button15_val_len 0

//!VAR int[] button16_txt 128
//!VAR int button16_len 0
//!VAR int[] button16_val 128
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
    vec2 line2_pos = vec2(
        OFFSET_X + (float(j) * BUTTON_W),
        OFFSET_Y + (float(i) * BUTTON_H) + FONT_H + 4.0
    );

    if (uv.y > line1_pos.y && uv.y < line1_pos.y + BUTTON_H) {
        int txt[128];
        int len;
        int val_txt[128];
        int val_len;
        switch (i * 4 + j) {
            case 0:
                txt = button1_txt;
                len = button1_len;
                val_txt = button1_val;
                val_len = button1_val_len;
                break;
            case 1:
                txt = button2_txt;
                len = button2_len;
                val_txt = button2_val;
                val_len = button2_val_len;
                break;
            case 2:
                txt = button3_txt;
                len = button3_len;
                val_txt = button3_val;
                val_len = button3_val_len;
                break;
            case 3:
                txt = button4_txt;
                len = button4_len;
                val_txt = button4_val;
                val_len = button4_val_len;
                break;
            case 4:
                txt = button5_txt;
                len = button5_len;
                val_txt = button5_val;
                val_len = button5_val_len;
                break;
            case 5:
                txt = button6_txt;
                len = button6_len;
                val_txt = button6_val;
                val_len = button6_val_len;
                break;
            case 6:
                txt = button7_txt;
                len = button7_len;
                val_txt = button7_val;
                val_len = button7_val_len;
                break;
            case 7:
                txt = button8_txt;
                len = button8_len;
                val_txt = button8_val;
                val_len = button8_val_len;
                break;
            case 8:
                txt = button9_txt;
                len = button9_len;
                val_txt = button9_val;
                val_len = button9_val_len;
                break;
            case 9:
                txt = button10_txt;
                len = button10_len;
                val_txt = button10_val;
                val_len = button10_val_len;
                break;
            case 10:
                txt = button11_txt;
                len = button11_len;
                val_txt = button11_val;
                val_len = button11_val_len;
                break;
            case 11:
                txt = button12_txt;
                len = button12_len;
                val_txt = button12_val;
                val_len = button12_val_len;
                break;
            case 12:
                txt = button13_txt;
                len = button13_len;
                val_txt = button13_val;
                val_len = button13_val_len;
                break;
            case 13:
                txt = button14_txt;
                len = button14_len;
                val_txt = button14_val;
                val_len = button14_val_len;
                break;
            case 14:
                txt = button15_txt;
                len = button15_len;
                val_txt = button15_val;
                val_len = button15_val_len;
                break;
            case 15:
                txt = button16_txt;
                len = button16_len;
                val_txt = button16_val;
                val_len = button16_val_len;
                break;
            default:
                txt = button1_txt;
                len = 0;
                val_txt = button1_val;
                val_len = 0;
        }
        if (len < 10) {
            for (int k = len; k < 10; k++) {
                txt[k] = 32; // space
            }
        } else {
            len = 10;
        }
        if (val_len < 12) {
            for (int k = val_len; k < 12; k++) {
                val_txt[k] = 32; // space
            }
        } else {
            val_len = 12;
        }

        if (uv.y < line2_pos.y) {
            color = draw_text(
                color,
                src_coord0 * iResolution.xy,
                line1_pos,
                iResolution.xy,
                txt,
                len
            );
        } else {
            color = draw_text(
                color,
                src_coord0 * iResolution.xy,
                line2_pos,
                iResolution.xy,
                val_txt,
                val_len
            );
        }
    }

   
}

 if (uv.x > OFFSET_X && 
     uv.x < OFFSET_X + AREA_W && 
     uv.y > OFFSET_Y + AREA_H &&
     uv.y < OFFSET_Y + AREA_H + FONT_H) {
    int txt[128];
    int len = 3;
    txt[0] = selected_button / 100 + 48;
    txt[1] = (selected_button / 10) % 10 + 48;
    txt[2] = selected_button % 10 + 48;
    color = draw_text(
        color,
        src_coord0 * iResolution.xy,
        vec2(OFFSET_X, OFFSET_Y + AREA_H),
        iResolution.xy,
        txt,
        len
    );
}