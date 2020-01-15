{!(vp, main_vol) = @;
    vp.fill main_vol 0
        0 0 0
        128 128 128
        0.0;
    !seed = 124334;
#    vp.sample_fbm
#        main_vol 0
#        0 0 0
#        128 128 128
#        seed    # seed
#        64      # size
#        1.0     # noise scale
#        2       # octaves
#        1.97    # lacunarity
#        1.05    # gain
#        $[$[:multi_remap,
#            0.0, 0.5, 0.0, 0.5,
#            0.5, 0.7, 0.0, 0.0,
#            0.7, 1.0, 0.5, 1.0,
#        ], $[:map, 0.00001, 1.0, 1.0 / 255.0, 8.0 / 255.0]];
    vp.fill main_vol 0
        60 0 0
        20 128 128
        0.0;
    vp.fill_noise
        main_vol 0
        0 0 0
        128 128 128
        seed    # seed
        4      # size
        1.97    # noise scale
        $[:map,
			0.0000001, 1.0, 0.0 / 255.0, 10.0 / 255.0,
		];
    vp.fill main_vol 0
        60 0 0
        16 128 128
        0.0;
    vp.sample_fbm
        main_vol 0
        0 0 0
        128 128 128
        (seed + 1)   # seed
        64      # size
        1.0     # noise scale
        6       # octaves
        1.94    # lacunarity
        1.05    # gain
        $[:mask_map,
            0.42, 0.7, 0.0, 0.0
        ];
#    vp.fill main_vol 0
#        0 0 0
#        128 1 128
#        1.0 / 255.0;
#    vp.fill main_vol 0
#        0 1 0
#        128 1 128
#        2.0 / 255.0;
#    vp.fill main_vol 0
#        0 2 0
#        128 1 128
#        6.0 / 255.0;
#    vp.fill main_vol 0
#        0 3 0
#        128 1 128
#        8.0 / 255.0;
#    vp.fill main_vol 0
#        0 4 0
#        128 1 128
#        47.0 / 255.0;

    std:displayln "pat.wl2!";
#    "8bit"
    $t
}
