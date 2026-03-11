if (false) {
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

// $FILT: djf(".5").orbit("1,2").gain(0);

setCpm(135 / 4);


$BREAK: tog(s("breaks:2"), 48).note("g#1").scrub("<.17>").hpf(8000).lpf(9000).room(.4).roomsize(1).gain(3);
$KICK: tog(s("<[bd bd bd [bd|bd!2]]>"), 49).bank("rolandtr808").n(2).room(.8).roomsize(4).delay(.2).delayfb(.1).gain(2)
$DRIVER: tog("[[1 0] [1 0] [1 0] [1 0]]", 49).ccv().ccn(0).midi('IAC Driver Bus 1');
$CLAP: tog(s("<[cp cp cp cp]>"), 50).late(1 / 8).bank("rolandtr808").n(1)
$RIM: tog(s("<[rim!8]>"), 51).bank("rolandtr808").hpf(4000).sometimes(ply(2))
$HORN: tog(note("<c2 d2 e2@0.5 f2@0.5 d2 c2 d2 e2@0.5 f2@0.5 g2>"), 44).s("gm_brass_section");
const tones = "<a4 a#4 g4 e4 a4 a#4 d4 e4 f4 c4 e4 d4 g4>";
$KEYS: tog(note(tones), 45).s("gm_piano").n("<0,3>")
    .fast(4).degradeBy(.2).sometimes(ply(2))
    .att(.01).dec(.2).delay(.4).dry(2)
    .rib("<0@4 0@4 1@4>", 4)
    .gain(1)
    // .bmod({b: 1})
    ._pianoroll()


$CHORD: tog("<G A G E F A B>", 46).chord().voicing().transpose(-16).s("supersaw").slow(2).lpf(500).gain(1).bmod({ b: 1 }).scope()

$BUS_CTL: tog("tri", 47).s().freq(4).bus(1).gain(1.5).dry(0)._scope();