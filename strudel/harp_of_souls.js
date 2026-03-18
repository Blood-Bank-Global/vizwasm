setCpm(135 / 4);
// $: ccv("<[[10 0] [20 0] [30 0] [127 0]]>".div(127)).ccn(2).midichan(5).midi('IAC Driver Bus 1');
$: "bd!4".s().bank("tr909").delay(.5).delayfb(.3).lpf(300).gain(1);
$: "<- - - [- cp cp cp]>".late(1 / 8).s().bank("tr909").hpf(6000);
$: "[lt:1 mt:1 mt -] lt lt:1 lt".late(1 / 16).s().bank("rolandtr808").gain(1);;
$: s("one").bus("1").gain(slider(2.5, 0, 2.5)).dry(0);
$: "c5".add.squeeze(rand.range(0, 4).floor().seg(8).degradeBy(.5)).sometimes(ply(2)).note().s("square")
    .att(.1).dec(.1).sus(.25).rel(.75).room(.4).roomsize(1).delay(0.05).delayfb(0.05).hpf(1000).lpf(1500)
    .rib("<0@4 0@4 0@4 8@4>", "<4@4 4@4 8@8>")
    ._pianoroll().gain(0).bmod({ bus: 1 });

$: "c4".add.squeeze(rand.range(0, 4).floor().seg(8).degradeBy(.5)).sometimes(ply(2)).note().s("pulse")
    .att(.05).dec(.1).sus(.25).rel(.75).room(.4).roomsize(2).delay(.1).delayfb(.2).hpf(500).lpf(3000)
    .rib("<7@4 11@4 7@4 7@4>", "<4@4 4@4 8@8>")
    ._pianoroll().gain(0).bmod({ bus: 1 });


$: "<C D E C F A>".chord().voicing().s("square").lpf(600).lfo({ r: 4.5, sh: 4 }).hpf(400)._scope();

// $:"[1 -]!32".note().gain(saw.seg(32)).lfo({r:4, sh:4}).midichan(1).midi('IAC Driver Bus 1');