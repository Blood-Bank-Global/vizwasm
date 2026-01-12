//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0
//!VAR float cc_iac_driver_bus_1_0_3 0.0
//!VAR float cc_iac_driver_bus_1_0_4 0.0
//!VAR float cc_iac_driver_bus_1_0_5 0.0

#define HORIZON 0.65
#define HORIZON_PX (HORIZON * iResolution.y)
#define STRETCH 3.5
#define BLOCK_PX 10.0
#define SUN_RAD 250.0
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
    // peak_points - array of x ranging from 0 to 1.0 for mountain peaks sorted smallest to largest
    float peak_points[10] = float[10](0.0, 0.15, 0.25, 0.4, 0.55, 0.65, 0.75, 0.85, 0.9, 1.0);
    // peak_heights - array of heights for each peak point between 0 and 0.09
    float peak_heights[10] = float[10](0.075, 0.03, 0.05, 0.08, 0.06, 0.03, 0.062, 0.04, 0.02, 0.07);

    float curr_h = 0.0;
    float r = 0.0;
    float g = 0.0;
    if (src_coord0.x <= peak_points[0]) {
        curr_h = peak_heights[0];
    } else if (src_coord0.x >= peak_points[9]) {
        curr_h = peak_heights[9];
    } else {
        // find the two peak points src_coord0.x is between
        
        for (int i = 0; i < 10; i++) {
            if (mod(i,2) == 0) {
                g = 0.5;
            } else {
                g = 0.0;
            }

            r += 0.1;
            if (src_coord0.x <= peak_points[i]) {
                float ss = smoothstep(peak_points[i-1], peak_points[i], src_coord0.x);
                curr_h = mix(peak_heights[i-1], peak_heights[i], ss);
                break;
            }
        }
    }
    
    float peak = HORIZON - curr_h;
    if (src_coord0.y >= peak) {
        color = vec4(r, g, r, 1.0);
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
