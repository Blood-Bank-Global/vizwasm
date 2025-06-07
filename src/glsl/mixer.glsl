//!VAR float rr 1.0
//!VAR float rg 0.0
//!VAR float rb 0.0
//!VAR float ra 0.0
//!VAR float gr 0.0
//!VAR float gg 1.0
//!VAR float gb 0.0
//!VAR float ga 0.0
//!VAR float br 0.0
//!VAR float bg 0.0
//!VAR float bb 1.0
//!VAR float ba 0.0
//!VAR float ar 0.0
//!VAR float ag 0.0
//!VAR float ab 0.0
//!VAR float aa 1.0
//!VAR float thresh 0.0
//!VAR float rh 0.0
//!VAR float gh 0.0
//!VAR float bh 0.0
//!VAR float ah 0.0
//!VAR float rv 0.0
//!VAR float gv 0.0
//!VAR float bv 0.0
//!VAR float av 0.0
//!VAR float boost 0.0
//!VAR uint color_key_enable 0
//!VAR float color_key_sim 0.001
//!VAR float color_key_blend 0.0
//!VAR uint negate 0
//!VAR float dx 0.0
//!VAR float dy 0.0
//!VAR float distort_level 0.2
//!VAR float warp_level 0.2
//!VAR uint distort_edge 0
//!VAR float feedback_rotation 0.0
//!VAR mat4x2 new_corners 0.0 0.0 1.0 0.0 0.0 1.0 1.0 1.0
//!VAR float scrolled_h 0.0
//!VAR float scrolled_v 0.0
//!VAR uint flash_enable 0

vec2 base_coord = src_coord0.xy;

// Scroll as needed
base_coord = coord_mirror(vec2(base_coord.x + scrolled_h, base_coord.y + scrolled_v));

//skew
base_coord = skew3(base_coord, new_corners);

//Shift colors
vec4 warp_dx = vec4(base_coord.x) + vec4(distort(base_coord, src_tex4, warp_level), 0.0);
vec4 warp_dy = vec4(base_coord.y) + vec4(distort(base_coord, src_tex5, warp_level), 0.0);

mat4x2 warp_matrix = mat4x2(
    warp_dx[0] - rh, warp_dy[0] - rv,
    warp_dx[1] - gh, warp_dy[1] - gv,
    warp_dx[2] - bh, warp_dy[2] - bv,
    warp_dx[3] - ah, warp_dy[3] - av
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
    rr, rg, rb, ra,
    gr, gg, gb, ga,
    br, bg, bb, ba,
    ar, ag, ab, aa
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

vec2 distort_coord = src_coord0.xy;
// rotation
mat2 rot = mat2(
    cos(feedback_rotation), -sin(feedback_rotation),
    sin(feedback_rotation),  cos(feedback_rotation)
);

vec2 center = vec2(0.5, 0.5);
distort_coord -= center;
distort_coord *= rot;
distort_coord += center;

vec4 distort_dx = vec4(distort_coord.x - dx) + vec4(distort(distort_coord, src_tex2, distort_level), 0.0);
vec4 distort_dy = vec4(distort_coord.y - dy) + vec4(distort(distort_coord, src_tex3, distort_level), 0.0);
mat4x2 distort_matrix = mat4x2(
    distort_dx[0], distort_dy[0],
    distort_dx[1], distort_dy[1],
    distort_dx[2], distort_dy[2],
    distort_dx[3], distort_dy[3]
);

vec4 feedback = vec4(handle_edge(src_tex0, distort_matrix[0], distort_edge).r,
                     handle_edge(src_tex0, distort_matrix[1], distort_edge).g,
                     handle_edge(src_tex0, distort_matrix[2], distort_edge).b,
                     texture(src_tex0, distort_matrix[3]).a);

// underlay feedback
color = blend_by_mode(feedback, base, BLEND_ALPHA);
