!SHIP_PANEL_ID = 0;
!STATUS_PANEL_ID = 1;
!status_panel = ${ };
!x = $&0;

${

init = {!(ship) = @;
    std:displayln "INIT GAME";
    !sys = game :add_system 0 0 ${};
    game :add_entity sys 20  20  ${ type = :station };
    game :add_entity sys 300 300 ${ type = :station };
    game :add_entity sys 200 100 ${ type = :asteroid_field };
    ship :set_system sys;

    win :set_window SHIP_PANEL_ID ${
        title       = "Test Window",
        title_color = "e8e",
        x           = -481,
        y           = 0,
        w           = 1000,
        h           = 1000,
        child       = ${
            t = "vbox",
            childs = $[
                ${
                    t    = "l_button",
                    ref  = "REF1",
                    text = "10",
                    fg   = "F0F",
                    bg   = "303",
                    h    = 1,
                },
                ${ t = "hbox", childs = $[
                    ${ t = "r_button", text = "Status:",
                       fg = "FFF", bg = "000", w = 300, h = 0},
                    ${ t = "l_label", ref = "SHIP_STATE",
                       fg = "FFF", bg = "F00", w = 700, h = 0},
                ]},
            ],
        },
    } {|| std:displayln "FOO" @ };

    win :set_window STATUS_PANEL_ID ${
        title       = "Status",
        title_color = "ee8",
        x           = 0,
        y           = -481,
        w           = -481,
        h           = 1000,
        child       = ${
            t    = "l_button",
            text = "10",
            fg   = "000",
            bg   = "F0F",
            h    = 0,
        },
    } {|| std:displayln "MO" @ };
},

game_tick = {||
    std:displayln "GAME TICK";
},
ship_tick = {
    !(ship, system) = _;
    std:displayln "SHIP TICK" @;
    win :set_label SHIP_PANEL_ID "SHIP_STATE" ship._state;
},
system_tick = {
    std:displayln "SYS TICK" @;
},

}
