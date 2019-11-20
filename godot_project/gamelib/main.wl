!@import wlambda;
!@import std std;
!@import sscg sscg;

!:global STATE = ${
    ship = ${
        pos       = $[5000, 2000],
        system_id = 0,
    },
    entity_types = ${
        station = ${ visual = "station" },
    },
    systems = $[
        ${
            name = "Testaris 1",
            entities = $[
                ${ t = "station", pos = $[2000, 2000] },
                ${ t = "station", pos = $[500,  2000] },
                ${ t = "station", pos = $[9000, 4000] },
                ${ t = "station", pos = $[4000, 4000] },
            ],
        }
    ],
};

!@export init {
    sscg:win.set_window 0 ${
        x = 100, y = 200, w = 250, h = 250,
        title = "Status",
        title_color = "F00",
        child = ${
            t = :vbox,
            w = 1000,
            childs = $[
                ${ t = :l_text,   fg = "000", bg = "0F0", text = "Test 123" },
                ${ t = :l_button, ref = "xx", fg = "000", bg = "0F0", text = "Test 123" },
            ],
        }
    } {
        std:displayln "FOO" @;
    };
    std:displayln "DISPLAY INIT";
    STATE
};
