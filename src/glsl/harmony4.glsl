//!VAR vec2 iResolution0 1.0 1.0

color = vec4(0.0, 0.0, 0.0, 1.0);

float vx = 30 / iResolution0.x;
float vy = 30 / iResolution0.y;

float dx = vx / 2.0 *cos(src_coord0.y * 8 * M_PI + frame/30.0)
         * cos(src_coord0.y * 20 * M_PI + frame/300.0) + 1.2 * vx;

float dy = vy / 2.0 *cos(src_coord0.x * 8 * M_PI + frame/30.0)
         * cos(src_coord0.x * 20 * M_PI + frame/300.0) + 1.2 * vy;


if (src_coord0.x <= dx || src_coord0.x >= 1.0 - dx || 
    src_coord0.y <= dy || src_coord0.y >= 1.0 - dy) {
    color = vec4(1.0, 1.0, 1.0, 1.0);
}