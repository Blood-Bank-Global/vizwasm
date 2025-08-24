//!VAR vec2 iResolution0 1.0 1.0
//!VAR int usr_var 0
color = vec4(1.0, 1.0, 1.0, 1.0);

float dim = iResolution0.y / 5.0;;
vec2 coord = src_coord0 * iResolution.xy;
float row = floor(coord.y / dim);

//float vari = 3.0 + 3.0 * sin(mod(float(frame), 600.0)/600.0 * 2.0 * M_PI) * dim + dim * 0.25;
float vari = dim * 0.7;
if (mod(row, 2.0) == 1.0) {
    coord.x +=  vari; // Offset for odd rows
 } else {
    coord.x -= vari; // Offset for even rows
 }

float col = floor(coord.x/ dim);

if (mod(row, 2.0) == 0.0) {
    col += 1; // Offset for even rows
}

if (mod(col, 2.0) == 0.0) {
    color = vec4(0.0, 0.0, 0.0, 1.0); // Black
}

if (fract(coord.x/dim) < 0.05 || fract(coord.y/dim) < 0.05) {
    color = vec4(0.2, 0.2, 0.2, 1.0);
}