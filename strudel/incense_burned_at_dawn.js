if (false) {
    window.myrepl = {};
    window.myrepl.values = {
        36: 0, // bottom left
        37: 0,
        38: 0,
        39: 0,
        40: 0,
        41: 0,
        42: 0,
        43: 0,
        44: 1,
        45: 1,
        46: 1,
        47: 1,
        48: 1,
        49: 1,
        50: 1,
        51: 1 // top right
    };
} else if (true && !window.doNotUpdate) {
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

setCpm(155 / 4);

$TRANCE: tog("<C4 D4 E4>*4", 48)
    .add("2 _ 4 2 _ _ _ 4")
    .s("dungeon_pads:8")
    .struct(rand.mul(1).seg(8).round())
    .room(.4)
    .delay(.1)
    .delayfb(.2)
    .att(.1)
    .dec(.3)
    .lpf(2000)
    // .duckdepth(0.0)
    // .duck(2)
    .note()
    .rib(4, "<4@4 4@4 8@8>")
    ._pianoroll();

$MELODY: tog("<C Em@0.5 D5@0.5 C E5 D@2 D@0.5 E@0.5 E>", 49)
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
    ._scope()

$CHASE: tog("[mt lt mt ht]*2", 50).s().bank("tr909").lpf("3000 100").gain("<1 1.2 1.3 1>").ply(1)._punchcard()
$KICK: tog("bd:2!4", 51).s().bank("tr909").almostNever(ply(4))._punchcard()
$VIDEO: ccv(tog("[.2 0]!4", 51)).ccn(2).midichan(5).midi('IAC Driver Bus 1');

$CLAP: tog("cp!8", 44).n(rand.seg(16).round().add(1)).s().bank('tr909')._punchcard()
$HAT: tog("[[hh*2] - [hh*4] -]", 45).s().bank("tr909")._punchcard()

$DRONE: tog("c1@4", 46).s("tri").postgain(4).lpf(50).att(.3).dec(1.0)._scope()
$VIDEO_DRONE: ccv(tog(tri.mul(0.05).seg(127), 46)).ccn(1).midichan(5).midi('IAC Driver Bus 1');
// $METRO: "rim!4".s().bank("tr909").gain(0.25)


// const filt_p = [
//   "0.15",
//   "0.5",
//   "0.8"
// ];

// $FILT: djf(pick(filt_p, picks('filt1').add(picks('filt2').mul(2)))).o("<0,1,2>").gain(0)