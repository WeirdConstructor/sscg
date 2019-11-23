!@import sscg sscg;
!@import wlambda;
!@import std std;
!@import c          colors;

!@export ml_l_vtext {!(w, h, fg, lines) = @;
    !per_line = 1000 / len[lines];
    ${ t = :vbox, w = w, h = h, childs =
        lines {
            ${ t = :l_text, text = _,
               w = 1000, h = per_line,
               fg = fg, bg = "000" }
        }
    };
};

