setCps(135 / 60 / 4);

const akai = midin("MPK mini 3")

// $: s("bd").bank("tr909").beat("0,8", 16).dec(.4);
// $: s("[hh hh]!4").bank("tr909").att(.1).dec(2);
// $: s("cp").bank("tr909").beat("0,2,4", 32).slow(2).att(.075);


$: s("dungeon_perc:9").beat("0,8", 16).dec(.4);
$: s("[hh hh]!4").bank("tr808").att(.1).dec(2);
$: s("cp").bank("tr909")
    .sometimesBy(.25, x => x.mask())
    .beat("0,2,4", 32)
    .slow(2)
    .att(.05)
    .rib(0, 8)
    ._punchcard();

$: note("c3 a3 b3 [a3 d3] g3 b3 [a3 b3] a3".add(irand(-1, 3)))
    .s("dungeon_lead:0")
    .att(.7)
    .dec(1)
    .rel(.3)
    .gain(.7).slow(2)
    .room(.8)
    .degradeBy(.15)
    .rib(0, 8)
    .duckorbit(2)
    ._punchcard();

$: note(chooseCycles(
    "<c3,b3,e3,c2>",
    "<a3,g3,f3,c4>",
    "<b3,e3,g3>",
    "<b3,e3,g3,c2>"))
    .s(sine).att(.15).dec(.5).rel(.4).sus(.4).gain(1)
    .rib(92, 16)
    .orbit(2)
    ._scope();

$: note(chooseCycles(
    "<c3,b3,e3,c2>",
    "<a3,g3,f3,c4>",
    "<b3,e3,g3>",
    "<b3,e3,g3,c2>"))
    .s("dungeon_lead:2").att(.15).dec(.5).rel(.4).sus(.4).gain(.1)
    .rib(92, 16)
    ._scope();