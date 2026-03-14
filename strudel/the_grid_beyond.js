if (true) {
    window.myrepl = {};
    window.myrepl.values = {
        36: 0, // bottom left
        37: 0,
        38: 0,
        39: 0,
        40: 1,
        41: 1,
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
setCpm(145 / 4);
$KICK: tog(s("<[bd bd bd bd]>"), 48).bank("tr909").n(2).sometimesBy(.25, x => x.ply(2)).room(.8).roomsize(4).delay(.2).delayfb(.1).gain(1)._scope();
$SNARE: tog(s("<[sd - sd -]>".late(1 / 8)), 49).bank("tr909")._scope();
$RIM: tog(s("<[rim!16]>"), 50).bank("tr909").degradeBy(.4)._scope();
$CLAP: tog(s("<[[cp!4] - - - ] [cp - - -]!3>"), 51).bank("tr909")._scope();
$SHAKE: tog(s("<[sh!8]>"), 44).bank("rolandtr808")._scope();
$HATS: tog(s("<[hh:1 hh:2 hh:1 hh:0]>"), 45).bank("tr909").fast("<1 1 1 2>")._pianoroll();
$PLUCKS: tog("[0 2 4 6 0 4 2 6]", 46).transpose(60).note().s("supersaw").dec(.3).room(.5).roomsize(3)._pianoroll();
$CHORDS: tog("<C5 D5 C5 E5 D5 F5 D5 G5>", 47).chord().voicing().s("dark_pad:3").dec(.25).sus(.5).rel(.5).gain(1)
    .lfo({ r: 4, da: 1 }).hpf(800).lpf(1000)._scope();
$DRONE: tog("c1", 40).s("dark_pad:1").clip(1).lpf(800)._scope();
$CHOIR: tog("<e5@2 f5@2>", 41).note().s("gm_choir_aahs").hpf(1000).gain(1).lfo({ r: 0.5, da: 1 })._scope()