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
        45: 1,
        46: 1,
        47: 1,
        48: 1,
        49: 1,
        50: 1,
        51: 0 // top right
    };
}

$FILT: djf(".5").orbit("1,2").gain(0);

setCpm(165 / 4);
$METRO: tog("rim".struct("[1 1 1 1]"), 39).s().bank("tr909").postgain(.4).spectrum()

$SNARE: n("2").s("hftu_snare").struct(tog("<[1 1 1 1] [1 1 1 [1 1]] [1 1 [1 1] [1 1]]>", 47)).offset(.2).lpf(5000)
$KICK: tog("1 1 1 1", 45).n().s("hftu_kick").almostNever(ply("2")).postgain(.6).room(.5).roomsize(1).delay(.1).delayfb(.1)
$HAT: n(tog("0", 46)).s("hftu_hat").struct("<[1 1 1 1]!2 0>".late(.125))

$GLIDE: s(tog("sine", 44)).note("g2")
    .att(.04).sus(.5).dec(.9).rel(1)
    .postgain(4).lpf(200).room(.2).roomsize(2)

$STAB: tog("[c2 c3 a2 c3]*2", 50)
    .s("hftu_stab:5")
    // .add(24).s("sine").gain(3)
    .note()
    .degradeBy(.4)
    .room(.9)
    .roomsize(2)
    .duck(2)
    .duckatt(.15)
    .duckdepth(.9)
    .rib("<0@10 _@10 8@10>", "<2@2 2@2 6@6>")
    .postgain(.6)
    ._punchcard()

$MELODY: tog("<Cm Dm F@2 Cm Dm Gm Gm>", 48)
    .chord()
    .hpf(1000)
    .voicing()
    .s("gm_synth_strings_1:6")
    .delay(.2).delayfb(.3).room(.5)


$VOCAL: tog("g4", 49).note().s("gm_choir_aahs:2").gain(1).o(2).lpf(1000).hpf(500)