const kick = "bd:1".s().bank("rolandtr808")
    .dec(1)
    .sus(.5)
    .rel(.9)
    .lpf(50);

setCpm(120 / 4);

$: djf(slider(0.5, 0.000, 1.000))

$KICK: arrange(
    [1, kick.struct("1 1 1 0")],
    [1, kick.struct("[1 1] 1 1 0")]
);

$SNARE: "sd:1".s().bank("rolandtr808").struct("0 1 1 0")
    .lpf(2000)
    .hpf(800)

const hats = s("casio:2").att(.3).dec(.2).hpf(4000)
$HATS: arrange(
    [2, hats.struct("[1 1] 1 [1 1] 1")],
    [1, hats.struct("1 [1 1] 1 1")]
)

$CHORDS: "<c3,e3,g3> <e3,gb3,a4>@2 <f2,c3,e3> <d3,fb3,g3> <f2,c3,e3>@2 <f2,c3,e3>"
    .note()
    .slow(4)
    .s("gm_pad_poly:1")
    .o(2)
    .lpf(800)
    .hpf(700)

$BASELINE: "c1"
    .add.squeeze(
        saw.range(0, 16).seg(8)
            .add(sine.range(-4, 12).seg(8))
            .floor())
    .degradeBy(.3)
    .note()
    .s("dungeon_plucked:1")
    .att(.2)
    .dec(.7)
    .lpf(300)
    .hpf(50)
    .delay(.2)
    .rib(20, 8)
    ._pianoroll()