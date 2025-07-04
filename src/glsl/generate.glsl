#define border_max 85.0
#define border_min -40.0

float amp_horizontal = border_max + border_min + border_max
    * sin((src_coord0.x + mod(frame, 154.0) / 154.0) * 2.0 * M_PI) 
    * cos(src_coord0.x + mod(frame, 233.0) / 233.0 * 2.0 * M_PI) 
    * sin((src_coord0.x + mod(frame, 312.0) / 312.0) * 2.0 * M_PI)
    * sin((src_coord0.x + mod(frame, 58.0) / 58.0) * 2.0 * M_PI)
    * sin((src_coord0.x + mod(frame, 90.0) / 90.0) * 2.0 * M_PI)
    * sin((src_coord0.x + mod(frame, 1312.0) / 1312.0) * 2.0 * M_PI)
    * sin((src_coord0.x + mod(frame, 208.0) / 208.0) * 2.0 * M_PI)
    * cos(src_coord0.x + mod(frame, 401.0) / 401.0 * 2.0 * M_PI);

float amp_vertical = border_max + border_min + border_max
    * sin((src_coord0.y + mod(frame, 154.0) / 154.0) * 2.0 * M_PI)
    * cos(src_coord0.y + mod(frame, 233.0) / 233.0 * 2.0 * M_PI)
    * sin((src_coord0.x + mod(frame, 312.0) / 312.0) * 2.0 * M_PI)
    * sin((src_coord0.x + mod(frame, 58.0) / 58.0) * 2.0 * M_PI)
    * sin((src_coord0.y + mod(frame, 312.0) / 312.0) * 2.0 * M_PI)
    * cos(src_coord0.y + mod(frame, 401.0) / 401.0 * 2.0 * M_PI);

amp_horizontal = amp_horizontal / iResolution.x;
amp_vertical = amp_vertical / iResolution.y;

if (src_coord0.x < amp_horizontal || src_coord0.x > 1.0 - amp_horizontal || src_coord0.y < amp_vertical || src_coord0.y > 1.0 - amp_vertical) {
    color = vec4(1.0, 1.0, 1.0, 1.0);
} else {
    color = vec4(0.0, 0.0, 0.0, 1.0);
}

