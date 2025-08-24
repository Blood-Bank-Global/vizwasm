// https://www.shadertoy.com/view/tcG3zy

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;

    vec3 slug = vec3(
        length(uv - vec2(0.5,0.5)),
        length(uv - vec2(0.1,0.1)),
        length(uv - vec2(0.7,0.7))
    );
    // Time varying pixel color
    vec3 col = vec3(1.0) * (cos(iTime + slug));

    // Output to screen
    fragColor = vec4(col,1.0);
}
