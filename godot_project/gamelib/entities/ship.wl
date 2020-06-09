!@import std;
!@import wlambda;

!ship_types = ${
    scout_mk1 = ${
        fuel_capacity           = 1000,
        fuel_per_sec            = 10,
        max_units               = 256,
        max_units_fuel_factor   = 5.0,
    },
};

!ship = ${
    new = {!(t) = @;
        ${
            _proto = $self,
            _data = ${
                t               = t,
                system_id       = 0,
                docked          = $f,
                engine_on_secs  = 0,
                fuel            = ship_types.(t).fuel_capacity,
                cargo           = ${
                    fuel_factor = 0,
                    units       = 0,
                    goods       = ${},
                },
            },
        }
    },
    load = {!(data) = @;
        !self = ${ _proto = $self, _data = data };
        self.recalc_cargo[];
        self
    },
    save = { $d },
    get_fuel_gui_str = {
        std:str:cat $d.fuel " / " ship_types.($d.t).fuel_capacity
    },
    get_refuel_amount = {
        ship_types.($d.t).fuel_capacity - $d.fuel
    },
    calc_fuel_usage = {
        !ship_type = ship_types.($d.t);

        !fuel_usage_factor =
            1.0
            + ($d.cargo.units * ship_type.max_units_fuel_factor)
              / ship_type.max_units;

        !engine_on_delta =
            $s.action_state.engine_on_secs - $s.engine_on_secs;
        $s.engine_on_secs = $s.action_state.engine_on_secs;

        $d.fuel = int ~
            $d.fuel
            - (fuel_usage_factor * ship_type.fuel_per_sec * engine_on_delta);

        ? $d.fuel <= 0 {
            $d.fuel = 0;
        };
    },
    is_fuel_empty = { $d.fuel <= 0 },
    set_action_state = { $s.action_state = _; },
    get_speed_i = {
        !speed_i = std:num:ceil ~ 1000.0 * $s.action_state.speed;
        .speed_i = speed_i >= 100 { str speed_i } { std:str:cat "(docking) " speed_i };
        $s.speed_i = speed_i;
        $s.speed_i
    },
    get_fuel_usage_factor = { $s.fuel_usage_factor },
    sell_good_units = {!(good_types, good_t, units) = @;
        !good_type = good_types.(good_t);
        !avail_units = $d.cargo.goods.(good_t);
        .avail_units = (avail_units < units) { avail_units } { units };

        !credits = good_type.baseprice * avail_units;
        $d.cargo.goods.(good_t) = $n;
        $s.recalc_cargo[];
        $[avail_units, credits]
    },
    get_cargo_units = { $d.cargo.units },
    # TODO: Test this code!
    recalc_cargo = {
        !cargo = $d.cargo;
        cargo.units       = 0;
        cargo.fuel_factor = 0;
    },
};


!@export default_ship_types = ship_types;
!@export ship               = ship;
