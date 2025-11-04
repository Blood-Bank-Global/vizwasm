//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0
//!VAR vec2 iResolution2 1.0 1.0
//!VAR vec2 iResolution3 1.0 1.0
//!VAR vec2 iResolution4 1.0 1.0

//!VAR int usr_var 0;

//!VAR float cc_iac_driver_bus_1_0_0 0.0
//!VAR float cc_iac_driver_bus_1_0_1 0.0
//!VAR float note_iac_driver_bus_1_1_36 0.0


#define AGENT_TEX src_tex0
#define AGENT_RES iResolution0
#define AGENT_COORDS src_coord0
#define FAULT_TEX src_tex1
#define FAULT_RES iResolution1
#define FAULT_COORDS src_coord1
#define FLOW_TEX src_tex2
#define FLOW_RES iResolution2
#define FLOW_COORDS src_coord2
#define OPS_ERR_TEX src_tex3
#define OPS_ERR_RES iResolution3
#define OPS_ERR_COORDS src_coord3
#define TRACE_TEX src_tex4
#define TRACE_RES iResolution4
#define TRACE_COORDS src_coord4

vec2 base_coord = FAULT_COORDS;

vec4 agent_color = texture(AGENT_TEX, base_coord);
vec4 top_color;

switch (usr_var) {
    case 0:
        top_color = texture(FAULT_TEX, FAULT_COORDS);
        break;
    case 1:
        top_color = texture(FLOW_TEX, FLOW_COORDS);
        break;
    case 2:
        top_color = texture(OPS_ERR_TEX, OPS_ERR_COORDS);
        break;
    case 3:
        top_color = texture(TRACE_TEX, TRACE_COORDS);
        break;
    default:
        top_color = vec4(0.0);
        break;
}

if (distance(top_color.rgb, vec3(0,0,0)) > 10 * EPSILON && cc_iac_driver_bus_1_0_0 > 5.0) {
    top_color.a = min(dot(top_color.rgb, vec3(0.299, 0.587, 0.114)), cc_iac_driver_bus_1_0_0 / 127.0);
} else {
    top_color.a = 0.0;
}
color = blend_by_mode(agent_color, top_color, BLEND_ALPHA);