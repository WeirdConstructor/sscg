!@import wgt wordgen_tmpl;

!word_set = ${
    who = $[$[1, "You"], $[1, "I"], $[1, "Bob"], $[1, "Alice"]],
    where = $[$[1, "to the mall"], $[1, "to the cinema"], $[4, "shopping"]],
};

range 1 10 1 {||
    std:displayln ~
        wgt:tmpl_gen word_set "{who} wants to go {where}.";
}
