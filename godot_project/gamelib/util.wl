!@import std std;
!@import wlambda;

!strip_ws = { std:re:replace_all "\\s+" {|| "" } _ };
!@export strip_ws = strip_ws;

!trim = \:r {
    std:re:match  "^\\s*(.*?)\\s*$" _ { return :r _.1; };
    _
};

!@export trim = trim;

!@export table2map_trimmed = {!(table) = @;
    !header = $&$none;
    !elements = $[];
    block :r {
        table {!(row) = @;
            (is_none header) {
                .header = row;
            } {
                !element = ${};
                range 0 (len header) 1 {!col = _;
                    !cell  = trim row.(col);
                    !title = strip_ws header.(col);
                    (title != "") { element.(title) = cell; };
                };
                std:push elements element;
            };
        };
    };

    elements
};
