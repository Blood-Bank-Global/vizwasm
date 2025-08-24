vec4 blast(in vec4 color,
    in uint usr_var, 
    in vec2 error2_coord,
    in sampler2D error2_tex, 
    in vec2 error3_coord, 
    in sampler2D error3_tex) {

    // Blast on error effects
    switch (usr_var) {
        case 0:
            break;
        case 1:
            {
                vec4 blast = texture(error2_tex, error2_coord);
                if (distance(blast.rgb, vec3(0.0)) < 0.3) {
                    blast.a = 0.0;  // If blast is almost black, set alpha to 0
                } 
                color = blend_by_mode(color, blast, BLEND_ALPHA);
                break;
            }
        case 2:
            {
                vec4 blast = texture(error3_tex, error3_coord);
                if (distance(blast.rgb, vec3(0.0)) < 0.3) {
                    blast.a = 0.0;  // If blast is almost black, set alpha to 0
                } 
                color = blend_by_mode(color, blast, BLEND_ALPHA);
                break;
            }
        default:
            break;
    }
    return color;
}