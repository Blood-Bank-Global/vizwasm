setCpm(145 / 4);

$CEES: "c1!8"
  .add(perlin.range(0, 5).floor().seg(8))
  .note()
  .s("dark_key:7")
  .hpf(1300)
  .dec(.5)
  .degradeBy(.4)
  .room(2)
  .roomsize(6)
  .rib(0, 8)
  ._pianoroll();

$DEES: "d2 - d2 -"
  .note()
  .s("square")
  .dec(.7)
  .att(.2)
  .sus(.25)
  .rel(.1)
  .lpf("700|400|150")
  .rib(0, 8);

const rim_pat = "rim".s().bank("rolandtr808");

$RIM: arrange(
  [1, rim_pat.struct("1 0 1 [1 1]")],
  [1, rim_pat.struct("1 1 1 1")],
  [6, rim_pat.struct("1 0 0 0")],
);

$KICK: "bd".s().bank("rolandtr808").n("1|8|9")
  .lpf(800)
  .fast(4).gain(2).rib(0, 12)

const pad_pat = "c2!16"
  .s("dungeon_keys:7");

$PAD: arrange(
  [3, pad_pat.struct("[1 0 0 0 1 0 1? 0 1? 0 0 0 0 0 0 0]")],
  [1, pad_pat.struct("[1 0 0 0 [1 1] 0 1 0 [1 1] 0 0 1 0 0 0 0]")],
)
  .dec(.4)
  .att(.1)
  .lpf(800)
  .rib(0, 8)
  ._pianoroll();


