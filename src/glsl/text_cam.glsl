#include "utils.glsl"
#include "font_cyber.glsl"

//!VAR vec3 iResolution0 0.0 0.0 0.0
//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!STR words "ERROR WARNING ALERT DANGER CRITICAL ALARM BEEP OVERLOAD"

void pass0(out vec4 color) {
    vec2 uv = src_coord.xy * iResolution.xy;

    float resize = iResolution0.y / iResolution.y;
    float offx = (iResolution0.x / resize - iResolution.x)/2.0;
    color = texture(src_tex0, (uv / iResolution0.xy) * resize + vec2(offx, 0.0) / iResolution.xy);


    uint lens[8] = uint[8](5, 7, 5, 6, 8, 5, 4, 8);
    uint starts[8] = uint[8](0, 6, 14, 20, 27, 36, 42, 47);

    float scale = 0.85;
    uv *= scale;
    uv += vec2(-60.0, -80.0);

    // uint char = uint(mod(iTime*20, words.length() ));
    uint char = uint(cc_iac_driver_bus_1_0_1/127.0 * float(words.length()));
    uint cx = 0;
    uint cy = 0;
    for (uint i = 0u; i < char; i++) {
        if (cx >= lens[cy]) {
            cx = 0;
            cy++;
        }
        cx++;
    }

    if (((uint(uv.x) / uint(font_cyber_width) < cx && uint(uv.y) / uint(font_cyber_height) == cy))
        || (uint(uv.y) / uint(font_cyber_height) < cy)) {
        if (multiline_cyber(uv, vec2(0.0, 0.0), words, starts, lens)) {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    }
}