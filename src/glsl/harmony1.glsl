float thickness = 0.05;

float delta = thickness * sin((mod((frame + src_coord0.x*100), 100)/100 + src_coord0.x) * M_PI * 2.0) + 0.06;

if (src_coord0.y >= (1 - delta)) {
    color = vec4(1.0, 0.0, 0.0, 1.0); // Red
} else {
    color = vec4(0.0, 0.0, 0.0, 1.0); 
}
