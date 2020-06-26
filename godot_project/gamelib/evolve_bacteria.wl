!@import u  util;
!@import el elements;


!elements =
    el:read_elements ~
        std:io:file:read_text "../data/elements.csv";
#std:displayln std:ser:json <& elements.90;
#!a = elements.10;
!a = elements.4;
#!a = elements.117;
!pa = a.Period;

!filter_duplicate_elements = {!(list) = @;
    !seen = ${};
    $@v iter c list {
        !id =
            ? c.0.NumberofProtons > c.1.NumberofProtons
                { std:str:cat c.0.NumberofProtons "/" c.1.NumberofProtons }
                { std:str:cat c.1.NumberofProtons "/" c.0.NumberofProtons };
        ? not <& seen.(id) {
            $+ c;
        };

        seen.(id) = $t;
    }
};

!fission_outputs = {!(input_element) = @;
    filter_duplicate_elements ~ $@v iter e1 elements {
        !p1 = e1.Period;
        iter e2 elements {
            !p2 = e2.Period;

            !p_diff = a.NumberofProtons - (e1.NumberofProtons + e2.NumberofProtons);

            ? pa > p1
              &and pa > p2
              &and p_diff > 0
              &and p_diff < 5
              &and e1.NumberofProtons != e2.NumberofProtons {
                $+ e1 => e2;
                std:displayln :* pa p1 p2 a.name "=>" e1.name e2.name "(" p_diff ")";
            };
        };
    };
};

!fusion_candidate = {!(e1) = @;
#    filter_duplicate_elements ~
        $@v iter e2 elements {
            !res_protons = (e1.NumberofProtons + e2.NumberofProtons) - 2;
            !res_elem = elements.(res_protons - 1);
            ?      e1.Group != e2.Group
              &and e1.Period != e2.Period
              &and res_elem &> is_some
              &and res_elem.Period > e1.Period
              &and res_elem.Period > e2.Period
              &and res_protons <= 118 {

                $+ e1 => $p(e2, res_protons);
            }
        };
};

!e_lbl = { std:str:cat _.name "(" _.NumberofProtons ")" };

!candidates = fission_outputs a;
iter c candidates {
    std:displayln e_lbl[a] " => " e_lbl[c.0] "/" e_lbl[c.1];
};

iter c (fusion_candidate a) {
    !rel = elements.(c.1.1 - 1);
    std:displayln e_lbl[c.0] "x" e_lbl[c.1.0] "=>" e_lbl[rel];
};
