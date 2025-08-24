float thickness = 0.1;
//float thickness = 0.01;

float wave1 = src_coord0.x * 0.8 * M_PI + fract(frame/120.0) * 2.0 * M_PI;
float delta = thickness * (1 + cos(wave1))/2.0;

float wave2 = src_coord0.x * 0.1 * M_PI + fract(frame/300.0) * 2.0 * M_PI;
float jitter = thickness * (1 + cos(wave2))/2.0;

float superposition = delta + jitter * 1.0;

if (src_coord0.y  >= (1.0 - clamp(superposition, 5.0/iResolution.y, 1.0))) {
  color = vec4(1.0, 1.0, 1.0, 1.0);  // White
} else {
  color = vec4(0.0, 0.0, 0.0, 1.0);
}