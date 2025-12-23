// let akai = await midin('MPK mini 3');

// $: note("c3").when(s("piano")
//     .note("c3")
//   .when(akai(36).eq("0"), x=>x.mask("0"))
//     .gain(2)
//   ._scope();
setCps(135 / 60 / 4);

$: stack(
  s("dungeon_perc:3").gain(slider(0.4495, 0, .5)),
  ccv(0.06).ccn("0").midi('IAC Driver Bus 1'))
  .beat("0.1,8.1", 16)
  ._punchcard();

$: s("dungeon_perc")
  .n(irand(3)
    .add(5)
    .seg(4)
    .degradeBy(slider(0.77, 0, 1))
    .rib(40, 2))
  .beat("0,2,4,6,8,10,12,14", 16)
  .gain(.5)
  ._punchcard();

$: s("dungeon_strings:0")
  .scrub(".7")
  .note("<b2 a2 e2 f2 b2 a2 e2 g2>"
    .sub("<[0,4,7] [0,3,6] [0,4,7] [0,3,8]>"))
  .sus(.4)
  .gain(slider(0.4275, 0, .5));

$: note(
  "g1!4".add(rand.mul(5)))
  .s("dungeon_plucked:6")
  .penv("0.4")
  .att(1 / 8)
  .dec(1 / 8)
  .sus(.5)
  .rel(1 / 8)
  .gain(.5)
  .degradeBy(slider(1, 0, 1))
  .sometimes(ply(2))
  .room(1)
  .rib(20, 8)
  ._punchcard()

$: s("casio:2!4")
  .degradeBy(slider(0.369, 0, 1))
  .sometimesBy(slider(0.729, 0, 1), x => x.ply("2"))
  .rib(100, 4)
  .att(.1)
  .dec(.1)
  .sus(.1)
  .rel(.1)
  .clip(.25)
  .hpf(8000)
  .speed("1 -1 1 -1")
  .fit()
  .gain(slider(2.5, 0, 2.5))
  ._punchcard();

$: s("sleighbells:0").gain(slider(1.78, 0, 4)).beat("0,8?", 16).rib(4, 4);

// $: ccn(0).ccv(0.5).midi('IAC Driver Bus 1').beat("0 4 8 12", 16);
// $: ccn(1).ccv(sine.seg(2)).midi('IAC Driver Bus 1').fast(4);
