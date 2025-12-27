setCps(135 / 60 / 4);

$: s("dungeon_perc:9").beat("0,8", 16).dec(.4);
$: s("hh").fast(8).bank("tr909").n(irand(4).seg(8)).rib(0, 2).att(.1).dec(2);
$: s("tr909_cp:4")
    .sometimesBy(.25, x => x.mask())
    .beat("0,2,4", 32)
    .slow(2)
    .att(.05)
    .rib(0, 8)
    ._punchcard();

$: note("c3 a3 b3 [a3 d3] g3 b3 [a3 b3] a3")
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


let iac = await midin('IAC Driver Bus 1');
$: note(iac(-1, 0, 'notes'))
    .s("dungeon_lead:2").att(.15).dec(.5).rel(.4).sus(.4).gain(.25)
    .room(1)
    .phaser(4)
    ._scope()