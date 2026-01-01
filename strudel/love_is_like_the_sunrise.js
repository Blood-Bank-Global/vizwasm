setCpm(135 / 4);

const mk_signal = (beat, gain, signal, n) => {
    return "0"
        .apply(beat)
        .when(gain.lt(.1), x => x.mask())
        .add.squeeze(signal)
        .ccv()
        .ccn(n)
        .midi('IAC Driver Bus 1')
};

let akai = await midin('MPK mini 3');

///////////// DRUMS
let drum_beat = beat("0,8", 16);
let drum_gain = akai(70, 0);
DRUM: s("dungeon_perc:9")
    .apply(drum_beat)
    .dec(.4)
    .gain(drum_gain);

DRUM_SIG: mk_signal(drum_beat, drum_gain, "1 0", "0");

//////////////// CLAPS
let clap_beat = x => x
    .sometimesBy(.25, x => x.mask())
    .beat("0,2,4", 32)
    .slow(2)
    .rib(0, 8);
let clap_gain = akai(71, 0);
CLAPS: s("tr909_cp:4")
    .att(.05)
    .gain(clap_gain)
    .apply(clap_beat)
    ._punchcard();

CLAP_SIG: mk_signal(clap_beat, clap_gain, "1 0", "1");

//////////////// HATS
let hat_beat = x => x.fast(8).rib(0, 4);
let hat_gain = akai(72, 0);

HATS: s("hh")
    .bank("tr909")
    .n(irand(4).seg(8))
    .att(.1)
    .dec(2)
    .gain(hat_gain)
    .apply(hat_beat);

HAT_SIG: mk_signal(hat_beat, hat_gain, "1 0", "2")

/////////////// LEAD
let lead_beat = x => x
    .slow(2)
    .degradeBy(.1)
    .rib(0, 8);
let lead_gain = akai(73, 0);

LEAD: note("c3 a3 b3 [a3 d3] g3 b3 [a3 b3] a3")
    .s("dungeon_lead:0")
    .att(.7)
    .dec(1)
    .rel(.3)
    .room(.8)
    .duckorbit(2)
    .gain(lead_gain)
    .apply(lead_beat)
    ._punchcard();

LEAD_SIG: mk_signal(
    x => x.struct("x x x [x x] x x [x x] x").apply(lead_beat),
    lead_gain,
    "1 0",
    "3"
);

/////////////// CHORDS
let chord_gain = akai(74, 0);
CHORDS: note(chooseCycles(
    "<c3,b3,e3,c2>",
    "<a3,g3,f3,c4>",
    "<b3,e3,g3>",
    "<b3,e3,g3,c2>"))
    .s(sine)
    .att(.15)
    .dec(.5)
    .rel(.4)
    .sus(.4)
    .gain(chord_gain)
    .rib(92, 16)
    .orbit(2)
    ._scope();

CHORD_SIG: mk_signal(
    beat("0", 1),
    chord_gain,
    stepcat([15 / 16, isaw.seg(8)], [1 / 16, "0"]),
    "4"
);

//////////////// CHOIR

let choir_chord = akai(-1, 0, 'notes', true);
let choir_gain = akai(75, 0);

CHOIR: choir_chord
    .note()
    .s("gm_pad_choir:5")
    .att(.3)
    .dec(.8)
    .rel(.5)
    .sus(.4)
    .gain(choir_gain)
    .room(1)
    .phaser(4)
    ._pianoroll();



CLEAR: "~".when(akai(20, 9).gt(0), x => akai(-1, 0, 'clear'));

