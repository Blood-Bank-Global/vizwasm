vec4 patch_check_scroll_px(
    vec2 uv,
    vec2 resolution,
    vec4 color_in,
    vec4 square1,
    vec4 square2,
    vec2 block_dim,
    vec2 offset,
    mat4x2 corners) {

    if (!pointInRhombus(uv, corners)) {
        return color_in;
    }

    vec2 normalized_uv = uv / resolution;
    mat4x2 normalized_corners = mat4x2(
        corners[0].x / resolution.x, corners[0].y / resolution.y,
        corners[1].x / resolution.x, corners[1].y / resolution.y,
        corners[2].x / resolution.x, corners[2].y / resolution.y,
        corners[3].x / resolution.x, corners[3].y / resolution.y
    );

    vec2 skewed = skew3(normalized_uv, normalized_corners);
    vec2 skewed_uv = vec2(
        mod(skewed.x * resolution.x + offset.x, resolution.x),
        mod(skewed.y * resolution.y + offset.y, resolution.y)
    );

    float block = mod(mod(floor(skewed_uv.x/block_dim.x),2.0) + mod(floor(skewed_uv.y/block_dim.y), 2.0),2.0);
    if (block == 0) {
        return square1;
    } 

    return square2;
}