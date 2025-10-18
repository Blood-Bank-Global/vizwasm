
float jitter = 0;
for (int i = 0; i < 10; i++) {
    jitter += sin((src_coord0.y*8.3+fract(iTime/10.0)+i*113.0)) * 0.005;
}


color = texture(src_tex0, src_coord0 + vec2(jitter, 0));


    