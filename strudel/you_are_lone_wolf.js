setCpm(160 / 4);

$HATS: stack(
    "hh".beat("2,6,10,14", 16).gain(2),
    "oh".beat("4,12", 16).ply(2)
)
    .n(irand(4).seg(16))
    .att(.025)
    .dec(.1)
    .rib(0, 4)
    .s()
    .bank("tr909")
    .gain(1)
    ._punchcard();

$DRUM: stack(
    "dungeon_perc:6".beat("0,8", 16),
    "dungeon_perc:5".beat("2,10", 16).degradeBy(.5)
)
    .rib(0, 4)
    .gain(1)
    .s()
    ._punchcard();

$KICK: "bd".beat("0,4,8,12", 16)
    .bank("tr909")
    .s()
    .att(.01)
    .gain(1)
    .dec(.3);

$ARPS: "c3 e3 g3 d3"
    .add(berlin.range(-1, 2).seg(16))
    .note()
    .s("gm_pad_new_age:0")
    .struct("x x x ~ ~ x ~ ~ x x ~ ~ ~ x ~ ~")
    .dec(.1)
    .rel(.5)
    .sus(.5)
    .rib(0, 4)
    .gain(2)
    .duckorbit(2)
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
    ._punchcard();

// $DRONE: s("dungeon_sfx:2 dungeon_sfx:2").clip(1.0).dec(2).speed("1 1")