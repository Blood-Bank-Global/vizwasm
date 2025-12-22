
//ANGEL  ALPHA
//!VAR float cc_iac_driver_bus_1_0_1 0.0
//ANGEL HUE
//!VAR float cc_iac_driver_bus_1_0_2 0.0

//Chord note
//!VAR float cc_iac_driver_bus_1_0_3 0.0
//chord trigger
//!VAR float cc_iac_driver_bus_1_0_4 0.0

//TBD
//!VAR float cc_iac_driver_bus_1_0_5 0.0
//CLAPS - H_BLINDS
//!VAR float cc_iac_driver_bus_1_0_6 0.0
//TAPS -- V_BLINDS
//!VAR float cc_iac_driver_bus_1_0_7 0.0

color = texture(src_tex0, src_coord0);

vec2 base_coord = src_coord0;

float v_blinds = step(60, cc_iac_driver_bus_1_0_7) * step(0.5, fract(base_coord.x / 0.05));
color.rb = (1.0 - v_blinds) * color.rb + v_blinds * color.br;

float h_blinds = step(60, cc_iac_driver_bus_1_0_6) * step(0.5, fract(base_coord.y / 0.05));
color.rg = (1.0 - h_blinds) * color.rg + h_blinds * color.gr;

vec3 color_hsv = rgb2hsv(color.rgb);
float ramp = cc_iac_driver_bus_1_0_3 / 127.0 * 0.25;
color.rgb = hsv2rgb(vec3(color_hsv.x, clamp(color_hsv.y + ramp, 0.0, 1.0), clamp(color_hsv.z + ramp, 0.0, 1.0)));


vec4 angel = texture(src_tex1, src_coord1);
vec3 angel_hsv = rgb2hsv(angel.rgb);
angel.rgb = hsv2rgb(vec3(mod(angel_hsv.x + cc_iac_driver_bus_1_0_2/127.0, 1.0), 0.5, angel_hsv.z));
angel.a = min(angel_hsv.z, cc_iac_driver_bus_1_0_1/127.0);

color = blend_by_mode(color, angel, BLEND_ALPHA); 


