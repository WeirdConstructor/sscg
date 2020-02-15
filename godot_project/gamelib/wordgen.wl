!@import std std;
!@import wlambda;

!@export gen {!(input, weighted_word_set, rng_num_gen) = @;
    std:str:join "" ~
        input \:next{
            !key = _;
            !weighted_list = weighted_word_set.(key);
            (is_none weighted_list) { return :next key; };

            !weight_sum = $&0;
            weighted_list { .weight_sum = weight_sum + _.0; };

            !sel_weight = $&(std:num:ceil ~ rng_num_gen[] * $*weight_sum);
            !out = \:r { weighted_list {!(weighted_elem) = @;
                .sel_weight = sel_weight - weighted_elem.0;
                (sel_weight <= 0) { return :r weighted_elem.1; };
                weighted_elem.1
            } }[];
            out
        }
};
