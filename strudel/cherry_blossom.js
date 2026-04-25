
setCpm(135 / 4);

$DRONE: "0".transpose(36).note().att(.5).dec(2).sus(1).rel(2).s("supersaw").lpf(200).gain(0.7).lfo({ sync: 0.25 })._spectrum()

$HAT: "[- 1 - 1]".s("white").dec(.1).room(.3).sometimes(ply("2|4")).hpf(13000).postgain(2.5)

$LEAD: "<0!2 2 2 0 0 4 2 4 4 0 2 4>*8".add(60).add("0,8").almostNever(x => x.sub("4")).note()
    .dec(".05!3 0.5").sus(.1).rel(.1).lpf(900)
    .orbit(2)
    ._pianoroll()//.almostNever(x=>x.rev())

$WALK:
"<[0 2 4 2]*2 - [4 2 0 2]*2 - [2 4 2 4]*2 ->"
    .add(72).add("0")
    .floor().note().s("sine").hpf(900)
    .sometimes(ply(2))
    .att(.1).dec(.1).sus(.5).rel(.1).delay(.1)
    ._pianoroll()

$CHORD: "<0 2 4 2 4 2 6 2>".add("0,5,9").add(60).note().s("saw").att(.1).duck(2).duckattack(".5")
    .duckdepth(.9).hpf(900).lpf(1300).gain(0.5).lfo({ sync: "<4 2 1>" })