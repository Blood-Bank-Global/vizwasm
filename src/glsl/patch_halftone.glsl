void patch_ordered_dither4x4(out vec4 color, in vec2 uv, in vec2 resolution, in sampler2D tex) {
    // https://en.wikipedia.org/wiki/Ordered_dithering
    // 4x4 Bayer matrix
    int x = int(mod(uv.x * resolution.x, 4.0));
    int y = int(mod(uv.y * resolution.y, 4.0));
    int index = x + y * 4;
    mat4x4 bayer = mat4x4(
        0.0,  8.0,  2.0, 10.0,
        12.0, 4.0, 14.0, 6.0,
        3.0, 11.0, 1.0, 9.0,
        15.0, 7.0, 13.0, 5.0
    ) / 16.0;
    vec4 c = texture(tex, uv);
    color.rgba = step(vec4(bayer[index / 4][index % 4]), c.rgba);
}

void patch_ordered_dither8x8(out vec4 color, in vec2 uv, in vec2 resolution, in sampler2D tex) {
    // 8x8 Bayer matrix
    int x = int(mod(uv.x * resolution.x, 8.0));
    int y = int(mod(uv.y * resolution.y, 8.0));
    int index = x + y * 8;
    float bayer[64] = float[64](
        0.0, 32.0, 8.0, 40.0, 2.0, 34.0, 10.0, 42.0,
        48.0, 16.0, 56.0, 24.0, 50.0, 18.0, 58.0, 26.0,
        12.0, 44.0, 4.0, 36.0, 14.0, 46.0, 6.0, 38.0,
        60.0, 28.0, 52.0, 20.0, 62.0, 30.0, 54.0, 22.0,
        3.0, 35.0, 11.0, 43.0, 1.0, 33.0, 9.0, 41.0,
        51.0, 19.0, 59.0, 27.0, 49.0, 17.0, 57.0, 25.0,
        15.0, 47.0, 7.0, 39.0, 13.0, 45.0, 5.0, 37.0,
        63.0, 31.0, 55.0, 23.0, 61.0, 29.0, 53.0, 21.0
    );
    vec4 c = texture(tex, uv);
    color.rgba = step(vec4(bayer[index / 8 + index % 8]/64.0), c.rgba);
}


void patch_halftone45(out vec4 color, in vec2 uv, in vec2 resolution, in sampler2D tex, float scale, float angle_deg) {
    // https://en.wikipedia.org/wiki/Halftone#45-degree_angle
    vec2 coord = uv * resolution / scale;
    float angle = radians(angle_deg);
    mat2 rot = mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
    vec2 rotatedCoord = rot * coord;
    vec2 localPos = fract(rotatedCoord);
    float radius = length(localPos - vec2(0.5));
    vec4 c = texture(tex, uv);
    color.rgba = step(vec4(radius), c.rgba);
}

void patch_dither_vertical(out vec4 color, in vec2 uv, in vec2 resolution, in sampler2D tex, uint line_width) {
    // Simple vertical lines halftone
    uint x = uint(resolution.x / float(line_width));
    // average color for each channel
    vec4 avg = vec4(0.0);
    for (uint i = 0u; i < line_width; i++) {
        avg += texture(tex, (uv * resolution + vec2(float(i), 0.0)) / resolution)/vec4(float(line_width));
    }
    vec4 fill = avg * vec4(float(line_width));

    int dx = int(line_width/2) - int(mod(uv.x * resolution.x, float(line_width)));
    int pos = dx < 0 ? abs(dx) * 2 + 1 : dx * 2;
    color.rgba = step(vec4(float(pos)), floor(fill) - 1.0);
}

void patch_dither_horizontal(out vec4 color, in vec2 uv, in vec2 resolution, in sampler2D tex, uint line_width) {
    // Simple horizontal lines halftone
    uint y = uint(resolution.y / float(line_width));
    // average color for each channel
    vec4 avg = vec4(0.0);
    for (uint i = 0u; i < line_width; i++) {
        avg += texture(tex, (uv * resolution + vec2(0.0, float(i))) / resolution)/vec4(float(line_width));
    }
    vec4 fill = avg * vec4(float(line_width));

    int dy = int(line_width/2) - int(mod(uv.y * resolution.y, float(line_width)));
    int pos = dy < 0 ? abs(dy) * 2 + 1 : dy * 2;
    color.rgba = step(vec4(float(pos)), floor(fill) - 1.0);
}

void patch_wave_dither(
    out vec4 color,
    sampler2D t0,
    vec2 uv,
    vec2 res,
    float time,
    bool clip_black,
    float baseFrequency,
    float modulationIntensity,
    float warpStrength,
    float bias,
    bool vertical,
    uint channel_mask
) {
    
    // 2. Sample local color to find the directional gradient
    // We sample slightly offset pixels to see how the image's brightness changes
    // vec2 offsetStep = vec2(0.003, 0.003); 
    vec2 offsetStep = vec2(1.0/res.x, 1.0/res.y); // Scale offset by resolution and user control
    
    float lumCenter = dot(texture(t0, uv).rgb, vec3(0.299, 0.587, 0.114));
    float lumRight  = dot(texture(t0, uv + vec2(offsetStep.x, 0.0)).rgb, vec3(0.299, 0.587, 0.114));
    float lumUp     = dot(texture(t0, uv + vec2(0.0, offsetStep.y)).rgb, vec3(0.299, 0.587, 0.114));
    
    // 3. Compute the 2D gradient vectors (direction of brightness change)
    float gradX = lumRight - lumCenter;
    float gradY = lumUp - lumCenter;
    vec2 imageGradient = vec2(gradX, gradY);

    // 4. THE 2D OFFSET STEP
    // Displace the coordinate reading path along the contours of the image.
    // Adding time here makes the warp jitter like fluctuating voltage.
    vec2 warpedCoords = uv + (imageGradient * warpStrength);
    
    // 5. Re-sample the definitive luminance using the warped coordinate grid
    vec4 finalTexColor = texture(t0, warpedCoords);
    if ((channel_mask & 0x1u) == 0u) finalTexColor.b = 0.0;
    if ((channel_mask & 0x2u) == 0u) finalTexColor.g = 0.0;
    if ((channel_mask & 0x4u) == 0u) finalTexColor.r = 0.0;
    float finalLuminance = dot(finalTexColor.rgb, vec3(0.299, 0.587, 0.114));

    if (clip_black && finalLuminance < 0.01) {
        color = vec4(vec3(finalLuminance), 1.0);
        return;
    }

    // 6. Generate the frequency calculation using the distorted coordinates
    // Adding the u_time uniform here creates a rolling analog tracking line effect
    float screenPattern = (vertical ? warpedCoords.x : warpedCoords.y) * baseFrequency + time;
    float modifiedFrequency = screenPattern * (1.0 + (finalLuminance * modulationIntensity));
    
    // 7. Output the crisp black and white vector lines
    // bias shifts the threshold: 0.0 = 50/50, positive = more black, negative = more white
    float waveOutput = sin(modifiedFrequency);
    float finalBinaryColor = step(bias, waveOutput);
    
    color = vec4(vec3(finalBinaryColor), 1.0);
}



vec2 rotate_about(vec2 point, vec2 pivot, mat2 rot) {
    return pivot + rot * (point - pivot);
}

vec4 check_pos_value(sampler2D tex, vec2 pos, vec2 res, vec2 sz, float threshold, vec2 pivot, mat2 inv_rot) {
    vec4 value = vec4(0.0);
    for (int i = -int(sz.y/2); i < int(sz.y/2); i++) {
        for (int j = -int(sz.x/2); j < int(sz.x/2); j++) {
            vec2 sample_rot = pos + vec2(float(j), float(i));
            vec2 sample_pos = rotate_about(sample_rot, pivot, inv_rot);
            vec4 c = texture(tex, sample_pos / res);
            value += step(vec4(threshold), c) / vec4(sz.x * sz.y);
        }
    }
    return value;
}

void patch_ink(out vec4 color, sampler2D tex, vec2 uv, vec2 res, vec2 sz, float threshold, float angle_deg, uint seed) {
    //make sure the size is even so we can dived by 2
    sz = floor(sz / 2.0) * 2.0;

    float angle = radians(angle_deg);
    mat2 rot = mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
    mat2 inv_rot = mat2(cos(-angle), -sin(-angle), sin(-angle), cos(-angle));
    vec2 pivot = res * 0.5;

    vec2 coord = uv * res;
    vec2 coord_rot = rotate_about(coord, pivot, rot);
    
    vec2 region = floor((coord_rot + sz * 0.5) / sz);
    vec2 pos = region * sz;
    vec4 value = check_pos_value(tex, pos, res, sz, threshold, pivot, inv_rot);
    vec4 darkness = vec4(1.0) - value;
    
    // check lum of the 9 neighboring regions to create a more organic ink blot effect
    // to calculate a gravity effect, we can use the distance from the center of the region to the center of the neighboring regions
    mat4x2 grav = mat4x2(0.0);
    for (int i = -1; i <= 1; i++) {
        for (int j = -1; j <= 1; j++) {
            vec2 neighborPos = pos + vec2(float(j), float(i)) * sz;
            vec4 neighborDarkness = vec4(1.0) - check_pos_value(tex, neighborPos, res, sz, threshold, pivot, inv_rot).r;
            grav += outerProduct(vec2(float(j), float(i)), step(vec4(threshold), neighborDarkness));
        }
    }

    // move the pos based on the grav vector to create a more organic ink blot effect
    mat4x2 grav_pos = mat4x2(pos, pos, pos, pos) + grav * 0.5;
    
    // float noise = 0.05 * randf(uint(coord.x * 73856093u) ^ uint(coord.y * 19349663u) ^ seed);
    // if (darkness > 0.1 && darkness < 0.95) {
    //     darkness = clamp(darkness + noise, 0.0, 1.0);
    // }
    vec4 ranges = vec4(
        distance(coord_rot, grav_pos[0]),
        distance(coord_rot, grav_pos[1]),
        distance(coord_rot, grav_pos[2]),
        distance(coord_rot, grav_pos[3])
    ) / (length(sz) / 2.0);

    color = step(darkness, ranges);
    // if ( > 0.5) {
    //     color = vec4(0.0, 0.0, 0.0, 1.0);
    // } else {
    //     color = vec4(1.0, 1.0, 1.0, 1.0);
    // }
}
