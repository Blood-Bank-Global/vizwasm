//!VAR int[] txt 2500
//!VAR int[] line_start 30
//!VAR int[] line_end 30

color = vec4(0.0, 0.0, 0.0, 1.0);

vec2 uv = src_coord.xy * iResolution.xy;

int n = int(uv.y / 16.0);
int m = int(uv.x / 8.0);
if (m < (line_end[n] - line_start[n]) && n < 30) {
    int char_idx = line_start[n] + m;
    int ch = txt[char_idx];
    color = draw_char(color, uv, vec2(m * 8.0, n * 16.0), iResolution.xy, ch);
}