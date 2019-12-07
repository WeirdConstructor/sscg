!@import std std;
!@import wlambda;

!GALAXY_SYSTEM_COUNT = 20;
!GALAXY_SIZE         = 10;

!distribute_poses_accross_galaxy = {!(poses) = @;
    !systems = ${};

    # Generate the GALAXY_SYSTEM_COUNT indices in the systems map:
    range 0 ((GALAXY_SYSTEM_COUNT * 2) - 1) 2 {
        !idx = (std:num:abs poses.(_)) % (GALAXY_SIZE ^ 2);
        !out_idx = range 0 99 1 {
            !try_idx = (idx + _) % (GALAXY_SIZE ^ 2);
            (not systems.(try_idx))
                \break try_idx;
        };
        std:displayln out_idx;
        systems.(out_idx) = $t;
    };

    # Annotate the systems map with x, y positions:
    !system_list = $[];
    range 0 (GALAXY_SIZE ^ 2) - 1 1 {!idx = _;
        !x = idx % GALAXY_SIZE;
        !y = idx / GALAXY_SIZE;
        (bool systems.(idx)) {
            std:displayln x ":" y;
            !item = $[x, y, idx];
            std:push system_list item;
            systems.(idx) = item;
        };
    };

    systems.list = std:sort { std:cmp:num:asc _.2 _1.2 } system_list;

    systems
};


!@export gen_galaxy_at = {!(seed, x, y) = @;
    !hash_seed = std:hash:fnv1a seed x y;
    !rng       = std:rand:split_mix64_new_from hash_seed;
    !sys_rands = std:rand:split_mix64_next rng GALAXY_SYSTEM_COUNT;

    !pos_seed        = sys_rands.0;
    !list_order_seed = sys_rands.1;
    !pos_rng         = std:rand:split_mix64_new_from pos_seed;
    !list_order_rng  = std:rand:split_mix64_new_from list_order_seed;
    !poses    = std:rand:split_mix64_next pos_rng 2 * GALAXY_SYSTEM_COUNT;

    !systems = distribute_poses_accross_galaxy poses;
    std:shuffle { std:rand:split_mix64_next list_order_rng } systems.list;
    std:displayln systems;
    std:displayln systems.list;

    systems.edges = ${};

    # Calculate the distances between the systems and find the closest ones:
    for systems.list {
        !sys = _;
        !close_sys =
            (std:sort { std:cmp:num:asc _.1 _1.1 }
                ~ systems.list {
                    $[_.2, std:num:sqrt
                         (  (sys.0 - _.0) ^ 2
                          + (sys.1 - _.1) ^ 2)]
                }) { _.0 };
        sys.3 = close_sys;
        std:displayln sys;
    };
    std:displayln systems;

    # - For each node take the 4? closest ones and make a connection.
    # - Make it possible to jump anywhere, but if there is no direct
    #   connection make the price a higher by a factor that depends
    #   on the distance to the destination system.
};
