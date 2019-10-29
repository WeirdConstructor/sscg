!@import sscg sscg;
!@import wlambda;
!@import std std;

!ticks_per_hour  = 1;
!ticks_per_day   = ticks_per_hour * 24;
!ticks_per_month = ticks_per_day * 30;
!ticks_per_year  = ticks_per_day * 365;


!time->str = {!(time) = @;
    !year       = (time / ticks_per_year) + 2170;
    !rest_ticks = time - ((time / ticks_per_year) * ticks_per_year);
    !month      = rest_ticks / ticks_per_month;
    .rest_ticks = rest_ticks - month * ticks_per_month;
    !day        = rest_ticks / ticks_per_day;
    .rest_ticks = rest_ticks - day * ticks_per_day;
    !hour       = rest_ticks / ticks_per_hour;
    !min        = (rest_ticks - (hour * ticks_per_hour)) * 10;

    std:str:cat
        year
        "-" (std:str:padl 2 "0" (month + 1))
        "-" (std:str:padl 2 "0" (day + 1))
        " " (std:str:padl 2 "0" hour)
        ":" (std:str:padl 2 "0" min)
};

!@export tick { sscg:game.clock = sscg:game.clock + 1; };
!@export now_str { time->str sscg:game.clock };
!@export time->str time->str;
