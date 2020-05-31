!@wlambda;
!@import std;
!@import sscg;
!@import c              colors;
!@import e_station      station;
!@import e_structure    structure;
!@import WID            gui_window_ids;
!@import gui            gui_common;
!@import w_count        count_window;
!@import credits        credits;
!@import u              util;
!@import el             elements;
!@import def_colors     default_colormap;

!@import e:ship         entities:ship;

# Took colors from https://en.wikipedia.org/wiki/CPK_coloring
# The color scheme of Jmol
!color_map = def_colors:color_map;

!:global STATE = ${
    good_types = ${
        rock = ${
            name        = "Unknown Minerals",
            kg_p_m3     = 1800,
            unit_g      = 100000,
            baseprice   = 10,
        },
#        element_c = ${
#            short       = "C",
#            name        = "Carbon",
#            kg_p_m3     = 1300, # 1300 kg/m3 is stone coal, 2,27g/cm3 graphite, 3,51g/cm3 is diamond
#            unit_g      = 50000,
#            baseprice   = 20,
#            mineable    = $true,
#            vol_color   = 6,
#        },
#        element_h = ${
#            short       = "H",
#            name        = "Hydrogen",
#            kg_p_m3     = 1, # gas
#            unit_g      = 50,
#            baseprice   = 50,
#            mineable    = $true,
#            vol_color   = 1,
#        },
#        element_he = ${
#            short       = "He",
#            name        = "Helium",
#            kg_p_m3     = 2, # gas
#            unit_g      = 100,
#            baseprice   = 90,
#            mineable    = $true,
#            vol_color   = 2,
#        },
#        element_o = ${
#            short       = "O",
#            name        = "Oxygen",
#            kg_p_m3     = 2, # gas
#            unit_g      = 200,
#            baseprice   = 100,
#            mineable    = $true,
#            vol_color   = 8,
#        },
#        element_ag = ${
#            short       = "Ag",
#            name        = "Silver",
#            kg_p_m3     = 10490,
#            unit_g      = 1000,
#            baseprice   = 200,
#            mineable    = $true,
#            vol_color   = 47,
#        },
    },
    ship_types = e:ship:default_ship_types,
    player = ${
        base_tax        = 0.2,
        credits         = 1000,
    },
    ship = e:ship:ship.new :scout_mk1,
    entity_types = ${
        station         = ${ visual = "station",    gui = "station"   },
        stargate        = ${ visual = "stargate",   gui = "stargate"  },
        alien_struct    = ${ visual = "structure",  gui = "structure" },
    },
    systems = $[
        ${
            name = "Testaris 1",
            entities = $[
                ${ t = "station",       name = "Station 1",    pos = $[200,   0] },
                ${ t = "alien_struct",  name = "Voxel Struct", pos = $[572, 200] },
            ],
        }
    ],
    code      = ${},
    callbacks = ${},
};

!@export STATE STATE;

STATE.code.build_color_to_element_index = {||
    !vol_color_goods = $[];
    STATE.good_types {!(v, k) = @;
        v.t = k;
        (not ~ is_none v.vol_color) { vol_color_goods.(v.vol_color) = v; };
    };
#    std:displayln vol_color_goods;
    STATE.vol_color_goods = vol_color_goods;
};

STATE.code.enumerate_entities = {||
    !i = $&0;
    STATE.systems {!(sys) = @;
        sys.id = i;
        .i = i + 1;

        !j = $&0;
        sys.entities {!(ent) = @;
            ent.id = j;
            .j = j + 1;
        };
    };
};

STATE.code.sell_all_ship_cargo = {
    for STATE.ship.cargo.goods {!(k, v) = _;
#        std:displayln "SELLGOOD" k "||" v;
        STATE.code.sell_ship_cargo_good k;
    };
};

STATE.code.sell_ship_cargo_good = {!(good_t) = @;
    !good_units  = STATE.ship.cargo.goods.(good_t);
    !units_money = STATE.good_types.(good_t).baseprice * good_units;
    STATE.player.credits =
        STATE.player.credits
        + (float units_money) * (1.0 - STATE.player.base_tax);
#    std:displayln "SELL Ship Goods=" STATE.ship.cargo.goods;
    STATE.ship.cargo.goods.(good_t) = $n;
    STATE.code.recalc_ship_cargo[];
};

STATE.code.calc_unit_capacity_for_good = \:r {!(good_t) = @;
    (is_none good_t) { return :r $none; };

    !good_type  = STATE.good_types.(good_t);
    !ship_type  = STATE.ship_types.(STATE.ship.t);
    !kg_free    = ship_type.cargo_max_kg - STATE.ship.cargo.kg;
    !m3_free    = ship_type.cargo_max_m3 - STATE.ship.cargo.m3;
    !m3_free_kg = (m3_free * good_type.kg_p_m3) / 1000;
    !min_kg_free = (kg_free < m3_free_kg) { kg_free } { m3_free_kg };
    (min_kg_free * 1000) / good_type.unit_g
};

STATE.code.update_hud_cargo_meters = {||
    !m3_perc = (100 * STATE.ship.cargo.m3)
               / STATE.ship_types.(STATE.ship.t).cargo_max_m3;
    !kg_perc = (100 * STATE.ship.cargo.kg)
               / STATE.ship_types.(STATE.ship.t).cargo_max_kg;

    sscg:game.gd_call "GUI" :set_cargo_meter $[kg_perc, m3_perc];
};

STATE.code.recalc_ship_cargo = {
    !s = STATE.ship;
    s.cargo.units       = 0;
    s.cargo.fuel_factor = 0;
    for s.cargo.goods \:good_loop {!(k, v) = _;
        (v <= 0) { return :good_loop $n };

        !good_type = STATE.good_types.(k);
#        std:displayln k "::" s.cargo.kg ";" s.cargo.m3
#        std:displayln k "::" v "=" (std:ser:json good_type);
#        std:displayln k "=> m³="
#            (good_type.unit_g * v) / 1000
#            "; kg="
#            (good_type.unit_g * v * 1000)
#             / (1000 * good_type.kg_p_m3);

        s.cargo.kg =
            s.cargo.kg + (good_type.unit_g * v) / 1000;
        s.cargo.m3 =
            s.cargo.m3 + ((good_type.unit_g * v * 1000)
                          / (1000 * good_type.kg_p_m3));
    };

    STATE.code.update_hud_cargo_meters[];
};

# Actions
# - display cargo space
# - leave menu
# - start mining
STATE.code.get_good_by_color = {!(color) = @;
    block :ret {
        STATE.good_types {!(v, k) = @;
            (int[v.vol_color] == int[color]) {
                return :ret $[k, v];
            }
        };
        $none
    }
};

STATE.callbacks.on_update_mining_hud = \:r {!(mining_info) = @;
    (is_none mining_info) {
        sscg:game.gd_call "GUI" :set_hud_info "";
    } {
        !good_type = STATE.vol_color_goods.(int mining_info.material);
        (bool good_type) {
            sscg:game.gd_call "GUI" :set_hud_info
                ~ std:str:cat good_type.name " (" good_type.short ")";
        } {
            sscg:game.gd_call "GUI" :set_hud_info "";
        };
    };
};

STATE.callbacks.on_mine = \:r {
    !(k, v) = STATE.code.get_good_by_color[_3];

    (is_none k) { return :r $false; };

    std:displayln "MINE:" @;

    !capacity_units =
        STATE.code.calc_unit_capacity_for_good k;

    (capacity_units > 0) &and (_2 != 0)
};

STATE.callbacks.on_mined_voxel = {||
#    std:displayln "MINEDD:" @ STATE.code.get_good_by_color[_2];
    !(k, v) = STATE.code.get_good_by_color[_2];
    not[is_none[k]] {
        STATE.ship.cargo.goods.(k) =
            STATE.ship.cargo.goods.(k) + 1;
#        std:displayln "CARGO:" STATE.ship.cargo;
        STATE.code.recalc_ship_cargo[];
    };
    $t
};

!vp = $&&$n;

STATE.callbacks.on_texture_description = {|| std:displayln "Describing textures ..."; $[
    $["image", "res://icon.png"],
] };

STATE.callbacks.on_draw_voxel_structure = {!(sys_id, ent_id) = @;
    !vp = $*vp;
#    std:displayln "LOADDED on_draw_voxel_structure " vp "|" sys_id ent_id;
    vp.clear[];
    !main_vol = vp.new 128 0.0;
    !pattern = on_error { std:displayln "Couldn't load pat.wl: "
                                        @; "{ std:displayln :NOPATERR @; }" } ~
                   sscg:game.read_data_text "pat.wl";
    !fun = std:eval pattern;
    !cm = fun[vp, main_vol];

    .cm =
        (is_bool[cm] &and cm) {
            color_map
        } {
            cm
        };

#    std:displayln "DONE!" $[vp.id[], main_vol, cm];
    $[vp.id[], main_vol, cm]
};

STATE.callbacks.on_saved_godot_state = {!(state) = @;
    std:displayln "STATE:" state;

    on_error {||
        std:displayln "ERROR WRITING SAVEGAME: " @
    } ~ sscg:game.write_savegame "sv1" ${
        version     = 1,
        player      = STATE.player,
        ship        = STATE.ship.save[],
        ship_dyn    = state,
    };
};

STATE.callbacks.on_recall_drone = {||
    std:displayln "RECALLED DRONE IN WLAMBDA!";
    STATE.ship.docked = $f;
};

STATE.callbacks.on_arrived = {!(too_fast, sys_id, ent_id) = @;
    (bool too_fast) {
        STATE.ship.fuel = std:num:floor 0.5 * STATE.ship.fuel
    };

    STATE.ship.docked = $t;
    !ent     = STATE.systems.(sys_id).entities.(ent_id);
    !ent_typ = STATE.entity_types.(ent.t);

    std:displayln "ARRIVED! " ent_typ;

    match ent_typ.gui
        "structure" => { e_structure:show[STATE, ent, ent_typ]; }
        "station"   => { e_station:show  [STATE, ent, ent_typ]; };
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
    sscg:game.gd_call "GUI" :open_window;

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
    } {||
        sscg:win.set_window WID:MAIN_MENU;
        sscg:game.gd_call "GUI" :close_window;
    };
};

!open_credits = {
    !credits = credits:list;

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

!load_save = \:r {
    !state =
        on_error {|| std:displayln "Couldn't load savegame: " @; return :r $n; }
            ~ sscg:game.read_savegame "sv1";
    (bool state) {
        STATE.player = state.player;
        STATE.ship   = e:ship:ship.load(state.ship);
        STATE.code.enumerate_entities[];
        STATE.code.build_color_to_element_index[];
        sscg:game.cmd "load_state" state.ship_dyn;
    };
};

!open_menu = {

    sscg:game.gd_call "GUI" :open_window;
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
            "start"     => { open_start_info[]; }
            "save"      => { sscg:game.cmd "save_state" $n; }
            "credits"   => { open_credits[]; }
            "load"      => { load_save[]; }
            {
                sscg:win.set_window WID:MAIN_MENU;
                sscg:game.gd_call "GUI" :close_window;
            };
    };
};

!count = $&&(
    w_count:new
        $[$[:cred, "Credits:"],
          $[:wust, "Wurst:"]]
        { _1.cred = int[_ * 1000];
          _1.wust = int[_ * 102010] });

STATE.callbacks.on_tick = {!(ship_action_state) = @;
#    count.tick[];

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
        std:str:cat (STATE.ship.cargo.units) " / " ship_type.max_units;
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

    !elements =
        el:read_elements ~
            sscg:game.read_data_text "data/elements.csv";

    !i = $&0;
    !adj_kg_p_m3 = {!(kg/m³, unit/kg) = @;
        !kgpm³  = $&(float[kg/m³]);
        !unit_g = $&(float[unit/kg] * 1000.0);

        while { int[kgpm³] <= 1000 } {
            .kgpm³  = kgpm³         * 1000.0;
            .unit_g = float[unit_g] * 1000.0;
        };

        $[int $*kgpm³, int $*unit_g]
    };
    elements {
        color_map.(i + 1) = _.cpkHexColor;
        !adjusted_weights = adj_kg_p_m3 _.kg/m³ _.kgperunit;
        !good = ${
            short       = _.symbol,
            name        = _.name,
            baseprice   = std:num:round 1000.0 * _.BasePrice,
            mineable    = $true,
            vol_color   = i + 1,
            orig_record = _,
        };
        STATE.good_types.(std:str:cat "element_" _.symbol | std:str:to_lowercase) = good;
        .i = i + 1;
        (i < 20) {
#            std:displayln "EL:" ~ std:ser:json good;
            std:displayln "EL:" (std:str:pad_start 20 " " good.name) "; g/unit=" (std:str:pad_start 10 " " good.unit_g) ", kg/m³=" good.kg_p_m3;
        };
    };
#    std:displayln :ELEMENS ">>" elements "<<" ;

    STATE.code.enumerate_entities[];
    STATE.code.build_color_to_element_index[];
#    open_menu[];
    load_save[];

    std:displayln "READY:" count;
#    count.open[];
};

!@export init = {

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
                ${ t = :field, text = "1.0", w = 1000, bg = c:CON, fg = "000", ref = "foo", },
                ${ t = :field_numeric, text = "1.0", w = 1000, bg = c:CON, fg = "000", ref = "bar",
                    num_default        = 2.0,
                    num_normal_step    = 0.1,
                    num_fine_step      = 0.01,
                    num_very_fine_step = 0.001,
                    num_coarse_step    = 1.0, },
                ${ t = :texture, idx = 0, w = 200, h = 200 },
            ]
        }
    } {||
        match _1
            "menu" => { open_menu[]; };
    };

    std:displayln "DISPLAY INIT: " STATE &> str;

    .*vp = sscg:new_voxel_painter[];
#    std:displayln "VOXPAINT INIT " vp;

#    STATE.ship.cargo.goods.rock = 100;
#    STATE.code.recalc_ship_cargo[];

    STATE
};
