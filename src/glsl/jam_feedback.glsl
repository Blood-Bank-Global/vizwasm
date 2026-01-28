#define CUSTOM_FEEDBACK_TRANSFORM jam_feedback

vec4 jam_feedback(vec4 base) {
    // if (distance(base.rgb, vec3(0.0, 0.0, 0.0)) > 0.01 && base.a >= 1.0) {
    if (base.a >= 1.0) {
        return base;
    } 
    // else if (distance(base.rgb, vec3(0.0, 0.0, 0.0)) <= 0.01) {
    //     base.a = 0.0;
    // }

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

    if (true) {
        vec3 hsv = rgb2hsv(feedback.rgb);
        hsv[0] = fract(hsv[0] + 29.0/256.0);
        hsv[1] = 0.5;
        // hsv[2] += 0.01;
        feedback.rgb = hsv2rgb(hsv);
        feedback.a = 1.0;
    }
    
    if (false) { 
        if (distance(feedback.rgb, vec3(0.0, 0.0, 0.0)) <= 0.01) {
            feedback = vec4(0.0, 0.0, 0.0, 1.0);
        } else if (feedback.b < feedback.r) {
            feedback.b = 1.0;
            feedback.r = feedback.r * .8;
        } else {
            feedback.r = 1.0;
            feedback.b = feedback.r * .8;
        }
    }

    if (false) {
        vec3 hsv = rgb2hsv(feedback.rgb);
        float h = mod(hsv[0] * 360.0 + 5.0, 90.0);
        hsv[0] = h / 360.0;
        hsv[1] = 0.5;
        hsv[2] = 0.8;
        feedback.rgb = hsv2rgb(hsv);
        feedback.a = 1.0;
    }

    vec4 ret = blend_by_mode(feedback, base, BLEND_ALPHA);
    return ret;
}