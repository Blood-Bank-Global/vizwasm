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


vec2 uv = src_coord0.xy * iResolution.xy;
color = patch_check_scroll_px(
    vec4(0.0, 0.0, 0.0, 1.0), // color in
    vec4(0.0, 1.0, 0.0, 1.0), // square1
    vec4(0.0, 0.5, 1.0, 1.0), // square2
    uv,                       // coord in px
    vec2(10.0, 10.0),         // block dim in px
    vec2(0.0, -fract(iTime/5.0) * iResolution.y),           // offset
    mat4x2(
        0.0, iResolution.y * 0.65,
        iResolution.x, iResolution.y * 0.65,
        -1250.0, iResolution.y,
        iResolution.x + 1250.0, iResolution.y)
);
