#include "utils.glsl"

#define BPM (130.0 * 1.0)
//!VAR float user7 0.0
void pass0(out vec4 color) {
    vec2 uv = (src_uv - vec2(0.5)) * 1.0 * (1.01 - user7) + 0.5;

    float slice = BPM / 60.0 ;
    float bar = 4 * slice;

    if (beat4x4(0xF0F0, BPM, iTime)) {
        color = texture(src_tex0, uv);
    } else {
        color = texture(src_tex1, uv);
    } 
    
    vec4 other = vec4(0.0);
    if (beat4x4(0x8888, BPM, iTime)) {
        color = texture(src_tex2, uv);
    } else if (beat4x4(0x4444, BPM, iTime)) {
        color = texture(src_tex3, uv);
    } 

    color = blend_by_mode(color, other, BLEND_SCREEN);
}