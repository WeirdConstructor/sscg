!@import wlambda;
!@import std        std;
!@import sscg       sscg;
!@import c          colors;
!@import e_station  station;
!@import WID        gui_window_ids;

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
        station    = ${ visual = "station",    gui = "station"  },
        asteroid_1 = ${ visual = "asteroid_1", gui = "asteroid" },
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
                ${ t = "asteroid_1", pos = $[6500, 6500] },
            ],
        }
    ],
};

!@export STATE STATE;

!show_asteroid_win = {!(ent, ent_type) = @;
    sscg:win.set_window WID:STATION ${
        x = 250, y = 250, w = 500, h = 500,
        title = std:str:cat["Asteroid ", ent],
        title_color = c:PRI_L,
    } {||
    };
};

!@export on_arrived {!(too_fast, sys_id, ent_id) = @;
    std:displayln "ARRIVED!";
    (bool too_fast) {
        STATE.ship.fuel = std:num:floor 0.5 * STATE.ship.fuel
    };
    STATE.ship.docked = $t;
    !ent = STATE.systems.(sys_id).entities.(ent_id);
    !ent_typ = STATE.entity_types.(ent.t);

    match ent_typ.gui
        "station" {|| e_station:show[STATE, ent, ent_typ]; }
                  {|| show_asteroid_win[ent, ent_typ] };
};

!display_fuel_out_warning = \:warn {
    (bool STATE.ship.fuel_warning) { return :warn $n; };
    STATE.ship.fuel_warning = $t;
    std:displayln "WARNING DISPLAY!";

    sscg:win.set_window WID:OUTOFFUEL ${
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
        sscg:win.set_window WID:OUTOFFUEL;
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

    sscg:win.set_label WID:STATUS :speed speed_i;
    sscg:win.set_label WID:STATUS :engine_on_secs (str STATE.ship.engine_on_secs);
    sscg:win.set_label WID:STATUS :fuel ~
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

    sscg:win.set_window WID:STATUS ${
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
