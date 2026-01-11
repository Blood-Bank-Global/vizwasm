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
    .gain(slider(1, 0, 1))
    .bpf(1500, 200)
    ._punchcard();

$DRUM: stack(
    "dungeon_perc:6".beat("0,8", 16),
    "dungeon_perc:5".beat("2,10", 16).degradeBy(.25)
)
    .rib(0, 4)
    .gain(slider(0, 0, 1))
    .s()
    .lpf(200)
    ._punchcard();

$KICK: "bd".beat("0,4,8,12,14", 16)
    .bank("tr909")
    .s()
    .att(.01)
    .gain(slider(1, 0, 1))
    .dec(.3)
    .bpf("300:1");

$ARPS: "e3"
    .add(saw.range(0, 4).floor().seg(16))
    .note()
    .s("dark_key:4")
    .struct("[x ~ ~ ~ x ~ ~ ~ [x ~ x ~] ~ ~ ~ x ~ ~ x]/2")
    .att(.1)
    .dec(.3)
    .delay(.2)
    .delayfb(.2)
    .rib(0, 8)
    .gain(slider(2, 0, 2))
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
    .roomsize(6)
    .gain(slider(1, 0, 1))
    .bpf("800:1.5")
    ._punchcard();

$DRONE: s("dungeon_sfx:1")
    .cpm(5)
    .gain(slider(0, 0, 1))
    .lpf(100)
    ._scope();