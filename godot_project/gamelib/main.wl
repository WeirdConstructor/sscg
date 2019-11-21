!@import wlambda;
!@import std std;
!@import sscg sscg;

!STATE = ${
    ship = ${
        pos       = $[5000, 2000],
        system_id = 0,
        docked    = $f,
    },
    entity_types = ${
        station    = ${ visual = "station" },
        asteroid_1 = ${ visual = "asteroid_1" },
    },
    systems = $[
        ${
            name = "Testaris 1",
            entities = $[
                ${ t = "station",    pos = $[2000, 2000] },
                ${ t = "station",    pos = $[ 500, 2000] },
                ${ t = "station",    pos = $[9000, 4000] },
                ${ t = "station",    pos = $[4000, 4000] },
                ${ t = "asteroid_1", pos = $[ 300,  300] },
            ],
        }
    ],
};

!@export STATE STATE;

!@export on_arrived {!(too_fast, sys_id, ent_id) = @;
    std:displayln "ARRIVED!";
    STATE.ship.docked = $t;
    sscg:win.set_window 1 ${
        x = 250, y = 250, w = 500, h = 500,
        title = std:str:cat["Station ", sys_id, ":", ent_id],
        title_color = "FF0",
        child = ${
            t = :l_button,
            fg = "000",
            bg = "0F0",
            text = "Arrived!",
            w = 1000,
        }
    } {||
        STATE.ship.docked = $f;
        sscg:win.set_window 1;
    };
};

!@export on_tick {
    std:displayln "TICK";
};

!@export init {
#    sscg:win.set_window 0 ${
#        x = 100, y = 200, w = 250, h = 250,
#        title = "Status",
#        title_color = "F00",
#        child = ${
#            t = :vbox,
#            w = 1000,
#            childs = $[
#                ${ t = :l_text,   fg = "000", bg = "0F0", text = "Test 123" },
#                ${ t = :l_button, ref = "xx", fg = "000", bg = "0F0", text = "Test 123" },
#            ],
#        }
#    } {
#        std:displayln "FOO" @;
#    };
    std:displayln "DISPLAY INIT";
    STATE
};
