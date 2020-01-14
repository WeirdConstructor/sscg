!table =
    std:deser:csv ";" "\r\n"
        ~ std:io:file:read_text
            "elements_adjusted.csv";
!trim = \:r { std:re:match  "^\\s*(.*?)\\s*$" _ { return :r _.1; }; _ };

std:displayln "Table elements: " ~ len table.0;

!header = $&$none;
!elements = $[];
block :r {
    table {!(row) = @;
        (is_none header) {
            .header = row;
        } {
            !element = ${};
            range 0 (len header) 1 {!col = _;
                !cell = trim row.(col);
                !title = trim header.(col);
#                std:displayln "CELL:" title ":::" row.(col) "=>" cell;
                element.(title) = cell;
            };
            std:push elements element;
#            return :r $n;
        };
    };
};

!occurence_weight          = { 0.9 ^ _.gScore };
!gen_compound_count_weight = { (1.0 - (float[_] / 20.0))^float[_] };

!compound_counts = $[];
range 1 10 1 {
    std:push compound_counts $[_, gen_compound_count_weight _];
};
std:displayln compound_counts;

!get_weighted = \:r {
#        elems { .sum = sum + _.0; };
#        !sel_weight = $&(std:num:ceil ~ gen_cb[] * $*sum);
#        !out = \:r { elems {!(x) = @;
#            .sel_weight = sel_weight - x.0;
#            (sel_weight <= 0) { return :r x.1; };
#            x.1
#        } }[];
};

#elements {!el = _;
#    std:displayln el.symbol el.gScore occurence_weight[el];
#};

#std:displayln elements.10;
