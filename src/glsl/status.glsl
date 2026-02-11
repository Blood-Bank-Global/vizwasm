//!VAR int num_labels 0
//!VAR vec2[] label_pos
//!VAR int[] label_idx
//!VAR int[] label_len
//!VAR int[] label_txt

#define FONT_W 8.0
#define FONT_H 16.0

vec2 uv = src_coord.xy * iResolution.xy;
color = vec4(0.0, 0.0, 0.0, 1.0);
if (fract(uv.x/10) < 0.5) {
    color = vec4(0.3, 0.3, 0.3, 1.0);
}

for (int i = 0; i < num_labels; i++) {
    vec2 pos = label_pos[i];
    int idx = label_idx[i];
    int len = label_len[i];
    if (uv.x > pos.x && uv.x < pos.x + float(len) * FONT_W &&
        uv.y > pos.y && uv.y < pos.y + FONT_H) {
        int m = int((uv.x - pos.x) / FONT_W);
        int char = label_txt[idx + m];
        color = char8x16(color, uv, pos + vec2(float(m) * FONT_W, 0.0), 1.0, char);
    }
}