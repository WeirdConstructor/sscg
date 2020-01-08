!map = $[];
!size = 5;

!init_map = {
    range 0 size 1 {||
        !o = $[];
        range 0 size 1 {||
            std:push o ${ t = " ", s = " ", };
        };
        std:push map o;
    };
};

!print_map = {!(map) = @;
    !sep = $&"";
    range 0 (len[map.0] - 1) 1 {|| .sep = sep "---"; };
    std:displayln $*sep;
    std:displayln ~ std:str:join "\n" (map {!(row) = @;
        std:str:join " " (row { !cell = _; std:str:cat cell.t cell.s });
    });
};

!m = init_map[];
m.0.1.t = "@";
m.1.1.t = "v";
m.2.1.t = "v";
m.3.1.t = ">";
m.3.2.t = ">";
m.0.3.t = "@";
m.1.3.t = "v";
m.2.3.t = "v";
m.3.3.t = ">";
#m.3.4.t = "o";
m.3.3.t = "o";
m.3.4.t = "@";
m.4.3.t = "@";
print_map m;

!rand_gen = std:rand:split_mix64_new_from "9939";

!dir_cell = {!(map, x, y, dir) = @;
    !pos = $[x, y];

    match dir
        "v" {|| pos.1 = pos.1 + 1; }
        "^" {|| pos.1 = pos.1 - 1; }
        "<" {|| pos.0 = pos.0 - 1; }
        ">" {|| pos.0 = pos.0 + 1; };
    (pos.1 < 0)           { return $n; };
    (pos.1 > (len map))   { return $n; };
    (pos.0 < 0)           { return $n; };
    (pos.0 > (len map.0)) { return $n; };

    $[pos.0, pos.1, map.(pos.1).(pos.0)]
};

!sim_step = {!(map, cell_step, stack) = @;
    !(x, y, cell) = cell_step;


    !get_neighbors = {
        !neighbors = $[];
        !up    = dir_cell map x y "^";
        !down  = dir_cell map x y "v";
        !left  = dir_cell map x y "<";
        !right = dir_cell map x y ">";
        (up    != $n) { std:push neighbors up; };
        (down  != $n) { std:push neighbors down; };
        (left  != $n) { std:push neighbors left; };
        (right != $n) { std:push neighbors right; };

        (len[neighbors] == 0) { return neighbors; };

        range 0 (cell.gen % 3) 1 {||
            !el = std:pop neighbors;
            std:unshift neighbors el;
        };

        neighbors
    };

    !neighbour_cell_move = {!(dir) = @;
        !neigh_cell = dir_cell map x y dir;

        (neigh_cell.2.s == " ") {
            neigh_cell.2.s = cell.s;
            cell.s = " ";
        };

        !neighbors = get_neighbors[];
        !ok = {|| $t };
        neighbors {!(ncell) = @;
            (match ncell.2.t "@" ok "<" ok ">" ok "^" ok "v" ok) {
                std:unshift stack ncell;
            }
        }
    };

    match cell.t
        "@" {||
            !neighbors = get_neighbors[];

            !current_elem = $&(str cell.s);

            block :map {
                neighbors {!(ncell) = @;
                    (ncell.2.t != " " &and ncell.2.s == " ") {
                        ncell.2.s = current_elem;
                        .current_elem = " ";
                        return :map $n;
                    };
                };
            };

            ($*current_elem == " ") {
                cell.s = (std:rand:split_mix64_next rand_gen) % 10 | std:num:abs;
            } {
                cell.s = current_elem;
            };
        }
        "^" neighbour_cell_move
        "<" neighbour_cell_move
        ">" neighbour_cell_move
        "v" neighbour_cell_move
        "o" {||
            (cell.s != " ") {
                std:displayln "CONSUME[" cell.s "]";
                cell.s = " ";
            };

            !neighbors = get_neighbors[];
            neighbors {!(ncell) = @;
                std:unshift stack ncell;
            };
        };
};

!for_each_cell = {!(map, fn) = @;
    range 0 10 1 {!y = _;
        range 0 10 1 {!x = _;
            fn x y map.(y).(x);
        };
    };
};

!simulate = {!(map) = @;
    !this_gen = $&0;
    !stack = $[];
    for_each_cell map {!(x, y, cell) = @;
        cell.t == "o" {
            std:push stack $[x, y, cell];
            .this_gen = (cell.gen == $n) { 0 } { cell.gen };
        }
    };

    .*this_gen = $*this_gen + 1;

    std:displayln :source_sim;

    while { len[stack] > 0 } {
        !cell_step = std:pop stack;
        (cell_step.2.gen == this_gen) { next[]; };

        sim_step map cell_step stack;
        cell_step.2.gen = this_gen;
    };

    std:displayln :dummy_sim;

    !dummy_stack = $[];
    for_each_cell map {!(x, y, cell) = @;
        (cell.gen != this_gen) {
            sim_step map $[x, y, cell] dummy_stack;
            cell.gen = this_gen;
        };
    };
};

simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
simulate m; print_map m;
