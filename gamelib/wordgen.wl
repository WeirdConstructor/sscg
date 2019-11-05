#!@import sscg sscg;
!@import std std;
!@import wlambda;

!@export set1 ${
    v = $[$[1, "o"], $[50, "a"], $[1, "e"]],
    c = $[$[4, "k"], $[10, "l"]],
};

!r = std:rand:split_mix64_new[];

!@export gen {!(input, set, gen_cb) = @;
std:str:join "" ~
    input {
        !elems = _ set;
        !sum = $&0;
        elems { .sum = sum + _.0; };
        !sel_weight =
            $&(std:num:ceil ~ std:rand:split_mix64_next_open01[r] * $*sum);
        !out = \:r { elems {!(x) = @;
            .sel_weight = sel_weight - x.0;
            (sel_weight <= 0) { return :r x.1; };
            x.1
        } }[];
        out
    } # || std:str:join ""
};
