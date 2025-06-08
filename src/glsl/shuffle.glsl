float delta = src_coord0.x + randf(uint(mod(frame,120) + floor(src_coord0.y/0.01)))/50.0;
color.rgb = handle_edge(src_tex0, vec2(delta, src_coord0.y), EDGE_MODE_SMEAR);

color.a = 1.0;