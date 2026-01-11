setCpm(135 / 4);
const mock_akai = (cc, chan, kind, latch) => {
    if (kind == 'notes') {
        return "c4,g4,e4";
        // return "~";
    }
    if (cc == 1 && chan == 0) { //DJF
        return "0.5";
    } else if (cc == 20 && chan == 9) { //CLEAR
        return "0";
    } else if (cc == 70) { // DRUMS
        return "1";
    } else if (cc == 71) { // CLAPS
        return "0";
    } else if (cc == 72) { // HATS
        return "0";
    } else if (cc == 73) { // LEADS
        return "0";
    } else if (cc == 74) { // CHORDS
        return "0";
    } else if (cc == 75) { // CHOIR
        return "1";
    } else if (cc == 76) { // KEYS
        return "0";
    } else if (cc == 77) { // EEPS
        return "0";
    } else {
        return "0";
    }
}

let akai = mock_akai; //await midin('MPK mini 3');

//SWITCHBOARD
let djf_value = akai(1, 0).range(.25, .75);
$DJF1: "0".gain(0).orbit(1).djf(djf_value);
$DJF2: "0".gain(0).orbit(2).djf(djf_value);

///////////// DRUMS
let drum_beat = "dungeon_perc:0".beat("0,8", 16);
let drum_gain = akai(70, 0);
$DRUM: drum_beat
    .s()
    .dec(1)
    .lpf(100)
    .orbit(1)
    .gain(drum_gain);

//////////////// CLAPS
let clap_beat = "tr909_cp:4"
    .sometimesBy(akai(81), x => x.mask())
    .beat("0,2,4", 32)
    .slow(2)
    .rib(0, 8);
let clap_gain = akai(71, 0);

$CLAPS: clap_beat
    .s()
    .att(.05)
    .gain(clap_gain)
    .hpf(500)
    ._punchcard();

//////////////// HATS
let hat_beat = "hh!8"
    .n(irand(4).seg(8))
    .degradeBy(akai(82))
    .rib(0, 4);
let hat_gain = akai(72, 0);

$HATS: hat_beat
    .s()
    .bank("tr909")
    .att(.1)
    .dec(2)
    .hpf(500)
    .gain(hat_gain)
    ._punchcard();

/////////////// LEAD
let lead_beat =
    "c3 a3 b3 [a3 d3] g3 b3 [a3 b3] a3"
        .note()
        .slow(2)
        .degradeBy(.1)
        .rib(0, 8);

let lead_gain = akai(73, 0);

$LEAD: lead_beat
    .s("dungeon_lead:0")
    .att(.7)
    .dec(1)
    .rel(.3)
    .hpf("500".add(akai(83).mul(1000)))
    .duckorbit(2)
    .gain(lead_gain)
    ._punchcard();

/////////////// CHORDS
let chord_seq = chooseCycles(
    "<c3,b3,e3,c2>",
    "<a3,g3,f3,c4>",
    "<b3,e3,g3>",
    "<b3,e3,g3,c2>")
    .add("0")
    .rib(92, 16);
let chord_gain = akai(74, 0);

$CHORDS: chord_seq
    .note()
    .s(sine)
    .att(.15)
    .dec(.5)
    .rel(.4)
    .sus(.4)
    .gain(chord_gain)
    .hpf(300)
    .lpf(500)
    .orbit(2)
    ._scope();

//////////////// CHOIR

let choir_seq = akai(-1, 0, 'notes', true);
let choir_gain = akai(75, 0);

$CHOIR: choir_seq
    .note()
    .s("gm_pad_choir:5")
    .cpm(10)
    .att(2)
    .dec(.8)
    .rel(.5)
    .sus(.4)
    .gain(choir_gain)
    .room(1)
    .phaser(4)
    .hpf(500)
    ._pianoroll();

$CLEAR: "~".when(akai(20, 9).gt(0), x => akai(-1, 0, 'clear'));

////////////////// KEYS
let keys_beat = "dungeon_keys:7"
    .struct("[x ~ ~ ~ x ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ x]")
    // .struct("[x!16]")
    .delay(.1)
    .delayfb(.1)
    .room(1)
    .size(4)
    .rib(40, 4);

let keys_gain = akai(76);

KEYS: keys_beat
    .s()
    .note("c3".add.squeeze(saw.range(0, 8).seg(16)))
    .hpf(1000)
    .att(.025)
    .gain(keys_gain)
    .delay(.1)
    .delayfeedback(.2)
    ._punchcard();

////// EEPS
let eep_beat = "a4 c5 b4 g4 e4 a4 c5 d5"
    .degradeBy(.2)
    .rib(0, 4);

let eep_gain = akai(77)
$EEPS: eep_beat
    .slow(2)
    .s("gm_pad_new_age")
    .n("4,3") //3,4,5 yeah
    .note()
    .gain(eep_gain)
    ._pianoroll();