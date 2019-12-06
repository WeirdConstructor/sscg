!@import std std;
!@import wlambda;

!GALAXY_SYSTEM_COUNT = 20;
!GALAXY_SIZE         = 10;

!distribute_poses_accross_galaxy = {!(poses) = @;
    !systems = ${};

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

    systems.list = system_list;

    systems
};


!@export gen_galaxy_at = {!(seed, x, y) = @;
    !hash_seed = std:hash:fnv1a seed x y;
    !rng       = std:rand:split_mix64_new_from hash_seed;
    !sys_rands = std:rand:split_mix64_next rng GALAXY_SYSTEM_COUNT;

    !pos_seed = sys_rands.0;
    !pos_rng  = std:rand:split_mix64_new_from pos_seed;
    !poses    = std:rand:split_mix64_next pos_rng 2 * GALAXY_SYSTEM_COUNT;

    !systems = distribute_poses_accross_galaxy poses;
    std:displayln systems;
};
