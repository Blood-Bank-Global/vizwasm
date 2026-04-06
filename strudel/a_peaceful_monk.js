setCpm(92 / 4)


let energy = "<0 1 2 3!2 4!2 5!8 3!1 2!1 1!1 0!8>";

const eget = register('eget', (p) => {
    return pick(p, energy);
})

let beat_pat = [
    "-",
    "[bd - bd -]",
    "[bd - bd -]",
    "[bd!4]"
];

$BEAT: eget(beat_pat).s("bard_perc:4").almostNever(ply(2)).lpf("100").gain(1.5)._punchcard()

let bass_pat = [
    "-",
    "-",
    "[g1!8]".late(1 / 8)
];
$BASS: eget(bass_pat).note().s("gm_acoustic_bass:0").dec(.5).lpf(500)._scope()

$WARP: eget(bass_pat).bind(_ => "0").add.squeeze(fastcat([saw.range(0, 0.3).seg(16), "[0!96]"])).ccv().ccn(1).midichan(2).midi('IAC Driver Bus 1')._punchcard()

let string_pat = [
    "-",
    "-",
    "-",
    "-",
    "<{G F E@2 C D E@2 F G A@2}%4/2>"
]

$STRING: eget(string_pat).chord().voicing()
    .s("gm_sitar")
    .transpose(0)
    .att(0.1).dec(0.3).sus(1).rel(1)
    .gain(.2)
    .lfo({ sync: 4, dc: 0, da: .7, sh: "sine" })
    .superimpose(x => x.collect().squeezeBind(_ => "[1 0 -!6]").ccv().ccn(0).midichan(1).midi('IAC Driver Bus 1'))
    ._scope()
let arp_pat = [
    "-",
    "-",
    "-",
    "-",
    "-",
    "<{0 2 4 2 4 6 4 6 8}%4>"
        .sometimes(ply(2)).degradeBy(8 / 16).rib(0, 16)
];
$ARP: eget(arp_pat)
    .note()
    .transpose(64).transpose("12,0")
    .s("gm_sitar").gain(.5)
    .dec(0.9).delay(.3).delayfb(.3)
    .superimpose(x => x.collect().squeezeBind(_ => "[1 0]").ccv().ccn(0).midichan(2).midi('IAC Driver Bus 1'))
    ._punchcard()
let bell_pat = [
    "-",
    "<1 -!3>"
];
$BELL: eget(bell_pat).late(1 / 8).degradeBy(.4).att(.1).dec(.8).sus(.9).rel(.5).s("gm_tinkle_bell")._punchcard();

let shake_pat = [
    "-",
    "-",
    "-",
    "[sh!16]"
];
$SHAKE: eget(shake_pat).s().gain("{1 .2 .7 .05 .8 .1 .1}%16")._punchcard()


