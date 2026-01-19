// // distort the src coord using perlin noise as vec2 distorted
// vec2 distorted = src_coord0;
// float scale = 0.02;
// float strength = 0.03;

// // no noise library function exists so implement it here
// #define permute(x) mod(((x*34.0)+1.0)*x, 289.0)

// distorted.x += (permute((src_coord0.xy * scale * fract(iTime) * 0.1)) - 0.5) * strength;
// distorted.y += (permute((src_coord0.yx * scale * fract(iTime) * 0.1)) - 0.5) * strength;

// color = texture(src_tex0, src_coord0);

// vec2 uv = distorted.xy * iResolution.xy;
// vec2 center = iResolution.xy * vec2(0.5,0.5);
// if (distance(uv, center) < 50.0) {
//     color.rgb = vec3(1.0, 1.0, 1.0);
// }

#define RESOLUTION (iResolution.xy)
#define WIDTH (iResolution.x)
#define HEIGHT (iResolution.y)

vec2 uv = src_coord0.xy * RESOLUTION;
color = patch_check_scroll_px(
    uv,                       // coord in px
    RESOLUTION,           // resolution
    vec4(0.0, 0.0, 0.0, 1.0), // color in
    vec4(0.0, 1.0, 0.0, 1.0), // square1
    vec4(0.0, 0.5, 1.0, 1.0), // square2
    vec2(10.0, 10.0),         // block dim in px
    vec2(0.0, -fract(iTime/5.0) * HEIGHT), // offset
    mat4x2(                   // corners
        0.0, HEIGHT * 0.65,
        WIDTH, HEIGHT * 0.65,
        -1250.0, HEIGHT,
        WIDTH + 1250.0, HEIGHT)
);

if (true) {
    vec2 scale = vec2(1.0, 1.0);
    vec2 uv = uv * scale;
    vec2 center = RESOLUTION * vec2(0.5, 0.5) * scale;
    float rad = 190.0;
    vec4 blob_color = texture(src_tex1, src_coord1.xy);
    color = patch_blob_px(
        uv,                          // coord in px
        RESOLUTION,                  // resolution
        color,                       // color in
        blob_color,    // blob color
        center, // blob center
        rad,                        // blob radius
        iTime                // offset
    );
}