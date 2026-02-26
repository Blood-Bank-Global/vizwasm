
setCpm(155 / 4);

const kick = "bd:1".s().bank("tr909");
$KICK: arrange(
    [1, kick.struct("[1 1] 1 1 0")],
    [1, kick.struct("[1 1] 1 1 [1 1]")]
).spectrum();

$SIGNAL: ccv("0 [.5 0 ] 0 [.5 0 ]").ccn(0).midi('IAC Driver Bus 1', 0);


$BOOM: s("dungeon_perc:1").struct("0 1 0 1")._pianoroll();

RIM: "rim:1".s().bank("tr909").struct("1 0 0 0");

const hats = s("hh:1").bank("tr909");
$HATS: arrange(
    [1, hats.struct("1 1 1 1")],
    [1, hats.struct("1 [1 1] 1 1")]
).sometimesBy(.5, x => x.n(4))

S$CHORDS: chord("<Am C D Em Am C D E^>")
    .voicing()
    .slow(1)
    .s("supersaw")
    .room(.3)
    .roomsize(4)
    .delay(.25)
    .delayfb(.1).o(2)
    ._scope()
$SIGNAL2: ccv(saw.range(0, 1).seg(8)).ccn(1).midi('IAC Driver Bus 1', 0);
_$SIGNAL2: ccv("0").ccn(1).midi('IAC Driver Bus 1', 0);


_$KEYS: "c3"
    .add
    .squeeze(saw.range(-3, 3).seg(8))
    .note()
    .s("dark_key")
    .att(.1)
    .dec(.45)
    .degradeBy(.25)
    .rib(0, 8)
    .lpf(1500)
    ._pianoroll()
_$KEYS2: "{cb4 d4 e4 f4 a4 b4 cb4 d4}%8"
    .add(12)
    .gain(1)
    .sometimes(x => x.rev())
    .note()
    .s("triangle")
    .att(.1)
    .dec(.45)
    .degradeBy(.25)
    .rib(0, 8)
    ._pianoroll()

//$ALLLPF: all(apply(x=>x.lpf(1000)))
// $ALLHPF: all(apply(x => x.hpf(2000)))
