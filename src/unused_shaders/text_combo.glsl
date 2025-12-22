color = texture(src_tex0, src_coord0);

vec3 color_hsv = rgb2hsv(color.rgb);
color_hsv.x = mod(iTime/2.0, 1.0);
color_hsv.y = 1.0;

color.rgb = hsv2rgb(color_hsv);
