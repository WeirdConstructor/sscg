!@import sscg sscg;
!@import wlambda;
!@import std std;
!@import c   colors;
!@import gui gui_common;
!@import WID gui_window_ids;

!calc_refuel = {!(STATE) = @;
    !cc_per_fuelunit = 125;

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
        std:str:cat STATE.player.credits " available credits",
        std:str:cat refuel.fuel_delta " fuel units",
        std:str:cat "= " refuel.price " credits",
    ]
};

!show = $&&$n;
.*show = {!(STATE, ent, ent_type) = @;
    gui:window WID:STATION {
        $[ent.name, $[
            gui:hpanel 700 { $[
                gui:action_button 500 1000 :refuel "Refuel",
                refuel_text STATE,
            ] },
            gui:hpanel 300 { $[
                gui:action_button 500 1000 :sell "Sell",
                gui:button        500 1000 :depart "Depart",
            ] }
        ] ]
    } ${
        refuel = {||
            !refuel = calc_refuel STATE;
            STATE.ship.fuel = STATE.ship.fuel + refuel.fuel_delta;
            STATE.player.credits = STATE.player.credits - refuel.price;
        },
        sell = {||
            STATE.code.sell_all_ship_cargo[];
        },
    } {
        STATE.ship.docked = $f;
    };
};

!@export show show;
