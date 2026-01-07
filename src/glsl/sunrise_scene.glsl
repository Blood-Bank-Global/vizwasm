//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0
//!VAR float cc_iac_driver_bus_1_0_3 0.0
//!VAR float cc_iac_driver_bus_1_0_4 0.0
//!VAR float cc_iac_driver_bus_1_0_5 0.0

#define HORIZON 0.65
#define HORIZON_PX (HORIZON * iResolution.y)
#define STRETCH 3.5
#define BLOCK_PX 10.0
#define SUN_RAD 200.0
#define SPEED (1/(160.0/60.0/4.0) * (4 * BLOCK_PX/iResolution.y))
#define PEAK_H 30.0
#define PEAK_W 50.0
#define BUCKETS_START (HORIZON_PX-SUN_RAD)
#define BUCKET_SZ 20.0
#define BUCKET_COUNT (floor(SUN_RAD/ BUCKET_SZ))


vec4 sky_color = vec4(0.1 * cc_iac_driver_bus_1_0_4/127, 0.0, 0.5 * cc_iac_driver_bus_1_0_4/127, 1.0);
vec4 square1 = vec4(0, 0.1, 0.4, 1.0);
vec4 square2 = vec4(0, 0.1, 0.6, 1.0);

color = sky_color;

if (src_coord0.y <= HORIZON) {
    // Sun sphere
    float flicker = ((1.0 + sin((fract(iTime / SPEED / 2.0)+src_coord0.y) * 2*M_PI))/2.0) * .05;
    vec2 px = src_coord0.xy * iResolution.xy;
    vec2 center = vec2(0.5, HORIZON) * iResolution.xy;
    float radius = distance(px, center);

    if (radius <= SUN_RAD) {
        float cloud = smoothstep(pow(SUN_RAD,2.0), 0.0, pow(radius,2));
        color.rgb = blend_by_mode(
            sky_color,
            vec4(0.5, 0.25, 0.1, clamp((cloud + flicker), 0.0, 1.0)),
            BLEND_ALPHA
        ).rgb;
        color.a = 1.0;
    }

    // stripes
    if (px.y <= HORIZON_PX && px.y >= BUCKETS_START) {
        float bucket_num = (floor((px.y - BUCKETS_START)/BUCKET_SZ));
        float bucket_num_start = bucket_num * BUCKET_SZ + BUCKETS_START;
        float y = px.y - bucket_num_start;
        float fill_sz = clamp((BUCKET_COUNT - bucket_num)*2.5, 0, BUCKET_SZ);
        float offset = fract(iTime/2) * (BUCKET_SZ );
        
        if (mod(y + offset, BUCKET_SZ) > fill_sz) {
            color = sky_color;
        } 
    }

    // Mountains
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
        color = square1;
    } else {
        color = square2;
    }
    color.r = depth * (1+cc_iac_driver_bus_1_0_5/256);
}
