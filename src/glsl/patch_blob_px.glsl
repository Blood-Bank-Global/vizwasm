

#define PATCH_BLOB_PX_ANGLE_BUCKET_COUNT 36.0
#define PATCH_BLOB_PX_OFFSET_CYCLE 40.0
#define PATCH_BLOB_PX_OFFSET_BUCKET_COUNT 60.0
#define PATCH_BLOB_PX_BUCKET_SEED 43758.5453

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
    float angle_bucket = angle_deg / (360.0 / PATCH_BLOB_PX_ANGLE_BUCKET_COUNT);
    float angle_bucket_next = angle_bucket + 1.0;
    if (angle_bucket_next >= PATCH_BLOB_PX_ANGLE_BUCKET_COUNT) {
        angle_bucket_next = 0.0;
    }

    // current point, next point
    uint i1 = uint(floor(angle_bucket));
    uint i2 = uint(floor(angle_bucket_next));
    // interpolation factor of angle
    float f = fract(angle_bucket);

    float r1 = randf(i1);
    float r2 = randf(i2);

    // bucket offset (aka time) into slices and loop them
    float offset_bucket = mod(offset, PATCH_BLOB_PX_OFFSET_CYCLE) / 
        (PATCH_BLOB_PX_OFFSET_CYCLE / PATCH_BLOB_PX_OFFSET_BUCKET_COUNT);
    float offset_bucket_next = offset_bucket + 1.0;
    if (offset_bucket_next >= PATCH_BLOB_PX_OFFSET_BUCKET_COUNT) {
        offset_bucket_next = 0.0;
    }

    // interpolation factor of time
    float t = fract(offset_bucket);

    // point 1, current time mixed with next time
    uint j11 = uint(floor(mod(offset_bucket, PATCH_BLOB_PX_OFFSET_BUCKET_COUNT)))
        + uint(float(i1) * PATCH_BLOB_PX_BUCKET_SEED);
    uint j12 = uint(floor(mod(offset_bucket_next, PATCH_BLOB_PX_OFFSET_BUCKET_COUNT)))
        + uint(float(i1) * PATCH_BLOB_PX_BUCKET_SEED);
    float s1 = mix(randf(j11), randf(j12), t);

    // point 2, current time mixed with next time
    uint j21 = uint(floor(mod(offset_bucket, PATCH_BLOB_PX_OFFSET_BUCKET_COUNT)))
        + uint(float(i2) * PATCH_BLOB_PX_BUCKET_SEED);
    uint j22 = uint(floor(mod(offset_bucket_next, PATCH_BLOB_PX_OFFSET_BUCKET_COUNT)))
        + uint(float(i2) * PATCH_BLOB_PX_BUCKET_SEED);
    float s2 = mix(randf(j21), randf(j22), t);


    // radius is fixed radius of point plus current offet at point
    float d1 = blob_radius * 0.6 + r1 * blob_radius * 0.05 + s1 * blob_radius * 0.2;
    float d2 = blob_radius * 0.6 + r2 * blob_radius * 0.05 + s2 * blob_radius * 0.2;
    float d = mix(d1, d2, f);
    
    vec4 return_color = color_in;

    float m = distance(uv, blob_center);
    if (m < d) {
        if (mod(i1, 2u) == 0u) {
            return_color = vec4(0.0, 0.5, 1.0, 1.0);
        } else {
            return_color = blob_color;
        }
        
        float alpha = 1.0;
        if (m > blob_radius * .4) {
            alpha = mix(1.0, 0.0, (m - blob_radius * 0.4) / (blob_radius * 0.4));
        }
        return_color.a = clamp(alpha, 0.0, 1.0);
        return_color = blend_by_mode(color_in, return_color, BLEND_ALPHA);
    }
    
    
    return return_color;
}