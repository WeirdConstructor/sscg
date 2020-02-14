!txt = std:io:file:read_text "input.txt";

!words = ${};
!last_word = $&$none;
std:re:map "([a-zA-Z]+|[.,])" \:next {
    !word = _.1;
#    std:displayln _;
    ((len word) < 2 &and word != "." &and word != ",") {
        return :next $n;
    };
    (is_none last_word) {
        .last_word = word;
        return :next $n;
    };
    !lc_last_word = std:str:to_lowercase last_word;
    (is_none words.(lc_last_word)) {
        words.(lc_last_word) = $[word];
    } {
        std:push words.(lc_last_word) word;
    };
    .last_word = word;
} txt;

!r = std:rand:split_mix64_new[]; #_from 23023;

!word_vec = $[];
words {
    std:push word_vec _1;
};

#!idx = (std:rand:split_mix64_next r) % ;
#!first = idx % (len word_vec);

range 1 10 1 {!(i) = @;
    !out = $&$[];
    !vec = word_vec;

    !word_min_len =
        5 + std:num:abs[std:rand:split_mix64_next r] % 4;
    block :out {
        while { $t } \:retry {||
            !idx = std:num:abs[std:rand:split_mix64_next r] % (len vec);
            !word = vec.(idx);
#            std:displayln word;
            std:push out word;
            .vec = words.(word);
            ((len vec) == 0) { .vec = word_vec };
            (word == "." &or word == ",") {
                ((len out) < word_min_len) {
                    .out = $[];
                    return :retry $none;
                } {
#                    std:displayln "LEN:" (len out);
                    return :out out;
                };
            };
        };
    };

    !s = std:str:join " " $*out | std:re:replace_all "\\s+([.,])" { _.1 };
    std:displayln "(" i ")" s;
}
