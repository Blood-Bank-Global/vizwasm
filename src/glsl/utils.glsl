#define BLEND_ADDITION 1
#define BLEND_AND 2
#define BLEND_AVERAGE 3
#define BLEND_DARKEN 4
#define BLEND_DIFFERENCE 5
#define BLEND_DIVIDE 6
#define BLEND_LIGHTEN 7
#define BLEND_OR 8
#define BLEND_OVERLAY 9
#define BLEND_SCREEN 10
#define BLEND_SUBTRACT 11
#define BLEND_XOR 12
#define BLEND_ALPHA 13
#define BLEND_DISABLE 0
#define BLEND_MODE_COUNT 14
#define BLEND_MODE_DEFAULT 0
vec4 blend_by_mode(in vec4 below, in vec4 above, in uint kind) {
    vec4 color = vec4(0.0);
    switch (kind) {
    case BLEND_ADDITION:
        //addition
        color = clamp((below + above), 0.0, 1.0);
        break;
    case BLEND_AND:
        // and
        color = vec4((uvec4(below * 255.0) & uvec4(above * 255.0)) / 255.0);
        break;
    case BLEND_AVERAGE:
        //average
        color = (below + above) * 0.5;
        break;
    case BLEND_DARKEN:
        //darken          
        color = min(below, above);
        break;
    case BLEND_DIFFERENCE:
        //difference
        color = clamp(below - above, 0.0, 1.0);
        break;
    case BLEND_DIVIDE:
        //divide
        color = below / above;
        break;
    case BLEND_LIGHTEN:
        //lighten
        color = max(below, above);
        break;
    case BLEND_OR:
        // or
        color = vec4((uvec4(below * 255.0) | uvec4(above * 255.0)) / 255.0);
        break;
    case BLEND_OVERLAY:
        //overlay
        if (below.r < 0.5) {
            color = 2.0 * below * above;
        } else {
            color = 1.0 - 2.0 * (1.0 - below) * (1.0 - above);
        }
        break;
    case BLEND_SCREEN:
        //screen
        color = 1 - ((1 - below) * (1 - above));
        break;
    case BLEND_SUBTRACT:
        //subtract
        color = clamp(below - above, 0.0, 1.0);
        break;
    case BLEND_XOR:
        //xor
        color = vec4((uvec4(below * 255.0) ^ uvec4(above * 255.0)) / 255.0);
        break;
    case BLEND_ALPHA:
        //alpha
        color = vec4(above.a * above.rgb + below.a * below.rgb * (1.0 - above.a), 1.0);
        break;
    default:
        //disable
        color = below;
        break;
    }
    return color;
}

vec3 distort(in vec2 coord, in sampler2D map, in float level) {
    vec3 delta = texture(map, coord).rgb - 0.5;

    #define EPSILON 0.001
    if (abs(delta.r) < EPSILON) {
        delta.r = 0.0;
    }
    if (abs(delta.g) < EPSILON) {
        delta.g = 0.0;
    }
    if (abs(delta.b) < EPSILON) {
        delta.b = 0.0;
    }

    return clamp(delta * level, -1.0, 1.0);
}

#define EDGE_MODE_SMEAR 0
#define EDGE_MODE_WRAP 1
#define EDGE_MODE_MIRROR 2
#define EDGE_MODE_BLANK 3

vec3 handle_edge(sampler2D tex, in vec2 coord, in uint mode) {
    switch (mode) {
        case EDGE_MODE_SMEAR:
            return texture(tex, clamp(coord, 0.01, 0.99)).rgb;

        case EDGE_MODE_WRAP:
            if (coord.x < 0.0) {
                coord.x = 1.0 + fract(coord.x);
            } else if (coord.x > 1.0) {
                coord.x = fract(coord.x);
            }
            if (coord.y < 0.0) {
                coord.y = 1.0 + fract(coord.y);
            } else if (coord.y > 1.0) {
                coord.y = fract(coord.y);
            }
            return texture(tex, coord).rgb;

        case EDGE_MODE_MIRROR:
            if (coord.x < 0.0) {
                coord.x = -fract(coord.x);
            } else if (coord.x > 1.0) {
                coord.x = 1 - fract(coord.x);
            }
            if (coord.y < 0.0) {
                coord.y = -fract(coord.y);
            } else if (coord.y > 1.0) {
                coord.y = 1 - fract(coord.y);
            }
            return texture(tex, coord).rgb;

        case EDGE_MODE_BLANK:
            if (coord.x < 0.0 || coord.x > 1.0 || coord.y < 0.0 || coord.y > 1.0) {
                return vec3(0.0, 0.0, 0.0);
            } else {
                return texture(tex, coord).rgb;
            }

        default:
            return vec3(1.0, 0.0, 0.0);
    }
}

/**
 * Performs a bilinear interpolation for point 'p' based on 'corners'.
 * 'p' is assumed to be normalized coordinates (e.g., p.x and p.y in [0,1]).
 * 'corners' is a mat4x2 where:
 *   corners[0] is the new position for the original (0,0) corner.
 *   corners[1] is the new position for the original (1,0) corner.
 *   corners[2] is the new position for the original (0,1) corner.
 *   corners[3] is the new position for the original (1,1) corner.
 * Returns the new interpolated 2D position for 'p'.
 */
 
vec2 skew(in vec2 p, in mat4x2 corners) {
    // Extract the new corner positions
    vec2 nc00 = corners[0]; // New position for original (0,0)
    vec2 nc10 = corners[1]; // New position for original (1,0)
    vec2 nc01 = corners[2]; // New position for original (0,1)
    vec2 nc11 = corners[3]; // New position for original (1,1)

    // Interpolate along the x-axis for the bottom edge (y=0)
    vec2 bottom_interp = mix(nc00, nc10, p.x);
    
    // Interpolate along the x-axis for the top edge (y=1)
    vec2 top_interp = mix(nc01, nc11, p.x);
    
    // Interpolate along the y-axis between the bottom and top interpolated points
    vec2 new_p = mix(bottom_interp, top_interp, p.y);
    
    return new_p;
}

// Helper function to get luminance (brightness) of a color
float get_luminance(vec3 color_val) {
    return dot(color_val, vec3(0.299, 0.587, 0.114));
}