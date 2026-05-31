setCpm(92/4);

let lead = "<C C7 D D7 >".chord().voicing().transpose("<-12@8 -8@8>").s("dungeon_lead:0").adsr("0.1:1:1:0").crush(4)
  .gain(.25).lfo({sh:"sine", s: "<4@4 2@4>", dc: -.25, dr: 1.0})._scope()

let tom = "<[6!8]>".n().velocity(rand.range(.7,1)).s("dungeon_perc").rarely(late(.01)).lpf(1000).sometimes(ply(2))._pianoroll()

let kick = "[0!4]".s("tr909_bd")

let many_hats = [
  "hh:0:1",
  "hh:0:.9",
  "hh:0:.7",
  "oh:0:1.1",
  "hh:1:1",
  "hh:1:.9",
  "hh:1:.7",
  "oh:1:1.1"
];

let hats = irand(many_hats.length).pick(many_hats).as("s:n:velocity").seg(16).bank("tr909").cut(1).rib(0, 8)._pianoroll()
let snare = "[- 1 - 1]".s("white").dec(.3).diode(1).room(.4).roomsize(4)


let flute = "<C D>".chord().voicing().s("bard_flute:6").arp(irand(8).seg(8).fast(2)).transpose("-24").degradeBy(0.2).room(.2).roomsize(2)._pianoroll()
let elems = {
  lead: lead,
  tom: tom,
  kick: kick,
  hats: hats,
  snare: snare,
  flute: flute
}

MUSIC: arrange(
  [2, "-"],
  [2, "tom".pick(elems)],
  [2, "tom,snare,kick".pick(elems)],
  [4, "tom,snare,kick,hats,lead".pick(elems)],
  [8, "tom,snare,kick,hats,lead,flute".pick(elems)],
  [4, "tom,lead,flute".pick(elems)],
  [4, "tom,snare,kick,hats,lead,flute".pick(elems)],
  [2, "tom,hats,lead,flute".pick(elems)],
  [2, "tom,hats".pick(elems).gain(1).lfo({s:.5, sh: "saw", dc: -1, dr: 1.0 })],
  [2, "-"]
  // "lead,tom,snare,flute,hats,kick".pick(elems)
)._scope()