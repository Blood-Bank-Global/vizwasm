//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0
//!VAR float cc_iac_driver_bus_1_0_3 0.0
//!VAR float cc_iac_driver_bus_1_0_4 0.0
//!VAR float cc_iac_driver_bus_1_0_5 0.0

#define HORIZON 0.65
#define STRETCH 3.5
#define BLOCK_PX 10.0
#define SUN_RAD 200.0
#define SPEED (1/(160.0/60.0/4.0) * (4 * BLOCK_PX/iResolution.y))
#define PEAK_H 30.0
#define PEAK_W 50.0

color = vec4(0.0);

if (src_coord0.y <= HORIZON) {
    float flicker = ((1.0 + sin((fract(iTime / SPEED / 2.0)+src_coord0.y) * 2*M_PI))/2.0) * .05;
    vec2 px = vec2(src_coord0.x * iResolution.x, src_coord0.y * iResolution.y);
    vec2 center = vec2(0.5, HORIZON) * iResolution.xy;
    float radius = distance(px, center);
    float cloud = smoothstep(pow(SUN_RAD,2.0), 0.0, pow(radius,2));
    color = vec4(cloud * (0.5 + flicker), cloud * 0.25, cloud * (0.1 + flicker), 1.0);

    
    float peak = HORIZON - (PEAK_H / iResolution.y)*abs(PEAK_W/2.0 - mod((src_coord0.x * iResolution.x), PEAK_W))/PEAK_W;
    if (src_coord0.y >= peak) {
        color = vec4(0.4, 0.0, 0.4, 1.0);
    }
} else {

    float block_x = BLOCK_PX / iResolution.x;
    float block_y = BLOCK_PX / iResolution.y;

    vec2 block_coord = skew3(
        src_coord0, mat4x2(
            0.0, HORIZON,
            1.0, HORIZON,
            (0.0 - STRETCH), 1.0,
            (1.0 + STRETCH), 1.0
        )
    );

    float depth = pow(block_coord.y, 2.0)/4.0;

    block_coord.y = mod((block_coord.y - fract(iTime * SPEED)), 1.0);
    float block = mod(mod(floor(block_coord.x/block_x),2.0) + mod(floor(block_coord.y/block_y), 2.0),2.0);
    if (block == 0) {
        color = vec4(depth, 0.1, 0.4, 1.0);
    } else {
        color = vec4(depth, 0.1, 0.6, 1.0);
    }
}
