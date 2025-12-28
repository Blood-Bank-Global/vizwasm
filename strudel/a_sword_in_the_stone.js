setCpm(135 / 4);

$: stack(
  s("dungeon_perc:3").gain(slider(0.3795, 0, 1.5)),
  ccv("1,0").ccn("0").midi('IAC Driver Bus 1'))
  .beat("0.1,8.1", 16)
  ._punchcard();

$: s("dungeon_perc")
  .n(irand(3)
    .add(5)
    .seg(4)
    .degradeBy(slider(0.792, 0, 1))
    .rib(40, 2))
  .beat("0,2,4,6,8,10,12,14", 16)
  .gain(.9)
  ._punchcard();

$: s("dungeon_strings:0")
  .scrub(".7")
  .note("<b2 a2 e2 f2 b2 a2 e2 g2>"
    .sub("<[0,4,7] [0,3,6] [0,4,7] [0,3,8]>"))
  .sus(.4)
  .gain(slider(0.591, 0, 1.5));

$: note(
  "g1!4".add(rand.mul(5).floor()))
  .s("dungeon_plucked:6")
  .penv(pm(4, rand.mul(4).floor().seg(32)))
  .att(0.6)
  .dec(0.4)
  .rel(0)
  .gain(.7)
  .degradeBy(.2)
  .sometimes(ply(2))
  .room(1)
  .rib(20, 8)
  ._punchcard()

$: s("casio:2!4")
  .speed("{1 -1}%4")
  .degradeBy(slider(0.429, 0, 1))
  .sometimesBy(slider(0.69, 0, 1), x => x.ply("2"))
  .rib(100, 4)
  .att(.1)
  .dec(.1)
  .sus(.1)
  .rel(.1)
  .clip(.25)
  .hpf(8000)
  .fit()
  .gain(slider(0, 0, 2.5))
  ._punchcard();

$: stack(
  s("sleighbells:0"),
  // ccv("0").ccn(7).midi('IAC Driver Bus 1'),
)
  .gain(slider(1.692, 0, 4))
  .beat("0,8?", 16)
  .rib(4, 4)
  ._punchcard();