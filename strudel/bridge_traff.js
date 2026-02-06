const kick = "bd".s().bank("tr909").lpf(500);

setCpm(94 / 4);
$KICK: arrange(
    [3, kick.struct("1 1 1 1")],
    [1, kick.struct("1 1 1 [1 1]")]
);

$SOP1: s("sop:0").scrub(".015@1").speed(1.0).gain(2)


const ahh = 0;

$AHH: s("sop:0")
    .scrub("0")
    .rib(0.0, 0.25)
    .gain(4)
    .dec(.25)
    .lpf(1000)
    .hpf(300)
    .fast(2)
    .note("<c3,e3> - <d3,f3> <c3,e3> - <d3,f3> <d3,f3> -")

const rim = "rim"
    .s()
    .bank("rolandtr808")
    .gain(1)
    .hpf(1400);

$RIM:
arrange(
    [1, rim.struct("1 1 1 1 1 1 1 1")],
    [3, "-"]
)

$DRONE: s('dungeon_pads')
    .n("0")
    .note("c2")
    .delay(.8)
    .delayfb(.2)
    .room(.5)
    .roomsize(3)
    .gain(2)
    .rel(.1)
    .sus(.1)
    .dec(.99)