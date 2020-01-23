!@import std std;
!@import wlambda;

!@export game = ${
    read_data_text = {!(filename) = @; std:io:file:read_text ~ std:str:cat "../" filename },
};
