#include "utils.glsl"
void pass0(out vec4 color) {
    color = vec4(hsv2rgb(vec3(src_uv.x, 0.5, src_uv.y)), 1.0);
}