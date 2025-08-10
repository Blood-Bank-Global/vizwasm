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


vec4 below = texture(src_tex3, src_coord3);
vec4 above = texture(src_tex4, src_coord4);

if (distance(above.rgb, vec3(0.0)) < 0.3) {
    above.a = 0.0;  // If above is almost black, set alpha to 0
} 

color = blend_by_mode(below, above, BLEND_ALPHA);

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
