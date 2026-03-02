if (false && !window.doNotUpdate) {
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

setCpm(155 / 4);

$TRANCE: //"<G5 E5 F5 A7 D5 G7 E5 A5>"
"<C4 D4 E4>*4"
    .add("2 _ 4 2 _ _ _ 4")
    .s("dungeon_pads:8")
    .struct(rand.mul(1).seg(8).round())
    .room(.4)
    .delay(.1)
    .delayfb(.2)
    .att(.1)
    .dec(.3)
    .lpf(2000)
    .duckdepth(0.15)
    .duck(2)
    .note()
    .rib(4, "<4@4 4@4 8@8>")
    ._pianoroll();

$MELODY: "<C Em@0.5 D5@0.5 C E5 D@2 D@0.5 E@0.5 E>"
    .chord()
    .voicing()
    .s("supersaw")
    .lpf(9000)
    .dec(1.5)
    .rel(1)
    .sus(1)
    .o(2)
    .room(.8)
    .delay(.1)

$: "[mt lt mt ht]*2".s().bank("tr909").lpf("3000 100").gain("<1 1.2 1.3 1>")
$: "bd:2!4".s().bank("tr909").almostNever(ply(4))
$: "cp!8".n(rand.seg(16).round().add(1)).s().bank('tr909')._punchcard()

$: "[[hh*2] - [hh*4] -]".s().bank("tr909")

$: "c1@4".s("tri").postgain(2).lpf(50).att(.3).dec(1.0)

// $METRO: "rim!4".s().bank("tr909").gain(0.25)


// const filt_p = [
//   "0.15",
//   "0.5",
//   "0.8"
// ];

// $FILT: djf(pick(filt_p, picks('filt1').add(picks('filt2').mul(2)))).o("<0,1,2>").gain(0)