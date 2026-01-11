////// 909
$KICK: "bd".beat("0,4,8,12,14", 16)
    .bank("tr909")
    .s()
    .att(.01)
    .gain(slider(1, 0, 1))
    .dec(.3)
    .bpf("300:1");

////////////////// MORE KEYS
let more_keys_beat = "dark_key"
    .n("3")
    .struct("[x!8]/2")
    .degradeBy(.7)
    .rib(40, 8);

let more_keys_gain = get_cc(CC_MAP.MORE_KEYS, 0);


$MORE_KEYS: more_keys_beat
    .s()
    .note("c3".add.squeeze(saw.range(0, 8).seg(16)))
    .gain(more_keys_gain)
    ._punchcard();

////// EEPS
let eep_beat = "a4 c5 b4 g4 e4 a4 c5 d5"
    .degradeBy(.2)
    .rib(0, 4);

let eep_gain = get_cc(CC_MAP.EEPS, 0);
$EEPS: eep_beat
    .slow(2)
    .s("dark_pad") //gm_pad_new_age
    .n("2") //2,3,4,5 yeah
    .note()
    .dec(1)
    .gain(eep_gain)
    ._pianoroll();

//////////////// CHOIR

let choir_seq = get_cc(-1, 0, 'notes', true);
let choir_gain = get_cc(CC_MAP.CHOIR, 0);

$CHOIR: choir_seq
    .note()
    .s("dark_organ") //dark_organ:5 | gm_pad_choir:5
    .n("5")
    .cpm(10)
    .att(2)
    .dec(.8)
    .rel(.5)
    .sus(.4)
    .gain(choir_gain)
    .room(1)
    .phaser(4)
    .hpf(500)
    ._pianoroll();

$CLEAR: "~".when(get_cc(20, 9).gt(0), x => get_cc(-1, 0, 'clear'));
