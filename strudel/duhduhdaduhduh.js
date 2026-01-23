setCpm(155 / 4);

const bds = s("bd")
    .bank("tr909");

$BDS: arrange(
    [1, bds.struct("1 1 1 1")],
    [1, bds.struct("1 0 1 1")],
    [1, bds.struct("1 1 [1 1] 1")],
    [1, bds.struct("1 1 1 1")],
)
    .duckorbit(2)

const sds = s("sd")
    .bank("tr909")
$SDS: arrange(
    [1, sds.struct("0 1 0 1")]
)

const claps = s("cp").bank("tr909");

$CLAPS: arrange(
    [1, claps.struct("[1 1 0 1 1 0 1 0 0 0 0 0 0 0 0 0 0]")],
    [1, claps.struct("<0>")]
);

const ss = s("supersaw")
    .gain(1)
    .lpf("700")
    .dec(.9)
    .sus(.6)
    .crush(4)
    .fast(2)
    .rib(0, 4)
    .room(1)
    .roomsize(6)
    .delay(.25)
    .delayfb(.2)
    .rib(0, 8)
    ;

$SS: arrange(
    [16, ss.note("<c3 - c3 a3? e3 - c3 d3? - a3 e3? d3 - c3 d3 e3?>"
        .add("<0,3,7> <2,5,9>"))]
)
    ._pianoroll();


const keys = s("gm_pad_poly")
    .n(2)
    .dec(.9)
    .note("c3".add(tri.range(0, 7).seg(16)))
    .gain(3);

$KEYS: arrange(
    [1, keys.struct("[1 0 0 1 0 [1 1] 0 1]")],
    [1, keys.struct("[1 0 0 1 1? 0 1 1]")],
    [1, keys.struct("[1 1 0 1 0 0 1 0]")],
    [1, keys.struct("[1 1 0 [1 1 1] 0 0 1 0]")],
)
    .rib(0, 8)
    ._pianoroll()