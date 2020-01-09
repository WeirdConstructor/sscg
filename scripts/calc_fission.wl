!@import elems elements;

!max = {!(a, b) = @;
    (a > b) { a } { b }
};

!min = {!(a, b) = @;
    (a < b) { a } { b }
};

!calc_costs = {!(a, b) = @;
    (a.4 == 1 &or b.4 == 1) { return 0; };

    !p_cost = max[(std:num:abs a.4 - b.4) 0] - 1;
    !g_cost = max[(std:num:abs a.5 - b.5) 0] - 1;
    min 2^p_cost 2^g_cost
};

!elems = elems:elems;

!p3 = { std:str:padl 3 " " _ };
!pe = {
    !e = elems.(_ - 1);
    std:str:cat p3[e.2] "(" p3[e.0] ")"
};


!possible_result_pairs = $[];

elems {!input = _;
    elems {!a = _;
        elems \:fo {!b = _;
            (b.0 < a.0) { return :fo $n; };

            !cost = 2^max[-1, (input.4 - 1) - a.4]
                  + 2^max[-1, (input.4 - 1) - b.4]
                  + 1;
            !sum = cost + a.0 + b.0;
            (     a.0 != b.0
             &and (input.4 <= 2 &or input.4 > a.4)
             &and (input.4 <= 2 &or input.4 > b.4)
             &and sum == input.0) {
                std:push possible_result_pairs $[input, a, b, cost];
            };
        };
        $n
    };
    $n
};

possible_result_pairs {!(i, a, b, cost) = _;
    std:displayln pe[i.0] " => " pe[a.0] " + " pe[b.0] " {" p3[cost] "}";
};
