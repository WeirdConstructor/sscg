!@import std std;
!@import wlambda;
!@import u util;

!@export read_elements = {!(data) = @;
    u:table2map_trimmed ~ std:deser:csv ";" "\n" data;
};
