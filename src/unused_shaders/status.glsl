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
    if (font_fantasy(uv, label_pos[i], label_txt, label_idx[i], label_len[i])) {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }
}

for (int i = 0; i < num_labels; i++) {
    vec2 pos = label_pos[i];
    pos.y += 100;
    if (font_8x8(uv, pos, label_txt, label_idx[i], label_len[i])) {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }
}
