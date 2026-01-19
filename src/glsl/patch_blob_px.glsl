

#define PATCH_BLOB_PX_ANGLE_BUCKET_COUNT 10.0
#define PATCH_BLOB_PX_OFFSET_CYCLE 600.0
#define PATCH_BLOB_PX_OFFSET_BUCKET_COUNT 600.0
#define PATCH_BLOB_PX_BUCKET_SEED 43758.5453

float patch_blob_px_get_radius(
    float angle_deg,
    int index,
    float offset,
    float blob_radius
) {

    // bucket offset (aka time) into slices and loop them
    float offset_bucket1 = mod(offset, PATCH_BLOB_PX_OFFSET_CYCLE) / 
        (PATCH_BLOB_PX_OFFSET_CYCLE / PATCH_BLOB_PX_OFFSET_BUCKET_COUNT);
    
    float offset_bucket2 = mod(offset_bucket1 + 1.0, PATCH_BLOB_PX_OFFSET_BUCKET_COUNT);
   
    // interpolation factor of time
    float t = fract(offset_bucket1);


    float angle_bucket = mod(
        (angle_deg / (360.0 / PATCH_BLOB_PX_ANGLE_BUCKET_COUNT) + float(index)), 
        PATCH_BLOB_PX_ANGLE_BUCKET_COUNT);
    uint i = uint(floor(angle_bucket));

    float r = randf(i);

    // point 0, current time mixed with next time
    uint j1 = uint(floor(mod(offset_bucket1, PATCH_BLOB_PX_OFFSET_BUCKET_COUNT)))
        + uint(float(i) * PATCH_BLOB_PX_BUCKET_SEED);
    uint j2 = uint(floor(mod(offset_bucket2, PATCH_BLOB_PX_OFFSET_BUCKET_COUNT)))
        + uint(float(i) * PATCH_BLOB_PX_BUCKET_SEED);
    float s = mix(randf(j1), randf(j2), t);

    return blob_radius * 0.6 + r * blob_radius * 0.05 + s * blob_radius * 0.2;
}

vec4 patch_blob_px(
    vec2 uv,
    vec2 resolution,
    vec4 color_in,
    vec4 blob_color,
    vec2 blob_center,
    float blob_radius,
    float offset
) {

    float angle = atan(uv.y - blob_center.y, uv.x - blob_center.x);
    // convert angle to degrees manually since there is no degrees() function
    float angle_deg = mod(angle * (180.0 / 3.14159265), 360.0);
    float bucket = angle_deg / (360.0 / PATCH_BLOB_PX_ANGLE_BUCKET_COUNT);
    // interpolation factor of angle
    float f = fract(bucket);

    float d0 = patch_blob_px_get_radius(angle_deg, -1, offset, blob_radius);
    float d1 = patch_blob_px_get_radius(angle_deg, 0, offset, blob_radius);
    float d2 = patch_blob_px_get_radius(angle_deg, 1, offset, blob_radius);
    float d3 = patch_blob_px_get_radius(angle_deg, 2, offset, blob_radius);

    vec4 return_color = color_in;
    float q = bicubic_mix(d0, d1, d2, d3, f);
    float m = distance(uv, blob_center);

    if (m < q) {
        return_color = blob_color;
        float alpha = 1.0;
        if (m > blob_radius * .4) {
            alpha = mix(1.0, 0.0, (m - blob_radius * 0.4) / (blob_radius * 0.4));
        }
        return_color.a = clamp(alpha, 0.0, 1.0);
        return_color = blend_by_mode(color_in, return_color, BLEND_ALPHA);
    }
    
    
    return return_color;
}