if (true) {
    window.myrepl = {};
    window.myrepl.values = {
        36: 0, // bottom left
        37: 0,
        38: 0,
        39: 0,
        40: 0,
        41: 0,
        42: 0,
        43: 0,
        44: 1,
        45: 1,
        46: 1,
        47: 1,
        48: 1,
        49: 1,
        50: 1,
        51: 1 // top right
    };
}

// $FILT_CTL: s("one").seg(8).gain(slider(0.5, 0, 1)).bus(2).dry(0)
$FILT: djf("0.5")
    // .bmod({b: 2})
    .orbit("1,2")
    .gain(0);

setCpm(165 / 4);
$METRO: tog("rim".struct("[1 1 1 1]"), 39).s().bank("tr909").postgain(1)

$BEAT: s(tog("[mt mt mt mt mt mt lt mt]", 44)).n("0").bank("spacedrum").note("c3").sometimes(ply(2)).lpf(200)._scope()

$KICK: tog(arrange(
    [1, "0!4"],
    [1, "0!2"]
), 47).n().s("hftu_kick")._punchcard()
$SHAKE: tog(n("[0 1 0 0]*2"), 45).s("hftu_shaker")
$HATS: tog("[hh hh:1 hh oh hh:2 hh oh? hh]".late(1 / 8), 46).s().bank("tr909")

$RISER_CTL: s("<one>").seg(128).gain(saw.range(0, 2).seg(128)).bus(1).dry(0)
$RISER: s("<hftu_sustained:2 -!7 >").fit().gain(0).bmod({ b: 1, dc: 0 })._scope()
$SAW: tog("<Cm@0.5 Dm@0.5 Em C@0.5 Em@0.5 E Dm>", 48).s("dark_organ:2").chord().voicing().dec(.5).sus(.5).rel(.6).trans(-24)
    .lpf(1000)
    ._scope()

$KEY: tog("C4", 49).add.squeeze(perlin.range(0, 16).seg(8).floor()).note().s("square").room(.5).roomsize(4).delay(.2).delayfb(.2).degradeBy(.5)
    .att(0)
    .dec(.2)
    .rib("<0@4 0@4 5@4 2@4>", 4)
    ._punchcard()