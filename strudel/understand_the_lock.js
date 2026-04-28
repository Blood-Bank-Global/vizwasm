
setCpm(145 / 4);


$VOCAL: s("acapella:0").struct(pick(irand(6).seg(1), ["x@2 x@4 -@2", "x@4 -@4", "-@8"]))
    .gain(0.5).lfo({ sync: 8, dc: 0.0, da: 0.5, sh: "sine" })
    .room(.5).roomsize(8)
    .o(2)._scope()
$BOOPS: "{0 1 2 0 1 2}%8".add(60).add("<[0,7,14] [4,11,18]>").note().gain(.25)
    .degradeBy(.5)
    // .sometimes(ply("2|4"))
    .rib("<0 0 16>/16", 48)
    .s("saw").dec(.01).sus(.8).rel(.1)
    .superimpose(x => x.squeezeBind(_ => "[10 0]".div(127).ccv().ccn(2).midichan(5).midi('IAC Driver Bus 1')))
    ._pianoroll()

$BASE: "<2 4 6 4>/2".add(36).add("[0,7]").note().s("supersaw").lpf(2000).o(3).postgain(1.4)._scope()

$BREAK: s("more_breaks:40").slice(16, irand(16).seg(4)).fit().rib("<0 4 0 16>/2", 2).hpf(2000).gain(1.25)._scope()

$: "bd!4".s().bank("tr909").o(4).duck(1).sometimes(ply(2)).duckdepth(.25).gain(1.25).lpf(2000)
$: "[1 0 0 0]".ccv().ccn(0).midichan(1).midi('IAC Driver Bus 1')