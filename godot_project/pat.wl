{!(vp, main_vol) = @;
    vp.fill main_vol 0
        0 0 0
        128 128 128
        0.0;
    !seed = 12;
    vp.sample_fbm
        main_vol 0
        0 0 0
        16 16 16
        seed    # seed
        16      # size
        1.0     # noise scale
        2       # octaves
        1.97    # lacunarity
        1.05;   # gain
    vp.fill_noise
        main_vol 0
        16 16 16
        16 16 16
        seed    # seed
        4      # size
        1.97    # noise scale
        ;
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
}
