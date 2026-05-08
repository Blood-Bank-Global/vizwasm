
setCpm(93 / 4);

_$DRONE: "<sqr:C1:2 ->".as("s:note:clip").lpf(100).gain(1)._scope()


$LEAD: "C3:.5:4,C3:1:1,E3:0.5:4,E3:1:1".as("note:velocity:clip").arp(irand(4).seg(8))
    .transpose("<0@4 8@4>")
    .s("bard_plucked_string:5")
    //^^^ supersaw
    .lpf(5000)
    .room(.5).roomsize(2)
    .rib("<1@4 1@4>", 8)
    ._pianoroll()


let kick = [
    "1",
    "1.1:.8",
    "1.1:1",
    ".5:1.2",
    "1:1",
    ".75:1.1"
];


_$KICK: stack(...kick).as("velocity:clip").arp("{0 1 2 3 4 5}%4").s("dungeon_perc:5").lpf(300)._pianoroll()

_$SHAKE: stack(
    "1:1",
    ".9:.9",
    "1.1:1.1",
    ".5:.8",
    "0.25:2",
    "0.1:1",
    "0.1:0.5"
).as("velocity:clip")
    .s("white")
    .arp(irand(4).seg(8)).hpf(12000)
    .att(.1).dec(.05).sus(.25).rel(.1)
    .postgain(0.5)
    ._pianoroll()


let vox = [
    "G4:1",
    "F4:.6",
    "E4:.9",
    "A4:1"
];

_$VOX: stack(...vox).as("note:velocity")
    .arp(irand(vox.length).seg(vox.length))
    .transpose("0,7")
    // .s("acapella:0").clip(1)
    // .s("gm_voice_oohs")
    .s("gm_choir_aahs:6")
    // .s("num:7")
    .slow(vox.length)
    .gain(1)
    .lfo({ s: 16, dc: -0.25, dr: 1.0, sh: "square" })
    // .lpf(1400)
    .dec(.8).sus(.25).rel(.25)
    .rib(7, 8)
    ._pianoroll()

_$CLAP: s("<drumulator_cp ~!7>").delay(.2).delayfb(.7).room(.7).roomsize(8)


let lines = [
    "[1!128]",
    "[0 0 0 0]".add.squeeze(tri.range(60, 127).div(127).seg(128))
];

$: pick("0", lines).ccv().ccn(0).midichan(2).midi('IAC Driver Bus 1')
let blanks = [
    "[1!128]",
    sine.range(0, 0.5).seg(128)
];
$: pick("1", blanks).slow(4).ccv().ccn(1).midichan(2).midi('IAC Driver Bus 1')
// $: "[[1 0]!4]".ccv().ccn(49).midichan(5).midi('IAC Driver Bus 1')