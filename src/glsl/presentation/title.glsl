#include "utils.glsl"
#include "font_willow.glsl"

//!STR title1 "A Labyrinthine Framework"
//!STR title2 "for Live Coding"
//!STR subtitle "Vampire Exec"

#define TITLE_POS vec2(20.0, 20.0)
#define SUBTITLE_POS (vec2(320, 240) - vec2((subtitle_length + 3.0) * float(font_willow_width),float(font_willow_height) * 3.0))
void pass0(out vec4 color) {
    color = texture(src_tex0, src_uv0);
    vec2 coord = src_uv.xy/2 * iResolution.xy;

    if (font_willow(coord, TITLE_POS, title1, 0, title1_length)
        || font_willow(coord, TITLE_POS + vec2(0.0, float(font_willow_height)), title2, 0, title2_length)
        || font_willow(coord, SUBTITLE_POS, subtitle, 0, subtitle_length)) {
        color = vec4(1.0);
    }
}
