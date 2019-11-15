!@import wlambda;
!@import std std;

!:global STATE = ${
    ship = ${
        pos       = $[5000, 2000],
        system_id = 0,
    },
    entity_types = ${
        station = ${ visual = "station" },
    },
    systems = $[
        ${
            name = "Testaris 1",
            entities = $[
                ${ t = "station", pos = $[2000, 2000] },
                ${ t = "station", pos = $[500,  2000] },
                ${ t = "station", pos = $[9000, 4000] },
                ${ t = "station", pos = $[4000, 4000] },
            ],
        }
    ],
};

!@export init {
    std:displayln "DISPLAY INIT";
    STATE
};
