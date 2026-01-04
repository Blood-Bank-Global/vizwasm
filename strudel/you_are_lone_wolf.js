setCpm(160 / 4);

$HATS: stack(
    "hh".beat("2,6,10,14", 16).gain(2),
    "oh".beat("4,12", 16).ply(2)
)
    .n(irand(4).seg(16))
    .att(.05)
    .dec(.1)
    .rib(0, 4)
    .s()
    .bank("tr909")
    .gain(slider(0, 0, 1))
    .bpf(1500, 200)
    ._punchcard();

$DRUM: stack(
    "dungeon_perc:6".beat("0,8", 16),
    "dungeon_perc:5".beat("2,10", 16).degradeBy(.5)
)
    .rib(0, 4)
    .gain(slider(0, 0, 1))
    .s()
    .lpf(200)
    ._punchcard();

$KICK: "bd".beat("0,4,8,12", 16).duckorbit(2)
    .bank("tr909")
    .s()
    .att(.01)
    .gain(slider(0, 0, 1))
    .dec(.3)
    .bpf("300:1");

$ARPS: "<e3!16>"
    .add(saw.range(0, 4).floor().seg(16))
    .note()
    .s("gm_pad_new_age:0")
    .struct("<x ~ ~ ~ x ~ ~ ~ x x ~ ~ ~ ~ ~ ~>*8")
    .dec(.3)
    .rel(.5)
    .sus(.5)
    .room(.5)
    .delay(0)
    .rib(0, 8)
    .gain(slider(0, 0, 1))
    .hpf(1500)
    ._punchcard();

$LEAD: chooseCycles(
    "g3,d4,e4",
    "c4,e4,g4",
    "c3,d4,g4",
)
    .rib(0, 4)
    .note()
    .s("gm_synth_strings_1")
    .orbit(2)
    .room(2)
    .gain(slider(0, 0, 1))
    .bpf("800:1.5")
    ._punchcard();

$DRONE: s("dungeon_sfx:1").loopAt(8).loop(1).gain(slider(0, 0, 1)).lpf(100)
    ._scope();