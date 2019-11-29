!@import sscg sscg;
!@import wlambda;
!@import std std;
!@import c          colors;

!@export ml_l_vtext = {!(w, h, fg, lines) = @;
    !per_line = 1000 / len[lines];
    ${ t = :vbox, w = w, h = h, childs =
        lines {
            ${ t = :l_text, text = _,
               w = 1000, h = per_line,
               fg = fg, bg = "000" }
        }
    };
};

!@export dialog_window = {!(wid, title, child_fun, ev_cb) = @;
    sscg:win.set_window wid ${
        x = 250, y = 250, w = 500, h = 500,
        title       = title,
        title_color = c:PRI_L,
        child       = ${
            t       = :vbox,
            w       = 1000,
            h       = 1000,
            spacing = 5,
            childs  = child_fun[],
        },
    } ev_cb;
};

!@export hpanel = {!(h, child_cb) = @;
    ${
        t            = "hbox",
        border       = 1,
        border_color = c:SE1_D2,
        margin       = 5,
        w            = 1000,
        h            = h,
        spacing      = 10,
        childs       = $[
            ${ t       = "hbox",
               h       = 1000,
               w       = 1000,
               margin  = 5,
               spacing = 5,
               childs  = child_cb[] },
        ]
    }
};

!@export button = {!(w, h, ref, text) = @;
    ${ t = :l_button, text = text, ref = ref,
       w = w, h = h, fg = "000", bg = c:SE2 }
};

!@export action_button = {!(w, h, ref, text) = @;
    ${ t = :r_button, text = text, ref = ref,
       w = w, h = h, fg = "000", bg = c:SE1 }
};
