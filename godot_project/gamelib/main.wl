!@import wlambda;
!@import std        std;
!@import sscg       sscg;
!@import c          colors;
!@import e_station  station;
!@import WID        gui_window_ids;
!@import gui        gui_common;

!STATE = ${
    good_types = ${
        rock = ${
            kg_p_m3     = 1800,
            unit_kg     = 100,
            baseprice   = 10,
        },
    },
    ship_types = ${
        scout_mk1 = ${
            fuel_capacity       = 1000,
            fuel_per_sec        = 14,
            max_kg_fuel_factor  = 200,
            cargo_max_m3        = 2000,
            cargo_max_kg        = 10000,
        },
    },
    player = ${
        base_tax        = 0.2,
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
        stargate   = ${ visual = "stargate",   gui = "stargate" },
        asteroid_1 = ${ visual = "asteroid_1", gui = "asteroid" },
    },
    systems = $[
        ${
            name = "Testaris 1",
            entities = $[
                ${ t = "station",    name = "Station 1",  pos = $[2000, 2000] },
                ${ t = "station",    name = "Station 2",  pos = $[ 500, 2000] },
                ${ t = "station",    name = "Station 3",  pos = $[9000, 4000] },
                ${ t = "station",    name = "Station 4",  pos = $[4000, 4000] },
                ${ t = "stargate",   name = "Stargate Alpha",  pos = $[200, 200] },
                ${ t = "asteroid_1", name = "Asteroid 1", pos = $[ 300,  300] },
                ${ t = "asteroid_1", name = "Asteroid 2", pos = $[6500, 6500] },
#                ${ t = "station", name = "Asteroid 2", pos = $[6500, 6500] },
            ],
        }
    ],
    code = ${},
};

!@export STATE STATE;

STATE.code.sell_ship_cargo_good = {!(good_t) = @;
    !good_units  = STATE.ship.cargo.goods.(good_t);
    !units_money = STATE.good_types.(good_t).baseprice * good_units;
    STATE.player.credits =
        STATE.player.credits
        + (float units_money) * (1.0 - STATE.player.base_tax);
    STATE.ship.cargo.goods.(good_t) = $n;
    STATE.code.recalc_ship_cargo[];
};

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
!:global on_tick_mining_update = $n;
!show_asteroid_win = $&&$n;
.*show_asteroid_win = {!(ent, ent_type) = @;
    gui:dialog_window WID:STATION ent.name {
        $[
            ${ t = :hbox, spacing = 5, w = 1000, childs = $[
                ${ t = :l_text, text = "", w = 333, fg = "F00", bg = "000" },
                ${ t = :l_text, text = "Current", w = 333, fg = c:SE2_L, bg = "000" },
                ${ t = :l_text, text = "Ship Max", w = 333, fg = c:SE2_L, bg = "000" },
            ]},
            ${ t = :hbox, spacing = 5, border = 1, border_color = c:SE2, w = 1000, min_h = 25, childs = $[
                ${ t = :l_text, text = "m³",                                         w = 333, fg = c:SE2_L, bg = "000" },
                ${ t = :l_text, ref = :m3, text = STATE.ship.cargo.m3,                          w = 333, fg = c:SE1_L, bg = "000" },
                ${ t = :l_text, text = STATE.ship_types.(STATE.ship.t).cargo_max_m3, w = 333, fg = c:SE2, bg = "000" },
            ]},
            ${ t = :hbox, spacing = 5, border = 1, border_color = c:SE2, w = 1000, min_h = 25, childs = $[
                ${ t = :l_text, text = "kg³",                                        w = 333, fg = c:SE2_L, bg = "000" },
                ${ t = :l_text, ref = :kg, text = STATE.ship.cargo.kg,                          w = 333, fg = c:SE1_L, bg = "000" },
                ${ t = :l_text, text = STATE.ship_types.(STATE.ship.t).cargo_max_kg, w = 333, fg = c:SE2, bg = "000" },
            ]},
            ${ t = :hbox, spacing = 5, w = 1000, h = 300, childs = $[
                gui:action_button 500 1000 :start_mining "Start mining",
                gui:button 500 1000 :depart "Depart",
            ]},
#            ${ t = :canvas,
#                w = 1000,
#                h = 400,
#                ref = "map",
#                cmds = $[
#                    $[:circle,      10, 500, 500, 100, "F00"],
#                    $[:line,        11, 500, 500, 600, 900, 4, "FF0"],
#                    $[:rect,        20, 200,   0, 100, 200, "9F3"],
#                    $[:rect_filled, 20, 300,   0, 100, 200, "FF3"],
#                    $[:text,        12, 600, 900, 1000, 1, "Test", 0, "FFF"],
#                ],
#            },
        ]
    } {||
        std:displayln @;
        match _1
            "start_mining" {||
                STATE.player.is_mining = $t;
                .on_tick_mining_update = {
                    sscg:win.set_label WID:STATION :m3 STATE.ship.cargo.m3;
                    sscg:win.set_label WID:STATION :kg STATE.ship.cargo.kg;
                };
            }
            {||
                STATE.player.is_mining = $f;
                sscg:win.set_window WID:STATION;
                STATE.ship.docked = $f;
            };
    };
};

!@export on_saved_godot_state {!(state) = @;
    std:displayln "STATE:" state;

    on_error {||
        std:displayln "ERROR WRITING SAVEGAME: " @
    } ~ sscg:game.write_savegame "sv1" ${
        version     = 1,
        player      = STATE.player,
        ship        = STATE.ship,
        ship_dyn    = state,
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

!open_start_info = {
    sscg:win.set_window WID:MAIN_MENU ${
        x = 200, y = 100, w = 550, h = 800,
        title = std:str:cat["Infomation"],
        title_color = c:CON,
        child = ${
            t = :vbox,
            w = 1000,
            h = 1000,
            spacing = 10,
            childs = $[
                ${ t = :l_label, fg = c:SE1_L2, bg = "000", h = 100, w = 1000,
                   text = "Welcome to SSCG - Space Ship Cargo Game!" },
                ${ t = :c_text, fg = c:SE1_L, bg = "000", h = 700, w = 1000,
                   margin = 10,
                   text = std:str:join " " $[
                        "This is an early alpha tech release. ",
                        "Expect bugs and missing features. ",
                        "Please consult the key binding help in the top ",
                        "left of the screen for help about the controls.",
                        "To interact with space stations and other objects ",
                        "just fly head on into them at 'docking' speed. ",
                        "There is currently not much more information ",
                        "available. Please explore the game by yourself ",
                        "or ask the developer(s).",
                   ]
                },
                ${ t = :r_button, fg = "000", bg = c:SE2,
                   w = 300, h = 10, text = "Close", ref = "close" },
            ]
        }
    } {|| sscg:win.set_window WID:MAIN_MENU; };
};

!open_credits = {
    !credits = $[
        $["Game Design and Programming", $[
            "Weird Constructor",
        ] ],
        $["Music & Sound", $[
        ] ],
        $["Artwork", $[
        ] ],
        $["Feedback, Hints, Ideas and Testing", $[
            "Gargaj",
            "Itmuckel",
            "Ilmuri",
            "Tom from 'Recall Singularity'",
            "szczm",
        ] ],
        $["Engine", $[
            "Godot game engine developers",
            "Godot-rust binding Developers ('karroffel', 'toasteater' and all others)",
        ] ],
    ];

    sscg:win.set_window WID:MAIN_MENU ${
        x = 200, y = 100, w = 550, h = 800,
        title = std:str:cat["Credits"],
        title_color = c:CON,
        child = ${
            t = :vbox,
            w = 1000,
            h = 1000,
            spacing = 10,
            childs = $[
                *credits | std:fold $[] { !(section, out) = @;
                    std:push out ${
                        t = :l_label,
                        w = 1000,
                        text = section.0,
                        fg = c:SE2_L,
                        bg = "000",
                    };
                    std:append out ~ section.1 { ${
                        t = :l_text,
                        w = 1000,
                        text = std:str:cat "- " _,
                        fg = c:SE1_L,
                        bg = "000",
                    } }
                },
                ${ t = :r_button, fg = "000", bg = c:SE2,
                   w = 300, h = 10, text = "Close", ref = "close" },
            ],
        },
    } {|| sscg:win.set_window WID:MAIN_MENU; };
};

!open_menu = {
    sscg:win.set_window WID:MAIN_MENU ${
        x = 200, y = 200, w = 550, h = 550,
        title = std:str:cat["Main Menu"],
        title_color = c:CON,
        child = ${
            t = :vbox,
            w = 1000,
            h = 1000,
            spacing = 10,
            childs = $[
                ${ t = :l_button, fg = "000", bg = c:SE1,
                   w = 300, h = 200, text = "Start", ref = "start" },
                ${ t = :l_button, fg = "000", bg = c:SE1,
                   w = 300, h = 100, text = "Load", ref = "load" },
                ${ t = :l_button, fg = "000", bg = c:SE1,
                   w = 300, h = 100, text = "Save", ref = "save" },
                ${ t = :l_button, fg = "000", bg = c:CON,
                   w = 300, h = 100, text = "Credits", ref = "credits" },
                ${ t = :r_button, fg = "000", bg = c:SE2,
                   w = 300, h = 200, text = "Close", ref = "close" },
            ],
        },
    } {||
        match _1
            "start" {|| open_start_info[]; }
            "save" {|| sscg:game.cmd "save_state" $n; }
            "credits" {|| open_credits[]; }
            "load" {||
                !state =
                    on_error {|| std:displayln "Couldn't load savegame: " @ }
                        ~ sscg:game.read_savegame "sv1";
                (bool state) {
                    STATE.player = state.player;
                    STATE.ship   = state.ship;
                    sscg:game.cmd "load_state" state.ship_dyn;
                };
            }
            {|| sscg:win.set_window WID:MAIN_MENU; };
    };
};

!@export on_tick {!(ship_action_state) = @;
    (bool STATE.player.is_mining) {
        !capacity_units =
            STATE.code.calc_unit_capacity_for_good :rock;
        (capacity_units > 0) {
            STATE.ship.cargo.goods.rock =
                STATE.ship.cargo.goods.rock + 1;
            STATE.code.recalc_ship_cargo[];
            on_tick_mining_update[];
        }
    };

    !engine_on_delta =
        ship_action_state.engine_on_secs - STATE.ship.engine_on_secs;
    STATE.ship.engine_on_secs = ship_action_state.engine_on_secs;

    !typ = STATE.ship.t;
    !ship_type = STATE.ship_types.(typ);

    !fuel_usage_factor =
        (STATE.ship.cargo.kg * ship_type.max_kg_fuel_factor)
        / ship_type.cargo_max_kg;
    .fuel_usage_factor = fuel_usage_factor + 100;

    STATE.ship.fuel =
        STATE.ship.fuel
        - (fuel_usage_factor * ship_type.fuel_per_sec * engine_on_delta) / 100;
    (STATE.ship.fuel <= 0) {
        display_fuel_out_warning[];
        STATE.ship.fuel = 0;
    };

    #d# std:displayln "TICK" ship_type;

    !speed_i = std:num:ceil ~ 1000.0 * ship_action_state.speed;
    .speed_i = speed_i >= 100 { str speed_i } { std:str:cat "(docking) " speed_i };

    sscg:win.set_label WID:STATUS :speed speed_i;
    sscg:win.set_label WID:STATUS :engine_on_secs (str STATE.ship.engine_on_secs);
    sscg:win.set_label WID:STATUS :fuel_usage ~ std:str:cat fuel_usage_factor "%";
    sscg:win.set_label WID:STATUS :fuel ~
        std:str:cat STATE.ship.fuel " / " ship_type.fuel_capacity;
    sscg:win.set_label WID:STATUS :credits STATE.player.credits;
    sscg:win.set_label WID:STATUS :cargo_load ~
        std:str:cat (STATE.ship.cargo.m3) " / " STATE.ship.cargo.kg;
};

!@export on_ready {
    std:displayln "GAME READY!";
    sscg:game.cmd :load_state ${
        engine_on_fract = 0.0,
        engine_on_secs  = 0.0,
        thruster_speed  = 0.0,
        speed           = 0.0,
        x               = 5000,
        y               = 4990,
        rot_z           = 0,
    };

    open_menu[];
};

!@export init {

    !status_value = {!(lbl, ref) = @;
        ${ t = "hbox", w = 1000, spacing = 2, childs = $[
            ${ t = :r_label, fg = "000", bg = c:PRI_L,
               font = :small,
               text = lbl, w = 500 },
            ${ t = :r_label, fg = "000", bg = c:PRI_L,
               font = :small,
               text = "", ref = ref, w = 500 },
        ] }
    };

    sscg:win.set_window WID:STATUS ${
        x = 0, y = 720, w = 300, h = 280,
        title = std:str:cat["Ship"],
        title_color = c:PRI,
        child = ${
            t = :vbox,
            w = 1000,
            h = 1000,
            spacing = 2,
            childs = $[
                status_value "Engine Time" :engine_on_secs,
                status_value "Speed"       :speed,
                status_value "Fuel"        :fuel,
                status_value "Credits"     :credits,
                status_value "Cargo m³/kg" :cargo_load,
                status_value "Fuel usage"  :fuel_usage,
                ${ t = :l_button, text = "Menu", w = 1000, bg = c:CON, fg = "000", ref = "menu" },
            ]
        }
    } {||
        match _1
            "menu" {|| open_menu[]; };
    };

    std:displayln "DISPLAY INIT";

#    STATE.ship.cargo.goods.rock = 100;
#    STATE.code.recalc_ship_cargo[];

    STATE
};
