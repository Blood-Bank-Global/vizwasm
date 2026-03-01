if (!window.doNotUpdate) {
    window.doNotUpdate = true;
    window.myrepl = {}
    window.myrepl.mapping = {
        36: 'filt0',
        37: 'filt1',
        38: 'filt2',
        39: 'metro',
        //skip 40,41,42,43
        44: 'pad',
        45: 'chord',
        46: 'drum',
        //skip 47
        48: 'choir',
        49: 'choir2',
        50: 'tubular',
        51: 'jingle',
    };
    window.myrepl.values = {};
    for (let cc in window.myrepl.mapping) {
        window.myrepl.values[window.myrepl.mapping[cc]] = 0;
    }
    window.myrepl.values['filt1'] = 1;

    function onEnabled() {
        let spectra_out = WebMidi.getOutputByName('Midi Fighter Spectra');
        let spectra = WebMidi.getInputByName('Midi Fighter Spectra');
        spectra.channels[4].removeListener()
        spectra.channels[4].addListener('midimessage', e => {
            const data = e.data;
            if (data[0] != 179 || data[2] != 127) {
                return;
            }
            let mapping = window.myrepl.mapping[data[1]];
            if (mapping != undefined) {

                if (mapping == 'filt0' || mapping == 'filt1' || mapping == 'filt2') {
                    window.myrepl.values['filt0'] = window.myrepl.values['filt1'] = window.myrepl.values['filt2'] = 0;
                    window.myrepl.values[mapping] = 1;
                } else {
                    window.myrepl.values[mapping] = window.myrepl.values[mapping] > 0 ? 0 : 1;
                }
                for (let mapping in window.myrepl.mapping) {
                    if (window.myrepl.values[window.myrepl.mapping[mapping]] >= 1.0) {
                        spectra_out.channels[3].sendNoteOn(mapping, 127)
                    } else {
                        spectra_out.channels[3].sendNoteOff(mapping, 0)
                    }
                }
            }
        });

        for (let mapping in window.myrepl.mapping) {
            if (window.myrepl.values[window.myrepl.mapping[mapping]] >= 1.0) {
                spectra_out.channels[3].sendNoteOn(mapping, 127)
            } else {
                spectra_out.channels[3].sendNoteOff(mapping, 0)
            }
        }
    };

    WebMidi
        .enable()
        .then(onEnabled)
        .catch(err => alert(err));
}

const picks = (x) => {
    let v = window.myrepl.values[x];
    if (v == undefined) {
        return 0;
    }
    return ref(() => window.myrepl.values[x]);
};

setCpm(72 / 4);

const choir_p = ["-", "d4"];
$CHANT: s("gm_choir_aahs:7")
    .note(pick(choir_p, picks('choir')))
    .att(1.0)
    .dec(.5)
    .sus(0.9)
    .rel(1.0)
    .room(.4)
    .delay(.2)
    .postgain(1.0)
    ._scope();

const choir2_p = ["-", "d2"];
$CHANT2:
s("gm_fx_goblins:7")
    .note(pick(choir2_p, picks('choir2')))
    .att(1.1)
    .sus(0.9)
    .rel(1.0)
    .room(.4)
    .delay(.2)
    .postgain(2.0)
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
const drum_p = ["-", "<[1 1] [1 1] [1 1] [1 1 1 1]>"];
$DRUM: s("gm_taiko_drum:2").struct(pick(drum_p, picks('drum'))).hpf(75).postgain(2);

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

$FILT: djf(pick(filt_p, picks('filt1').add(picks('filt2').mul(2)))).o("<0,1,2>").gain(0)