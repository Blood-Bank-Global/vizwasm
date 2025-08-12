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

vec2 below_normalize = vec2((iResolution0.x/iResolution1.x) * src_coord1.x, (iResolution0.y/iResolution1.y) * src_coord1.y);
vec2 below_offset = vec2((1.0 - (iResolution0.x/iResolution1.x)) / 2.0, 0.0);
vec2 below_coord = below_normalize + below_offset;
vec4 below = vec4(handle_edge(src_tex0, below_coord, EDGE_MODE_BLANK), 1.0);

mat4x2 new_corners = mat4x2(0.0, 0.667, 1.0, 0.667, -2.0, 1.0, 2.0, 1.0);
vec2 above_coord = skew3(src_coord1, new_corners);
vec4 above = texture(src_tex1, above_coord);

if (src_coord1.y <= 0.6667) {
    above.a = 0.0;  // If above the midpoint, set alpha to 0
} 

color = blend_by_mode(below, above, BLEND_ALPHA);


color = blast(color, usr_var, src_coord3, src_tex3, src_coord4, src_tex4);
