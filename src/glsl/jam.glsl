//!VAR vec3 iResolution0 1.0 1.0 1.0
//!VAR vec3 iResolution1 1.0 1.0 1.0
//!VAR vec3 iResolution2 1.0 1.0 1.0
//!VAR vec3 iResolution3 1.0 1.0 1.0

#define VIEW_RESOLUTION (iResolution.xy)
#define BLANK_RESOLUTION (iResolution0.xy)  
#define BLOB_RESOLUTION (iResolution1.xy)
#define CLOUD_RESOLUTION (iResolution2.xy)
#define FG_RESOLUTION (iResolution3.xy)

#define WIDTH (iResolution.x)
#define HEIGHT (iResolution.y)

vec2 uv = src_coord0.xy * VIEW_RESOLUTION;
color = texture(src_tex2, src_coord2);
if (true) {
    vec4 s1 = vec4(0.25, 0.25, 0.25, 1.0);
    vec4 s2 = vec4(0.35, 0.35, 0.35, 1.0);
    color = patch_check_scroll_px(
        uv,                       // coord in px
        VIEW_RESOLUTION,           // resolution
        color, // color in
        s1,//vec4(0.0, 0.3, 0.05, 1.0), // square1
        s2,//vec4(0.0, 0.35, 0.0, 1.0), // square2
        vec2(10.0, 10.0),         // block dim in px
        vec2(0.0, -fract(iTime/5.0) * HEIGHT), // offset
        mat4x2(                   // corners
            0.0, HEIGHT * 0.65,
            WIDTH, HEIGHT * 0.65,
            -1250.0, HEIGHT,
            WIDTH + 1250.0, HEIGHT)
    );
}

if (true) {
    vec2 scale = vec2(1.0, 1.0);
    vec2 uv = uv * scale;
    uv.y -= 0.0;
    vec2 center = VIEW_RESOLUTION * vec2(0.5, 0.5) * scale;
    float rad = 190.0;
    vec2 blob_coord = (src_coord0.xy * VIEW_RESOLUTION.xy) / BLOB_RESOLUTION.xy;
    vec4 blob_color = texture(src_tex1, blob_coord);
    color = patch_blob_px(
        uv,                  // coord in px
        VIEW_RESOLUTION,     // resolution
        color,               // color in
        blob_color,          // blob color
        center,              // blob center
        rad,                 // blob radius
        iTime                // offset
    );
}

if (true) {
    float scale =  float(VIEW_RESOLUTION.x) / float(FG_RESOLUTION.x);
    vec2 fg_coord = (src_coord0.xy / scale);

    vec3 fg = handle_edge(src_tex3, fg_coord, EDGE_MODE_MIRROR);
    if (distance(fg.rgb, vec3(0.0, 0.0, 0.0)) > 0.08) {
        color = vec4(fg, 1.0);
    }
}

//!STR debug_str v=0000x0000 k=0000x0000
if (true) {
    
    debug_str[2] = 0x30 + int(mod(VIEW_RESOLUTION.x/1000.0, 10.0));
    debug_str[3] = 0x30 + int(mod(VIEW_RESOLUTION.x/100.0, 10.0));
    debug_str[4] = 0x30 + int(mod(VIEW_RESOLUTION.x/10.0, 10.0));
    debug_str[5] = 0x30 + int(mod(VIEW_RESOLUTION.x, 10.0));

    debug_str[7] = 0x30 + int(mod(VIEW_RESOLUTION.y/1000.0, 10.0));
    debug_str[8] = 0x30 + int(mod(VIEW_RESOLUTION.y/100.0, 10.0));
    debug_str[9] = 0x30 + int(mod(VIEW_RESOLUTION.y/10.0, 10.0));
    debug_str[10] = 0x30 + int(mod(VIEW_RESOLUTION.y, 10.0));

    debug_str[14] = 0x30 + int(mod(BLANK_RESOLUTION.x/1000.0, 10.0));
    debug_str[15] = 0x30 + int(mod(BLANK_RESOLUTION.x/100.0, 10.0));
    debug_str[16] = 0x30 + int(mod(BLANK_RESOLUTION.x/10.0, 10.0));
    debug_str[17] = 0x30 + int(mod(BLANK_RESOLUTION.x, 10.0));
    
    debug_str[19] = 0x30 + int(mod(BLANK_RESOLUTION.y/1000.0, 10.0));
    debug_str[20] = 0x30 + int(mod(BLANK_RESOLUTION.y/100.0, 10.0));
    debug_str[21] = 0x30 + int(mod(BLANK_RESOLUTION.y/10.0, 10.0));
    debug_str[22] = 0x30 + int(mod(BLANK_RESOLUTION.y, 10.0));

    color = draw_text(color, uv, vec2(100,10), VIEW_RESOLUTION.xy, debug_str, debug_str_length);
}