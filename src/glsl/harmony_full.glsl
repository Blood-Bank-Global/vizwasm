
/**
/* 
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

//!VAR usr_var 0

vec4 below = texture(src_tex3, src_coord3);
vec4 above = texture(src_tex4, src_coord4);

if (distance(above.rgb, vec3(0.0)) < 0.3) {
    above.a = 0.0;  // If above is almost black, set alpha to 0
} 

color = blend_by_mode(below, above, BLEND_ALPHA);

if (usr_var == 1) {
    vec4 blast = texture(src_tex1, src_tex1);
    color = blend_by_mode(color, blast, BLEND_SCREEN);
}