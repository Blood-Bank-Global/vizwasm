#include "utils.glsl"
#include "patch_rototrans.glsl"
#include "patch_mixer.glsl"
#include "patch_feedback.glsl"

//!VAR mat4x2 new_corners 0.0 0.0 1.0 0.0 0.0 1.0 1.0 1.0
//!VAR float scrolled_h 0.0
//!VAR float scrolled_v 0.0
//!VAR float distort_dx 0.0
//!VAR float distort_dy 0.0
//!VAR float distort_level 0.2
//!VAR float warp_level 0.2
//!VAR uint distort_edge 0
//!VAR float feedback_rotation 0.0
//!VAR float luma_blur 0.0
//!VAR float bloom 0.0

void main_frag(out vec4 color) {
    // Scroll as needed
    vec2 base_coord = coord_wrap(vec2(src_coord.x + scrolled_h, src_coord.y + scrolled_v), true, true);

    //skew
    base_coord = skew3(base_coord, new_corners);

    vec4 base = patch_rototrans(
        base_coord.xy,
        src_tex1, // main tex
        src_tex4, // warp tex x
        src_tex5, // warp tex y
        0,
        0,
        0,
        warp_level,
        EDGE_MODE_BLANK
    ); 

    //luma blur from the next N pixels, where N is determined by the luma_blur setting.
    // incorporate the luma of each neighbor pixel into the weight so that the brighter 
    // pixels contribute more to the blur, and the darker pixels contribute less. This 
    if (luma_blur > 0.0) {
        int radius = min(int(ceil(luma_blur)), 50);
        float sigma = float(radius) * 0.5;
        vec2 base_px = base_coord.xy * iResolution.xy;
        // Pass 1: horizontal blur for each row, store intermediate results
        vec3 row_color[2501];
        for (int j = -radius; j <= radius; j++) {
            vec3 h_accum = vec3(0.0);
            float h_weight = 0.0;
            for (int i = -radius; i <= radius; i++) {
                float dist = distance(vec2(i, j), vec2(0.0));
                if (dist > float(radius)) {
                    continue; // skip pixels outside the circular radius
                }
                vec2 sample_uv = (base_px + vec2(float(i), float(j))) / iResolution.xy;
                vec3 s = patch_rototrans(
                    sample_uv, src_tex1, src_tex4, src_tex5,
                    0, 0, 0, warp_level, EDGE_MODE_BLANK
                ).rgb;
                float luma = rgb2hsv(s).z;
                float g = exp(-float(i * i) / (2.0 * sigma * sigma));
                float w = g * luma;
                h_accum += s * w;
                h_weight += w;
                
            }
            row_color[j + radius] = h_accum / max(h_weight, 0.001);
        }

        // Pass 2: vertical blur over the horizontal results
        vec3 accum = vec3(0.0);
        float total_weight = 0.0;
        for (int j = -radius; j <= radius; j++) {
            vec3 rc = row_color[j + radius];
            float luma = rgb2hsv(rc).z;
            float g = exp(-float(j * j) / (2.0 * sigma * sigma));
            float w = g * luma;
            accum += rc * w;
            total_weight += w;
        }

        base = vec4(accum / max(total_weight, 0.001), base.a);
    } else {
        base = texture(src_tex1, base_coord);
    }

    // color mix, basic transform of main image
    color = patch_mixer(base);

    // distort feedback
    vec4 feedback = patch_rototrans(
        base_coord.xy,
        src_tex0, // feedback
        src_tex2, // distort tex x
        src_tex3, // distort tex y
        feedback_rotation,
        distort_dx,
        distort_dy,
        distort_level,
        distort_edge
    );

    color = patch_feedback(color, feedback);
}