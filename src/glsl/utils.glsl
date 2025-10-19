/* Shader toy compatibility inputs */
//!VAR vec3 iResolution 1.0 1.0 1.0
//!VAR float iTime 0.0
//!VAR float iTimeDelta 0.0
//!VAR float iFrame 0.0
//!VAR float iSampleRate 0.0

//unimplimented //!VAR vec4 iChannelTime 0.0 0.0 0.0 0.0
//unimplimented //!VAR vec4 iMouse 0.0 0.0 0.0 0.0
//unimplimented //!VAR vec4 iDate 0.0 0.0 0.0 0.0
//unimplimented //!VAR mat4x3 iChannelResolution 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0

#define iChannel0 src_tex0
#define iChannel1 src_tex1
#define iChannel2 src_tex2
#define iChannel3 src_tex3

#define M_PI 3.1415926535897932384626433832795
#define M_E 2.7182818284590452353602874713527
#define EPSILON 1e-10

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
#define EDGE_MODE_WRAP_LR 4
#define EDGE_MODE_WRAP_UD 5
#define EDGE_MODE_MIRROR_LR 6
#define EDGE_MODE_MIRROR_UD 7

vec2 coord_smear(in vec2 coord) {
    // Smear the coordinates to avoid edge artifacts
    return clamp(coord, EPSILON, 1 - EPSILON);
}

vec2 coord_wrap(in vec2 coord, in bool wrap_lr, in bool wrap_ud) {
    // Wrap the coordinates to stay within [0, 1]
    if (coord.x < 0.0 && wrap_lr) {
        coord.x = 1.0 + fract(coord.x);
    } else if (coord.x > 1.0 && wrap_lr) {
        coord.x = fract(coord.x);
    }
    if (coord.y < 0.0 && wrap_ud) {
        coord.y = 1.0 + fract(coord.y);
    } else if (coord.y > 1.0 && wrap_ud) {
        coord.y = fract(coord.y);
    }
    return fract(coord);
}

vec2 coord_mirror(in vec2 coord, in bool mirror_lr, in bool mirror_ud) {
    vec2 base_coord = vec2(fract(coord.x), fract(coord.y));
    if (base_coord.x == 0.0 && coord.x != 0.0 && mirror_lr) {
        base_coord.x = 1.0;
    }
    if (base_coord.y == 0.0 && coord.y != 0.0 && mirror_ud) {
        base_coord.y = 1.0;
    }
    if ((int(floor(coord.x)) & 1) == 1 && mirror_lr) {
        // If the x coordinate is odd, flip the x coordinate
        base_coord.x = 1.0 - base_coord.x;
    }
    if ((int(floor(coord.y)) & 1) == 1 && mirror_ud) {
        // If the y coordinate is odd, flip the y coordinate
        base_coord.y = 1.0 - base_coord.y;
    }
    return base_coord;
}

vec3 handle_edge(sampler2D tex, in vec2 coord, in uint mode) {
    switch (mode) {
        case EDGE_MODE_SMEAR:
            return texture(tex, coord_smear(coord)).rgb;

        case EDGE_MODE_WRAP:
            return texture(tex, coord_wrap(coord, true, true)).rgb;

        case EDGE_MODE_MIRROR:
            return texture(tex, coord_mirror(coord, true, true)).rgb;

        case EDGE_MODE_BLANK:
            if (coord.x <= EPSILON || coord.x >= (1.0 - EPSILON) ||
                coord.y <= EPSILON || coord.y >= (1.0 - EPSILON)) {
                return vec3(0.0, 0.0, 0.0);
            } else {
                return texture(tex, coord).rgb;
            }

        case EDGE_MODE_WRAP_LR:
            if (coord.y <= EPSILON || coord.y >= (1.0 - EPSILON)) {
               return vec3(0.0, 0.0, 0.0);
            }
            return texture(tex, coord_wrap(coord, true, false)).rgb;

        case EDGE_MODE_WRAP_UD:
            if (coord.x <= EPSILON || coord.x >= (1.0 - EPSILON)) {
                return vec3(0.0, 0.0, 0.0);
            }
            return texture(tex, coord_wrap(coord, false, true)).rgb;

        case EDGE_MODE_MIRROR_LR:
            if (coord.y <= EPSILON || coord.y >= (1.0 - EPSILON)) {
               return vec3(0.0, 0.0, 0.0);
            }
            return texture(tex, coord_mirror(coord, true, false)).rgb;

        case EDGE_MODE_MIRROR_UD:
            if (coord.x <= EPSILON || coord.x >= (1.0 - EPSILON)) {
                return vec3(0.0, 0.0, 0.0);
            }
            return texture(tex, coord_mirror(coord, false, true)).rgb;

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

/**
 * Performs a perspective transformation for point 'p' based on 'corners'.
 * This maps straight lines in the source (p) space to straight lines in the destination space.
 * 'p' is assumed to be normalized coordinates (e.g., p.x and p.y in [0,1]).
 * 'corners' is a mat4x2 where:
 *   corners[0] is the new position for the original (0,0) corner.
 *   corners[1] is the new position for the original (1,0) corner.
 *   corners[2] is the new position for the original (0,1) corner.
 *   corners[3] is the new position for the original (1,1) corner.
 * Returns the new transformed 2D position for 'p'.
 */
vec2 skew2(in vec2 p, in mat4x2 corners) {
    vec2 P00 = corners[0]; // New position for original (0,0)
    vec2 P10 = corners[1]; // New position for original (1,0)
    vec2 P01 = corners[2]; // New position for original (0,1)
    vec2 P11 = corners[3]; // New position for original (1,1)

    float u = p.x;
    float v = p.y;

    // Coefficients for solving for g, h in perspective transform
    // x' = (ax*u + bx*v + cx) / (g*u + h*v + 1)
    // y' = (ay*u + by*v + cy) / (g*u + h*v + 1)
    // System to find g, h:
    // A1*g + B1*h = C1
    // A2*g + B2*h = C2
    float A1 = P10.x - P11.x;
    float B1 = P01.x - P11.x;
    float C1 = P11.x - P10.x - P01.x + P00.x;

    float A2 = P10.y - P11.y;
    float B2 = P01.y - P11.y;
    float C2 = P11.y - P10.y - P01.y + P00.y;

    float det_gh = A1 * B2 - A2 * B1;
    float math_epsilon = 1e-6; // Small epsilon for floating point comparisons

    vec2 new_p;

    if (abs(det_gh) < math_epsilon) {
        // Quadrilateral is a parallelogram or degenerate.
        // Bilinear interpolation is affine and thus "linear" in this case.
        vec2 bottom_interp = mix(P00, P10, u);
        vec2 top_interp    = mix(P01, P11, u);
        new_p              = mix(bottom_interp, top_interp, v);
    } else {
        float g = (C1 * B2 - C2 * B1) / det_gh;
        float h = (A1 * C2 - A2 * C1) / det_gh;

        // Numerator for x: Nx = c0*u + c1*v + c2
        // c0 = (g+1)*P10.x - P00.x
        // c1 = (h+1)*P01.x - P00.x
        // c2 = P00.x
        float num_x = ((g + 1.0) * P10.x - P00.x) * u +
                      ((h + 1.0) * P01.x - P00.x) * v +
                      P00.x;

        // Numerator for y: Ny = c3*u + c4*v + c5
        // c3 = (g+1)*P10.y - P00.y
        // c4 = (h+1)*P01.y - P00.y
        // c5 = P00.y
        float num_y = ((g + 1.0) * P10.y - P00.y) * u +
                      ((h + 1.0) * P01.y - P00.y) * v +
                      P00.y;

        float den = g * u + h * v + 1.0;

        if (abs(den) < math_epsilon) {
            // Denominator is zero or very close to zero.
            // This can happen if p maps to a point at infinity (e.g., non-convex or degenerate quad).
            // Fallback to bilinear interpolation for stability.
            vec2 bottom_interp = mix(P00, P10, u);
            vec2 top_interp    = mix(P01, P11, u);
            new_p              = mix(bottom_interp, top_interp, v);
        } else {
            new_p.x = num_x / den;
            new_p.y = num_y / den;
        }
    }
    
    return new_p;
}


// Helper function to get luminance (brightness) of a color
float get_luminance(vec3 color_val) {
    return dot(color_val, vec3(0.299, 0.587, 0.114));
}

// Helper function for 2D cross product (z-component)
// Returns > 0 if p is to the left of segment v_start -> v_end
// Returns < 0 if p is to the right
// Returns = 0 if p is collinear
float cross_product_z(in vec2 v_start, in vec2 v_end, in vec2 p) {
    return (v_end.x - v_start.x) * (p.y - v_start.y) - (v_end.y - v_start.y) * (p.x - v_start.x);
}

/**
 * Performs an inverse perspective transformation.
 * Given a point 'p_target' in the destination space and the 'corners_dest' of a quadrilateral,
 * this function finds the (u,v) coordinates in the normalized [0,1]x[0,1] source square
 * that map to 'p_target'.
 * 'corners_dest' is a mat4x2 where:
 *   corners_dest[0] is the destination for the original (0,0) source corner (P00).
 *   corners_dest[1] is the destination for the original (1,0) source corner (P10).
 *   corners_dest[2] is the destination for the original (0,1) source corner (P01).
 *   corners_dest[3] is the destination for the original (1,1) source corner (P11).
 * Returns the (u,v) coordinates in the source unit square.
 */
vec2 skew3(in vec2 p_target, in mat4x2 corners_dest) {
    vec2 P00 = corners_dest[0];
    vec2 P10 = corners_dest[1];
    vec2 P01 = corners_dest[2];
    vec2 P11 = corners_dest[3];

    float math_epsilon = 1e-6;
    vec2 uv_result;

    // Calculate g_fwd, h_fwd (perspective parameters for P00..P11 as destination)
    // This is the same setup as in skew2 for calculating g, h
    float A1_gh = P10.x - P11.x;
    float B1_gh = P01.x - P11.x;
    float C1_gh = P11.x - P10.x - P01.x + P00.x;
    float A2_gh = P10.y - P11.y;
    float B2_gh = P01.y - P11.y;
    float C2_gh = P11.y - P10.y - P01.y + P00.y;

    float det_gh = A1_gh * B2_gh - A2_gh * B1_gh;

    if (abs(det_gh) < math_epsilon) { // Affine case (quadrilateral is a parallelogram)
        // The forward transform is: p_target = P00 + u*(P10-P00) + v*(P01-P00)
        // We need to solve for u,v:
        // (P10.x - P00.x)*u + (P01.x - P00.x)*v = p_target.x - P00.x
        // (P10.y - P00.y)*u + (P01.y - P00.y)*v = p_target.y - P00.y
        float ax_aff = P10.x - P00.x;
        float bx_aff = P01.x - P00.x;
        float cx_target = p_target.x - P00.x;

        float ay_aff = P10.y - P00.y;
        float by_aff = P01.y - P00.y;
        float cy_target = p_target.y - P00.y;

        float det_affine_uv = ax_aff * by_aff - bx_aff * ay_aff;

        if (abs(det_affine_uv) < math_epsilon) {
            // Degenerate parallelogram (P00, P10, P01 are collinear).
            // Fallback: project onto basis vectors if possible.
            vec2 dPu = P10 - P00;
            vec2 dPv = P01 - P00;
            vec2 p_rel = p_target - P00;
            if (dot(dPu, dPu) > math_epsilon) {
                 uv_result.x = dot(p_rel, dPu) / dot(dPu, dPu);
                 uv_result.y = 0.0; // Assume v=0 if P01-P00 is problematic
                 // A more robust solution might check if p_rel is also collinear with dPv
            } else if (dot(dPv, dPv) > math_epsilon) {
                 uv_result.y = dot(p_rel, dPv) / dot(dPv, dPv);
                 uv_result.x = 0.0; // Assume u=0
            } else { // P00=P10=P01, very degenerate
                 uv_result = vec2(0.0,0.0); // Or (0.5,0.5) or based on p_target vs P00
            }
        } else {
            uv_result.x = (cx_target * by_aff - bx_aff * cy_target) / det_affine_uv;
            uv_result.y = (ax_aff * cy_target - cx_target * ay_aff) / det_affine_uv;
        }
    } else { // Perspective case
        float g_fwd = (C1_gh * B2_gh - C2_gh * B1_gh) / det_gh;
        float h_fwd = (A1_gh * C2_gh - A2_gh * C1_gh) / det_gh;

        // Coefficients of the forward transform:
        // p_target.x = (a_fwd*u + b_fwd*v + c_fwd) / (g_fwd*u + h_fwd*v + 1)
        // p_target.y = (d_fwd*u + e_fwd*v + f_fwd) / (g_fwd*u + h_fwd*v + 1)
        float c_fwd = P00.x;
        float f_fwd = P00.y;
        float a_fwd = P10.x * (g_fwd + 1.0) - P00.x;
        float d_fwd = P10.y * (g_fwd + 1.0) - P00.y;
        float b_fwd = P01.x * (h_fwd + 1.0) - P00.x;
        float e_fwd = P01.y * (h_fwd + 1.0) - P00.y;
        
        // Rearrange to solve for u,v:
        // (a_fwd - p_target.x*g_fwd)*u + (b_fwd - p_target.x*h_fwd)*v = p_target.x - c_fwd
        // (d_fwd - p_target.y*g_fwd)*u + (e_fwd - p_target.y*h_fwd)*v = p_target.y - f_fwd
        float A_uv = a_fwd - p_target.x * g_fwd;
        float B_uv = b_fwd - p_target.x * h_fwd;
        float C_target = p_target.x - c_fwd;

        float D_uv = d_fwd - p_target.y * g_fwd;
        float E_uv = e_fwd - p_target.y * h_fwd;
        float F_target = p_target.y - f_fwd;

        float det_uv = A_uv * E_uv - B_uv * D_uv;

        if (abs(det_uv) < math_epsilon) {
            // Denominator is zero. p_target might be on the horizon line of the transformation.
            // Fallback to affine approximation for stability.
            float ax_aff_fallback = P10.x - P00.x;
            float bx_aff_fallback = P01.x - P00.x;
            float cx_target_fallback = p_target.x - P00.x;
            float ay_aff_fallback = P10.y - P00.y;
            float by_aff_fallback = P01.y - P00.y;
            float cy_target_fallback = p_target.y - P00.y;
            float det_affine_uv_fallback = ax_aff_fallback * by_aff_fallback - bx_aff_fallback * ay_aff_fallback;

            if (abs(det_affine_uv_fallback) < math_epsilon) {
                 uv_result = vec2(0.5,0.5); // Center of square if all else fails
            } else {
                 uv_result.x = (cx_target_fallback * by_aff_fallback - bx_aff_fallback * cy_target_fallback) / det_affine_uv_fallback;
                 uv_result.y = (ax_aff_fallback * cy_target_fallback - cx_target_fallback * ay_aff_fallback) / det_affine_uv_fallback;
            }
        } else {
            uv_result.x = (C_target * E_uv - B_uv * F_target) / det_uv;
            uv_result.y = (A_uv * F_target - C_target * D_uv) / det_uv;
        }
    }
    return uv_result;
}

// PCG-based hash function to generate a pseudo-random uint from a uint seed.
// Adapted from: https://www.reedbeta.com/blog/hash-functions-for-gpu-rendering/
uint pcg(uint v) {
    v = v * 747796405u + 2891336453u; // LCG step
    uint state = v;
    // XSH RR (xorshift and random rotate)
    uint word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word; // Output mixing
}

// Converts a uint hash to a float in [0, 1)
// Using 2^32 as the divisor for [0,1) range.
float uint_to_float01(uint h) {
    return float(h) * (1.0 / 4294967296.0);
}

// Generates a pseudo-random float uniformly distributed between -1.0 and 1.0.
float randf(uint seed) {
    // Generate a uniform random number in [0,1) from the seed
    uint hash1 = pcg(seed);
    float u1 = uint_to_float01(hash1); // u1 is in [0, 1)

    // Scale and shift to [-1.0, 1.0)
    // (u1 * 2.0) maps [0, 1) to [0, 2)
    // (u1 * 2.0 - 1.0) maps [0, 2) to [-1.0, 1.0)
    return u1 * 2.0 - 1.0;
}

vec3 hsv2rgb(in vec3 c) {
    vec4 K = vec4(1.0, 2.0/3.0, 1.0/3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}