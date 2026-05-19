setCpm(94 / 4)

DRONE: "G2".note().s("pulse").lpf(300).lfo({ dc: -1, dr: 1, s: 0.5, sh: "tri" }).diode(1).postgain(1.5)._scope()
let kicks = [
    "dark_perc:4:1",
    "dark_perc:4:.85",
    "dark_perc:4:.85",
    "dark_perc:4:.75",
    "dark_perc:4:.75",
    "dark_perc:4:.75",
];

// let kicks = [
//   "tr909_bd:2:1",
//   "tr909_bd:2:.75",
//   "tr909_bd:2:.85"
// ]
VIZ: stack(
    "[64 96 64 96]".div(127).ccv().midichan(5).ccn(16).midi('IAC Driver Bus 1'),
    "[96 64 96 64]".div(127).ccv().midichan(5).ccn(26).midi('IAC Driver Bus 1')
)
KICK: stack(...kicks)
    .arp(irand(kicks.length).seg(4)).as("s:n:velocity").room(.5).roomsize(4).sometimes(x => x.begin(0.01)).att(0)
    .sometimesBy(0.05, ply(2))
    .sometimes(x => x.clip(1.1))
    .hpf(300)
    ._pianoroll()

CHORD: "<Am Cm Dm Gm Dm Gm D>".chord().voicing().transpose("<-24@7 -32@7>").s("dark_pad:0").dec(1).sus(.5).rel(1).hpf(300).lpf(1500)
    .lfo({ s: 16, dc: -1, dr: 1, sh: "saw" })

BELL: "<sleighbells:1 -!1>".s().postgain(3).o(2).duck(1).duckatt(.15).duckdepth(.5)

xSH: "0!16".s("bard_perc:12").degradeBy(0.2).hpf(12000).postgain(3)

ARP: "<Gm Fm>".chord().voicing().transpose("<12@2 8@2>").arp(run(8).fast("<2@4 4@4>")).s("supersaw")
    .room(.7).roomsize(8).hpf(300).lpf("1000").postgain(2.5)
    .superimpose(x => x.squeezeBind(_ => "[1 0]").ccv().midichan(5).ccn(60).midi('IAC Driver Bus 1'))
    ._pianoroll()


VIZ: silence
KICK: silence
CHORD: silence
xSH: silence
