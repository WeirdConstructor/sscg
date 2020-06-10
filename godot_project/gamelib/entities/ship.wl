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

!observable = ${
    reg_event = {!(event, cb) = @;
        ? not <& $s.events { $s.events = ${} };
        ? not <& $s.events.(event) {
            $s.events.(event) = $[];
        };
        std:push $s.events.(event) cb;
    },
    event = {!(event) = @;
        !cbs = $s.events.(event);
        std:displayln "EVENT: " cbs;
        !args = @;
        std:unshift args $s;
        iter cb cbs { cb[[args]] };
    },
};

!ship = ${
    _proto = observable,
    new = {!(t) = @;
        ${
            _proto = $self,
            _type = ship_types.(t),
            _data = ${
                t               = t,
                system_id       = 0,
                docked          = $f,
                engine_on_secs  = 0,
                fuel            = ship_types.(t).fuel_capacity,
                cargo           = ${
                    units       = 0,
                    goods       = ${},
                },
            },
        }
    },
    load = {!(data) = @;
        !self = ${
            _proto = $self,
            _type  = ship_types.(data.t),
            _data  = data,
        };
        self.recalc_cargo[];
        self
    },
    save = { $d },
    loaded = { $s.recalc_cargo[]; },
    get_fuel_gui_str = {
        std:str:cat $d.fuel " / " $s._type.fuel_capacity
    },
    get_refuel_amount = {
        $s._type.fuel_capacity - $d.fuel
    },
    calc_fuel_usage = {
        !fuel_usage_factor =
            1.0
            + ($d.cargo.units * $s._type.max_units_fuel_factor)
              / $s._type.max_units;

        !engine_on_delta =
            $s.action_state.engine_on_secs - $s.engine_on_secs;
        $s.engine_on_secs = $s.action_state.engine_on_secs;

        $d.fuel = int ~
            $d.fuel
            - (fuel_usage_factor * $s._type.fuel_per_sec * engine_on_delta);

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
    store_good_unit = {!(good_t, units) = @;
        std:displayln "STORE:" good_t units;
        $d.cargo.goods.(good_t) =
            $d.cargo.goods.(good_t) + units;
        $s.recalc_cargo[];
    },
    get_cargo_units         = { $d.cargo.units },
    get_cargo_units_gui_str = { std:str:cat $s.get_cargo_units[] " / " $s._type.max_units  },
    get_free_units          = { $s._type.max_units - $d.cargo.units },
    get_cargo_percent_full = {
        (100 * $s.get_cargo_units[]) / $s._type.max_units
    },
    # TODO: Test this code!
    recalc_cargo = {
        !cargo = $d.cargo;
        cargo.units = $@int iter c cargo.goods { $+ c.0 };
        $s.event :recalc_cargo;
    },
};


!@export default_ship_types = ship_types;
!@export ship               = ship;
