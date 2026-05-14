
setCpm(145 / 4);

$DRONE: "<sqr:C1:2 ->".as("s:note:clip").lpf(100).gain(2).lfo({ sh: "square", s: "2" }).postgain(1.5)._scope()

$LEADS: "<{52 54 52 54 56 [60 56] [54 52] 50}%0.5>"
    .note()
    .transpose("0,4")
    .transpose("<0@8 -8@8>")
    .s("saw")._pianoroll()


$KICK: "<[1 1 [1|[0.75 0.75]] 1] [1 1 1 [0.5 .75]]>".as("velocity").s("bd").bank("tr909")._scope()

$SH: "sh!16".s().degradeBy(.1).o(2).lpf(8000)._scope()
$SNARE: "[1:0 1:1 1:0 1:1]".as("velocity:n").s("tr909_sd").almostNever(ply(2))

let hats = [
    ".7:0", ".8:0", ".9:0", "1:0", ".8:1", ".9:1"
];

$HATS: stack(...hats).as("velocity:n").arp(irand(hats.length).seg(8)).s("hh").duck(2).duckdepth(.75).hpf(8000).postgain(2)