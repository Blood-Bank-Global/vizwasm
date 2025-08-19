//float thickness = 0.05;
float thickness = 0.02;

float wave1 = src_coord0.x * 1.9 * M_PI + fract(frame/120.0) * 2.0 * M_PI;
float delta = thickness * cos(wave1) + thickness * 3.1;

float wave2 = src_coord0.x * 0.1 * M_PI + fract(frame/300.0) * 2.0 * M_PI;
float jitter = thickness * 2.0 *(1 + cos(wave2))/2.0;

float superposition = delta + jitter * 1.5 + thickness * 2.0;

if (src_coord0.y >= (1.0 - superposition)) {
  color = vec4(1.0, 1.0, 1.0, 1.0);  // White
} else {
  color = vec4(0.0, 0.0, 0.0, 1.0);
}