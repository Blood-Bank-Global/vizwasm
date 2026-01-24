setCpm(135 / 4);

const wb = s("wonderboy3:11");
const p1_0 = ".50 .55 .50 .50 .55 .50  .55 - ";
const p1_1 = ".55 .50 .55 .55 .50 .545 .55 -";
const p1_2 = ".50 [.55!2] .50 [.50!2] .55 .50  .55 - ";
const p1_3 = ".50 .55 - .50 .55 - .50  .55";
const p1_4 = ".55 .50 .50 - .55 .545 .50 - ";
const p1_5 = "[.50!2] [.55!2] .50@2 [.50!2] .55 .50  - ";

$: arrange(
    [0, wb.scrub(p1_0)],
    [0, wb.scrub(p1_1)],
    [0, wb.scrub(p1_3)],
    [0, wb.scrub(p1_4)],
    [0, wb.scrub(p1_5)],
    [0, wb.scrub(p1_1)],
)
    .lpf(2400)
    .hpf(2200)
    .penv(.2)
    .gain(1.5)
    .delay(.4)
    .delayfb(.4)
    .room(.5)
    .roomsize(2)
    ._scope();


const p2_0 = ".2 .2 .25 .2 .25 .25 -@2";
const p2_1 = ".275 .2 .25 .275 .2 .25 .2 -";
const p2_2 = ".275 [.2!2] .25 .275 - .25 .2 -";

_$: arrange(
    [2, wb.scrub(p2_0)],
    [1, wb.scrub(p2_1)],
    [2, wb.scrub(p2_0)],
    [1, wb.scrub(p2_2)],
)
    .lpf(900)
    .hpf(100)
    .penv(.2)
    .gain(2)
    .delay(.5)
    .delayfb(.4)
    .room(1)
    .roomsize(3)
    ._scope()

_$: arrange(
    [0, "bd".struct("[1 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0]")],
    [0, "bd".struct("[1 0 0 0 1 0 0 0 1 0 0 1 1 0 0 0]")],
    [1, "bd".struct("[1 1 1 0 1 0 0 0 1 0 0 0 1 0 0 0]")]
)
    .s()
    .bank("tr909")
    .lpf(200)

_$: arrange(
    [1, "cp".struct("[0 0 1 1 0 0 0 0 0 0 0 0 1 1 1 0]")],
    [0, "-"]
)
    .s()
    .bank("tr909")

