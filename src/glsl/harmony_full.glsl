//!VAR int usr_var 0

/**
 * Notes:
 * 0. blank
 * 1. error2
 * 2. error3
 * 3. logo
 * 4. harmony1
 * 5. harmony2
 * 6. harmony3
 * 7. harmony4
 * 8. full
 */

#define SELECT_WAVE 0
#define SELECT_TUNNEL 1
#define SELECT_WALLS 2
#define SELECT_MOON 3

int selection = SELECT_MOON;

if (selection == SELECT_WAVE) { // wave feedback
    vec4 below = texture(src_tex3, src_coord3);
    vec4 above = texture(src_tex5, src_coord5);

    if (distance(above.rgb, vec3(0.0)) < 0.3) {
        above.a = 0.0;  // If above is almost black, set alpha to 0
    } 

    color = blend_by_mode(below, above, BLEND_ALPHA);
}

if (selection == SELECT_MOON) { // moon mode
    vec2 below_coord = src_coord0;
    vec2 xform = (src_coord4 - vec2(0.0, 0.0)) * 2.0 ;
    vec4 below = vec4(handle_edge(src_tex4, xform, EDGE_MODE_BLANK), 1.0);
    vec4 above = texture(src_tex7, src_coord7);

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
