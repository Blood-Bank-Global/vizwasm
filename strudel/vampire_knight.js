setCpm(84 / 4);


KICK: "[bd:0 [bd:1 bd:0] bd:0 bd:0]".as("s:n").bank("tr909").room(.2).roomsize(2).diode(0.5)

let xnare = [
    "sd:0:1.1",
    "sd:0:1.0",
    "sd:0:0.9",
    "sd:0:0.8",
    "sd:0:0.75",
];

XNARE: stack(...xnare).arp(irand(xnare.length).struct("[- x - x]")).as("s:n:velocity").bank("tr909")

CHORD: "<Gm Fm Dm Cm>".chord().voicing().s("wt_digital").lpf(6000)

PAD: "<Gaug Faug>".chord().voicing().arp(run(8).seg(8).fast(2)).s("dungeon_lead:3").transpose("<-24@16 -12@16>").lpf(1000).adsr(".05:1:.5:.5")
    .room(.1).roomsize(2).degradeBy(.25).rib("<0@16 0@16>", 32)._pianoroll()
// XFX: "<gm_bird_tweet:0 -!7>".s().gain(2)
XFX: "<g4 -!7>".note().s("gm_flute:0").gain(2).adsr("0:2:.5:1")

BREAK: s("more_breaks:0").splice("8", "<[0 2 2 0] [2 3 2 0] [3 0 3 2] [3 3 3 2]>").hpf(4000).lpf(5000).adsr("0.5:1:.8:.2")

DRONE: "g2".note().s("saw").gain(2).lpf("<2500 4000>").lfo({ sh: "isaw", s: "2", dc: -1, dr: 1 })._scope()
KICK: silence
XNARE: silence
CHORD: silence
PAD: silence
XFX: silence
BREAK: silence
DRONE: silence