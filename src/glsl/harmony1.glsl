float thickness = 0.05;

float wave1 = src_coord0.x * 4.0 * M_PI + fract(frame/30.0) * 2.0 * M_PI;
float delta = thickness * cos(wave1) + thickness * 3.1;

float wave2 =  fract(frame/200.0) * 2.0 * M_PI;
float variation = thickness * cos(wave2);

float wave3 = src_coord0.x * 12.0 * M_PI + fract(frame/30.0) * 2.0 * M_PI;
float jitter = variation * cos(wave3);

float superposition = delta + jitter * 0.0;

if (src_coord0.y >= (1.0 - superposition)) {
  color = vec4(1.0, 1.0, 1.0, 1.0);  // White
} else {
  color = vec4(0.0, 0.0, 0.0, 1.0);
}