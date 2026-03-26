$CTL: "[[10 0 8 0 0 0 0 0]!4]".div(127).ccv().ccn(2).midichan(5).midi('IAC Driver Bus 1');
$KICK: "[bd!4]".s().bank("tr909").gain(.5)
$SHAKE: "sh!8".s().sometimes(ply(2));
$HAT: "hh - hh -".late(1 / 8).s();
$OH: "oh? - - -".late(2 / 8).s();
$DRONE: "c3".note().s("sine").dec(.95).sus(.5).rel(1).att(.5).lpf(700)
$SPARKLE: "{[c4|d4] c4 f4 [g4|d4]}%16"
    .sometimes(rev())
    .note()
    .transpose("0,16")
    .s("tri")
    .hpf(700)
    .lpf(1000)
    .dec(.2).sus(.5).rel(.2).delay(.3).delayfb(.1)
    .room(.3).roomsize(4)
    .gain(0.0)
    .lfo({ sync: "8", sh: "saw", skew: .5, da: 1.0, dc: 0, })
    .rib(0, 8)
    .scope()