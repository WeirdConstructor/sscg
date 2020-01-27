!@import sscg sscg;
!@import wlambda;
!@import std std;
!@import c   colors;
!@import gui gui_common;
!@import WID gui_window_ids;

!new = {
    !count = $&&0;

    !rng = std:rand:split_mix64_new[];

    !self = $&${};
    self.tick = {
        !digits = $[];
        range 0 9 1 {||
            std:push digits ~
                std:num:floor ~
                    1.0 + std:rand:split_mix64_next_open01[rng] * 9.0
        };
        .*count = std:str:cat[[digits]];
        self.update[];
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
        !nums = 
        sscg:win.set_label WID:COUNTING :cnt_lbl $*count;
    };

    self.on_destroy = std:to_drop $true {
        sscg:win.set_window WID:COUNTING;
    };

    std:strengthen self;
};

!@export new = new;
