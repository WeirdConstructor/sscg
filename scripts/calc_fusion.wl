!@import elems elements;

!elems = elems:elems;

!max = {!(a, b) = @;
    (a > b) { a } { b }
};

!min = {!(a, b) = @;
    (a < b) { a } { b }
};

!calc_costs = {!(a, b) = @;
    (a.4 == 1 &or b.4 == 1) { return 0; };

    !p_cost = (max (std:num:abs a.4 - b.4) 0) - 1;
    !g_cost = (max (std:num:abs a.5 - b.5) 0) - 1;
    min 2^p_cost 2^g_cost
};

!p3 = { std:str:padl 3 " " _ };
!pe = {
    !e = elems.(_ - 1);
    std:str:cat p3[e.2] "(" p3[e.0] ")"
};

!calc_products = {
    elems {!(a) = @;
        elems {!(b) = @;
            !cost = calc_costs a b;
            !res  = a.0 + b.0 - cost;
            std:displayln pe[a.0] " * " pe[b.0] "{" p3[cost] "}" "=>" pe[res];
        };
    };
};

calc_products[];
