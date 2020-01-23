!@import u util;

!tests = $[];
!add_test = { std:push tests $[_, _1]; };

add_test :strip_ws {
    !r = u:strip_ws " f foe feo f ";
    std:assert_eq r "ffoefeof";
};

add_test :test_table2map_trimmed {
    !table = $[$["A", "B", " X Y "], $[" 3 232", "foof", "foo"]];
    !map = u:table2map_trimmed table;
    std:assert_eq map.0.A "3 232";
    std:assert_eq map.0.B "foof";
    std:assert_eq map.0.XY "foo";
    std:assert_eq map.0.("") $n;
};

tests {
    _.1[];
    std:displayln "OK - " _.0;
};
