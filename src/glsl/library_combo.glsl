
//Bells trig
//!VAR float cc_iac_driver_bus_1_0_2 0.0
//Bells Note
//!VAR float cc_iac_driver_bus_1_0_3 0.0

//Snaps trigger
//!VAR float cc_iac_driver_bus_1_0_4 0.0
//Snaps note
//!VAR float cc_iac_driver_bus_1_0_5 0.0

vec2 base_coord = src_coord0;

float wiggle = step(60, cc_iac_driver_bus_1_0_4);
wiggle *= sin(base_coord.y * M_PI * 10.0) * sin(iTime * 10.0);
wiggle = step(0.5, wiggle) - step(wiggle, -0.5);
wiggle *= 0.01;

base_coord.x += wiggle;

base_coord.y += step(60.0,cc_iac_driver_bus_1_0_4) * sin(base_coord.x * randf(uint(cc_iac_driver_bus_1_0_5)) * randf(uint(base_coord.y*1000.0)) * M_PI * 20.0) * 0.01;

color = vec4(handle_edge(src_tex0, base_coord, EDGE_MODE_MIRROR), 1.0);

vec3 color_hsv = rgb2hsv(color.rgb);
float shift_bucket = fract(cc_iac_driver_bus_1_0_3 / 5.0);
float shift_amt = (1.0 - step(0.33, shift_bucket)) * 0.25 +
(step(0.33, shift_bucket) - step(0.66, shift_bucket)) * 0.5 + 
step(0.66, shift_bucket) * 0.7;

shift_amt *= step(60, cc_iac_driver_bus_1_0_2);
shift_amt *= step(0.5, color_hsv.z);

color_hsv.x = mod(color_hsv.x + shift_amt, 1.0);

color.rgb = hsv2rgb(color_hsv);