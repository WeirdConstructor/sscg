!@import sscg sscg;
!@import wlambda;
!@import std std;
!@import c   colors;
!@import gui gui_common;
!@import WID gui_window_ids;

!new = {
    !count = $&&0;

    std:displayln "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    !self = $&${};
    self.tick = {
        .*count = $*count + 1;
        self.update[];
        $*count
    };
    self.open = {
        sscg:game.gd_call "GUI" :open_window;
        gui:dialog_window WID:COUNTING "Counting" { $[
            gui:hpanel 1000 {
                $[
                    ${ t = :l_label, fg = c:SE1, bg = "000", h = 1000, w = 500,
                       text = "Count:" },
                    ${ t = :l_label, fg = c:SE1, bg = "000", h = 1000, w = 500,
                       ref = :cnt_lbl,
                       text = str $*count },
                ]
            },
        ] } {
            sscg:win.set_window WID:COUNTING;
        };
    };
    self.update = {
        sscg:win.set_label WID:COUNTING :cnt_lbl count;
    };

    self.del = std:to_drop $true {
        std:displayln "DROPPED COUNTER WINDOW";
    std:displayln "YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYy";
        gui:dialog_window WID:COUNTING;
    };

    std:strengthen self;
};

!@export new = new;
