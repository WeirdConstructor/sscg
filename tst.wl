!@import w gamelib:wordgen;

!occ = ${};
range 1 1000 1 {||
    !res = w:gen "vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv" w:set1 { };
    res { occ.(_) = occ.(_) + 1; }
};
#0std:displayln occ;
occ {
    std:displayln @;
};
