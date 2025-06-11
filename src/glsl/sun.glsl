//!VAR int usr_var 0

float scale = 10.0;
float usr_p = (scale - mod(iTime, scale))/scale;
vec2 sun_center = vec2(0.5, (0.2 + 0.3 * usr_p)) * iResolution.xy;
vec2 pt = src_coord0 * iResolution.xy;

float radius = 100.0 + usr_p * 100.0;
if (distance(pt, sun_center) < radius) {
    color = vec4(1.0, 1.0, 1.0, 1.0);
} else {
    color = vec4(0.0, 0.0, 0.0, 1.0);
}

float sun_bottom = sun_center.y + radius;
float dy = sun_bottom - pt.y;
if (dy > 30.0 && 
   (50 * pow(M_E, -8*((sun_bottom - dy)-90)/sun_bottom) * (1 + sin(((sun_bottom-dy)/sun_bottom + mod(frame,720.0)/720.0) * 2 * M_PI * 20))/2.0 >= 0.85)) {
    color = vec4(0.0, 0.0, 0.0, 1.0);
}

if (src_coord0.y > 0.6) {
    color = vec4(0.0, 0.0, 0.0, 1.0);
}