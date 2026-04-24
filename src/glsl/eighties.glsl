#include "utils.glsl"
#include "font_8x16.glsl"
//!VAR float cc_iac_driver_bus_1_1_1 0.0

#define REGION (vec2(font_8x16_width*2, font_8x16_height*2))
#define DENSITY 10.0
float get_range(float i, float j, uint seed) {
    float f = abs(randf(seed ^ uint(0xDEAD * i) ^ uint(0xBEEF * j)));
    float sk = randf(seed ^ uint(0xDEED * i) ^ uint(0xF00D * j));
    float sz = floor(f < DENSITY ? iResolution.x/REGION.x/2.0 * (1.0 + sk): 0.0);
    float off = randf(seed ^ uint(0xFEED * i) ^ uint(0xC0DE * j));
    float sp = randf(seed ^ uint(0xBAAD * i) ^ uint(0xD00D * j));
    float t = sin(fract(iTime/(450.0 + 100.0 * sp)) * 2.0 * M_PI + off * 2.0 * M_PI);
    return smoothstep(0.99997, 1.0, t) * sz;
}

void pass0(out vec4 color) {
    vec2 coord = src_uv.xy * iResolution.xy;
    vec2 block = floor(vec2(coord.x/REGION.x, coord.y/REGION.y));
    color.a = 1.0;

    bool is_covered = false;
    
    for (uint i = 0; i < iResolution.x/REGION.x; i++) {
            float curr = get_range(float(i), block.y, 0xF3DABABEu);
            is_covered = is_covered || (float(i) <= block.x && step(curr, block.x - float(i)) < 0.5);
    }
    if (is_covered) {
        color = vec4(1.0);
    } else {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }
}

void pass1(out vec4 color) {

    color = texture(src_tex0, src_uv);

    if (cc_iac_driver_bus_1_1_1 > 10.0) {
        color.gb =  vec2(0.0);
    }    

    vec2 title_uv = src_uv.xy;
    vec4 title_color = texture(src_tex1, title_uv);


    float t = iTime*2; // 2 Hz flicker
    // strobe_bits need to be in hexadecimal
    uint strobe_bits[] = uint[](
        0x4F, 0x00, 0xDD, 0xF7,
         0x48, 0x7C, 0x29, 0xF0,
         0xDD, 0x1B, 0xFC, 0x3F,
         0x7C, 0x29, 0xF0, 0xDD,
         0x1B, 0xFC, 0x3F, 0x7C,
         0x29, 0xF0, 0xDD, 0x1B,
         0xFC, 0x3F, 0x7C, 0x29);
    uint strobe_pattern = strobe_bits[uint(t) % strobe_bits.length()];
    uint strobe_bit = (strobe_pattern >> uint(t * 8.0)) & 1u;
    vec3 title_color_hsv = rgb2hsv(title_color.rgb);
    if (strobe_bit == 1u && title_color_hsv.z < 0.25) {
        title_color.rgb = (color.rgb + title_color.rgb - vec3(1.0));
        title_color.rgb *= title_color.rgb;
        title_color.rgb = sqrt(title_color.rgb);
        color = title_color;
        color.a = 1.0;
    }

//!STR german "vorsicht+ueberraschung|panick*freiheit!zukunft"
    if (texture(pass_tex0, src_uv).r > 0.5) {
        vec2 coord = src_uv.xy * iResolution.xy;
        vec2 rc = floor(coord/REGION); 
       
        uint c[] = uint[](german[uint(mod(rc.y + rc.x, german.length()))]);
        if (abs(randf(0xd00d ^ uint(rc.y * 0xf00d) ^ uint(rc.x * 0xb00d))) > 0.5) {
            c[0] = uint(
                abs(randf(uint(0xffff * fract(iTime)) ^ uint(rc.x * 0xbabe) ^ uint(rc.y * 0xbead))) * 255.0
            );
        }
        if (font_8x16(coord/2.0, rc * REGION/2.0, c, 0, 1)) {
            color = vec4(vec3(0.7, 0.3, 0.0), 1.0);
        } 
        else {
            color.rgb = vec3(1.0) - color.rgb;//vec4(vec3(0.0), 1.0);
        }
    }
}