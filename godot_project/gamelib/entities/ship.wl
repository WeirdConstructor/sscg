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
