
 //0       "arch",
 //1       "bats_cut",
 //2       "bats_full",
 //3       "castle_cut",
 //4       "castle_full",
 //5       "doorway",

#include "patch_check_scroll_px.glsl"
#include "utils.glsl"
void pass0(out vec4 color) {
    color = texture(src_tex5, src_uv);
    if (distance(color.rgb, vec3(1.0, 0.0, 1.0)) < 0.8) {
        vec2 coord = src_uv.xy * iResolution.xy;
        color=vec4(vec3(0.0), 0.0);
        color = patch_check_scroll_px(
            coord.xy,
            iResolution.xy,
            color,
            vec4(vec3(0.5,0.5,0.5), 1.0),
            vec4(vec3(0.7,0.7,0.7), 1.0),
            vec2(20.0, 20.0),
            vec2(0.0, fract(iTime*2) * 40.0),
            mat4x2(
                vec2(0., iResolution.y/2.0),
                vec2(iResolution.x, iResolution.y/2.0),
                vec2(-1000.0, iResolution.y ),
                vec2(iResolution.x + 1000.0, iResolution.y )
            )
        );
    }


}
