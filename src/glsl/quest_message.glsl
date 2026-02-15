//!VAR vec2 iResolution0 1.0 1.0
////////////// 1234567890ABCDE_1234567890ABCDE_1234567890ABCDE_1234567890ABCDE_1234567890ABCDE_
//!STR quest0 "Welcome,        Adventurer"
//!STR quest1 "A Quest, should you accept it."
//!STR quest2 "Fear not the    danger you see, unleash the     danger within!"
//!STR quest3 "Restore the     reliquary,      Discover truth."
//!STR quest4 "The enemy does  not come in     dragon form."
//!STR quest5 "Is the treasure cursed, or is   the curse the   treasure itself?"
//!STR quest6 "Do not be       fooled by those who would say   your power is a  prison."
//!STR quest7 "A TRAP HAS BEEN       SPRUNG!    "

#define QUEST_LINE_LEN 16

color = vec4(0.0, 0.0, 0.0, 1.0);

if (true) {
   
    uint lenghts[8] = uint[8](
        quest0_length,
        quest1_length,
        quest2_length,
        quest3_length,
        quest4_length,
        quest5_length,
        quest6_length,
        quest7_length
    );

    struct extents_t {
        uint starts[6];
        uint lens[6];
    };

    extents_t quest_extents[8];
    for (int i = 0; i < 8; i++) {
        quest_extents[i].starts[0] = 0u;
        quest_extents[i].lens[0] = min(QUEST_LINE_LEN, lenghts[i]);
        quest_extents[i].starts[1] = min(16u, lenghts[i]);
        quest_extents[i].lens[1] = min(QUEST_LINE_LEN, lenghts[i] > 16u ? lenghts[i] - 16u : 0u);
        quest_extents[i].starts[2] = min(32u, lenghts[i]);
        quest_extents[i].lens[2] = min(QUEST_LINE_LEN, lenghts[i] > 32u ? lenghts[i] - 32u : 0u);
        quest_extents[i].starts[3] = min(48u, lenghts[i])   ;
        quest_extents[i].lens[3] = min(QUEST_LINE_LEN, lenghts[i] > 48u ? lenghts[i] - 48u : 0u);
        quest_extents[i].starts[4] = min(64u, lenghts[i]);
        quest_extents[i].lens[4] = min(QUEST_LINE_LEN, lenghts[i] > 64u ? lenghts[i] - 64u : 0u);
        quest_extents[i].starts[5] = min(80u, lenghts[i]);
        quest_extents[i].lens[5] = min(QUEST_LINE_LEN, lenghts[i] > 80u ? lenghts[i] - 80u : 0u);
    }

    vec2 uv = (iResolution.xy * src_coord.xy * 0.25) + vec2(0.0, -16.0);

#define display_msg(i) \
    multiline_fantasy( \
        uv, \
        vec2(0.0, 0.0), \
        quest ## i, \
        quest_extents[(i)].starts, \
        quest_extents[(i)].lens \
    )
    bool disp = false;
    int qindex = int(floor((1.0 + randf(uint(iTime)))/2.0 * 8.0));

    switch (qindex) {
        case 0: disp = display_msg(0); break;
        case 1: disp = display_msg(1); break;
        case 2: disp = display_msg(2); break;
        case 3: disp = display_msg(3); break;
        case 4: disp = display_msg(4); break;
        case 5: disp = display_msg(5); break;
        case 6: disp = display_msg(6); break;
        case 7: disp = display_msg(7); break;
        default: disp = true; break;
    }

    if (disp) {
        color = vec4(0.7, 0.7, 0.7, 1.0);
    }
}