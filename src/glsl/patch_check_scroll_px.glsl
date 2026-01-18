vec4 patch_check_scroll_px(
    vec4 color_in,
    vec4 square1,
    vec4 square2,
    vec2 uv,
    vec2 block_dim,
    vec2 offset,
    mat4x2 corners) {

    if (!pointInRhombus(uv, corners)) {
        return color_in;
    }

    vec2 normalized_uv = uv / vec2(iResolution.x, iResolution.y);
    mat4x2 normalized_corners = mat4x2(
        corners[0].x / iResolution.x, corners[0].y / iResolution.y,
        corners[1].x / iResolution.x, corners[1].y / iResolution.y,
        corners[2].x / iResolution.x, corners[2].y / iResolution.y,
        corners[3].x / iResolution.x, corners[3].y / iResolution.y
    );

    vec2 skewed = skew3(normalized_uv, normalized_corners);
    vec2 skewed_uv = vec2(
        mod(skewed.x * iResolution.x + offset.x, iResolution.x),
        mod(skewed.y * iResolution.y + offset.y, iResolution.y)
    );

    float block = mod(mod(floor(skewed_uv.x/block_dim.x),2.0) + mod(floor(skewed_uv.y/block_dim.y), 2.0),2.0);
    if (block == 0) {
        return square1;
    } 

    return square2;
}