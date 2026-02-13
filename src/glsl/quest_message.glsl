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
//!STR stats_nmcl "Nm: ____ Cl: ____"
//!STR stats_hpmp "HP: 0000 MP: 0000"
//!STR stats_xplv "Xp: 0000 Lv: 0000"

//!STR rosa_nmcl "Nm: Rosa Cl: Clrc"
//!STR barb_nmcl "Nm: Barb Cl: Wizd"
//!STR alex_nmcl "Nm: Alex Cl: Tief"
//!STR capp_nmcl "Nm: Capp Cl: Figt"

struct party_stats_t {
    int line_[3][128];
};
party_stats_t stats[4] = {
    party_stats_t(int[3][128](rosa_nmcl, stats_hpmp, stats_xplv)),
    party_stats_t(int[3][128](barb_nmcl, stats_hpmp, stats_xplv)),
    party_stats_t(int[3][128](alex_nmcl, stats_hpmp, stats_xplv)),
    party_stats_t(int[3][128](capp_nmcl, stats_hpmp, stats_xplv)),
};
#define get_stat(member, stat) stats[member].line_[stat]

vec2 uv = src_coord.xy/4.0 * iResolution.xy;
color = vec4(0.0, 0.0, 0.0, 1.0);

if (false) { // class stats
    int line_ = int(floor(uv.y/16.0));
    vec2 line_pos = vec2(50.0, float(line_) * 16.0);
    int member = int(mod(floor(line_ / 4.0), 4.0));
    int stat = int(mod(line_, 4.0));
    int seed1 = int(iTime * 3.0 + float(member) + float(stat) * 13);
    float stat1 = clamp(floor((1 + randf(seed1))/2.0 * 9999.0), 0, 9999);
    int seed2 = int(iTime * 3.0 + float(member) + float(stat) * 11);
    float stat2 = clamp(floor((1 + randf(seed2))/2.0 * 9999.0), 0, 9999);

    if (stat < 3) {
        int[128] txt = get_stat(member, stat);
        int len = 0;
        if (stat == 0) {
            len = stats_nmcl_length;
        } else if (stat == 1) {
            len = stats_hpmp_length;
            txt[4] = 0x30 + int(mod(stat1/1000, 10));
            txt[5] = 0x30 + int(mod(stat1/100, 10));
            txt[6] = 0x30 + int(mod(stat1/10, 10));
            txt[7] = 0x30 + int(mod(stat1, 10));
            txt[13] = 0x30 + int(mod(stat2/1000, 10));
            txt[14] = 0x30 + int(mod(stat2/100, 10));
            txt[15] = 0x30 + int(mod(stat2/10, 10));
            txt[16] = 0x30 + int(mod(stat2, 10));
        } else if (stat == 2) {
            len = stats_xplv_length;
            txt[4] = 0x30 + int(mod(stat1/1000, 10));
            txt[5] = 0x30 + int(mod(stat1/100, 10));
            txt[6] = 0x30 + int(mod(stat1/10, 10));
            txt[7] = 0x30 + int(mod(stat1, 10));
            // txt[13] = 0x30 + int(mod(stat2/1000, 10));
            // txt[14] = 0x30 + int(mod(stat2/100, 10));
            txt[13] = 0x20; // space
            txt[14] = 0x20; // space
            txt[15] = 0x30 + int(mod(stat2/10, 10));
            txt[16] = 0x30 + int(mod(stat2, 10));
        }
        // color = draw_text(color, uv, line_pos, iResolution.xy, txt, len);
        // color = draw_textMedieval(color, uv, line_pos, iResolution.xy, txt, len);
    }
}

#define QUEST_LINE_LEN 16
#define QUEST_LINE_H 8.0
if (true) {
    struct quest_message_t {
        int msg[128];
        int len;
    };
    quest_message_t quest_messages[8] = {
        quest_message_t(quest0, quest0_length),
        quest_message_t(quest1, quest1_length),
        quest_message_t(quest2, quest2_length),
        quest_message_t(quest3, quest3_length),
        quest_message_t(quest4, quest4_length),
        quest_message_t(quest5, quest5_length),
        quest_message_t(quest6, quest6_length),
        quest_message_t(quest7, quest7_length),
    };
    int qindex = int(floor((1.0 + randf(uint(iTime)))/2.0 * 8.0));


    quest_message_t qmsg = quest_messages[qindex];   
    int starts[9] = int[9](
        min(qmsg.len, 0), 
        min(qmsg.len, 16),
        min(qmsg.len, 32), 
        min(qmsg.len, 48),
        min(qmsg.len, 64), 
        min(qmsg.len, 80),
        min(qmsg.len, 96),
        min(qmsg.len, 112), 
        min(qmsg.len, 128));
    int lens[9] = int[9](
        qmsg.len > 0 ? min(QUEST_LINE_LEN, qmsg.len) : 0,
        qmsg.len > 16 ? min(QUEST_LINE_LEN, qmsg.len - 16) : 0,
        qmsg.len > 32 ? min(QUEST_LINE_LEN, qmsg.len - 32) : 0,
        qmsg.len > 48 ? min(QUEST_LINE_LEN, qmsg.len - 48) : 0,
        qmsg.len > 64 ? min(QUEST_LINE_LEN, qmsg.len - 64) : 0,
        qmsg.len > 80 ? min(QUEST_LINE_LEN, qmsg.len - 80) : 0,
        qmsg.len > 96 ? min(QUEST_LINE_LEN, qmsg.len - 96) : 0,
        qmsg.len > 112 ? min(QUEST_LINE_LEN, qmsg.len - 112) : 0,
        qmsg.len > 128 ? min(QUEST_LINE_LEN, qmsg.len - 128) : 0);
    
    vec2 uv = vec2(uv.x, uv.y - 16.0);

    int test_starts[2] = int[2](0,16);
    int test_lens[2] = int[2](16,16);
    if (multiline_fantasy(uv, vec2(0.0, 0.0), qmsg.msg, test_starts, test_lens)) {
        color = vec4(0.7, 0.7, 0.7, 1.0);
    }
}