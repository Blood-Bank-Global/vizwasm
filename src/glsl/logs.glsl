//!VAR int[] txt 0 0
//!VAR int[] line_start 0 0
//!VAR int[] line_end 0 0

#define SCALE 1.0
#define FONT_BITS_W 8.0
#define FONT_BITS_H 8.0
#define FONT_W (FONT_BITS_W*SCALE)
#define FONT_H (FONT_BITS_H*SCALE)
#define BOTTOM_MARGIN 32.0
#define NUM_LINES (int((iResolution.y - (BOTTOM_MARGIN)) / FONT_H))
#define LINE_OFFSET (int((100 - NUM_LINES)))

color = vec4(0.0, 0.0, 0.0, 1.0);

vec2 uv = src_coord.xy * iResolution.xy;
uv.y = uv.y + BOTTOM_MARGIN;

int n = int(uv.y / (FONT_H));
int m = int(uv.x / (FONT_W));
if (m < (line_end[n + LINE_OFFSET] - line_start[n + LINE_OFFSET]) && n < NUM_LINES) {
    int char_idx = line_start[n + LINE_OFFSET] + m;
    int ch = txt[char_idx];
    // color = char8x16(color, uv, ivec2(m, n), SCALE, ch);
    color = char8x8(color, uv, ivec2(m, n), SCALE, ch);
}