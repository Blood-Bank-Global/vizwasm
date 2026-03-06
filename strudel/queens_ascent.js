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
        44: 0,
        45: 0,
        46: 0,
        47: 0,
        48: 0,
        49: 0,
        50: 0,
        51: 0 // top right
    };
}

$FILT: djf(".75").orbit("1,2").gain(0);

setCpm(165 / 4);
$METRO: tog("rim".struct("[1 1 1 1]"), 39).s().bank("tr909").postgain(.4).spectrum()

$SNARE: n("2").s("hftu_snare").struct(tog("<[1 1 1 1] [1 1 1 [1 1]] [1 1 [1 1] [1 1]]>", 47)).offset(.2).lpf(5000)
$KICK: tog("1 1 1 1", 45).n().s("hftu_kick").almostNever(ply("2")).postgain(.6).room(.5).roomsize(1).delay(.1).delayfb(.1)
$HAT: n(tog("0", 46)).s("hftu_hat").struct("<[1 1 1 1]!2 0>".late(.125))

_$GLIDE: s("hftu_glide").att(.3).dec(.99).rel(1).sus(1).delay(.5).room(.1).lpf(3000)

$STAB: tog("[c2 c3 a2 c3]*2", 50)
    // .add.squeeze(saw.seg(8))
    .s("hftu_stab:11")
    // .add(24).s("sine").gain(3)
    .note()
    .degradeBy(.4)
    .room(.9)
    .roomsize(2)
    .duck(2)
    .duckatt(.25)
    .duckdepth(.5)
    .rib("0", "<2@2 2@2 6@6>")
    .postgain(.6)
    ._punchcard()

$MELODY: tog("<Cm Dm F@2 Cm Dm Gm Gm>", 48)
    .chord()
    .hpf(1000)
    .voicing()
    .s("gm_synth_strings_1:6")
    .o(2).delay(.2).delayfb(.3).room(.5)


$VOCAL: tog("g4", 49).note().s("gm_choir_aahs:2").gain(1).o(2).lpf(1000)