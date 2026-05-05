
setCpm(125 / 4);

$BOOMS: "[0 0 0 0]".add(12).s("pulse").lpf(1000).dec(.1).gain(3).room(.5).roomsize(4).almostNever(ply(2)).o(4).duck(2).duckdepth(.4).duckatt(.25)

$SNARE: "[- 0 - 0]".s("white").dec(.2).hpf(2500).lpf(4000).delay(.125).delayfb(.5)
    .superimpose(x => x.squeezeBind(_ => "[1 0]").ccv().ccn(0).midichan(1).midi('IAC Driver Bus 1'))._scope()
$CLAP: "[0 -]".s("white").att(0.01).dec(0.05).hpf(600).lpf(1000).fast("<4 8>").gain(0).lfo({ s: "<4 8>", sh: "saw", dc: 0, da: 4 }).diode(1)._scope()

$WAVE: "0".add(60).note().s("sine").gain(0.0).lfo({ s: "<8 4>", dc: 0, da: 1 }).diode(1).hpf(300)._scope()
$LEAD: "<0 4 6 2 4 0 2>".add(60).add("0,8").s("supersaw").o(2).note()._scope()
$ARP: irand(5).seg("<8!2 16!2>").add(72).note().s("supersaw").degradeBy(.25).dec(.1).att(0.01).room(.2).roomsize(8).rib("<0!3 10!2>", 2)._pianoroll()

$RISE: isaw.seg(128).slow(24).ccv().ccn(0).midichan(2).midi('IAC Driver Bus 1')