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


// scale to fill horizontal
float aspect0 = iResolution0.x / iResolution0.y;
float aspect1 = iResolution1.x / iResolution1.y;
float scale0 = aspect0 / aspect1;
vec2 offset0 = vec2(0.0, 0.25);
vec2 below_coord = src_coord0 * vec2(1.0, 1.0) + offset0;
vec4 below = vec4(handle_edge(src_tex0, below_coord, EDGE_MODE_BLANK), 1.0);

// skew the grid
mat4x2 new_corners = mat4x2(0.0, 0.6, 1.0, 0.6, -1.75, 1.0, 1.75, 1.0);
vec2 above_coord = skew3(src_coord1, new_corners);
vec4 above = texture(src_tex1, above_coord);

if (src_coord1.y <= 0.6) {
    above.a = 0.0;  // If above the midpoint, set alpha to 0
} 

color = blend_by_mode(below, above, BLEND_ALPHA);


color = blast(color, usr_var, src_coord3, src_tex3, src_coord4, src_tex4);
