!@import sscg sscg;
!@import wlambda;
!@import std std;
!@import c   colors;
!@import gui gui_common;
!@import WID        gui_window_ids;

!refuel_text = {!(STATE) = @;

    !fuel_delta =
        STATE.ship_types.(STATE.ship.t).fuel_capacity
        - STATE.ship.fuel;
    !cc_per_fuelunit = 150;
    !price = (float ~ fuel_delta * cc_per_fuelunit) / 100.0 | std:num:ceil;
    gui:ml_l_vtext 500 1000 c:SE1_L
        $[
            std:str:cat cc_per_fuelunit "cc/Unit",
            std:str:cat fuel_delta " fuel units",
            std:str:cat "= " price " credits",
        ]
};

!show = $&&$n;
.*show = {!(STATE, ent, ent_type) = @;

    sscg:win.set_window WID:STATION ${
        x = 250, y = 250, w = 500, h = 500,
        title = std:str:cat["Station ", ent],
        title_color = c:PRI_L,
        child = ${
            t = :vbox,
            w = 1000,
            h = 1000,
            childs = $[
                ${ t = "hbox", border = 1, border_color = c:SE1_D2,
                   w = 1000,
                   h = 200,
                   spacing = 10,
                   childs = $[
                        ${ t = :l_button, text = "Refuel", ref = :refuel,
                           w = 500, h = 1000, fg = "000", bg = c:SE1 },
                        refuel_text[STATE]
                   ]
                },
                ${
                    t    = :l_button,
                    fg   = "000",
                    bg   = c:SE1,
                    text = "Arrived!",
                    w    = 1000,
                }
            ]
        }
    } {||
        STATE.ship.docked = $f;
        match _1
            "refuel" {||
                STATE.ship.fuel =
                    STATE.ship_types.(STATE.ship.t).fuel_capacity;
                show[ent, ent_type];
            }
            {|| sscg:win.set_window WID:STATION; };
    };
};

!@export show show;
