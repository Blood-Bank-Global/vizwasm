const kick = "bd".s().bank("tr909").lpf(500).duckorbit(2);

$KICK: arrange(
    [3, kick.struct("1 1 1 1")],
    [1, kick.struct("1 1 1 [1 1]")]
);

const hat = "casio:2"
    .s()
    .att(0.3)
    .dec(0.2)
    .gain(3)
    .hpf(1400);

$HAT:
arrange(
    [1, hat.struct("1 1 1 1 1 1 1 1").penv(saw.seg(8).mul(4))]
)

$DRONE: s('dungeon_pads')
    .n("0")
    .delay(.3)
    .delayfb(.2)
    .room(.5)
    .roomsize(1)
    .gain(.2)
    .lpf(1400)

$PlUCKS: "c2!8"
    .add(saw.seg(8)).note().s("dark_key").n(2).degradeBy(.3)
    .rib(0, 12)


$CHORDS: "c4 c4 c4 c4"
    .add("<0,3,7> <5,8,10> <0,3,7> <3,7,8>")
    .add("0 1".slow(2))
    .slow(4)
    .note()
    .o(2)
    .s("gm_pad_new_age:4")