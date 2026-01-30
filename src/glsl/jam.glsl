//!VAR vec3 iResolution0 1.0 1.0 1.0
//!VAR vec3 iResolution1 1.0 1.0 1.0
//!VAR vec3 iResolution2 1.0 1.0 1.0
//!VAR vec3 iResolution3 1.0 1.0 1.0
//!VAR vec3 iResolution4 1.0 1.0 1.0
//!VAR vec3 iResolution5 1.0 1.0 1.0
//!VAR vec3 iResolution6 1.0 1.0 1.0
//!VAR vec3 iResolution7 1.0 1.0 1.0
//!VAR vec3 iResolution8 1.0 1.0 1.0

//!VAR float cc_iac_driver_bus_1_0_0 0.0
//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0
//!VAR float cc_iac_driver_bus_1_0_3 0.0
//!VAR float cc_iac_driver_bus_1_0_4 0.0

//!VAR int usr_var 0

#define VIEW_RESOLUTION (iResolution.xy)
#define BLANK_RESOLUTION (iResolution0.xy)  
#define STATUE_RESOLUTION (iResolution1.xy)
#define CLOUD_RESOLUTION (iResolution2.xy)
#define COL_RESOLUTION (iResolution3.xy)
#define FACADE_RESOLUTION (iResolution4.xy)
#define NIGHT_SKY_RESOLUTION (iResolution5.xy)
#define QUEST_RESOLUTION (iResolution6.xy)
#define HORIZON_RESOLUTION (iResolution7.xy)
#define QUEEN_RESOLUTION (iResolution8.xy)


#define VIEW_COORD (src_coord.xy)
#define BLANK_COORD (src_coord0.xy)
#define STATUE_COORD (src_coord1.xy)
#define CLOUD_COORD (src_coord2.xy)
#define COL_COORD (src_coord3.xy)
#define FACADE_COORD (src_coord4.xy)
#define NIGHT_SKY_COORD (src_coord5.xy)
#define QUEST_COORD (src_coord6.xy)
#define HORIZON_COORD (src_coord7.xy)
#define QUEEN_COORD (src_coord8.xy)

#define BLAN_TEX (src_tex0)
#define STATUE_TEX (src_tex1)
#define CLOUD_TEX (src_tex2)
#define COL_TEX (src_tex3)
#define FACADE_TEX (src_tex4)
#define NIGHT_SKY_TEX (src_tex5)
#define QUEST_TEX (src_tex6)
#define HORIZON_TEX (src_tex7)
#define QUEEN_TEX (src_tex8)

#define WIDTH (iResolution.x)
#define HEIGHT (iResolution.y)



//////////// GET TO PIXELS
vec2 uv = src_coord.xy * VIEW_RESOLUTION;
color = vec4(0.0, 0.0, 0.0, 1.0);

//////////// BACKGROUND 
if (true) {
    vec2 coord = CLOUD_COORD * 0.75 + vec2(0.125, 0.125);
    color = texture(CLOUD_TEX, (coord.xy * VIEW_RESOLUTION.x) / CLOUD_RESOLUTION.xy * (CLOUD_RESOLUTION.x / VIEW_RESOLUTION.x));
    vec3 hsv = rgb2hsv(color.rgb);
    color.rgb = hsv2rgb(vec3(hsv.x, hsv.y, hsv.z + cc_iac_driver_bus_1_0_0/127.0));
}

//////////// GROUND SCROLL
if (true) {
    vec4 s1 = vec4(0.0, 0.0, 0.20, 1.0);
    vec4 s2 = vec4(0.0, 0.15, 0.35, 1.0);
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


//////////// VISUBLOB
#if 0
#define BLOB_BG_TEX NIGHT_SKY_TEX
#define BLOB_BG_COORD NIGHT_SKY_COORD
#define BLOB_BG_RESOLUTION NIGHT_SKY_RESOLUTION
#elif 1
#define BLOB_BG_TEX QUEEN_TEX
#define BLOB_BG_COORD QUEEN_COORD
#define BLOB_BG_RESOLUTION QUEEN_RESOLUTION
#else
#define BLOB_BG_TEX STATUE_TEX
#define BLOB_BG_COORD STATUE_COORD
#define BLOB_BG_RESOLUTION STATUE_RESOLUTION
#endif
if (true) {
    vec2 scale = vec2(1.0, 1.0);
    vec2 uv = uv * scale;
    uv.y -= 20.0;
    vec2 center = VIEW_RESOLUTION * vec2(0.5, 0.5) * scale;
    float rad = 250.0 * mix(0.3, 1.0, (1.0 + sin(fract(iTime/10.0) * M_PI * 2.0)/2.0));

    vec2 blob_bg_resolution = BLOB_BG_RESOLUTION;
    vec2 blob_bg_coord = BLOB_BG_COORD;

    vec2 blob_bg_tx = (blob_bg_coord * VIEW_RESOLUTION.xy) / blob_bg_resolution.xy * blob_bg_resolution.x / VIEW_RESOLUTION.x;
    vec4 blob_color = texture(BLOB_BG_TEX, blob_bg_tx);

    color = patch_blob_px(
        uv,                  // coord in px
        VIEW_RESOLUTION,     // resolution
        color,               // color in
        blob_color,      // blob color
        center,              // blob center
        rad,                 // blob radius
        iTime                // offset
    );
}


//////////// FOREGROUND LAYER
#if 1
#define FG_RESOLUTION FACADE_RESOLUTION
#define FG_COORD FACADE_COORD
#define FG_TEX FACADE_TEX
#else
#define FG_RESOLUTION COL_RESOLUTION
#define FG_COORD COL_COORD
#define FG_TEX COL_TEX
#endif
if (true) {
    float scale = FG_RESOLUTION.x / VIEW_RESOLUTION.x;
    vec2 fg_coord = (FG_COORD * VIEW_RESOLUTION / FG_RESOLUTION) 
        *  scale - vec2(0.0, (FG_RESOLUTION.y * scale - VIEW_RESOLUTION.y)/VIEW_RESOLUTION.y);
    
    vec2 fg_uv = fg_coord * FG_RESOLUTION;

    fg_uv = patch_warp_px(fg_uv, vec2(25.0, 25.0), float(abs(usr_var))/100.0, FG_RESOLUTION, iTime/5.0);
    fg_coord = fg_uv / FG_RESOLUTION;
    // fg_coord = patch_warp_px(fg_coord, vec2(50.0, 50.0)/FG_RESOLUTION, 1.0, vec2(1.0,1.0), iTime/5.0);
    
    // fg_coord = fg_uv / FG_RESOLUTION;
    vec4 fg_color = vec4(handle_edge(FG_TEX, fg_coord, EDGE_MODE_MIRROR), 1.0);

    // float scale =  float(VIEW_RESOLUTION.x) / float(COL_RESOLUTION.x);
    // vec2 fg_coord = (COL_COORD / scale);
    // vec3 fg = handle_edge(COL_TEX, fg_coord, EDGE_MODE_MIRROR);
    if (distance(fg_color.rgb, vec3(0.0, 0.0, 0.0)) > 0.08) {
        color = fg_color;
    }
}

//!STR debug_str "v=0000x0000 k=0000x0000"
if (false) {
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

if (false) {
    vec4 quest_color = texture(QUEST_TEX, QUEST_COORD.xy);
    if (distance(quest_color.rgb, vec3(0.0, 0.0, 0.0))   > 0.1) {
        color.rgb = blend_by_mode(color, quest_color, BLEND_ALPHA).rgb;
    }
}

//!STR cc_value "cc=000"
if (false) {
    float v = cc_iac_driver_bus_1_0_0;
    cc_value[3] = 0x30 + int(mod(v/100.0, 10.0));
    cc_value[4] = 0x30 + int(mod(v/10.0, 10.0));
    cc_value[5] = 0x30 + int(mod(v, 10.0));
    color = draw_text(color, uv, vec2(100,10), VIEW_RESOLUTION.xy, cc_value, cc_value_length);
}