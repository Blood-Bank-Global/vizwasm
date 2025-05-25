float get_x(in uint seed, in float frame) {
    float center = (1 + randf(seed))/2.0;
    seed += 1;
    float dev = 0.01 + 0.009 * randf(seed) / 2.0;
    seed += 1;

    float round1 = 200.0 + 100.0 * randf(seed);
    seed += 1;
    float round2 = 500.0 + 250.0 * randf(seed);
    seed += 1;
    float round3 = 1000.0 + 500.0 * randf(seed);

    float curr_dev = 
        dev 
        * sin(mod(frame, round1) / round1 * 2.0 * M_PI)
        * sin(mod(frame, round2) / round2 * 2.0 * M_PI)
        * sin(mod(frame, round3) / round3 * 2.0 * M_PI);

    return center + curr_dev;
}

float get_y(in uint seed, in float frame) {
    float offset = 125.0 * (1 + randf(seed))/2.0 + frame;
    seed += 1;
    float period = 120.0 + 105.0 * randf(seed);
    return sin(mod(offset, period) / period * M_PI / 2.0);
}

vec4 draw_blood(in vec2 src_coord0, in vec2 pt1, in vec2 pt2, in float r, in uint seed) {
    // Draw a blood smear between two points pt1 and pt2 with radius r
    // src_coord0: the coordinate of the pixel being processed
    // pt1, pt2: the two points defining the smear
    // r: the radius of the smear

    // Default color
   vec4 color = vec4(0.0, 0.0, 0.0, 1.0); // Default color

    // calculate blur zone
    mat4x2 zone = mat4x2(
        pt1.x - r, pt1.y,
        pt1.x + r, pt1.y,
        pt2.x - r, pt2.y,
        pt2.x + r, pt2.y
    );

    // Vertices from the zone matrix
    vec2 v_pt1_left  = zone[0]; // (pt1.x - r, pt1.y)
    vec2 v_pt1_right = zone[1]; // (pt1.x + r, pt1.y)
    vec2 v_pt2_left  = zone[2]; // (pt2.x - r, pt2.y)
    vec2 v_pt2_right = zone[3]; // (pt2.x + r, pt2.y)

    bool is_inside_zone;
    if (pt1.y <= pt2.y) {
        // Counter-clockwise order: pt1_left, pt1_right, pt2_right, pt2_left
        float c1 = cross_product_z(v_pt1_left,  v_pt1_right, src_coord0);
        float c2 = cross_product_z(v_pt1_right, v_pt2_right, src_coord0);
        float c3 = cross_product_z(v_pt2_right, v_pt2_left,  src_coord0);
        float c4 = cross_product_z(v_pt2_left,  v_pt1_left,  src_coord0);
        is_inside_zone = (c1 >= 0.0 && c2 >= 0.0 && c3 >= 0.0 && c4 >= 0.0);
    } else { // pt1.y > pt2.y (pt2 is "below" pt1)
        // Counter-clockwise order: pt2_left, pt2_right, pt1_right, pt1_left
        float c1 = cross_product_z(v_pt2_left,  v_pt2_right, src_coord0);
        float c2 = cross_product_z(v_pt2_right, v_pt1_right, src_coord0);
        float c3 = cross_product_z(v_pt1_right, v_pt1_left,  src_coord0);
        float c4 = cross_product_z(v_pt1_left,  v_pt2_left,  src_coord0);
        is_inside_zone = (c1 >= 0.0 && c2 >= 0.0 && c3 >= 0.0 && c4 >= 0.0);
    }

    float red = clamp((randf(seed) + 1.0) / 2.0 + 0.25, 0.0, 1.0);
    // check if src_coord0 is inside the blur zone
    if (is_inside_zone) {
        color = vec4(red, 0.0, 0.0, 1.0); // Example: color green if inside
    }

    // check if src_coord0 is inside the ellipse
    if (distance(src_coord0, pt1) < r || distance(src_coord0, pt2) < r) {
        color = vec4(red, 0.0, 0.0, 1.0);
    }

    return color;
}