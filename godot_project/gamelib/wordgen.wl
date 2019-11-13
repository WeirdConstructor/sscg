#!@import sscg sscg;
!@import std std;
!@import wlambda;

!@export gen {!(input, set, gen_cb) = @;
std:str:join "" ~
    input \:next{
        !char = _;
        !elems = char set;
        (is_none elems) { return :next char; };

        !sum = $&0;
        elems { .sum = sum + _.0; };
        !sel_weight = $&(std:num:ceil ~ gen_cb[] * $*sum);
        !out = \:r { elems {!(x) = @;
            .sel_weight = sel_weight - x.0;
            (sel_weight <= 0) { return :r x.1; };
            x.1
        } }[];
        out
    } # || std:str:join ""
};
