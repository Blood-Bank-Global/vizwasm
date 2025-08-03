float thickness = 0.05;

float delta = thickness * sin((mod(frame, 100.0) / 100.0 + src_coord0.x * 1.0) * 2.0 * M_PI) + 0.06;

if (src_coord0.y >= (1.0 - delta)) {
  color = vec4(1.0, 1.0, 1.0, 1.0);  // White
} else {
  color = vec4(0.0, 0.0, 0.0, 1.0);
}