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
//!VAR uint flash_enable 0
//!VAR uint feedback_mode_selected 0
//!VAR int usr_var 0
//!VAR ivec4 usr_vec 0 0 0 0

vec4 patch_mixer(vec4 base) {
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

    return base;
}