setCpm(84 / 4);
let kick_on = "[bd:0 [bd:1 bd:0] bd:0 bd:0]".as("s:n").bank("ms404").room(.2).roomsize(2).diode(0.5).sometimes(ply(2))
    .sometimes(late(.01))
    .superimpose(x => x.bind(_ => "1").ccv().ccn(0).midichan(1).midi('IAC Driver Bus 1'))
    ._pianoroll()
let kick_off = "[0!128]".dry(0).superimpose(x => x.bind(_ => "0").ccv().ccn(0).midichan(1).midi('IAC Driver Bus 1'))

let xnare = ["sd:0:1.1", "sd:0:1.0", "sd:0:0.9", "sd:0:0.8", "sd:0:0.75",];
let snare_on = stack(...xnare).arp(irand(xnare.length).struct("[- x - x]")).as("s:n:velocity").bank("tr909")._scope()
let chord_on = "<Gm Fm Dm Cm>".chord().voicing().s("wt_digital").lpf(6000)._scope()
let pad_on = "<Gaug Faug>".chord().voicing().arp(run(8).seg(8).fast(2)).s("dungeon_lead:3").transpose("<-24@16 -12@16>").lpf(1000).adsr(".05:1:.5:.5")
    .room(.1).roomsize(2).degradeBy(.25).rib("<0@16 0@16>", 32)._pianoroll()
let fx_on = "<- g4 -!6>".note().s("gm_flute:0").gain(2)._scope()
let breaks_on = s("more_breaks:0").splice("8", "<[0 2 2 0] [2 3 2 0] [3 0 3 2] [3 3 3 2]>").hpf(2000).lpf(4000).adsr("0.5:1:.8:.2")._scope()
let drone_on = "g2".note().s("saw").gain(1.25).lpf("<1500 4000>").lfo({ sh: "isaw", s: "2", dc: -1, dr: 1 })._scope()
let elems = {
    off: kick_off,
    kick: kick_on,
    snare: snare_on,
    chord: chord_on,
    pad: pad_on,
    fx: fx_on,
    breaks: breaks_on,
    drone: drone_on
};

MUSIC: arrange(
    [4, "off".pick(elems)],
    [4, "kick".pick(elems)],
    [4, "kick,snare".pick(elems)],
    [4, "kick,snare,drone".pick(elems)],
    [8, "kick,snare,drone,chord,fx,breaks".pick(elems)],
    [8, "kick,snare,drone,chord,fx,breaks,pad".pick(elems)],
    [4, "kick,snare,drone,chord,fx,breaks,pad".pick(elems).lpf(1000)],
    [1, "kick,snare,drone,chord,fx,breaks,pad".pick(elems).rib(1, .25)],
    [8, "kick,snare,drone,chord,fx,breaks,pad".pick(elems)],
    [4, "kick,snare,drone,chord,fx,breaks,pad".pick(elems).hpf(5000)],
    [8, "kick,snare,chord,fx,breaks".pick(elems)],
    [4, "kick,snare,breaks".pick(elems)],
    [4, "breaks".pick(elems)],
)._scope()
