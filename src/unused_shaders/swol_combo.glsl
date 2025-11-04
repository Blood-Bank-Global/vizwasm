
//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0

//!VAR int usr_var 0;

//!VAR float cc_iac_driver_bus_1_0_0 0.0
//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float cc_iac_driver_bus_1_0_2 0.0

#define SMOKE_TEX src_tex0
#define SMOKE_RES iResolution0
#define SMOKE_COORDS src_coord0
#define HOW_TEX src_tex1
#define HOW_RES iResolution1
#define HOW_COORDS src_coord1

#define CC_KICK cc_iac_driver_bus_1_0_0
#define CC_CRASH cc_iac_driver_bus_1_0_2

vec2 base_coord2 = SMOKE_COORDS;
if (CC_CRASH > 5.0) {
    float angle = sin(dot(base_coord2.xy, vec2(2.9898, 3.0233)) + iTime * 7.0) * 3.14159;
    float radius = length(base_coord2 - 0.5) * CC_CRASH / 127.0;
    vec2 offset = vec2(cos(angle), sin(angle)) * radius * 0.1;
    base_coord2 = base_coord2 + offset; 
}
color.rgb = handle_edge(SMOKE_TEX, base_coord2, EDGE_MODE_MIRROR);
color.a = 1.0;



vec4 how_color = texture(HOW_TEX, HOW_COORDS);
if (distance(how_color.rgb, vec3(0,0,0)) > 10 * EPSILON && CC_KICK > 5.0) {
    how_color.a = min(dot(how_color.rgb, vec3(0.299, 0.587, 0.114)) + 0.2, CC_KICK / 127.0);
} else {
    how_color.a = 0.0;
}

color = blend_by_mode(color, how_color, BLEND_ALPHA);