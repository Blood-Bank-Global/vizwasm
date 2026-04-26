#include "patch_warp_px.glsl"
#include "utils.glsl"

#define REGION_SIZE vec2(30.0, 30.0)
#define DENSITY1 0.05
#define DENSITY2 0.10
#define DENSITY3 0.15
#define MAX_RADIUS 40.0

//!VAR vec2 iResolution1 0.0 0.0
//!VAR vec2 iResolution2 0.0 0.0
//!VAR vec2 iResolution3 0.0 0.0

void pass0(out vec4 color) {
    color = vec4(vec3(0.0), 1.0);
    vec2 coord = src_uv.xy * iResolution.xy;
    vec2 delta = ceil(vec2(MAX_RADIUS) / REGION_SIZE);
    vec2 region = floor(coord / REGION_SIZE);
    for (int i = -int(delta.x); i <= int(delta.x); i++) {
        for (int j = -int(delta.y); j <= int(delta.y); j++) {
            vec2 neighbor_region = region + vec2(float(i), float(j));
            // pick out regions and generate a radius
            float seq = fract(iTime/10.0 + randf(uint(neighbor_region.x) * 19349663u ^ uint(neighbor_region.y) * 73856093u)) * 2.0 * M_PI;

            float region_rad = randf(uint(neighbor_region.x) * 73856193u ^ uint(neighbor_region.y) * 19349663u ^ 0xBEEF) * MAX_RADIUS;
            float curr_rad = smoothstep(-1.0, 1.0, sin(seq)) * region_rad;
            if (abs(randf(uint(neighbor_region.x) * 73856093u ^ uint(neighbor_region.y) * 19349663u)) < DENSITY1
                && distance(coord, neighbor_region * REGION_SIZE + REGION_SIZE * 0.5) < curr_rad) {
                color.r = 1.0;
            }

            region_rad = randf(uint(neighbor_region.x) * 73856193u ^ uint(neighbor_region.y) * 19349663u ^ 0xCAFE) * MAX_RADIUS;
            curr_rad = smoothstep(-1.0, 1.0, sin(seq)) * region_rad;
            if (abs(randf(uint(neighbor_region.x) * 83492791u ^ uint(neighbor_region.y) * 1234567u)) < DENSITY2
                && distance(coord, neighbor_region * REGION_SIZE + REGION_SIZE * 0.5) < curr_rad) {
                color.g = 1.0;
            }

            region_rad = randf(uint(neighbor_region.x) * 73856193u ^ uint(neighbor_region.y) * 19349663u ^ 0xDEAD) * MAX_RADIUS;
            curr_rad = smoothstep(-1.0, 1.0, sin(seq)) * region_rad;
            if (abs(randf(uint(neighbor_region.x) * 19283749u ^ uint(neighbor_region.y) * 5647382u)) < DENSITY3
                && distance(coord, neighbor_region * REGION_SIZE + REGION_SIZE * 0.5) < curr_rad) {
                color.b = 1.0;
            }
        }
    }

    
}

void pass1(out vec4 color) {
    vec2 coord = src_uv.xy * iResolution.xy;
    vec2 warped = patch_warp_px(coord, vec2(10.0, 10.0), 1.75, iResolution.xy, iTime);
    vec4 test = texture(pass_tex0, warped / iResolution.xy);
    color = vec4(0.0);
    vec2 region = floor(coord / (MAX_RADIUS * 4.0) );
    vec2 dir = vec2(
        randf(uint(region.x) * 73856093u ^ uint(region.y) * 19349663u ^ 0x12345678u)/50.0,
        randf(uint(region.x) * 19349663u ^ uint(region.y) * 73856093u ^ 0x87654321u)/50.0
    );
    if (test.b > 0.5) {
        vec2 scaled = iResolution.xy / iResolution1.xy;
        vec2 scaled_uv = src_uv * scaled * 0.6;
        color = texture(src_tex1, scaled_uv + (iTime * dir));
    } else if (test.g > 0.5) {
        vec2 scaled = iResolution.xy / iResolution2.xy;
        vec2 scaled_uv = src_uv * scaled  * 0.5;
        color = texture(src_tex2, scaled_uv + (iTime * dir));
    } else if (test.r > 0.5) {
        vec2 scaled = iResolution.xy / iResolution3.xy;
        vec2 scaled_uv = src_uv * scaled  * 0.5;
        color = texture(src_tex3, scaled_uv + (iTime * dir));
    } else {
        color = vec4(0.0);
    }


    vec4 bg = texture(src_tex0, src_uv);
    if (distance(color.rgb, vec3(0.0)) < 0.1) {
        color.rgb = vec3(1.0) - bg.rgb;
    }
    color = blend_by_mode(bg, color, BLEND_ALPHA);
}