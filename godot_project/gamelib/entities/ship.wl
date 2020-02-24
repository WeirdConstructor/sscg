!@import std;
!@import wlambda;

!ship_types = ${
    scout_mk1 = ${
        fuel_capacity       = 1000,
        fuel_per_sec        = 10,
        max_units           = 256,
    },
};

!instanciate = {!(t) = @;
    ${
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
    }
};

!sell_good_units = {!(ship, good_types, good_t, units) = @;
    !avail_units = ship.cargo.goods.(good_t);
    .avail_units = (avail_units < units) { avail_units } { units };

};


!@export default_ship_types = ship_types;
!@export instanciate        = instanciate;
