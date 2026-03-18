#ifndef PATCH_EDGE_DETECT_GLSL
#define PATCH_EDGE_DETECT_GLSL
#include "utils.glsl"

// Basic edge detection in base_tex using uv
bool patch_edge_detect(vec2 base_coord, sampler2D base_tex, vec2 resolution, float strength) {     
   vec2 dx = vec2(1/resolution.x, 0.0);
    vec2 dy = vec2(0.0, 1/resolution.y);
    float grad_x = abs(
        rgb2hsv(texture(base_tex, base_coord - dx).rgb).z
        - rgb2hsv(texture(base_tex, base_coord + dx).rgb).z);
    float grad_y = abs(
        rgb2hsv(texture(base_tex, base_coord - dy).rgb).z
        - rgb2hsv(texture(base_tex, base_coord + dy).rgb).z);
    return (grad_x + grad_y > clamp(strength, 0.0, 255.0)/255.0);
}
#endif