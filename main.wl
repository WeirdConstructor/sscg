
!x = $&0;
std:displayln "GAMESTATE: " game;
${

init = {!(ship) = @;
    std:displayln "INIT GAME";
    !sys = game :add_system 0 0 ${};
    game :add_entity sys 20  20  ${ type = :station };
    game :add_entity sys 300 300 ${ type = :station };
    game :add_entity sys 200 100 ${ type = :asteroid_field };
    ship :set_system sys;
},

ship_entity_tick = {
    std:displayln "SHIP ENT TICK" @ game;
    .x = x + 1;
    _ :set_notification ~ std:str:cat "ARR" $*x;
},
ship_tick = {
    _ :set_notification "";
    std:displayln "SHIP TICK" @;
#    std:displayln "SHIP TICK" _;
#    std:displayln "SHIP TICK" (_ "foo");
#    _.ticky = 1 + _.ticky;
#    std:displayln "SHIP SYS " (:system_id _) "; " (_.ticky);
},
ship_arrived = {
    std:displayln "ARRIVED " @;
},
system_tick = {
    std:displayln "SYS TICK" @;
},

}