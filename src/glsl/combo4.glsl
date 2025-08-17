//!VAR int usr_var 0

// below
//!VAR vec2 iResolution0 1.0 1.0
// above
//!VAR vec2 iResolution1 1.0 1.0
// blank
//!VAR vec2 iResolution2 1.0 1.0
// error2
//!VAR vec2 iResolution3 1.0 1.0
// error3
//!VAR vec2 iResolution4 1.0 1.0

vec2 above_coord = src_coord0 + vec2(0.0, -30.0 / iResolution0.y);
vec4 below = texture(src_tex1, src_coord1);
vec4 above = texture(src_tex0, src_coord0);

// for when the statue is below
// float w = 400.0;
// float h = 400.0 * iResolution1.y / iResolution1.x;
// float left = (iResolution1.x - w)/2.0/iResolution1.x;
// float right = left + w / iResolution1.x;
// float top = (iResolution1.y - h)/2.0/iResolution1.y;
// float bottom = top + h / iResolution1.y;

// if (src_coord1.x > left && src_coord1.x < right && src_coord1.y > top && src_coord1.y < bottom) {
//     above.a = 0.0;  // If above is almost black, set alpha to 0
// } 

// for when the status is above
if (distance(above.rgb, vec3(0.0)) < 0.3) {
    above.a = 0.0;  // If above is almost black, set alpha to 0
}

color = blend_by_mode(below, above, BLEND_ALPHA);


color = blast(color, usr_var, src_coord3, src_tex3, src_coord4, src_tex4);