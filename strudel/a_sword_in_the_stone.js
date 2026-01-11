setCpm(155 / 4);

let akai = await midin('MPK mini 3');

/////////////////////// BASS DRUM
let bd_beat = beat("0.1,8.1", 16);
let bd_gain = akai(70);
$: "dungeon_perc:3"
  .s()
  .apply(bd_beat)
  .gain(bd_gain)
  ._punchcard();

$: "0"
  .apply(bd_beat)
  .when(bd_gain.gt(.1), x => x.add.squeeze("1 0"))
  .ccv()
  .ccn("0")
  .midi('IAC Driver Bus 1');

/////////////////////////// TOMS
let tom_degrade = "1".sub(akai(71));
let tom_beat2 = x => {
  return x
    .beat("0,2,4,6,8,10,12,14", 16)
    .degradeBy(tom_degrade)
    .rib(40, 2);
};

$: tom_beat2(
  s("dungeon_perc")
    .n(irand(3)
      .add(7)
      .seg(4)))
  .gain(1.5)
  ._punchcard();

$: tom_beat2("0!4")
  .add.squeeze(".6 0")
  .ccv()
  .ccn("2")
  .midi('IAC Driver Bus 1');

$: "0"
  .when(tom_degrade.lt(.9), x => x.mask())
  .ccv()
  .ccn("2")
  .midi('IAC Driver Bus 1');


///////////////////////// STRINGS
let str_gain = akai(72);

$: s("dungeon_strings:0")
  .scrub(".7")
  .note("<b2 a2 e2 f2 b2 a2 e2 g2>"
    .sub("<[0,4,7] [0,3,6] [0,4,7] [0,3,8]>"))
  .sus(.4)
  .gain(str_gain);

$: stepcat([15 / 16, isaw.seg(8)], [1 / 16, "0!4"])
  .when(str_gain.lt(.1), x => x.mask())
  .ccv()
  .ccn("1")
  .midi('IAC Driver Bus 1');

$: "0"
  .when(str_gain.gte(.1), x => x.mask())
  .ccv()
  .ccn("1")
  .midi('IAC Driver Bus 1');

////////////////////////// PLUCKS
let pluck_degrade = "1".sub(akai(73));
let pluck_beat = x => {
  return x.
    degradeBy(pluck_degrade)
    .sometimesBy(.7, ply("2"))
    .rib(20, 8)
}
$: pluck_beat(note(
  "c1!4".add(irand(5)))
  .s("dungeon_plucked:<4,5>")
  .gain("{.8 1.2}%8")
  .clip(.6)
  .penv(pm(4, rand.mul(2).seg(32))))
  ._punchcard();

$: pluck_beat("0!4")
  .add.squeeze(
    stepcat([15 / 16, ".2"], [1 / 16, "0"])
  )
  .ccv()
  .ccn("5")
  .midi('IAC Driver Bus 1');

$: "0"
  .when(pluck_degrade.lt(.95), x => x.mask())
  .ccv()
  .ccn("5")
  .midi('IAC Driver Bus 1');

///////////////////// SHAKER
let shaker_degrade = "1".sub(akai(74));
let shaker_beat = x => {
  return x
    .degradeBy(shaker_degrade)
    .sometimesBy(slider(0.638, 0, 1), x => x.ply("2"))
    .rib(100, 4)
}

$: shaker_beat(s("casio:2!4")
  .speed("{1 -1}%4")
  .att(.1)
  .dec(.1)
  .sus(.1)
  .rel(.1)
  .clip(.25)
  .hpf(8000)
  .fit()
  .gain(1))
  ._punchcard();

$: shaker_beat("0!8")
  .add.squeeze(stepcat([15 / 16, ".2"], [1 / 16, "0"]))
  .ccv()
  .ccn("6")
  .midi('IAC Driver Bus 1');

$: "0".when(shaker_degrade.lt(.95), x => x.mask())
  .ccv()
  .ccn("6")
  .midi('IAC Driver Bus 1');

//////////////////////////////// BELLS
let bell_gain = akai(75);
let bell_beat = x => {
  return x
    .beat("0,8?", 16)
    .rib(4, 4)
}

$: bell_beat(
  s("sleighbells:0")
    .gain(bell_gain.mul(4))
)
  ._punchcard();
$: bell_beat("0")
  .when(bell_gain.lt(.1), x => x.mask())
  .add.squeeze(stepcat([15 / 16, isaw.seg(4)], [1 / 16, 0]))
  .ccv()
  .ccn("7")
  .midi('IAC Driver Bus 1');

$: "0"
  .when(bell_gain.gte(.1), x => x.mask())
  .ccv()
  .ccn("7")
  .midi('IAC Driver Bus 1');