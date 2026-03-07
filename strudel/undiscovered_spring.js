if (true) {
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
        46: 0,
        47: 1,
        48: 1,
        49: 1,
        50: 1,
        51: 1 // top right
    };
}

$FILT: djf(".5").orbit("1,2").gain(0);

setCpm(85 / 4);
$METRO: tog("rim".struct("[1 1 1 1]"), 39).s().bank("tr909").postgain(.4)

$BEAT: s(tog("[mt lt mt:1 ht ht mt:1 mt lt]", 44)).n("0").bank("spacedrum").lpf(200)._scope()
$SHAKE: s(tog("sh!16", 45)).n(rand.range(0, 3).seg(16).floor()).bank("yamaharm50").hpf(10000)._scope()
$MELODY: tog("<G F D@2 C@2 G F A>", 48).chord().voicing().s("gm_pad_halo").hpf(500)._punchcard().o(2)
$PLUCK: tog("<C D E C D C D E F G>", 49).note().s("bard_plucked_string").hpf(2000)._punchcard()

$KEY: tog("[C4 D4 C4 F4]", 50)
    .note()
    .s("dungeon_keys:1")
    .hpf(500).lpf(2000).degradeBy(.25).rib("<0@12 2@12 4@12>", "<2@2 2@2 4@4 4@4>")
    .att(.2)
    .room(.5).roomsize(2).delay(.2).delayfb(.1).duck(2).duckdepth(.9).duckatt(.25)
    ._punchcard()

$WHISTLE: s(tog("<hftu_whistle@2 - - ->", 51)).room(.8).roomsize(8).delay(.25).att(.01).dec(1).sus(1).rel(2)._scope()
$HISS: s(tog("dark_hiss", 47)).clip(.999).postgain(2).lpf(4000)
    ._scope()