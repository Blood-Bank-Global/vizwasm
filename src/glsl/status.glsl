//!VAR int num_labels
//!VAR int[] label_pos 1
//!VAR int[] label_idx 1
//!VAR int[] label_len 1
//!VAR int[] label_txt 1

vec2 uv = src_coord.xy * iResolution.xy;
color = vec4(0.0, 0.0, 0.0, 1.0);
if (fract(uv.x/10) < 0.5) {
    color = vec4(0.3, 0.3, 0.3, 1.0);
}