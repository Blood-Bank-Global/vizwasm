void patch_ordered_dither4x4(out vec4 color, in vec2 uv, in vec2 resolution, in sampler2D tex) {
    // https://en.wikipedia.org/wiki/Ordered_dithering
    // 4x4 Bayer matrix
    int x = int(mod(uv.x * resolution.x, 4.0));
    int y = int(mod(uv.y * resolution.y, 4.0));
    int index = x + y * 4;
    mat4x4 bayer = mat4x4(
        0.0,  8.0,  2.0, 10.0,
        12.0, 4.0, 14.0, 6.0,
        3.0, 11.0, 1.0, 9.0,
        15.0, 7.0, 13.0, 5.0
    ) / 16.0;
    vec4 c = texture(tex, uv);
    color.rgba = step(vec4(bayer[index / 4][index % 4]), c.rgba);
}

void patch_ordered_dither8x8(out vec4 color, in vec2 uv, in vec2 resolution, in sampler2D tex) {
    // 8x8 Bayer matrix
    int x = int(mod(uv.x * resolution.x, 8.0));
    int y = int(mod(uv.y * resolution.y, 8.0));
    int index = x + y * 8;
    float bayer[64] = float[64](
        0.0, 32.0, 8.0, 40.0, 2.0, 34.0, 10.0, 42.0,
        48.0, 16.0, 56.0, 24.0, 50.0, 18.0, 58.0, 26.0,
        12.0, 44.0, 4.0, 36.0, 14.0, 46.0, 6.0, 38.0,
        60.0, 28.0, 52.0, 20.0, 62.0, 30.0, 54.0, 22.0,
        3.0, 35.0, 11.0, 43.0, 1.0, 33.0, 9.0, 41.0,
        51.0, 19.0, 59.0, 27.0, 49.0, 17.0, 57.0, 25.0,
        15.0, 47.0, 7.0, 39.0, 13.0, 45.0, 5.0, 37.0,
        63.0, 31.0, 55.0, 23.0, 61.0, 29.0, 53.0, 21.0
    );
    vec4 c = texture(tex, uv);
    color.rgba = step(vec4(bayer[index / 8 + index % 8]/64.0), c.rgba);
}


void patch_halftone45(out vec4 color, in vec2 uv, in vec2 resolution, in sampler2D tex, float scale, float angle_deg) {
    // https://en.wikipedia.org/wiki/Halftone#45-degree_angle
    vec2 coord = uv * resolution / scale;
    float angle = radians(angle_deg);
    mat2 rot = mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
    vec2 rotatedCoord = rot * coord;
    vec2 localPos = fract(rotatedCoord);
    float radius = length(localPos - vec2(0.5));
    vec4 c = texture(tex, uv);
    color.rgba = step(vec4(radius), c.rgba);
}