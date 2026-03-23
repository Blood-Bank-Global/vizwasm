setCpm(135 / 4);

_$KICK: "bd:1!4".s().bank("rolandtr808").delay(.5).delayfb(.4)

_$SHAKE: ccv("[[1 0 -1 0 0 0 0]!4]".mul(1).add(64).div(127)).ccn(32).midichan(5).midi('IAC Driver Bus 1')
_$SHAKE2: ccv("[[1 0 -1 0 0 0 0]!4]".mul(1).add(64).div(127)).ccn(10).midichan(5).midi('IAC Driver Bus 1')

_HAT: "hh - hh - hh - hh - hh - hh - hh - hh -".n("0").att(.025).s().bank("rolandtr808")
_$CLAP: "[[cp?]!4] - - -".late(1 / 16).s().bank("rolandtr808")
$TICK: "<rim!4>".s().bank("rolandtr808")


_$PLUCK: "<[c e d f@2 - - -] - [d e f g@2 - - -] ->"
    .add("<-2 _ _ _ 2 _ _ _>").o(2)
    .note().s("gm_pad_warm:1").delay(.25).delayfb(.1)._punchcard()

//Dave inspired vvvv
$ARP: "{g4 a4 b4}%8".note().s("saw").room(1)
    .transpose("<[0,12]@2 [12,24]>").transpose(-5).lpf(1000).degradeBy(.2).gain(1).rib(0, 16)._punchcard()

_$CHORD: "<C E>".chord().voicing().transpose(-24).s("dungeon_pads:8").o(3)
    .lpf(700).att(.25).dec(.5).sus(.25).rel(.9).room(.4).roomsize(4).gain(0.5).lfo({ r: 8 })
    ._scope()
_$DRONE: s("supersaw").note("<36 38@2 30>".sometimes(add(4))).lpf(2000)

$VOCAL: "<c5@4 d5@4>".note().s("gm_voice_oohs:5").sus(.3).dec(1)