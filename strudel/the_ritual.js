
setCpm(165 / 4);
// setCpm(100/4);

const kick = "bd:<0>".s().bank("tr909");
$KICK: arrange(
  [7, kick.struct("1 1 1 1")],
  [1, kick.struct("1 1 [1 1] [1 1]")],
  [7, kick.struct("1 1 1 1")],
  [1, kick.struct("[1 1] 1 1 [1 1]")],
)
  .lpf(1000)
  .hpf(200)
  ._scope();

$RIM: arrange(
  [4, "<[rim:1 - rim -] <[rim rim:1 rim -]> <-> <[rim - - -]>>"],
  [4, "-"]
)
  .s()
  .bank("tr909")
  .room(.5)
  .roomsize(1);

$BELL: s("bard_hand_cymbal").n("<4 <1,0> 1 <1,0> 4 1 1 <1,0>>")


$KEYS: arrange(
  [3, "0 1 2 3"],
  [1, "3 2 1 0"]
)
  .scale("C:Minor")
  .transpose(-12)
  .note()
  .gain(.8)
  .s("dark_key")
  .hpf(1000)
  .duckorbit(2)
  .duckdepth(0)
  .room(.3)
  .roomsize(5)
  ._punchcard()


$PROG: chord("<Cm Dm Am Em Cm Dm Em E Cm Dm Am E C5 D5 E7@2>")
  .voicing()
  .transpose(8)
  .s("dark_organ:4") //organ 2
  .lpf(1000)
  .hpf(200)
  .att(.9)
  .dec(1.5)
  .o(2)
  .postgain(.8);

$DRONE: "c1@2"
  .note()
  .s("saw")
  .lpf(200)
  .postgain(1.5)
  .spectrum()


$SHUFFLE: "casio:2!8"
  .s()
  .att(.2)
  .dec(.9)
  .sometimesBy(.2, x => x.ply(2))
  .rib(0, 8)
  .postgain(0.5)
  ._scope()

$FILT: djf(0.5).o("0,1,2")
