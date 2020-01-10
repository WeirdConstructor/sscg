!@import wlambda;
!@import std        std;
!@import sscg       sscg;
!@import c          colors;
!@import e_station  station;
!@import WID        gui_window_ids;
!@import gui        gui_common;

!color_map = $[
    "#000",     # 0
    "#f99",     # 1
    "#ff0",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 5
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 10
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 15
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 20
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 25
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 30
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 35
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 40
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 45
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 50
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 55
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 60
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 65
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 70
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 75
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 80
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 85
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 90
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 95
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 100
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 105
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 110
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 115
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",     # 120
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
    "#f0f",
];

!STATE = ${
    good_types = ${
        rock = ${
            name        = "Unknown Minerals",
            kg_p_m3     = 1800,
            unit_g      = 100000,
            baseprice   = 10,
        },
        element_c = ${
            short       = "C",
            name        = "Carbon",
            kg_p_m3     = 1300, # 1300 kg/m3 is stone coal, 2,27g/cm3 graphite, 3,51g/cm3 is diamond
            unit_g      = 50000,
            baseprice   = 20,
            mineable    = $true,
            vol_color   = 6,
        },
        element_h = ${
            short       = "H",
            name        = "Hydrogen",
            kg_p_m3     = 1, # gas
            unit_g      = 50,
            baseprice   = 50,
            mineable    = $true,
            vol_color   = 1,
        },
        element_he = ${
            short       = "He",
            name        = "Helium",
            kg_p_m3     = 2, # gas
            unit_g      = 100,
            baseprice   = 90,
            mineable    = $true,
            vol_color   = 2,
        },
        element_o = ${
            short       = "O",
            name        = "Oxygen",
            kg_p_m3     = 2, # gas
            unit_g      = 200,
            baseprice   = 100,
            mineable    = $true,
            vol_color   = 8,
        },
        element_ag = ${
            short       = "Ag",
            name        = "Silver",
            kg_p_m3     = 10490,
            unit_g      = 1000,
            baseprice   = 200,
            mineable    = $true,
            vol_color   = 47,
        },
    },
    ship_types = ${
        scout_mk1 = ${
            fuel_capacity       = 1000,
            fuel_per_sec        = 10,
            max_kg_fuel_factor  = 200,
            cargo_max_m3        = 2000,
#            cargo_max_m3        = 222,
            cargo_max_kg        = 10000,
        },
    },
    player = ${
        base_tax        = 0.2,
        credits         = 1000,
    },
    ship = ${
        t               = :scout_mk1,
        system_id       = 0,
        docked          = $f,
        engine_on_secs  = 0,
        fuel            = 1000,
        cargo           = ${ m3 = 0, kg = 0, goods = ${} },
    },
    entity_types = ${
        station         = ${ visual = "station",    gui = "station"   },
        stargate        = ${ visual = "stargate",   gui = "stargate"  },
        asteroid_1      = ${ visual = "asteroid_1", gui = "asteroid"  },
        alien_struct    = ${ visual = "structure",  gui = "structure" },
    },
    systems = $[
        ${
            name = "Testaris 1",
            entities = $[
                ${ t = "station",       name = "Station 1",    pos = $[200,   0] },
                ${ t = "alien_struct",  name = "Voxel Struct", pos = $[572, 200] },
                ${ t = "asteroid_1",    name = "Asteroid 1",   pos = $[400, 400] },
            ],
        }
    ],
    code      = ${},
    callbacks = ${},
};

!@export STATE STATE;

STATE.code.sell_ship_cargo_good = {!(good_t) = @;
    !good_units  = STATE.ship.cargo.goods.(good_t);
    !units_money = STATE.good_types.(good_t).baseprice * good_units;
    STATE.player.credits =
        STATE.player.credits
        + (float units_money) * (1.0 - STATE.player.base_tax);
    std:displayln "SELL Ship Goods=" STATE.ship.cargo.goods;
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
    (min_kg_free * 1000) / good_type.unit_g
};

STATE.code.recalc_ship_cargo = {
    !s = STATE.ship;
    s.cargo.m3  = 0;
    s.cargo.kg  = 0;
    s.cargo.goods {!(v, k) = @;
        !good_type = STATE.good_types.(k);
        s.cargo.kg =
            s.cargo.kg + (good_type.unit_g * v) / 1000;
        s.cargo.m3 =
            s.cargo.m3 + ((good_type.unit_g * v * 1000)
                          / (1000 * good_type.kg_p_m3));
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
                ${ t = :l_text, text = "kg",                                        w = 333, fg = c:SE2_L, bg = "000" },
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

STATE.code.get_good_by_color = {!(color) = @;
    block :ret {
        STATE.good_types {!(v, k) = @;
            std:displayln :COMP v ">>" k ">" color;
            (int[v.vol_color] == int[color]) {
                return :ret $[k, v];
            }
        };
        $none
    }
};

STATE.callbacks.on_mine = {
    std:displayln "MINE:" @;
    !(k, v) = STATE.code.get_good_by_color[_3];

    !capacity_units =
        STATE.code.calc_unit_capacity_for_good k;

    (capacity_units > 0) &and (_2 != 0)
};

STATE.callbacks.on_mined_voxel = {
    std:displayln "MINEDD:" @ STATE.code.get_good_by_color[_2];
    !(k, v) = STATE.code.get_good_by_color[_2];
    not[is_none[k]] {
        STATE.ship.cargo.goods.(k) =
            STATE.ship.cargo.goods.(k) + 1;
        STATE.code.recalc_ship_cargo[];
    };
    $t
};

!vp = $&&$n;

STATE.callbacks.on_draw_voxel_structure = {!(sys_id, ent_id) = @;
    !vp = $*vp;
    std:displayln "LOADDED on_draw_voxel_structure " vp "|" sys_id ent_id;
    vp.clear[];
    !main_vol = vp.new 128 0.0;
    vp.fill main_vol 0
        0 0 0
        128 128 128
        2.0 / 255.0;
    vp.fill main_vol 0
        0 0 0
        128 1 128
        1.0 / 255.0;
    vp.fill main_vol 0
        0 1 0
        128 1 128
        6.0 / 255.0;
    std:displayln "NEWVOL:" main_vol;

    std:displayln "DONE!";
    $[vp.id[], main_vol, color_map]
#    ; $n
};

STATE.callbacks.on_saved_godot_state = {!(state) = @;
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

STATE.callbacks.on_arrived = {!(too_fast, sys_id, ent_id) = @;
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
            "'Synthwave' by Ryan Andersen, CC-BY-NC 4.0 from Free Music Archive",
        ] ],
        $["Artwork", $[
        ] ],
        $["Feedback, Hints, Ideas and Testing", $[
            "Gargaj (shader suggestions)",
            "Ilmuri (help with shaders and feedback)",
            "Tom from 'Recall Singularity' (feedback and suggestions)",
            "Itmuckel (feedback)",
            "szczm (dragging me back into game dev)",
        ] ],
        $["Engine", $[
            "Godot game engine developers",
            "Godot-rust binding developers ('karroffel', 'toasteater' and all others)",
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

!load_save = {
    !state =
        on_error {|| std:displayln "Couldn't load savegame: " @ }
            ~ sscg:game.read_savegame "sv1";
    (bool state) {
        STATE.player = state.player;
        STATE.ship   = state.ship;
        sscg:game.cmd "load_state" state.ship_dyn;
    };
};

!open_menu = {
    sscg:win.set_window WID:MAIN_MENU ${
        x = 300, y = 200, w = 400, h = 550,
        title = std:str:cat["Main Menu"],
        title_color = c:CON,
        child = ${
            t = :vbox,
            w = 1000,
            h = 1000,
            spacing = 10,
            childs = $[
                ${ t = :l_button, fg = "000", bg = c:SE1,
                   w = 1000, h = 200, text = "Start", ref = "start" },
                ${ t = :l_button, fg = "000", bg = c:SE1,
                   w = 1000, h = 100, text = "Load", ref = "load" },
                ${ t = :l_button, fg = "000", bg = c:SE1,
                   w = 1000, h = 100, text = "Save", ref = "save" },
                ${ t = :l_button, fg = "000", bg = c:CON,
                   w = 1000, h = 100, text = "Credits", ref = "credits" },
                ${ t = :r_button, fg = "000", bg = c:SE2,
                   w = 1000, h = 200, text = "Close", ref = "close" },
            ],
        },
    } {||
        match _1
            "start"     {|| open_start_info[]; }
            "save"      {|| sscg:game.cmd "save_state" $n; }
            "credits"   {|| open_credits[]; }
            "load"      {|| load_save[]; }
            {|| sscg:win.set_window WID:MAIN_MENU; };
    };
};

STATE.callbacks.on_tick = {!(ship_action_state) = @;
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

STATE.callbacks.on_ready = {
    std:displayln "GAME READY!";
    sscg:game.cmd :load_state ${
        engine_on_fract = 0.0,
        engine_on_secs  = 0.0,
        thruster_speed  = 0.0,
        speed           = 0.0,
        x               = 0.0,
        y               = 0.0,
        rot_z           = 0,
    };

#    open_menu[];
    load_save[];
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

    .*vp = sscg:new_voxel_painter[];
    std:displayln "VOXPAINT INIT " vp;

#    STATE.ship.cargo.goods.rock = 100;
#    STATE.code.recalc_ship_cargo[];

    STATE
};
