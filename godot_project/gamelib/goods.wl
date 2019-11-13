!goods = ${
    rock = ${
        name         = "Unidentified Rock",
        kg_m3        = 1.7,
        unit_kg      = 10.0,
        base_price   = 20,
    },
};

!calc_cargo_bay_status = {!(cargobay) = @;
    !kg_sum = $&0.0;
    !m3_sum = $&0.0;

    cargobay.goods {!(kg, good_type) = @;
        .kg     = float kg;
        .kg_sum = kg_sum + kg;
        .m3_sum = m3_sum + kg / goods.(good_type).kg_m3;
    };

    $[kg_sum,
      m3_sum,
      cargobay.limits.weight_kg - kg_sum,
      cargobay.limits.volume_m3 - m3_sum]
};

# TODO: FIXME: Write a function that calculates the units that fit
#              into the bay. Leave fitting to the client code!
!load_until_full = {!(cargobay, good_type, ) = @;
    !bay_status = calc_cargo_bay_status cargobay;

    !good_kg_m3 = goods.(good_type).kg_m3;
    !mass_m3 = float[mass_kg] / good_kg_m3;

    (     mass_kg < bay_status.2
     &and mass_m3 < bay_status.3) {

        cargobay.goods.(good_type) =
            mass_kg + cargobay.goods.(good_type);
        $true
    } {
        !bay_free_m3_in_kg = bay_status.3 * good_kg_m3;
        !min_kg_free =
            (bay_free_m3_in_kg < bay_status.2)
                { bay_free_m3_in_kg }
                { bay_status.2 };

    }
};

!@export goods           goods;
!@export load_until_full load_until_full;
