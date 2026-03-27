#include "utils.glsl"
#include "patch_rototrans.glsl"
#include "patch_mixer.glsl"
#include "patch_feedback.glsl"

//!VAR mat4x2 new_corners 0.0 0.0 1.0 0.0 0.0 1.0 1.0 1.0
//!VAR float scrolled_h 0.0
//!VAR float scrolled_v 0.0
//!VAR float shift_rh 0.0
//!VAR float shift_gh 0.0
//!VAR float shift_bh 0.0
//!VAR float shift_ah 0.0
//!VAR float shift_rv 0.0
//!VAR float shift_gv 0.0
//!VAR float shift_bv 0.0
//!VAR float shift_av 0.0
//!VAR float distort_dx 0.0
//!VAR float distort_dy 0.0
//!VAR float distort_level 0.2
//!VAR float warp_level 0.2
//!VAR uint distort_edge 0
//!VAR float feedback_rotation 0.0
//!VAR float luma_blur 0.0
//!VAR float luma_blur_enable 0.0
//!VAR float luma_point 0.0
//!VAR float blur 0.0
//!VAR float blur_enable 0.0

void pass0(out vec4 color) {
    // Scroll as needed
    vec2 base_coord = coord_wrap(vec2(src_uv.x + scrolled_h, src_uv.y + scrolled_v), true, true);

    //skew
    base_coord = skew3(base_coord, new_corners);
    vec4 base = vec4(0.0);
    base.r = patch_rototrans(
        base_coord.xy + vec2(shift_rh, shift_rv),
        src_tex1, // main tex
        src_tex4, // warp tex x
        src_tex5, // warp tex y
        0,
        0,
        0,
        warp_level,
        EDGE_MODE_BLANK
    ).r;
    base.g = patch_rototrans(
        base_coord.xy + vec2(shift_gh, shift_gv),
        src_tex1, // main tex
        src_tex4, // warp tex x
        src_tex5, // warp tex y
        0,
        0,
        0,
        warp_level,
        EDGE_MODE_BLANK
    ).g;
    base.b = patch_rototrans(
        base_coord.xy + vec2(shift_bh, shift_bv),
        src_tex1, // main tex
        src_tex4, // warp tex x
        src_tex5, // warp tex y
        0,
        0,
        0,
        warp_level,
        EDGE_MODE_BLANK
    ).b;
    base.a = patch_rototrans(
        base_coord.xy + vec2(shift_ah, shift_av),
        src_tex1, // main tex
        src_tex4, // warp tex x
        src_tex5, // warp tex y
        0,
        0,
        0,
        warp_level,
        EDGE_MODE_BLANK
    ).a;

    // color mix, basic transform of main image
    color = patch_mixer(base);

}

void pass1(out vec4 color) {
    vec4 c_0 = texture(pass_tex0, pass_uv0);
    //only for luma blur
    if (luma_blur_enable > 0.0) { //luma blur takes precedence over normal blur
        float luma = rgb2hsv(texture(pass_tex0, pass_uv0).rgb).z;
        if (luma >= luma_point) {
            color = vec4(hsv2rgb(vec3(0, 0, luma)), c_0.a);
        } else {
            color = vec4(0.0, 0.0, 0.0, c_0.a);
        } 
    } else { // normal blur or no blur
        color = c_0;
    }
}

#define SIGMA (float(radius)/2.0)
void pass2(out vec4 color) {
     if (blur_enable > 0.0 || luma_blur_enable > 0.0) {
        int radius = luma_blur_enable > 0.0 ? min(int(ceil(luma_blur)), 50) : min(int(ceil(blur)), 50);
        float sigma = SIGMA;
        vec2 base_px = pass_uv0.xy * iResolution.xy;
        // Pass 1: vertical blur
        vec3 accum = vec3(0.0);
        float total_weight = 0.0;
        for (int j = -radius; j <= radius; j++) {
            vec2 sample_xy = (base_px + vec2(0.0, float(j))) / iResolution.xy;
            vec3 s = handle_edge(pass_tex1, sample_xy, EDGE_MODE_BLANK).rgb;
            float w = exp(-float(j * j) / (2.0 * sigma * sigma));
            accum += s * w;
            total_weight += w;
        }
        //preserve alpha from pass1
        float alpha = texture(pass_tex1, pass_uv1).a;
        color = vec4(accum / max(total_weight, 0.001), alpha);
    } else {
        color = texture(pass_tex1, pass_uv1);
    }
}

void pass3(out vec4 color) {
    // Pass 3: vertical blur over the horizontal results in pass2
    if (blur_enable > 0.0 || luma_blur_enable > 0.0) {
        int radius = luma_blur_enable > 0.0 ? min(int(ceil(luma_blur)), 50) : min(int(ceil(blur)), 50);
        float sigma = SIGMA;
        vec2 base_px = pass_uv1.xy * iResolution.xy;
        // Pass 3: horizontal blur
        vec3 accum = vec3(0.0);
        float total_weight = 0.0;
        for (int i = -radius; i <= radius; i++) {
            vec2 sample_uv = (base_px + vec2(float(i), 0.0)) / iResolution.xy;
            vec3 s = handle_edge(pass_tex2, sample_uv, EDGE_MODE_BLANK).rgb;
            float w = exp(-float(i * i) / (2.0 * sigma * sigma));
            accum += s * w;
            total_weight += w;
        }
        //preserve alpha from pass 2
        float alpha = texture(pass_tex2, pass_uv2).a;
        color = vec4(accum / max(total_weight, 0.001), alpha);
    } else {
        color = texture(pass_tex2, pass_uv2);
    }
}

void pass4(out vec4 color) {
    // Pass 4: diagonal blur (45°) to round out the separable passes
    if (blur_enable > 0.0 || luma_blur_enable > 0.0) {
        int radius = luma_blur_enable > 0.0 ? min(int(ceil(luma_blur * 0.707)), 35) 
                        : min(int(ceil(blur * 0.707)), 35); // scaled by 1/sqrt(2)
        float sigma = SIGMA;
        vec2 base_px = pass_uv2.xy * iResolution.xy;
        vec3 accum = vec3(0.0);
        float total_weight = 0.0;
        for (int d = -radius; d <= radius; d++) {
            vec2 sample_uv = (base_px + vec2(float(d), float(d))) / iResolution.xy;
            vec3 s = handle_edge(pass_tex3, sample_uv, EDGE_MODE_BLANK).rgb;
            float w = exp(-float(d * d) / (2.0 * sigma * sigma));
            accum += s * w;
            total_weight += w;
        }
        //preserve alpha from pass 3
        float alpha = texture(pass_tex3, pass_uv3).a;
        color = vec4(accum / max(total_weight, 0.001), alpha);
    } else {
        color = texture(pass_tex3, pass_uv3);
    }
}

void pass5(out vec4 color) {
    // Pass 5: bloom - add blurred bright areas back to the original
    vec4 blurred = texture(pass_tex4, pass_uv4);
    vec4 original = texture(pass_tex0, pass_uv0);
    if (luma_blur_enable > 0.0) {
        // color = blend_by_mode(original, blurred, BLEND_ADDITION);
        vec3 hsv_blurred = rgb2hsv(blurred.rgb);
        vec3 hsv_original = rgb2hsv(original.rgb);
        color = vec4(hsv2rgb(vec3(
            hsv_original.x,
            hsv_original.y,
            min(hsv_original.z + hsv_blurred.z, 1.0))),
            original.a
        );
    } else {
        if (blur > 0.0) {
        color = blurred;
        } else {
            color = original;
        }
    }
}

void pass6(out vec4 color) {
    // Scroll as needed
    vec2 base_coord = coord_wrap(vec2(src_uv.x + scrolled_h, src_uv.y + scrolled_v), true, true);

    //skew
    base_coord = skew3(base_coord, new_corners);

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

    color = patch_feedback(texture(pass_tex5, pass_uv5), feedback);
}