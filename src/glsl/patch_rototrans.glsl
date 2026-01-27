
vec4 patch_rototrans(
    vec2 coord, 
    sampler2D feedback_tex,
    sampler2D distort_x_tex, 
    sampler2D distort_y_tex,
    float rotation, 
    float distort_x,
    float distort_y,
    float distort_level, 
    uint distort_edge) {

    vec2 distort_coord = coord;
    // rotation
    mat2 rot = mat2(
        cos(rotation), -sin(rotation),
        sin(rotation),  cos(rotation)
    );

    vec2 center = vec2(0.5, 0.5);
    distort_coord -= center;
    distort_coord *= rot;
    distort_coord += center;

    vec4 distort_dx_combined = vec4(distort_coord.x - distort_dx) + vec4(distort(distort_coord, distort_x_tex, distort_level), 0.0);
    vec4 distort_dy_combined = vec4(distort_coord.y - distort_dy) + vec4(distort(distort_coord, distort_y_tex, distort_level), 0.0);
    mat4x2 distort_matrix = mat4x2(
        distort_dx_combined[0], distort_dy_combined[0],
        distort_dx_combined[1], distort_dy_combined[1],
        distort_dx_combined[2], distort_dy_combined[2],
        distort_dx_combined[3], distort_dy_combined[3]
    );

    return vec4(handle_edge(feedback_tex, distort_matrix[0], distort_edge).r,
                handle_edge(feedback_tex, distort_matrix[1], distort_edge).g,
                handle_edge(feedback_tex, distort_matrix[2], distort_edge).b,
                texture(feedback_tex, distort_matrix[3]).a);

}