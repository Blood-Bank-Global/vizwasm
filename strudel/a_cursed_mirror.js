setCpm(72 / 4);
const picks = (x) => {
    switch (x) {
        case 'choir':
            return 1;
        case 'choir2':
            return 1;
        case 'tubular':
            return 1;
        case 'jingle':
            return 1;
        case 'pad':
            return 1;
        case 'chord':
            return 1;
        case 'filt':
            return 1;
        case 'metro':
            return 0;
        default:
            return 0;
    }
};


const choir_p = ["-", "d4"];
$CHANT: s("gm_choir_aahs:7")
    .note(pick(choir_p, picks('choir')))
    .att(1.0)
    .dec(.5)
    .sus(0.9)
    .rel(1.0)
    .room(.4)
    .delay(.2)
    .postgain(.8)
    ._scope();

const choir2_p = ["-", "c2"];
$CHANT2:
s("gm_fx_goblins:7")
    .note(pick(choir2_p, picks('choir2')))
    .att(1.1)
    .sus(0.9)
    .rel(1.0)
    .room(.4)
    .delay(.2)
    .postgain(.8)
    ._scope();

const pad_p = ["-", "<c4 d4 e4 c#4 d4 e4 f4 d4 e#4 g4 d4 e4>"]
$PAD: s("gm_pad_new_age:0")
    .note(pick(pad_p, picks('pad')))
    .att(.1)
    .room(.2)
    .penv(.8)
    ._punchcard()

const bell_p = ["-", "<1 0!7>"];
$BELL: s("gm_tubular_bells:1").struct(pick(bell_p, picks('tubular'))).room(.9).sus(1.0).rel(2)._punchcard()
const jingle_p = ["-", "<0 1 0 0>"];
$JINGLE: s("sleighbells:3")
    .struct(pick(jingle_p, picks('jingle')))
    .room(.8)
    .roomsize(4)
    .delay(.2)
    .postgain(2.5)
    ._punchcard()

const chord_p = [
    "-",
    "<A^7 G5 D5 F^7 E^ F^7 A B^7>"
];

$CHORD: chord(pick(chord_p, picks('chord')))
    .voicing()
    .att(1)
    .dec(8)
    .s("pulse")
    .slow(2)
    .room(.75)
    .roomsize(1)
    .delay(.5)
    .lpf(1500)
    .postgain(.5)
    .spectrum()


const metro_p = [
    "-",
    "rim!4"
];

$METRO: pick(metro_p, picks('metro')).s().bank("tr909").gain(0.25)


const filt_p = [
    "0.15",
    "0.5",
    "1.0"
];

$FILT: djf(pick(filt_p, picks('filt'))).o("<0,1,2>").gain(0)