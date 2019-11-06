!@import w gamelib:wordgen;

!r = std:rand:split_mix64_new[];
!rand_gen = { std:rand:split_mix64_next_open01 r };

!occ = ${};
range 1 1000 1 {||
    !res = w:gen "vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv" w:set1 rand_gen;
    res { occ.(_) = occ.(_) + 1; }
};
#0std:displayln occ;
occ {
    std:displayln @;
};
