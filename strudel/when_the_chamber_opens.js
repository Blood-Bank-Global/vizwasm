setCpm(140 / 4)

$CHORDS: "<Cm Dm Gm Dm Am>".chord().voicing()
    .transpose(-12)
    .arp(run(8).fast("2")).transpose("[0,8]")
    .s("dark_pad:2")
    .dec(.25)
    .velocity("<0.9 1 1.2>")


$KICK: "dungeon_perc:5!4".s().room(.4).roomsize(4)._pianoroll()

$SNARE: n(pick(irand(2), [3, 2])).bank("tr909").s("sd").struct("- x - x")._pianoroll()
_$HH: "hh!8".bank("tr808").sometimes(x => x.n(1)).s()._pianoroll()

$TOM: "<[[4!2] [3!2] -!2] -!3>".as("n").s("bard_perc")

$BASS: "C1".note().s("square").clip(2).gain(1.0).lfo({ sh: "square", s: "2", dr: 1, dc: -0.5 }).lpf(300)


_$: "<0.25:.5 0.25:.25>".as("begin:clip").s("gm_choir_aahs").note("c4").gain(1).degradeBy(".2").sometimes(ply("4"))
    .room(.7).roomsize(8).delay(.5).delayfb(.25)
    .rib(11, 16)
    ._pianoroll()