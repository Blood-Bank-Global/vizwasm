setCpm(92/4);

LEAD: "<C C7 D D7 >".chord().voicing().transpose("<-12@8 -8@8>").s("dungeon_lead:2").adsr("0.1:1:1:0")
  .gain(.25).lfo({sh:"sine", s: "<4@4 2@4>", dc: -.25, dr: 1.0})._scope()


TOM: "<[6!8]>".n().velocity(rand.range(.7,1)).s("dungeon_perc").rarely(late(.01)).lpf(1000).sometimes(ply(2))._pianoroll()

KICK: "[0!4]".s("tr909_bd")

let hats = [
  "hh:0:1",
  "hh:0:.9",
  "hh:0:.7",
  "oh:0:1.1",
  "hh:1:1",
  "hh:1:.9",
  "hh:1:.7",
  "oh:1:1.1"
];

HATS: irand(hats.length).pick(hats).as("s:n:velocity").seg(16).bank("tr909").cut(1)._pianoroll()
XNARE: "[- 1 - 1]".s("white").dec(.3).diode(1).room(.4).roomsize(4)

FLUTE: "<C D>".chord().voicing().transpose("-20").s("bard_flute:6").arp(run(16).seg(8)).room(.5).roomsize(2)._pianoroll()