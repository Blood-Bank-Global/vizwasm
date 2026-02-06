const kick = "bd"
    .s()
    .bank("rolandtr808")
    .gain(2);

S$KICK: arrange(
    [3, kick.struct("1 0 1 0").n("0")],
    [1, kick.struct("1 0 1 1").n("0")],
    [3, kick.struct("1 0 [1 1] 0").n("0")],
    [1, kick.struct("1 1 1 1").n("0")]
);

const rim = "rim".s().bank("rolandtr808");
S$RIM: arrange(
    [2, rim.struct("1 0 0 0")],
    [1, rim.struct("[1 1 0 0] 1 1 0")]
)

const weeps = "casio:3"
    .s()
    .att(0.2)
    .dec(0.03)
    .gain(3)
    .lpf(1200)
    .hpf(1000);

S$WEEPS:
arrange(
    [4, weeps.struct("1 1 1? 1 1 1 1 1").penv(saw.seg(8).mul(4))],
    [1, weeps.struct("1 [1 1] 1 1 1 [1 1] 1 1").penv(saw.seg(8).mul(4))]
)

S$DRONE: s("bard_flute@2")
    .n("0")
    .delay(.1)
    .delayfb(.2)
    .room(1)
    .roomsize(2)
    .gain(.5)
    .lpf(1400)
    .hpf(1000)

S$PlUCKS: "c3!8"
    .add(saw.seg(8)).note().s("gm_bird_tweet")
    .n(0)
    .att(.3)
    .dec(.5)
    .degradeBy(.5)
    .rib(0, 12)
    .gain(3)


S$CHORDS: "c4 c4 c4 c4"
    .add("<0,3> <3,5> <3,7> <0,5>")
    .add("0 1".slow(8))
    .slow(4)
    .note()
    .o(2)
    .s("gm_pad_sweep:4")
    .lpf(1000)