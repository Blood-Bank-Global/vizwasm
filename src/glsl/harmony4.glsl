//!VAR vec2 iResolution0 1.0 1.0

color = vec4(0.0, 0.0, 0.0, 1.0);

float vx = 30 / iResolution0.x;
float vy = 30 / iResolution0.y;

// float dx = vx / 2.0 * cos(src_coord0.y + frame/50.0)
//          * cos(src_coord0.y * 8.0 + frame/10.0) + 1.2 * vx;

// float dy = vy / 2.0  * cos(src_coord0.x + frame/50.0)
//          * cos(src_coord0.x * 8.0 + frame/10.0)  + 1.2 * vy;

float dx = 0.1;
float dy = 0.1;

if (src_coord0.x <= dx || src_coord0.x >= 1.0 - dx || 
    src_coord0.y <= dy || src_coord0.y >= 1.0 - dy) {
    color = vec4(1.0, 1.0, 1.0, 1.0);
}