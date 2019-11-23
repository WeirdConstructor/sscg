!@import wlambda;
!@import std    std;
!@import sscg   sscg;
!@import c      colors;

!WID_STATUS     = 0;
!WID_STATION    = 1;
!WID_OUTOFFUEL  = 2;

!STATE = ${
    ship_types = ${
        scout_mk1 = ${
            fuel_capacity   = 1000,
            fuel_per_sec    = 7,
        },
    },
    player = ${
        credits         = 1000,
    },
    ship = ${
        t               = :scout_mk1,
        pos             = $[5000, 2000],
        system_id       = 0,
        docked          = $f,
        engine_on_secs  = 0,
        fuel            = 1000,
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
                ${ t = "station",    pos = $[6500, 6500] },
                ${ t = "asteroid_1", pos = $[ 300,  300] },
            ],
        }
    ],
};

!ml_l_vtext = {!(w, h, fg, lines) = @;
    !per_line = 1000 / len[lines];
    ${ t = :vbox, w = w, h = h, childs =
        lines {
            ${ t = :l_text, text = _,
               w = 1000, h = per_line,
               fg = fg, bg = "000" }
        }
    };
};

!refuel_text = {
    !fuel_delta =
        STATE.ship_types.(STATE.ship.t).fuel_capacity
        - STATE.ship.fuel;
    !cc_per_fuelunit = 150;
    !price = (float ~ fuel_delta * cc_per_fuelunit) / 100.0 | std:num:ceil;
    ml_l_vtext 500 1000 c:SE1_L
        $[
            std:str:cat cc_per_fuelunit "cc/Unit",
            std:str:cat fuel_delta " fuel units",
            std:str:cat "= " price " credits",
        ]
};

!@export STATE STATE;

!show_station_win = $&&$n;
.*show_station_win = {!(sys_id, ent_id) = @;

    sscg:win.set_window WID_STATION ${
        x = 250, y = 250, w = 500, h = 500,
        title = std:str:cat["Station ", sys_id, ":", ent_id],
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
                        refuel_text[]
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
                show_station_win[sys_id, ent_id];
            }
            {|| sscg:win.set_window WID_STATION; };
    };
};

!@export on_arrived {!(too_fast, sys_id, ent_id) = @;
    std:displayln "ARRIVED!";
    (bool too_fast) {
        STATE.ship.fuel = std:num:floor 0.5 * STATE.ship.fuel
    };
    STATE.ship.docked = $t;
    show_station_win[sys_id, ent_id];
};

!display_fuel_out_warning = \:warn {
    (bool STATE.ship.fuel_warning) { return :warn $n; };
    STATE.ship.fuel_warning = $t;
    std:displayln "WARNING DISPLAY!";

    sscg:win.set_window WID_OUTOFFUEL ${
        x = 250, y = 250, w = 500, h = 500,
        title = std:str:cat["Out Of Fuel"],
        title_color = "F00",
        child = ${
            t = :vbox,
            w = 1000,
            h = 1000,
            spacing = 0,
            childs = $[
                ${ t    = :l_text,
                   w    = 1000,
                   h    = 800,
                   fg   = "F60",
                   bg   = "000",
                   text = std:str:cat
                        "You ran out of fuel. Your max speed is capped. "
                        "Fly to the next station and refuel if you have the "
                        "credits. Otherwise you may have to find another chance "
                        "to refuel." },
                ${ t = :l_button, fg = "000", bg = "0F0",
                   w = 200, h = 200, text = "Ok", ref = "quit" },
            ]
        }
    } {||
        sscg:win.set_window WID_OUTOFFUEL;
    };
};

!@export on_tick {!(ship_action_state) = @;
    !engine_on_delta = ship_action_state.engine_on_secs - STATE.ship.engine_on_secs;
    STATE.ship.engine_on_secs = ship_action_state.engine_on_secs;

    !typ = STATE.ship.t;
    !ship_type = STATE.ship_types.(typ);

    STATE.ship.fuel = STATE.ship.fuel - ship_type.fuel_per_sec * engine_on_delta;
    (STATE.ship.fuel <= 0) {
        display_fuel_out_warning[];
        STATE.ship.fuel = 0;
    };

    std:displayln "TICK" ship_type;

    !speed_i = std:num:ceil ~ 1000.0 * ship_action_state.speed;
    .speed_i = speed_i >= 100 { str speed_i } { std:str:cat "(docking) " speed_i };

    sscg:win.set_label WID_STATUS :speed speed_i;
    sscg:win.set_label WID_STATUS :engine_on_secs (str STATE.ship.engine_on_secs);
    sscg:win.set_label WID_STATUS :fuel ~
        std:str:cat STATE.ship.fuel " / " ship_type.fuel_capacity;
};

!@export init {

    !status_value = {!(lbl, ref) = @;
        ${ t = "hbox", w = 1000, spacing = 5, childs = $[
            ${ t = :l_text, fg = "000", bg = "0F0",
               text = lbl, w = 500 },
            ${ t = :r_text, fg = "000", bg = "0F0",
               text = "", ref = ref, w = 500 },
        ] }
    };

    sscg:win.set_window WID_STATUS ${
        x = 0, y = 700, w = 300, h = 300,
        title = std:str:cat["Ship"],
        title_color = "0FF",
        child = ${
            t = :vbox,
            w = 1000,
            h = 1000,
            spacing = 5,
            childs = $[
                status_value "Engine Time" :engine_on_secs,
                status_value "Speed"       :speed,
                status_value "Fuel"        :fuel,
            ]
        }
    } {||};

    std:displayln "DISPLAY INIT";
    STATE
};
