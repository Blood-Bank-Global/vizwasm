//!VAR int usr_var 0

//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0
//!VAR vec2 iResolution2 1.0 1.0
//!VAR vec2 iResolution3 1.0 1.0
//!VAR vec2 iResolution4 1.0 1.0
//!VAR vec2 iResolution5 1.0 1.0
//!VAR vec2 iResolution6 1.0 1.0
//!VAR vec2 iResolution7 1.0 1.0
//!VAR vec2 iResolution8 1.0 1.0
//!VAR vec2 iResolution9 1.0 1.0

/**
 * Notes:
 * 0. blank
 * 1. error2
 * 2. error3
 * 3. logo
 * 4. the_moon
 * 5. harmony1 - wave
 * 6. harmony2 - flower
 * 7. harmony3 - grid
 * 8. harmony4 - walls
 * 9. statue
 * 10. full
 */

#define SELECT_FLOWER 0
#define SELECT_WAVE 1
#define SELECT_MOON 2
#define SELECT_WALLS 3

int selection = SELECT_WALLS;

if (selection == SELECT_FLOWER) { // flower feedback
    vec4 below = texture(src_tex0, src_coord0);
    vec4 above = texture(src_tex6, src_coord6);

    if (distance(above.rgb, vec3(0.0)) < 0.3) {
        above.a = 0.0;  // If above is almost black, set alpha to 0
    } 

    color = blend_by_mode(below, above, BLEND_ALPHA);
}

if (selection == SELECT_WAVE) { // wave feedback
    vec4 below = texture(src_tex3, src_coord3);
    vec4 above = texture(src_tex5, src_coord5);

    if (distance(above.rgb, vec3(0.0)) < 0.3) {
        above.a = 0.0;  // If above is almost black, set alpha to 0
    } 

    color = blend_by_mode(below, above, BLEND_ALPHA);
}

if (selection == SELECT_WALLS) {
    vec4 below = texture(src_tex9, src_coord9);
    vec4 above = texture(src_tex8, src_coord8);

    float w = 400.0;
    float h = 400.0 * iResolution8.y / iResolution8.x;
    float left = (iResolution8.x - w)/2.0/iResolution8.x;
    float right = left + w / iResolution8.x;
    float top = (iResolution8.y - h)/2.0/iResolution8.y;
    float bottom = top + h / iResolution8.y;

    if (src_coord8.x > left && src_coord8.x < right && src_coord8.y > top && src_coord8.y < bottom) {
        above.a = 0.0;  // If above is almost black, set alpha to 0
    } 

    color = blend_by_mode(below, above, BLEND_ALPHA);
}


if (selection == SELECT_MOON) { // moon mode
    vec2 below_normalize = vec2((iResolution.x/iResolution4.x) * src_coord4.x, (iResolution.y/iResolution4.y) * src_coord4.y);
    vec2 below_offset = vec2((1.0 - (iResolution.x/iResolution4.x)) / 2.0, 0.0);
    vec2 below_coord = below_normalize + below_offset;
    vec4 below = vec4(handle_edge(src_tex4, below_coord, EDGE_MODE_BLANK), 1.0);

    mat4x2 new_corners = mat4x2(0.0, 0.667, 1.0, 0.667, -2.0, 1.0, 2.0, 1.0);
    vec2 above_coord = skew3(src_coord7, new_corners);
    vec4 above = texture(src_tex7, above_coord);

    if (src_coord7.y <= 0.6667) {
        above.a = 0.0;  // If above the midpoint, set alpha to 0
    } 

    color = blend_by_mode(below, above, BLEND_ALPHA);
}


// Blast on error effects
switch (usr_var) {
    case 0:
        break;
    case 1:
        {
            vec4 blast = texture(src_tex1, src_coord1);
            if (distance(blast.rgb, vec3(0.0)) < 0.3) {
                blast.a = 0.0;  // If blast is almost black, set alpha to 0
            } 
            color = blend_by_mode(color, blast, BLEND_ALPHA);
            break;
        }
    case 2:
        {
            vec4 blast = texture(src_tex2, src_coord2);
            if (distance(blast.rgb, vec3(0.0)) < 0.3) {
                blast.a = 0.0;  // If blast is almost black, set alpha to 0
            } 
            color = blend_by_mode(color, blast, BLEND_ALPHA);
            break;
        }
    default:
        break;
}
