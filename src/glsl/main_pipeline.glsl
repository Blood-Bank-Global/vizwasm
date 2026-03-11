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