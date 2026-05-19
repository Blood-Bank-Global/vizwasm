#include "utils.glsl"
#include "patch_halftone.glsl"
#include "patch_warp_px.glsl"
#include "patch_drippy_px.glsl"
#include "patch_pixelate.glsl"
#include "patch_blob_px.glsl"

#include "font_8x16.glsl"
//!VAR vec2 iResolution0 0.0 0.0

//!VAR float user0 0.0
//!VAR float user1 0.0
//!VAR float user2 0.0
//!VAR float user3 0.0
//!VAR float user4 0.0
//!VAR float user5 0.0
//!VAR float user6 0.0
//!VAR float user7 0.0

#define user_p(i, seq) (user##i > abs(randf(uint((seq))^0xDEADBEEF ^ (0xCAFE * uint((i))))))

#define NEIGHBORHOOD_SIZE 8.0


// patten is a 16-bit integer, where each bit represents a 16th note in a 4/4 measure (16 steps total).
bool beat4x4(uint pattern, float bpm, float t) {
    float beat_duration = 60.0 / bpm;
    int step = 0;
    return (pattern & (1 << step)) != 0;
}

// ZOOM
//!STR foo "hello world"
void pass0(out vec4 color) {
    float scale = iResolution.y / iResolution0.y;
    float x_offset = (iResolution.x - iResolution0.x * scale) * 0.5;
    vec2 coord0 = src_uv.xy * iResolution.xy - vec2(x_offset, 0.0);
    vec2 uv0 = coord0 / (scale * iResolution0.xy);
    color = texture(
        src_tex0,
        src_uv0 * mix(0.1, 1.0, 1.0 - user7)
        + vec2(0.5) * (1.0 - mix(0.1, 1.0, 1.0 - user7)));
}
// quad / mirror layer
void pass1(out vec4 color) {
    if (false) {
        color = texture(pass_tex0, src_uv);
        return;
    }

    if (true) {
        // quad
        color = texture(pass_tex0, coord_mirror(src_uv + vec2(0.5), true, true));
        return;
    }

    // mirror
    color = texture(pass_tex0, coord_mirror(src_uv + vec2(0.5, 0.0), true, false));
}

// monochrome layer
void pass2(out vec4 color) {
    color = vec4(vec3(rgb2hsv(texture(pass_tex1, src_uv).rgb).z), 1.0);
}


// halftone layer
void pass3(out vec4 color) {
    if (true) {
        color = texture(pass_tex1, src_uv);
        return;
    }
    patch_halftone45(color, src_uv, iResolution.xy, pass_tex2, 8.0, 8.0);
}



// WARP
void pass4(out vec4 color) {
    if (user4 > 0.5) {
        color = texture(pass_tex3, src_uv);
        return;
    }
    vec2 coord = src_uv * iResolution.xy;
    float warp_strength = 2.0;
    coord = patch_warp_px(coord, vec2(10.0), warp_strength, iResolution.xy, iTime);
    color = texture(pass_tex3, coord/iResolution.xy);
}

// BLOB
void pass5(out vec4 color) {
    if (true) {
        color = texture(pass_tex4, src_uv);
        return;
    }
    vec4 color_blob = texture(pass_tex4, src_uv);
    float radius = 300 + 180 * sin(iTime*2.0);
    color = patch_blob_px(
        src_uv * iResolution.xy,
        iResolution.xy,
        vec4(0.0, 0.0, 0.0, 0.0),
        color_blob,
        vec2(320,240),
        radius,
        iTime
    );
    
}

// DRIP
void pass6(out vec4 color) {
    if (true) {
        color = texture(pass_tex5, src_uv);
        return;
    }
    float drip_strength = 32.0;
    float drip_cutoff = 200.0 + 400.0 * user3;
    patch_drippy_px(
        color, 
        pass_tex5, 
        src_uv, 
        iResolution.xy, 
        vec2(drip_strength), 
        vec2(8.0), 
        drip_cutoff,
        iTime
    );    
}
