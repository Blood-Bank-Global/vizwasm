#include "utils.glsl"
#include "patch_drippy_px.glsl"

#define REGION_SZ vec2(45.0, 45.0)
#define PIXEL_SZ vec2(8.0, 8.0)
#define CUTOFF_Y 8.0

void pass0(out vec4 color) {
    patch_drippy_px(color, src_tex0, src_uv, iResolution.xy, REGION_SZ, PIXEL_SZ, CUTOFF_Y, iTime);
}