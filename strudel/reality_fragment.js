
setCpm(135 / 4);

$DRONE: "0".transpose(48).note().att(.5).dec(2).sus(1).rel(2).s("sine").gain(0.0).lfo({ sync: 8, dc: 0.0, da: 4.0 }).o(3)._scope()
$KICK: "[bd:2!4]".bank("tr909").s().delay(.1).delayfb(.8).o(3)
$HAT: "<[- 1 - 1]!3 [1 1 1 1]>".s("white").dec(.03).room(.3).sometimes(ply("2|4")).postgain(2.5).o(3)
$CLAP: "<[cp -!15]!3 [cp!4 -!12]>".s().o(3)

$ARP: "<[0 2 4 6 2 4 6 6 0 2 4 6 2 4 6 6]>".add(60).add("<[0,8] [4,12]>").note().sometimes(rev).rib(0, 8)
    .s("supersaw")
    .room(.5)
    .roomsize(8)
    ._pianoroll()
$AHH: "<[0 4 8 4] [4 2 4 -] 2 4>".add(72).note().s("gm_choir_aahs:6")

$NUM: s("num").n("<- - 0 1 2 3 4 - - 5 6 7 8 8 8 8 8>").o(2).duck(1).duckdepth(.5).gain(2).room(.8)