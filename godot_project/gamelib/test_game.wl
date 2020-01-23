!@import u util;
!@import sscg game_mockup;
!@import el   elements;

!tests = $[];
!add_test = { std:push tests $[_, _1]; };

add_test :reading_elements_table {
    !elements =
        el:read_elements ~
            sscg:game.read_data_text "elements.csv";

    std:assert_eq (len elements) 118                "number of elements is right";
    std:assert_eq elements.2.symbol "Li"            "got the right elements";
    std:assert_eq elements.8.cpkHexColor "90E050"   "color was not messed up by librecalc";
};

tests {
    _.1[];
    std:displayln "OK - " _.0;
};
