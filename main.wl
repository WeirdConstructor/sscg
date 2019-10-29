!@import clock gamelib:clock;
!@import sscg sscg;

!SHIP_PANEL_ID   = 0;
!STATUS_PANEL_ID = 1;
!status_panel    = ${ };
!x               = $&0;

!g_ship = $&&$n;


!info_label = { !(lbl, ref) = @;
    ${ t = "hbox", w = 1000, childs = $[
        ${ t = "r_button", text = lbl, fg = "FFF", bg = "000", w = 300 },
        ${ t = "l_label",  ref = ref,  fg = "FFF", bg = "F00", w = 700 },
    ]}
};


${

init = {!(ship) = @;
    std:displayln "INIT GAME";
    !sys = sscg:game :add_system 0 0 ${};
    sscg:game :add_entity sys 20  20  ${ type = :station };
    sscg:game :add_entity sys 300 300 ${ type = :station };
    sscg:game :add_entity sys 200 100 ${ type = :asteroid_field };
    ship :set_system sys;

    sscg:win :set_window SHIP_PANEL_ID ${
        title       = "Test Window",
        title_color = "e8e",
        x           = -481,
        y           = 0,
        w           = 1000,
        h           = 1000,
        child       = ${
            t       = "vbox",
            w       = 1000,
            spacing = 3,
            childs  = $[
                ${
                    t    = "l_button",
                    ref  = "REF1",
                    text = "10",
                    fg   = "F0F",
                    bg   = "303",
                },
                info_label "Status:"   "SHIP_STATE",
                info_label "Cargo:"    "SHIP_CARGO_COUNT",
                info_label "Credits:"  "SHIP_CREDITS",
            ],
        },
    } {|| std:displayln "FOO" @ };

    sscg:win :set_window STATUS_PANEL_ID ${
        title       = "Status",
        title_color = "ee8",
        x           = 0,
        y           = -481,
        w           = -481,
        h           = 1000,
        child       = ${
            t       = "vbox",
            w       = 1000,
            spacing = 3,
            childs  = $[
                info_label "Time:" "STATUS_TIME",
            ],
        },
    } {|| std:displayln "MO" @ };
},

game_load = {||
    std:displayln "GAME LLLLLLLLLLLLLLLLLLLLOOOOOOAAAAAD";
    !ship = (sscg:game :list_by_type :ship).0;
    (is_none ship.cargo) {
        ship.cargo        = $[];
        ship.max_capacity = 10;
        ship.credits      = 0;
    };
    .*g_ship = ship;
},
game_tick = {||
    clock:tick[];
    sscg:win :set_label STATUS_PANEL_ID "STATUS_TIME" clock:now_str[];
},
ship_tick = {
    !(ship, system, entity) = _;
    sscg:win :set_label SHIP_PANEL_ID "SHIP_STATE" ship._state;

    match entity.typ
        "asteroid_field" {||
            ((len ship.cargo) < ship.max_capacity) {
                std:push ship.cargo "rock";
            };
        }
        "station" {||
            while { (len ship.cargo) > 0 } {
                match (std:pop ship.cargo)
                    "rock" {||
                        ship.credits = ship.credits + 1;
                    };
            };
        };

    sscg:win :set_label SHIP_PANEL_ID "SHIP_CARGO_COUNT" (len ship.cargo);
    sscg:win :set_label SHIP_PANEL_ID "SHIP_CREDITS"     ship.credits;
},
system_tick = {||
},

}
