//!VAR float mix_rr 1.0
//!VAR float mix_rg 0.0
//!VAR float mix_rb 0.0
//!VAR float mix_ra 0.0
//!VAR float mix_gr 0.0
//!VAR float mix_gg 1.0
//!VAR float mix_gb 0.0
//!VAR float mix_ga 0.0
//!VAR float mix_br 0.0
//!VAR float mix_bg 0.0
//!VAR float mix_bb 1.0
//!VAR float mix_ba 0.0
//!VAR float mix_ar 0.0
//!VAR float mix_ag 0.0
//!VAR float mix_ab 0.0
//!VAR float mix_aa 1.0
//!VAR float thresh 0.0
//!VAR float shift_rh 0.0
//!VAR float shift_gh 0.0
//!VAR float shift_bh 0.0
//!VAR float shift_ah 0.0
//!VAR float shift_rv 0.0
//!VAR float shift_gv 0.0
//!VAR float shift_bv 0.0
//!VAR float shift_av 0.0
//!VAR float boost 0.0
//!VAR uint color_key_enable 0
//!VAR float color_key_sim 0.001
//!VAR float color_key_blend 0.0
//!VAR uint negate 0
//!VAR float distort_dx 0.0
//!VAR float distort_dy 0.0
//!VAR float distort_level 0.2
//!VAR float warp_level 0.2
//!VAR uint distort_edge 0
//!VAR float feedback_rotation 0.0
//!VAR mat4x2 new_corners 0.0 0.0 1.0 0.0 0.0 1.0 1.0 1.0
//!VAR float scrolled_h 0.0
//!VAR float scrolled_v 0.0
//!VAR uint flash_enable 0
//!VAR int usr_var 0

vec2 base_coord = src_coord0.xy;

// Scroll as needed
base_coord = coord_wrap(vec2(base_coord.x + scrolled_h, base_coord.y + scrolled_v), true, true);

//skew
base_coord = skew3(base_coord, new_corners);

//Shift colors
vec4 warp_dx = vec4(base_coord.x) + vec4(distort(base_coord, src_tex4, warp_level), 0.0);
vec4 warp_dy = vec4(base_coord.y) + vec4(distort(base_coord, src_tex5, warp_level), 0.0);

mat4x2 warp_matrix = mat4x2(
    warp_dx[0] - shift_rh, warp_dy[0] - shift_rv,
    warp_dx[1] - shift_gh, warp_dy[1] - shift_gv,
    warp_dx[2] - shift_bh, warp_dy[2] - shift_bv,
    warp_dx[3] - shift_ah, warp_dy[3] - shift_av
);

vec4 base = vec4(handle_edge(src_tex1, warp_matrix[0], distort_edge).r,
                 handle_edge(src_tex1, warp_matrix[1], distort_edge).g,
                 handle_edge(src_tex1, warp_matrix[2], distort_edge).b,
                 texture(src_tex1, warp_matrix[3]).a);

if (negate > 0) {
    base.rgb = vec3(1.0) - base.rgb; // Negate colors
}

// boost brightness
base = clamp(base * (1.0 + boost), 0.0, 1.0);

// threshold
vec3 avg = vec3((base.r + base.g + base.b)/3.0);
base.rgb = base.rgb * float(greaterThan(avg, vec3(thresh)));

// color mixer
mat4x4 mix = mat4x4(
    mix_rr, mix_rg, mix_rb, mix_ra,
    mix_gr, mix_gg, mix_gb, mix_ga,
    mix_br, mix_bg, mix_bb, mix_ba,
    mix_ar, mix_ag, mix_ab, mix_aa
);

base = clamp(base * mix, 0.0, 1.0);

// flash
if (flash_enable > 0) {
    // Basic edge detection in src_tex1 using base_coord (src_coord0.xy at this point).
    
    // Define a small offset for sampling neighboring pixels.
    // For more robustness, this could be 1.0 / textureSize(src_tex1, 0).
    vec2 pixel_offset = vec2(0.002, 0.002); // Adjust based on texture resolution


    // Sample luminance of pixels for gradient calculation
    // Using base_coord, which is src_coord0.xy here
    float lum_left   = get_luminance(texture(src_tex1, base_coord - vec2(pixel_offset.x, 0.0)).rgb);
    float lum_right  = get_luminance(texture(src_tex1, base_coord + vec2(pixel_offset.x, 0.0)).rgb);
    float lum_down   = get_luminance(texture(src_tex1, base_coord - vec2(0.0, pixel_offset.y)).rgb); // Assuming Y=0 is bottom
    float lum_up     = get_luminance(texture(src_tex1, base_coord + vec2(0.0, pixel_offset.y)).rgb);

    // Calculate horizontal and vertical gradients
    float grad_x = abs(lum_right - lum_left);
    float grad_y = abs(lum_up - lum_down);
    
    // Optional: For a smoother flash effect based on strength, you could use:
    // active_flash_strength = smoothstep(flash_edge_threshold, flash_edge_threshold + 0.05, edge_strength);

    // Edge strength (sum of absolute gradients, scaled slightly)
    float edge_strength = (grad_x + grad_y) * 0.5;
    if (edge_strength >= 0.03) {
        // If the edge strength exceeds the threshold, apply a flash effect
        base.rgb = vec3(1.0);
    }
}

// color key
if (color_key_enable > 0) {
    vec3 key_color = vec3(0.0, 0.0, 0.0); // pure black

    float dist = distance(base.rgb, key_color);
    if (dist <= color_key_sim) {
        base.a = 0.0; // Make transparent if close to key color
    }

    //blend currently ignored
}

// distort feedback

#ifdef CUSTOM_FEEDBACK_TRANSFORM
    // use custom transform if provided - it should have access to all the mixer uniforms
    color = CUSTOM_FEEDBACK_TRANSFORM(base);
#else
//default is to just do distortion, rotation, translation
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

    // underlay feedback
    color = blend_by_mode(feedback, base, BLEND_ALPHA);
#endif
