//!VAR vec3 iResolution0 1.0 1.0 1.0
////////////// 1234567890ABCDE_1234567890ABCDE_1234567890ABCDE_
//!STR quest0 "Welcome,        Adventurer"
//!STR quest1 "A Quest, should you accept it."
//!STR quest2 "Fear not danger you see, unleashdanger within!"
//!STR quest3 "Restore the     reliquary,      heal the spirit."
//!STR quest4 "The enemy does  not come in     dragon form."
//!STR quest5 "                      AMBUSH!"
//!STR quest6 "                      PINCER!"
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

vec2 uv = src_coord.xy/2.0 * iResolution.xy;
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
        color = draw_text(color, uv, line_pos, iResolution.xy, txt, len);
    }
}

#define QUEST_LINE_LEN 16.0
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

    float scale = iResolution.x / (float(QUEST_LINE_LEN) * 16.0);
    vec2 uv = vec2(uv.x, uv.y - 16.0 * 2) / scale;

    float line_ = floor(uv.y / float(QUEST_LINE_LEN));
    int char_offset = int(line_) * int(QUEST_LINE_LEN);

    if (char_offset >= 0 && char_offset < qmsg.len) {
        int remainder = clamp(qmsg.len - char_offset, 0, (int(QUEST_LINE_LEN)-1));
        int txt[128];
        for (int i = 0; (i <= remainder) && i + char_offset < 128; i++) {
            txt[i] = qmsg.msg[i + char_offset];
        }

        color = draw_text(color, uv, vec2(0.0, floor(16.0*line_)), iResolution.xy, txt, (remainder+1));
    }
}