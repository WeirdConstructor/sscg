!@import std;
!@import wlambda;

!ship_types = ${
    scout_mk1 = ${
        fuel_capacity       = 1000,
        fuel_per_sec        = 10,
        max_units           = 256,
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
#    !fuel_usage_factor =
#        (STATE.ship.cargo.kg * ship_type.max_kg_fuel_factor)
#        / ship_type.cargo_max_kg;
#    .fuel_usage_factor = fuel_usage_factor + 100;
#
#    STATE.ship.fuel =
#        STATE.ship.fuel
#        - (fuel_usage_factor * ship_type.fuel_per_sec * engine_on_delta) / 100;
#    (STATE.ship.fuel <= 0) {
#        display_fuel_out_warning[];
#        STATE.ship.fuel = 0;
#    };
	},
	set_action_state = { $s.action_state = _; },
	get_speed_i = {
		!speed_i = std:num:ceil ~ 1000.0 * $s.action_state.speed;
		.speed_i = speed_i >= 100 { str speed_i } { std:str:cat "(docking) " speed_i };
		$s.speed_i = speed_i;
		$s.speed_i
	},
	get_fuel_usage_factor = {
		10
	},
    sell_good_units = {!(good_types, good_t, units) = @;
        !good_type = good_types.(good_t);
        !avail_units = $d.cargo.goods.(good_t);
        .avail_units = (avail_units < units) { avail_units } { units };

        !credits = good_type.baseprice * avail_units;
        $d.cargo.goods.(good_t) = $n;
        $s.recalc_cargo[];
        $[avail_units, credits]
    },
    # TODO: Test this code!
    recalc_cargo = {
        !cargo = $d.cargo;
        cargo.units       = 0;
        cargo.fuel_factor = 0;
    },
};


!@export default_ship_types = ship_types;
!@export ship               = ship;
