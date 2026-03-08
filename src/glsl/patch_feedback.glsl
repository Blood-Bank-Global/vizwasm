#define FEEDBACK_BASIC 0
#define FEEDBACK_JAM 1
#define FEEDBACK_MATH 2
#define FEEDBACK_XOR 3
#define FEEDBACK_SEA 4
#define FEEDBACK_ALIEN 5

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


vec4 patch_feedback_math(in vec4 base, in vec4 feedback) {
    vec3 hsv = rgb2hsv(feedback.rgb);
    hsv[0] = mod(hsv[0] * 360.0 + 17.0, 45.0);
    hsv[1] = 0.5;
    feedback.rgb = hsv2rgb(hsv);
    feedback.a = 1.0;

    return blend_by_mode(feedback, base, BLEND_ALPHA);
}

vec4 patch_feedback_xor(in vec4 base, in vec4 feedback) {
    vec3 hsv = rgb2hsv(feedback.rgb);
    if (hsv[1] > 0.2) {
        hsv[0] = mod(hsv[0] * 360.0 + 180.0, 360.0)/360.0;
        hsv[1] = 1.0;
    } else {
        hsv[1] = 0.0;
        hsv[2] = 0.0;
    }
    feedback.rgb = hsv2rgb(hsv);
    return blend_by_mode(feedback, base, BLEND_ALPHA);
}

vec4 patch_feedback_sea(in vec4 base, in vec4 feedback) {
    vec3 yuv = rgb2yuv_bt709(feedback.rgb);
    yuv.y = max(0.25, mod(yuv.y + 0.015, 0.75));
    yuv.x = 0.4 + 0.6 * ((1.0 - yuv.y - 0.25)/0.5);
    yuv.z = (1.0 - yuv.y);
    feedback.rgb = yuv2rgb_bt709(yuv);
    return blend_by_mode(feedback, base, BLEND_ALPHA);
}

vec4 patch_feedback_alien(in vec4 base, in vec4 feedback) {
    vec3 yuv = rgb2yuv_bt709(feedback.rgb);
    yuv.y = max(0.25, mod(yuv.y + 0.01, 0.75));
    yuv.x = 0.3 + 0.4 * ((1.0 - yuv.y - 0.25)/0.5);
    yuv.z = yuv.y;
    feedback.rgb = yuv2rgb_bt709(yuv);
    return blend_by_mode(feedback, base, BLEND_ALPHA);
}

vec4 patch_feedback(in vec4 base, in vec4 feedback) {
    if (base.a >= 1.0) {
        return base;
    } 

    switch (feedback_mode_selected) {
        case FEEDBACK_BASIC:
            // default feedback
            return patch_feedback_basic(base, feedback);
        case FEEDBACK_JAM:
            // jam feedback
            return patch_feedback_jam(base, feedback);
        case FEEDBACK_MATH:
            // math feedback
            return patch_feedback_math(base, feedback);
        case FEEDBACK_XOR:
            // xor feedback
            return patch_feedback_xor(base, feedback);
        case FEEDBACK_SEA:
            // sea feedback
            return patch_feedback_sea(base, feedback);
        case FEEDBACK_ALIEN:
            // alien feedback
            return patch_feedback_alien(base, feedback);
        default:
            return vec4(0, 0.5, 0, 1.0);
    }
}
