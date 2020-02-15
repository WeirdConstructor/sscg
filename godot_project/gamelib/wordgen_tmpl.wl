!@import std std;
!@import wlambda;
!@import wg wordgen;

!sm = std:rand:split_mix64_new[];

!new_gen = {!(word_set, tmpl) = @;
    wg:gen $[tmpl] word_set {
        std:rand:split_mix64_next_open01 sm
    };
};

!@export tmpl_gen = {!(word_set, s) = @;
    (std:fold $[] {
        !out       = _1;
        !tmpl_word = _.1;
        std:push out _.0;
        (not ~ is_none tmpl_word) {
            std:push out ~ new_gen word_set tmpl_word;
        };
        _1
    } ~ std:re:map $q$([^{]*)(?:\{([^}]+)\})?$ {
        $[_.1, _.2]
    } s) | std:str:join "";
};
