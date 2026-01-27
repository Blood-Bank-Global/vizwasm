int patch_warp_px_get_seed(int v, int sequence) {
    return int(mod((v * 73856093) ^ (sequence * 19349663), 2147483647));
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
                rounded.x + float(i - 1.0) * size.x,
                rounded.y + float(j - 1.0) * size.y
            );

            int gx = int(floor(uv.x/size.x)) + i - 1;
            int gy = int(floor(uv.y/size.y)) + j - 1;
            int h = (gx * 73856093) ^ (gy * 19349663);

            vec2 p_t[4];
            for (int k = 0; k<4; k++)  {
                ivec2 seed = ivec2(
                        patch_warp_px_get_seed(h, int(floor(sequence)) + k - 1),
                        patch_warp_px_get_seed(h ^ 0x31415926, int(floor(sequence)) + k - 1)
                );
                p_t[k] = vec2(
                    p.x + (size.x * sin(randf(uint(abs(seed.x))) * 2.0 * M_PI) * strength),
                    p.y + (size.y * sin(randf(uint(abs(seed.y))) * 2.0 * M_PI) * strength)
                );
            }
            web[i][j] = vec2(
                bicubic_mix(p_t[0].x, p_t[1].x, p_t[2].x, p_t[3].x, fract(sequence)),
                bicubic_mix(p_t[0].y, p_t[1].y, p_t[2].y, p_t[3].y, fract(sequence))
            );
        }
    }
    
    vec2 x_interps[4];
    float fx = fract(uv.x / size.x);
    for (int j = 0; j < 4; j++) {
        x_interps[j] = vec2(
            bicubic_mix(web[0][j].x, web[1][j].x, web[2][j].x, web[3][j].x, fx),
            bicubic_mix(web[0][j].y, web[1][j].y, web[2][j].y, web[3][j].y, fx)
        );
    }
    
    float fy = fract(uv.y / size.y);
    return vec2(
        bicubic_mix(x_interps[0].x, x_interps[1].x, x_interps[2].x, x_interps[3].x, fy),
        bicubic_mix(x_interps[0].y, x_interps[1].y, x_interps[2].y, x_interps[3].y, fy)
    );
}