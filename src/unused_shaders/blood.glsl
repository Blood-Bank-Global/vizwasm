

{
    uint seed = 0xcafebabe;
    color = vec4(0.0, 0.0, 0.0, 1.0); // Default color
    for (uint i = 0; i < 100; i++) {
        float fps = 30.0;
        float frames_per_period = fps * 1.5;

        float r = 0.01;
        vec2 pt1 = vec2(get_x(seed + i, frame-1), get_y(seed + i, frame - 1));
        vec2 pt2 = vec2(get_x(seed + i, frame), get_y(seed + i, frame));

        if (pt2.y < pt1.y) {
            // handle wrapping
            pt1 = pt2;
        }

        vec4 color2 = draw_blood(src_coord0, pt1, pt2, r, seed + i + 1213);
        if (color2.rgb != vec3(0.0)) {
            color = color2;
        }
    }
}
