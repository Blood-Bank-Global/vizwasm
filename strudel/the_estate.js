const kick = "bd:1".s().bank("rolandtr808").gain(1.5)
    .dec(1)
    .sus(.5)
    .rel(.9)
    ;

setCpm(98 / 4);

$KICK: arrange(
    [1, kick.struct("1 0 0 [1 1]")],
    [3, kick.struct("1 1 0 0")]
);

const wind = "bard_wind:3".s()
$WIND: wind
    .note("<c3 - - - - - - ->")

$CHIME: arrange(
    [1, note("[c2 c2 c2 - ]")],
    [3, note("[c3 -  -  - ]")]
).s("bard_hand_cymbal:5")


$CHORDS: arrange(
    [1, "c4 [f4 e4] c4 [d3 e3]"],
    [1, "c3 [e3 f3] g3 e3"]
)
    .slow(2)
    .add("<0,5,9>")
    .note()
    .s("supersaw")
    .att(.1)
    .delay(.4)
    .delayfb(.25)
    .hpf(600)
    .lpf(2000)
    .rib(0, 4)

$KEYS: "c4!8"
    .degradeBy(.5)
    .add(perlin.range(0, 12).floor().seg(8))
    .note()
    .s("gm_harpsichord:8")
    .gain(1)
    .rib(0, 8)
    ._punchcard()
