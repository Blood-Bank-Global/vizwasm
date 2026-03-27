setCpm(110 / 4)

$CTL: "[[10 9 8 0 0 0 0 0]!4]".div(127).ccv().ccn(2).midichan(5).midi('IAC Driver Bus 1');
// $KICK: "[bd!4]".s().bank("tr909").gain(.5)
_$KICK: s("[bard_perc:3!4]").gain(1).room(.1).roomsize(2).delay(.1).delayfb(.3).lpf(1000).duck(2);
_$cymbal: "bard_hand_cymbal:3!8".s().sometimes(ply(2));
$HAT: "hh hh:2 hh hh:3".late(1 / 8).s().bank("rolandtr808");
$OH: "oh? - oh:1 -".late(2 / 8).s().bank("rolandtr808");


$: "<Cm Em Gm>".chord().voicing().s("gm_pad_halo")
    .room(.1).roomsize(2).delay(.4).delayfb(.3)
    .hpf(200)
    .gain(0).lfo({ sync: "<2@3 8@3 4@6>", sh: "sine", skew: ".5", dc: "0", da: "1.0" })

$DRONE: "c2".note().s("supersaw").orbit(2).gain(2).dec(.95).sus(.5).rel(1).att(.5).lpf(200)._scope()
$WHISTLE: s("<bard_flute:5? - >")