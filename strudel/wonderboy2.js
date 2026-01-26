setCpm(145 / 4);

const wb = s("wonderboy3:11");
const p_bass = "<0.0 0.0 0.0079 0.0>" // double oh seven
$BASSLINE: wb.scrub(p_bass)
    .lpf(300)
    .note("f1")
    .fast(2)
    .penv(rand.range(0, 16).seg(4).fast(2))
    .dec(.75)
    .rel(.1)
    .sus(.1)
    .rib(0, 8)
    .gain(3)
    ._scope();

const p_lead0 = ".81"
    .add.squeeze(rand
        .range(0, 5)
        .floor()
        .div(100)
        .seg(8))
    .rib(1, 8);

$: arrange(
    [1, wb.scrub(p_lead0)]
)
    .lpf(800)
    .hpf(300)
    .att(0.01)
    .dec(.3)
    .sus(0)
    .rel(.1)
    .gain(1.5)
    .delay(.4)
    .delayfb(.6)
    .room(.5)
    .roomsize(2)
    ._scope();

$: arrange(
    [0, "bd".struct("[1 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0]")],
    [1, "bd".struct("[1 0 0 0 1 0 0 0 1 0 0 0 1 0 0 0]")],
    [0, "bd".struct("[1 1 1 0 1 0 0 0 1 0 0 0 1 0 0 0]")]
)
    .s()
    .bank("tr909")
    .lpf(200)

const hats = "hh!16";

$HATS: arrange(
    [1, hats.struct("1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0")],
    [2, hats.struct("1 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0")],
    [2, "-"],
    [3, hats.struct("1 0!7 1 0!7")]
)
    .att(.03)
    .dec(.5)
    .sus(.1)
    .rel(.2)
    .delay(.1)
    .delayfb(.25)
    .n("1 2 3 4 1 2 3 4 1 2 3 4 1 2 3 4")
    .s()
    .bank("tr909")