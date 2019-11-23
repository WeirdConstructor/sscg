!@import wlambda;
!@import std        std;
!@import sscg       sscg;
!@import c          colors;
!@import e_station  station;
!@import WID        gui_window_ids;

!STATE = ${
    good_types = ${
        rock = ${ kg_p_m3 = 1800, unit_kg = 100, },
    },
    ship_types = ${
        scout_mk1 = ${
            fuel_capacity   = 1000,
            fuel_per_sec    = 7,
            cargo_max_m3    = 2000,
            cargo_max_kg    = 10000,
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
        cargo           = ${ m3 = 0, kg = 0, goods = ${} },
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
    code = ${},
};

!@export STATE STATE;

STATE.code.calc_unit_capacity_for_good = {!(good_t) = @;
    !good_type  = STATE.good_types.(good_t);
    !ship_type  = STATE.ship_types.(STATE.ship.t);
    !kg_free    = ship_type.cargo_max_kg - STATE.ship.cargo.kg;
    !m3_free    = ship_type.cargo_max_m3 - STATE.ship.cargo.m3;
    !m3_free_kg = (m3_free * good_type.kg_p_m3) / 1000;
    !min_kg_free = (kg_free < m3_free_kg) { kg_free } { m3_free_kg };
    min_kg_free / good_type.unit_kg
};

STATE.code.recalc_ship_cargo = {
    !s = STATE.ship;
    s.cargo.m3  = 0;
    s.cargo.kg  = 0;
    s.cargo.goods {!(v, k) = @;
        !good_type = STATE.good_types.(k);
        s.cargo.kg =
            s.cargo.kg + good_type.unit_kg * v;
        s.cargo.m3 =
            s.cargo.m3 + ((good_type.unit_kg * v * 1000) / good_type.kg_p_m3);
    };
};

# Actions
# - display cargo space
# - leave menu
# - start mining
!show_asteroid_win = $&&$n;
.*show_asteroid_win = {!(ent, ent_type) = @;
    sscg:win.set_window WID:STATION ${
        x = 250, y = 250, w = 500, h = 500,
        title = std:str:cat["Asteroidx ", ent],
        title_color = c:PRI_L,
        child = ${
            t = :vbox,
            w = 1000,
            h = 1000,
            childs = $[
                ${ t = :hbox, spacing = 5, w = 1000, h = 500, childs = $[
                    ${ t = :r_button,
                       text = "Start mining",
                       ref = :start_mining,
                       fg = "000",
                       bg = c:SE2,
                       w = 500, h = 1000 },
                    ${ t = :l_button,
                       text = "Leave",
                       ref = :leave,
                       fg = "000",
                       bg = c:SE1,
                       w = 500, h = 1000 },
                ]},
            ]
        }
    } {||
        match _1
            "start_mining" {||
                STATE.player.is_mining = $t;
            }
            {||
                STATE.player.is_mining = $f;
                sscg:win.set_window WID:STATION;
                STATE.ship.docked = $f;
            };
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
        title_color = c:SE1,
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
    (bool STATE.player.is_mining) {
        !capacity_units = STATE.code.calc_unit_capacity_for_good :rock;
        (capacity_units > 0) {
            STATE.ship.cargo.goods.rock =
                STATE.ship.cargo.goods.rock + 1;
            STATE.code.recalc_ship_cargo[];
        }
    };

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
    sscg:win.set_label WID:STATUS :credits STATE.player.credits;
    sscg:win.set_label WID:STATUS :cargo_load ~
        std:str:cat
            (STATE.ship.cargo.m3) " / " STATE.ship.cargo.kg;
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
                status_value "Credits"     :credits,
                status_value "Cargo m³/kg" :cargo_load,
            ]
        }
    } {||};

    std:displayln "DISPLAY INIT";
    STATE
};
