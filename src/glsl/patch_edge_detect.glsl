#ifndef PATCH_EDGE_DETECT_GLSL
#define PATCH_EDGE_DETECT_GLSL
#include "utils.glsl"

// Basic edge detection in base_tex using uv
bool patch_edge_detect(vec2 uv, sampler2D base_tex, vec2 resolution) {     
    // Define a small offset for sampling neighboring pixels.
    // For more robustness, this could be 1.0 / textureSize(base_tex, 0).
    vec2 pixel_offset = vec2(1, 1); // Adjust based on texture resolution

    // Sample luminance of pixels for gradient calculation
    // Using uv, which is src_coord0.xy here
    vec2 texel_size = 1.0 / resolution;
    float lum_left   = get_luminance(texture(base_tex, uv - vec2(pixel_offset.x, 0.0) * texel_size).rgb);
    float lum_right  = get_luminance(texture(base_tex, uv + vec2(pixel_offset.x, 0.0) * texel_size).rgb);
    float lum_down   = get_luminance(texture(base_tex, uv - vec2(0.0, pixel_offset.y) * texel_size).rgb); // Assuming Y=0 is bottom
    float lum_up     = get_luminance(texture(base_tex, uv + vec2(0.0, pixel_offset.y) * texel_size).rgb);

    // Calculate horizontal and vertical gradients
    float grad_x = abs(lum_right - lum_left);
    float grad_y = abs(lum_up - lum_down);

    // Optional: For a smoother flash effect based on strength, you could use:
    // active_flash_strength = smoothstep(flash_edge_threshold, flash_edge_threshold + 0.05, edge_strength);

    // Edge strength (sum of absolute gradients, scaled slightly)
    float edge_strength = (grad_x + grad_y) * 0.5;
return edge_strength >= 0.03;
}
#endif