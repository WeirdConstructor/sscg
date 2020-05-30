!@wlambda;
!@import std;
!@import sscg;

!@export dialog_yes_no {!(id, title, text, yes_lbl, no_lbl, cb) = @;
    sscg:win :set_window id ${
        title = title,
        x = 250,
        y = 100,
        w = 500,
        h = 500,
        child = ${
            t = "vbox",
            w = 1000,
            h = 1000,
            childs = $[
                ${ t = "vbox",
                   h = 600,
                   w = 1000,
                   childs = $[
                    ${ t    = "c_text",
                       h    = 880,
                       w    = 1000,
                       fg   = "FFF",
                       bg   = "333",
                       text = text },
                ], },
                ${ t       = "vbox",
                   spacing = 5,
                   h       = 380,
                   w       = 300,
                   childs = $[
                    ${ t    = "r_button",
                       ref  = "yes_btn",
                       bg   = "8e8",
                       fg   = "000",
                       h    = 1000,
                       w    = 1000,
                       text = yes_lbl },
                    ${ t    = "r_button",
                       ref  = "no_btn",
                       bg   = "e88",
                       fg   = "000",
                       h    = 1000,
                       w    = 1000,
                       text = no_lbl },
                ], },
            ]
        }
    } {!(id, ref) = @;

        std:displayln "STATION ACTION" _;
        match ref
            "yes_btn"  {|| cb $t; }
            "no_btn" {||
                sscg:win :set_window id $n $n;
                cb $f;
            };
    };
};



