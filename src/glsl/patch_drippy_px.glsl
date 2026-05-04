#include "utils.glsl"

void patch_drippy_px(out vec4 color, in sampler2D tex, in vec2 uv, in vec2 resolution, in vec2 region_sz, in vec2 pixel_sz, in float cutoff_y, in float time) {
    vec2 coord = uv * resolution;
    coord = floor(coord / pixel_sz) * pixel_sz;
    vec2 region = floor(coord / region_sz);
    vec2 px = floor(coord / pixel_sz);
    vec2 px_coord = px * pixel_sz;
    float ctl_pts[] = float[](0.0, 0.0, 0.0, 0.0);
    for (int i = -1; i <= 2; i++) {
        float adjustment_s = -abs(randf(uint((region.x + float(i)) * 848619.0)) * region_sz.y * 3.5);
        float adjustment_e = abs(randf(uint((region.x + float(i)) * 474651.0)) * region_sz.y * 3.5);
        float off = randf(uint((region.x + float(i)) * 193813.0));
        float adjustment = mix(adjustment_s, adjustment_e, (1.0 + sin(time + off)) * 0.5);
        ctl_pts[i + 1] = adjustment;
    }
    float adjusted = bicubic_mix(
        ctl_pts[0],
        ctl_pts[1],
        ctl_pts[2],
        ctl_pts[3],
        fract(coord.x/region_sz.x));
    
    vec2 critical_px = floor(vec2(coord.x, cutoff_y + adjusted) / pixel_sz);
    vec2 critical_coord = critical_px * pixel_sz;
    if (coord.y >= critical_coord.y) {
        uint col = uint(px.x);
        uint row = uint(px.y);

        float pdist = distance(critical_coord.y, px_coord.y) / critical_coord.y;
        if ((col & 1u) == 0u
        || abs(randf(uint(float(col) * 193813.0) ^ uint(float(row) * 214371.0) ^ uint(fract(time/3) * 643521.0)))
            < pow(pdist, 0.125)) {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        } else {
            //vec2 pixel_uv = vec2(floor(coord.x / pixel_sz.x) * pixel_sz.x, (floor(coord.y / pixel_sz.y) * pixel_sz.y)) / resolution;
            color   = texture(tex, critical_coord / resolution);
        }
    } else {
        color = texture(tex, uv);
    }
}