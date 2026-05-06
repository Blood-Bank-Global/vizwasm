
setCpm(125 / 4);


$: "{60:.5:4 60:1:2 62:1:1.1 64:1:1.1 66:1.1:1.2 [68:1:4!2]}%8".as("note:velocity:clip")
    .s("saw").dec(.2).sus(.3).rel(.25).room(.5).roomsize(4).degradeBy(.5)

$: "bd!4".s().bank("tr909").duck(2).duckdepth(.75)
$: "<36:8 ~:1!7>".as("note:clip").s("pulse").lpf(500).o(2)._scope()
$: "[.18:2 .175:2 .185:2 .18:2]!2".as("begin:clip").s("more_breaks:40").hpf(13000)

$: "<2 ~!7>".as("clip").s("bard_flute:4").delay(.2).delayfb(.8).room(.5).roomsize(2).lpf(10000).duck(1).duckdepth(.3)

$: "{72 76 74 78}%16".add("-4,4").note().gain(0).lfo({ s: 0.125, sh: "tri", dc: 0, da: 1 })._scope()
$: "[cp ~!3]".bank("tr909").s().sometimes(ply(4))

$: "<0:1.25 4:1.25 2:1.25 4:1.5>".as("note:clip").transpose("48,55,59").s("dungeon_pads:0")
    .postgain(.7).sus(.5).dec(.5)

$: "<[0.1:1.25:.7 0.1:1.5:1 0:1.5:2 [.75:1.5:2!2]] [0.1:1.25:.7!4]>".as("begin:clip:velocity").s("acapella:1").gain(2)