setCpm(84 / 4);


let lead = "<Am Fm Bm Am Fm>".chord().voicing().transpose("<0@4 -4@4>").s("gm_electric_guitar_clean:2").adsr(".1:.2:.5:1").
    gain(1).lfo({ s: 8, sh: "tri", dc: -0.5, dr: 1 })


let shake_types = [
    "sh:3:1.1:.2",
    "sh:3:1.0:.2",
    "sh:3:1.0:.1",
    "sh:3:0.8:0",
];

let shake = irand(shake_types.length).pick(shake_types).as("s:n:velocity:room").roomsize(4).bank("tr808")
    .fast("<8@3 16@1>").rarely(ply(2))
    ._pianoroll()


let breaks = s("more_breaks:10").splice(8, "<[2 3 4 3] [2 3 2 3] [4 7 2 3]>").lpf(1000).postgain(2)._scope()
let tam_n = ["6", "0",]
let tam = irand(tam_n.length).seg(4).pick(tam_n).as("n").s("tambourine").fast(4).gain(4).rarely(ply(2))._pianoroll();

let elems = {
    lead: lead,
    shake: shake,
    breaks: breaks,
    tam: tam
}

let music = [
    [4, "breaks,lead,shake,tam"]
].map(x => [x[0], x[1].pick(elems)]);

S$: arrange(...music)._scope()