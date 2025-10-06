//!VAR vec2 iResolution0 1.0 1.0
//!VAR vec2 iResolution1 1.0 1.0

//!VAR float cc_iac_driver_bus_1_0_0 0.0
//!VAR float cc_iac_driver_bus_1_1_0 0.0

#define FLICKER_SCENE_TEX src_tex0
#define FLICKER_SCENE_COORD src_coord0
#define FLICKER_SCENE_RESOLUTION iResolution0

#define FLICKER_BOOK_TEX src_tex1
#define FLICKER_BOOK_COORD src_coord1
#define FLICKER_BOOK_RESOLUTION iResolution1

color = texture(FLICKER_SCENE_TEX, FLICKER_SCENE_COORD);

if (cc_iac_driver_bus_1_1_0 > 5.0) {
    vec4 book = texture(FLICKER_BOOK_TEX, FLICKER_BOOK_COORD);
    if (distance(book.rgb, vec3(0,0,0)) > 10 * EPSILON) {
    book.a = cc_iac_driver_bus_1_1_0 / 127.0;
    } else {
        book.a = 0.0;
    }
    color = blend_by_mode(color, book, BLEND_ALPHA);
}