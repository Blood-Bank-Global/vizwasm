#define FEEDBACK_BASIC 0
#define FEEDBACK_JAM 1

vec4 patch_feedback_basic(in vec4 base, in vec4 feedback) {
    // underlay feedback
    return blend_by_mode(feedback, base, BLEND_ALPHA);
}

vec4 patch_feedback_jam(in vec4 base, in vec4 feedback) {
    vec3 hsv = rgb2hsv(feedback.rgb);
    hsv[0] = fract(hsv[0] + 7.0/256.0);
    hsv[1] = 0.5;
    // hsv[2] += 0.01;
    feedback.rgb = hsv2rgb(hsv);
    feedback.a = 1.0;

    return blend_by_mode(feedback, base, BLEND_ALPHA);
}



vec4 patch_feedback(in vec4 base) {
    if (base.a >= 1.0) {
        return base;
    } 

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

    switch (feedback_mode_selected) {
        case FEEDBACK_BASIC:
            // default feedback
            return patch_feedback_basic(base, feedback);
        case FEEDBACK_JAM:
            // jam feedback
            return patch_feedback_jam(base, feedback);
        default:
            return base;
    }
}
