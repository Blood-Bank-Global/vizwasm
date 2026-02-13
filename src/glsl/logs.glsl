//!VAR int[] txt 0 0
//!VAR int[] line_start 0 0
//!VAR int[] line_end 0 0

#define SCALE 1.0
#define FONT_BITS_W 8.0
#define FONT_BITS_H 16.0
#define FONT_W (FONT_BITS_W*SCALE)
#define FONT_H (FONT_BITS_H*SCALE)
#define BOTTOM_MARGIN 32.0
#define NUM_LINES (int((iResolution.y - (BOTTOM_MARGIN)) / FONT_H))
#define LINE_OFFSET (int((100 - NUM_LINES)))

color = vec4(0.0, 0.0, 0.0, 1.0);

vec2 uv = src_coord.xy * iResolution.xy;
uv.y = uv.y + BOTTOM_MARGIN;

int n = int(uv.y / (FONT_H));
if (n < NUM_LINES) {
    int len = line_end[n + LINE_OFFSET] - line_start[n + LINE_OFFSET];
    if (font_8x16(uv, vec2(0.0, float(n) * FONT_H), txt, line_start[n + LINE_OFFSET], len)) {
        color = vec4(0.7, 0.7, 0.7, 1.0);
    }
}