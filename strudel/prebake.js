if (useMidiFighter && !window.doNotUpdate) {
    window.doNotUpdate = true;
    window.myrepl = {}
    window.myrepl.values = {};
    function onEnabled() {
        let spectra_out = WebMidi.getOutputByName('Midi Fighter Spectra');
        let spectra = WebMidi.getInputByName('Midi Fighter Spectra');
        spectra.channels[4].removeListener()
        spectra.channels[4].addListener('midimessage', e => {
            const data = e.data;
            if (data[0] != 179 || data[2] != 127) {
                return;
            }
            let note = data[1];
            window.myrepl.values[note] = (window.myrepl.values[note] == undefined || window.myrepl.values[note] == 0) ? 1 : 0;
            for (let note = 0; note < 127; note++) {
                if (window.myrepl.values[note] > 0.0) {
                    spectra_out.channels[3].sendNoteOn(note, 127)
                } else {
                    spectra_out.channels[3].sendNoteOff(note, 0)
                }
            }
        });

        for (let note = 0; note < 127; note++) {
            if (window.myrepl.values[note] > 1.0) {
                spectra_out.channels[3].sendNoteOn(note, 127)
            } else {
                spectra_out.channels[3].sendNoteOff(note, 0)
            }
        }
    };

    WebMidi
        .enable()
        .then(onEnabled)
        .catch(err => alert(err));
}

const tog = (on, note) => {
    return ref(() => (window.myrepl.values[note] == undefined || window.myrepl.values[note] == 0) ? "-" : on);
};
