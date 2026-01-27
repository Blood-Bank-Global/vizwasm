#define CUSTOM_FEEDBACK_TRANSFORM jam_feedback
//!VAR int usr_var 0
vec4 jam_feedback() {
    vec4 feedback = patch_rototrans(
        src_coord0.xy,
        src_tex0,
        src_tex2,
        src_tex3,
        feedback_rotation,
        distort_dx,
        distort_dy,
        distort_level,
        distort_edge
    );

    // vec4 feedback = texture(src_tex0, src_coord0.xy * 0.999);
    vec3 hsv = rgb2hsv(feedback.rgb);
    hsv[0] = fract(hsv[0] + 5.0/256.0);
    hsv[1] = 0.5;
    feedback.rgb = hsv2rgb(hsv);
    return feedback;
}