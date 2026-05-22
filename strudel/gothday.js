setCpm(120 / 4);


let kick = [
    "3:1",
    "3:.9",
    "3:.7",
    "5:1",
    "5:.9",
    "5:1.1"
]

KICK: stack(...kick).arp(irand(kick.length).seg(4)).as("n:velocity").s("spacedrum_bd").room(.7).roomsize(12).postgain(1.5).o(3)
    .duck(4).duckdepth(.2).duckatt(.01)._pianoroll()

let hats = [
    "hh:0:1",
    "hh:0:.9",
    "hh:0:.8",
    "hh:1:1",
    "hh:1:1.1",
    "oh:0:1",
    "oh:0:1.1"
];

HATS: stack(...hats).arp(irand(hats.length).seg(16)).as("s:n:velocity").bank("tr909").hpf(12000)

let snare = [
    "sd:2:1",
    "sd:3:0.8",
    "sd:4:0.5"
]

XNARE: stack(...snare).arp(irand(snare.length).seg(2)).as("s:n:velocity").bank("tr909")

BASS: s("wt_digital:2").lpf(200).lfo({ s: 8, sh: "tri", dc: -1, dr: 1.0 }).postgain(1).diode(1)._scope()
KEYS: "<C E D F D>".chord().voicing().transpose(-12).gain(2).s("sine").att(.1).hpf(500).lpf(2000).o(4)

ARP: "<E F>".chord().voicing().transpose("<0@2 -12@2>").arp(run(4).fast(2)).room(.3).roomsize(12).o(2)
    .hpf(2000).s("supersaw").degradeBy(.1).sometimes(ply(2)).rib(0, 8)._pianoroll()

KICK: silence
HATS: silence
XNARE: silence
BASS: silence
KEYS: silence
ARP: silence