setCpm(135 / 4);

const v = s("<voice_test ->").note("g2").scrub("0.15").room(.5).roomsize(2).hpf(1000).lfo({ r: 16, sh: 1 });

const cc = 0;

const v1 = ["-", v]
$: pick(v1, cc).gain(1).scope()
const v2 = ["-", "-", v];
$: pick(v2, cc).late(2 / 8).gain(.6).pan(1)
const v3 = ["-", "-", v];
$: pick(v3, cc).late(3 / 8).gain(.5).pan(-1)
const v4 = ["-", "-", "-", v]
$: pick(v4, cc).late(4 / 8).gain(.4).pan(1)
const v5 = ["-", "-", "-", "-", v]
$: pick(v5, cc).late(5 / 8).gain(.3).pan(-1)

_$: "[bd!4]".s().bank("tr909").delay(.1)._scope()
_$: "[mt mt mt -] - - -".s().bank("spacedrum").gain(.5)._scope()
_$: "{c3 e3 d3}%8".note().s("saw").transpose("[0,12]").transpose("<-5 -2>").lpf(500)._punchcard()
$: "<C G E D>".chord().voicing().transpose(-12).s("supersaw").slow(2).gain(.7).lfo({ r: 8 }).lpf(1000)._punchcard()