#include "font_8x14.glsl"
#include "font_8x16.glsl"
#include "font_8x8.glsl"
#include "font_ample.glsl"
#include "font_arcade.glsl"
#include "font_bubbly.glsl"
#include "font_cyber.glsl"
#include "font_fancy.glsl"
#include "font_fantasy.glsl"
#include "font_future.glsl"
#include "font_high.glsl"
#include "font_logic.glsl"
#include "font_nobel.glsl"
#include "font_retro.glsl"
#include "font_small.glsl"
#include "font_strong.glsl"
#include "font_typical.glsl"
#include "font_willow.glsl"
#include "font_berkeley_mono.glsl"
#include "utils.glsl"

//!STR demo "The quick brown fox jumps over the lazy dog"

#define font_offset_8x8 (0)
#define font_offset_8x16 (font_offset_8x8 + font_8x8_height)
#define font_offset_ample (font_offset_8x16 + font_8x16_height)
#define font_offset_arcade (font_offset_ample + font_ample_height)
#define font_offset_bubbly (font_offset_arcade + font_arcade_height)
#define font_offset_fantasy (font_offset_bubbly + font_bubbly_height)
#define font_offset_typical (font_offset_fantasy + font_fantasy_height)
#define font_offset_future (font_offset_typical + font_typical_height)
#define font_offset_fancy (font_offset_future + font_future_height)
#define font_offset_willow (font_offset_fancy + font_fancy_height)
#define font_offset_strong (font_offset_willow + font_willow_height)
#define font_offset_nobel (font_offset_strong + font_strong_height)
#define font_offset_high (font_offset_nobel + font_nobel_height)
#define font_offset_logic (font_offset_high + font_high_height)
#define font_offset_cyber (font_offset_logic + font_logic_height)
#define font_offset_small (font_offset_cyber + font_cyber_height)
#define font_offset_8x14 (font_offset_small + font_small_height)
#define font_offset_berkeley_mono (font_offset_8x14 + font_8x14_height)

void pass0(out vec4 color) {
    color = vec4(0.0, 0.0, 0.0, 1.0);
    uint starts[1] = uint[1](0);
    uint lengths[1] = uint[1](demo_length);

    vec2 uv = iResolution.xy * src_coord.xy + vec2(0.0, 0.0);

    #define demo_a_font(name) \
        if (multiline_##name( \
            uv, \
            vec2(0.0, float( font_offset_##name )), \
            demo, \
            starts, \
            lengths \
        )) { \
            color = vec4(1.0, 1.0, 1.0, 1.0); \
        }


    demo_a_font(8x8)
    demo_a_font(8x16)
    demo_a_font(ample)
    demo_a_font(arcade)
    demo_a_font(bubbly)
    demo_a_font(fantasy)
    demo_a_font(typical)
    demo_a_font(future)
    demo_a_font(fancy)
    demo_a_font(willow)
    demo_a_font(strong)
    demo_a_font(nobel)
    demo_a_font(high)
    demo_a_font(logic)
    demo_a_font(cyber)
    demo_a_font(small)
    demo_a_font(8x14)
    demo_a_font(berkeley_mono)
}
