!@import sscg sscg;
!@import wlambda;
!@import std std;
!@import c   colors;
!@import gui gui_common;
!@import WID gui_window_ids;

!calc_refuel = {!(STATE) = @;
    !cc_per_fuelunit = 150;

    !fuel_delta =
        STATE.ship_types.(STATE.ship.t).fuel_capacity
        - STATE.ship.fuel;

    !pay_fuel_max =
        (100.0 * STATE.player.credits) / cc_per_fuelunit | std:num:floor;

    .fuel_delta = (pay_fuel_max < fuel_delta) { pay_fuel_max } { fuel_delta };

    !price = (float ~ fuel_delta * cc_per_fuelunit) / 100.0 | std:num:ceil;

    ${
        cc_per_fuelunit = cc_per_fuelunit,
        fuel_delta      = fuel_delta,
        price           = price,
    }
};

!refuel_text = {!(STATE) = @;
    !refuel = calc_refuel STATE;

    gui:ml_l_vtext 500 1000 c:SE1_L $[
        std:str:cat refuel.cc_per_fuelunit "cc/Unit",
        std:str:cat refuel.fuel_delta " fuel units",
        std:str:cat "= " refuel.price " credits",
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
        match _1
            "refuel" {||
                !refuel = calc_refuel STATE;
                STATE.ship.fuel = STATE.ship.fuel + refuel.fuel_delta;
                STATE.player.credits = STATE.player.credits - refuel.price;
                show[STATE, ent, ent_type];
            }
            {||
                sscg:win.set_window WID:STATION;
                STATE.ship.docked = $f;
            };
    };
};

!@export show show;
