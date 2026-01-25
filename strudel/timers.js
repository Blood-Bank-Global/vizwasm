setCpm(160 / 2)
$ZERO:
ccv(fastcat("0!23", tri.range(0, 0.5).seg(8), "0!23"))
    .ccn("0")
    .midi('IAC Driver Bus 1');

$ONE: ccv(saw.seg(16))
    .ccn("1")
    .midi('IAC Driver Bus 1');

$TWO: ccv(saw.seg(16))
    .ccn("2")
    .midi('IAC Driver Bus 1');

$THREE: ccv(saw.seg(16))
    .ccn("3")
    .midi('IAC Driver Bus 1');

$FOUR: ccv(saw.seg(16))
    .ccn("4")
    .midi('IAC Driver Bus 1');