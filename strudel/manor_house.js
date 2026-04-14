setCpm(110 / 4)


let energy = "<0 1 2 3 4@2 3*2 2*2 1*2 0>/8";

const eget = register('eget', (p) => {
    return pick(p, energy);
})

let beat_p = [
    "[bd!4]"
];

$BEAT: eget(beat_p).bank("tr808").n(6).gain(1.1).room(.5).roomsize(4).s()
    .superimpose(x => x.bind(_ => "[1 0]").ccv().ccn(0).midichan(1).midi('IAC Driver Bus 1'))
    ._scope()

let snare_p = [
    "-",
    "[sd hh sd hh]"
];
$SNARE: eget(snare_p).bank("tr808").n(2).late(1 / 8).s()._scope()

let tom_p = [
    "-",
    "[lt mt ht lt ht lt mt mt]",
    "[lt mt ht lt ht lt mt mt]".sometimes(ply(2))
];

$TOM: eget(tom_p).bank("spacedrum").s().lpf(80)._scope()

let string_p = [
    "-",
    "-",
    "-",
    "<4@16 0 2 6@6 4@16 0 2 0 2 6@4>*16"
]

$STRING: eget(string_p).scale("g:major").add("0,7,9").transpose("<12 8>/3").note().s("gm_violin").n(3).delay(.25).delayfb(.25)


let horns_p = [
    "-",
    arrange([1, "<0@4 2 2 4@4 -!6>*8"], [1, "-"]),
];

$HORN: eget(horns_p).scale("g:major").note().s("gm_french_horn").n(1).gain("<1 1.25 1.5 1>*8").transpose("<0,12>")

let arp_p = [
    "-",
    arrange([3, "<0 0 [0 1 2 3 0 1 2 3]>*2"], [3, "-"]),
    arrange([3, "<[0 1] [0 1] [0 1 2 3 0 1 2 3]>*2"], [3, "-"]),
    arrange([3, "<[0 1] [2 4] [0 1 2 3 0 1 2 3]>*2"], [3, "-"]),
    "<[0 1] [2 4] [0 1 2 3 0 1 2 3] 0>*2"
];
$ARP: eget(arp_p).scale("g:major").note().transpose("4,16").s("gm_piano").n(2)
    .superimpose(x => x.squeezeBind(_ => "[1 0]".ccv().ccn(1).midichan(2).midi('IAC Driver Bus 1')))
