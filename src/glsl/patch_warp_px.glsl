uint patch_warp_px_hash(uint seed) {
    seed = (seed ^ 61u) ^ (seed >> 16u);
    seed *= 9u;
    seed = seed ^ (seed >> 4u);
    seed *= 668265261u;
    seed = seed ^ (seed >> 15u);
    return seed;
}

vec2 patch_warp_px(vec2 uv, vec2 size, float strength, vec2 resolution, float sequence) {
    vec2 rounded = vec2(
        floor(uv.x/size.x)*size.x,
        floor(uv.y/size.y)*size.y
    );

    vec2 web[4][4];
    for (int i = 0; i<4; i++) {
        for (int j = 0; j<4; j++) {
            vec2 p = vec2(
                rounded.x + (float(i) - 1.0) * size.x,
                rounded.y + (float(j) - 1.0) * size.y
            );
            
            int gx = int(floor(uv.x/size.x)) + i - 1;
            int gy = int(floor(uv.y/size.y)) + j - 1;

            // Robust spatial hash
            uint h = patch_warp_px_hash(uint(gx) * 73856093u ^ uint(gy) * 19349663u);

            vec2 p_t[4];
            for (int k = 0; k<4; k++)  {
                int seq_k = int(floor(sequence)) + k - 1;
                // Mix in time
                uint time_h = patch_warp_px_hash(uint(seq_k) * 29164289u);
                
                // Separate seeds for X and Y using distinct final hashes
                uint seed_x = patch_warp_px_hash(h ^ time_h);
                uint seed_y = patch_warp_px_hash(h ^ time_h ^ 0x9E3779B9u);

                p_t[k] = vec2(
                    p.x + (size.x * sin(randf(seed_x) * 2.0 * M_PI) * strength),
                    p.y + (size.y * sin(randf(seed_y) * 2.0 * M_PI) * strength)
                );
                
            }
            web[i][j] = vec2(
                bicubic_mix(p_t[0].x, p_t[1].x, p_t[2].x, p_t[3].x, fract(sequence)),
                bicubic_mix(p_t[0].y, p_t[1].y, p_t[2].y, p_t[3].y, fract(sequence))
            );
        }
    }
    
    // Bicubic interpolation of the web points
    // Pass 1: Interpolate each row along X
    vec2 rows[4];
    float fx = fract(uv.x / size.x);
    for (int j = 0; j < 4; j++) {
        rows[j].x = bicubic_mix(web[0][j].x, web[1][j].x, web[2][j].x, web[3][j].x, fx);
        rows[j].y = bicubic_mix(web[0][j].y, web[1][j].y, web[2][j].y, web[3][j].y, fx);
    }

    // Pass 2: Interpolate the results along Y
    float fy = fract(uv.y / size.y);
    return vec2(
        bicubic_mix(rows[0].x, rows[1].x, rows[2].x, rows[3].x, fy),
        bicubic_mix(rows[0].y, rows[1].y, rows[2].y, rows[3].y, fy)
    );
}