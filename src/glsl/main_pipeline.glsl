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
//!VAR float blur 0.0

void pass0(out vec4 color) {
    // Scroll as needed
    vec2 base_coord = coord_wrap(vec2(src_coord.x + scrolled_h, src_coord.y + scrolled_v), true, true);

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
    vec4 c_0 = texture(pass_tex0, pass_coord0);
    vec3 hsv = rgb2hsv(c_0.rgb);
    if (hsv.z >= luma_blur) {
        color = c_0;
    } else {
        color = vec4(0.0, 0.0, 0.0, c_0.a);
    }
}

void pass2(out vec4 color) {
     if (blur > 0.0) {
        int radius = min(int(ceil(blur)), 50);
        float sigma = float(radius) * 0.5;
        vec2 base_px = pass_coord0.xy * iResolution.xy;
        // Pass 1: vertical blur
        vec3 accum = vec3(0.0);
        float total_weight = 0.0;
        for (int j = -radius; j <= radius; j++) {
            vec2 sample_uv = (base_px + vec2(0.0, float(j))) / iResolution.xy;
            vec3 s = handle_edge(pass_tex0, sample_uv, EDGE_MODE_BLANK).rgb;
            float w = exp(-float(j * j) / (2.0 * sigma * sigma));
            accum += s * w;
            total_weight += w;
        }
        //preserve alpha from pass1
        float alpha = texture(pass_tex1, pass_coord1).a;
        color = vec4(accum / max(total_weight, 0.001), alpha);
    
    } else {
        color = texture(pass_tex1, pass_coord1);
    }
}

void pass3(out vec4 color) {
    // Pass 3: vertical blur over the horizontal results in pass2
    if (blur > 0.0) {
        int radius = min(int(ceil(blur)), 50);
        float sigma = float(radius) * 0.5;
        vec2 base_px = pass_coord1.xy * iResolution.xy;
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
        float alpha = texture(pass_tex2, pass_coord2).a;
        color = vec4(accum / max(total_weight, 0.001), alpha);
    } else {
        color = texture(pass_tex2, pass_coord2);
    }
}

void pass4(out vec4 color) {
    // Pass 4: diagonal blur (45°) to round out the separable passes
    if (blur > 0.0) {
        int radius = min(int(ceil(blur * 0.707)), 35); // scaled by 1/sqrt(2)
        float sigma = float(radius) * 0.5;
        vec2 base_px = pass_coord2.xy * iResolution.xy;
        vec3 accum = vec3(0.0);
        float total_weight = 0.0;
        for (int d = -radius; d <= radius; d++) {
            vec2 sample_uv = (base_px + vec2(float(d), float(d))) / iResolution.xy;
            vec3 s = handle_edge(pass_tex2, sample_uv, EDGE_MODE_BLANK).rgb;
            float w = exp(-float(d * d) / (2.0 * sigma * sigma));
            accum += s * w;
            total_weight += w;
        }
        //preserve alpha from pass 3
        float alpha = texture(pass_tex3, pass_coord3).a;
        color = vec4(accum / max(total_weight, 0.001), alpha);
    } else {
        color = texture(pass_tex3, pass_coord3);
    }
}

void pass5(out vec4 color) {
    // Pass 5: bloom - add blurred bright areas back to the original
    vec4 blurred = texture(pass_tex4, pass_coord4);
    vec4 original = texture(pass_tex0, pass_coord0);
    if (luma_blur > 0.0) {
        color = blend_by_mode(original, blurred*0.7, BLEND_SCREEN);
    } else {
        color = blurred;
    }
}

void pass6(out vec4 color) {
    // Scroll as needed
    vec2 base_coord = coord_wrap(vec2(src_coord.x + scrolled_h, src_coord.y + scrolled_v), true, true);

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

    color = patch_feedback(texture(pass_tex5, pass_coord5), feedback);
}