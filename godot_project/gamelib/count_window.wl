!@wlambda;
!@import sscg;
!@import std;
!@import c   colors;
!@import gui gui_common;
!@import WID gui_window_ids;

!new = {!(fields, update_counters) = @;

    !progress = $&&0;
    !field_values = ${};

    !rng = std:rand:split_mix64_new[];

    !self_ref = $&${};
    !self     = $w& $:self_ref;

    self.tick = {
        update_counters float[progress] / 100.0 field_values;
        for field_values \sscg:win.set_label WID:COUNTING _.0 _.1;
        .progress = progress + 1;
    };

    self.open = {
        !heigh_of_one = 1000 / len[fields];
        !panel_rows = fields {!(ref, lbl) = _;
            field_values.(ref) = 0;
            $[
                ${ t = :l_label, fg = c:SE1, bg = "000", h = 1000, w = 500,
                   text = lbl },
                ${ t = :l_label, fg = c:SE1, bg = "000", h = 1000, w = 500,
                   ref = ref, text = "" },
            ]
        };
        std:displayln "ROWS:" panel_rows;

        sscg:game.gd_call "GUI" :open_window;
        gui:dialog_window WID:COUNTING "Counting" {
            panel_rows {!cols = _;
                gui:hpanel heigh_of_one { cols } }
        } {
            sscg:win.set_window WID:COUNTING;
        };

        self.tick[];
    };

    self.on_destroy = std:to_drop {
        sscg:win.set_window WID:COUNTING;
    };

    $:self_ref;
};

!@export new = new;
