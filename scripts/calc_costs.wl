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
                std:displayln "CELL:" title ":::" row.(col) "=>" cell;
                element.(title) = cell;
            };
            std:push elements element;
#            return :r $n;
        };
    };
};

std:displayln elements.10;
