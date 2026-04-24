
setCpm(124 / 4);

$BASS: "[0 0 0 0]*2".transpose(36).note().s("tri").lpf(100).delay(.1).delayfb(.8).gain(1.2)

$ARP: "<[0!16]>".add(saw.seg(16).mul(12).floor()).note().transpose(48).s("casio:0").hpf(3000)

$LEAD: "<0@2 2 4 0@4 2 4 6 - 2 - 2 - 2 - 0 2 4 - 2@2 6@2 0 4 2@2 4 ->*8".add("0,4").note().transpose(60).s("sine").room(.9).roomsize(2).hpf(300)

$GUIT: "<0@8 2@8 0@4 2@4 4@4 6@4 2!4 4!4>*16".scale("c:major").s("gm_electric_guitar_clean")//.rib(2,0.5)

$FF: "bd:1 [bd:1,sd:1] bd:1 [bd:1,sd:1]".s().bank("tr808").hpf(100)

$CP: "[-!3 [cp]]".s().bank("tr808").sometimes(_ => "-").ply(4).hpf(3000)

$CTL: "[[1 0]- [1 0] -]".ccv().ccn(1).midichan(2).midi('IAC Driver Bus')

$CTL2: "[- [1 0] - [1 0]]".ccv().ccn(0).midichan(1).midi('IAC Driver Bus')