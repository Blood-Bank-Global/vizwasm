setCpm(84 / 4);


let lead = "<E F E G>".chord().voicing().transpose("<0@4 -4@4>").s("gm_pad_halo:0").adsr(".1:.2:.5:.5")._scope()
let drone = s("pulse").freq("100").lfo({ s: 0.5, sh: "tri", dc: -0.5, da: 20 }).gain(0.5).room(.3).roomsize(8)._scope()

let kick_types = [
    "bd:3:1.1:.2",
    "bd:3:1.0:.2",
    "bd:3:1.0:.1",
    "bd:3:0.8:0",
];

let kick = irand(kick_types.length).pick(kick_types).as("s:n:velocity:room").roomsize(4).bank("tr909")
    .fast("<4@3 8@1>").duck(2).duckattack(.05).duckdepth(.25).rarely(ply(2))._pianoroll()


let breaks = s("more_breaks:6").splice(32, "<[0 5 4 1 2 3 4 5] [2 1 2 1 2 3 4 4] [6 7 4 7 7 7 5 7]>").postgain(1).fast(1).hpf(2000)
    .delay(.1).delayfb(.1).roomsize(4).room(.1).o(2)._scope()
let tones = [-2, 0, 2];
let arp = stack(...tones).arp(irand(tones.length).seg(4)).as("note").transpose(52).s("supersaw").room(.3).roomsize(8).lpf(1000).degradeBy(.25)._pianoroll()

LEAD: lead
DRONE: drone
KICK: kick
BREAKS: breaks
ARP: arp

let elems = {
    drone: drone,
    lead: lead,
    tom: "-",
    kick: kick,
    breaks: breaks,
    snare: "-",
}

let music = [
    [2, "drone"]
].map(x => [x[0], x[1].pick(elems)]);

_: arrange(...music)._scope()