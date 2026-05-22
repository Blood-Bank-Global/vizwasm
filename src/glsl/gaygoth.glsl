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

#define BPM 155.0
void pass0(out vec4 color) {
    if (beat4x4(0x8888, BPM*2, iTime)) {
        color = texture(src_tex2, src_uv);
    } else if (beat4x4(0xCCCC, BPM, iTime)) {
        color = texture(src_tex0, src_uv);
    } else {
        color = texture(src_tex1, src_uv);
    }

    if (beat4x4(0x8000, BPM/2.0, iTime)) {
        color = blend_by_mode(
            color,
            texture(src_tex3, src_uv),
            BLEND_SCREEN
        );
    }
}


// quad / mirror layer
// void zzpass1(out vec4 color) {
    
//     if (beat4x4(0x4040, BPM, iTime)) {
//         // quad
//         color = texture(pass_tex0, coord_mirror(src_uv + vec2(0.5), true, true));
//         return;
//     }

//     if (beat4x4(0x0404, BPM, iTime)) {
//         // mirror
//         color = texture(pass_tex0, coord_mirror(src_uv + vec2(0.5, 0.0), true, false));
//         return;
//     }

//     color = texture(pass_tex0, src_uv);
// }

// monochrome layer
// void zzpass1(out vec4 color) {
//     // color = vec4(vec3(rgb2hsv(texture(pass_tex0, src_uv).rgb).z), 1.0);
//     color = texture(pass_tex0, src_uv);
//     vec3 hsv = rgb2hsv(color.rgb);
//     if ((hsv.x <= 0.1 || hsv.x >= 0.9) && hsv.s > 0.75) {
//         color.gb = vec2(0.0);
//     } else {
//         color.rgb = vec3(0.0);
//     }
// }

// halftone layer
// void pass1(out vec4 color) {
//     if (beat4x4(0x4444, BPM, iTime)) {
//         patch_halftone45(color, src_uv, iResolution.xy, pass_tex0, 8.0, 8.0);
//         return;
//     }
//     color = texture(pass_tex0, src_uv);
// }

// WARP
// void pass1(out vec4 color) {
//     if (beat4x4(0xf0f0, BPM * floor(8.0 * user0), iTime)) {
//         vec2 coord = src_uv * iResolution.xy;
//         float warp_strength = 4.0;
//         coord = patch_warp_px(coord, vec2(40.0), warp_strength, iResolution.xy, iTime);
//         color = texture(pass_tex0, coord/iResolution.xy);
//         return;
//     }
//     color = texture(pass_tex0, src_uv);
// }

// BLOB
// void pass1(out vec4 color) {
    
//     vec4 color_blob = texture(pass_tex0, src_uv);
//     color_blob.rgb = floor(color_blob.rgb * vec3(8.0)) / 8.0; // posterize to reduce number of blobs and create sharper edges 
//     vec4 color_bg = color_blob;
//     color_bg.rgb = vec3(1.0) - color_bg.rgb;
//     float radius = 400 + 180 * sin(iTime*2.0);
//     color = patch_blob_px(
//         src_uv * iResolution.xy,
//         iResolution.xy,
//         color_bg,
//         color_blob,
//         vec2(320,240),
//         radius,
//         iTime*10.0
//     );
// }

// void pass2(out vec4 color) {
//    patch_ordered_dither4x4(color, src_uv, iResolution.xy, pass_tex1);
// }

// DRIP
// void zzpass6(out vec4 color) {
//     if (true) {
//         color = texture(pass_tex5, src_uv);
//         return;
//     }
//     float drip_strength = 32.0;
//     float drip_cutoff = 200.0 + 400.0 * user3;
//     patch_drippy_px(
//         color, 
//         pass_tex5, 
//         src_uv, 
//         iResolution.xy, 
//         vec2(drip_strength), 
//         vec2(8.0), 
//         drip_cutoff,
//         iTime
//     );    
// }