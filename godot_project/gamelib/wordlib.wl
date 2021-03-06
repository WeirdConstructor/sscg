!@export particle_set ${
    v = $[ # vocals
        $[1, "a"], $[1, "e"], $[1, "i"], $[1, "o"], $[1, "u"],
    ],
    c = $[ # consonants
        $[10,  "b"], $[10,  "c"], $[10,  "d"], $[10,  "f"], $[10,  "g"],
        $[10,  "h"], $[ 7,  "j"], $[10,  "k"], $[ 8,  "l"], $[10,  "m"],
        $[10,  "n"], $[10,  "p"], $[ 6,  "q"], $[10,  "r"], $[10,  "s"],
        $[10,  "t"], $[ 6,  "v"], $[10,  "w"], $[ 1,  "x"], $[ 1,  "y"],
        $[ 2,  "z"],
    ],
    j = $[ # japanese
        $[1, "ka"], $[1, "ki"], $[1, "ku"], $[1, "ke"], $[1, "ko"],
        $[1, "sa"], $[1, "shi"], $[1, "su"], $[1, "se"], $[1, "so"],
        $[1, "ta"], $[1, "chi"], $[1, "tsu"], $[1, "te"], $[1, "to"],
        $[1, "na"], $[1, "ni"], $[1, "nu"], $[1, "ne"], $[1, "no"],
        $[1, "ha"], $[1, "hi"], $[1, "fu"], $[1, "he"], $[1, "ho"],
        $[1, "ma"], $[1, "mi"], $[1, "mu"], $[1, "me"], $[1, "mo"],
        $[1, "ya"], $[1, "yu"], $[1, "yo"],
        $[1, "ra"], $[1, "ri"], $[1, "ru"], $[1, "re"], $[1, "ro"],
        $[1, "wa"], $[1, "wo"],
    ],
    a = $[ # american name parts
        $[1,   "dan"],
        $[1,   "don"],
        $[1,   "bro"],
        $[1,   "nich"],
        $[1,   "ola"],
        $[1,   "rat"],
        $[1,   "sha"],
        $[1,   "alf"],
        $[1,   "bou"],
        $[1,   "geois"],
        $[1,   "wea"],
        $[1,   "pon"],

        $[1,   "mond"],
        $[1,   "ben"],
        $[1,   "ja"],
        $[1,   "min"],
        $[1,   "crown"],
        $[1,   "shield"],
        $[1,   "has"],
        $[1,   "kell"],
        $[1,   "dud"],
        $[1,   "ley"],
        $[1,   "bald"],
        $[1,   "ridge"],

        $[1,   "jo"],
        $[1,   "seph"],
        $[1,   "par"],
        $[1,   "ker"],
        $[1,   "cox"],
        $[1,   "cau"],
        $[1,   "pet"],
        $[1,   "er"],
        $[1,   "moore"],
        $[1,   "da"],
        $[1,   "vid"],
        $[1,   "will"],
        $[1,   "iam"],
        $[1,   "tem"],
        $[1,   "ple"],
    ],
    g = $[ # german name parts
        $[1,   "mül"],
        $[1,   "ler"],
        $[1,   "sch"],
        $[1,   "midt"],
        $[1,   "nei"],
        $[1,   "der"],
        $[1,   "we"],
        $[1,   "ber"],
        $[1,   "mey"],
        $[1,   "er"],
        $[1,   "beck"],
        $[1,   "hoff"],
        $[1,   "mann"],
        $[1,   "wag"],
        $[1,   "ner"],
        $[1,   "schä"],
        $[1,   "fer"],
        $[1,   "koch"],
        $[1,   "bau"],
        $[1,   "wolf"],
        $[1,   "klein"],
        $[1,   "schrö"],
        $[1,   "warz"],
        $[1,   "rich"],
        $[1,   "ter"],
        $[1,   "mitz"],
        $[1,   "kö"],
        $[1,   "wal"],
        $[1,   "hub"],
        $[1,   "kai"],
        $[1,   "ser"],
        $[1,   "fran"],
        $[1,   "ke"],
        $[1,   "berg"],
        $[1,   "pfei"],
        $[1,   "thom"],
        $[1,   "tho"],
        $[1,   "mas"],
        $[1,   "busch"],
        $[1,   "pohl"],
        $[1,   "ham"],
        $[1,   "mer"],
        $[1,   "graf"],
        $[1,   "diet"],
        $[1,   "rei"],
        $[1,   "hein"],
    ],
    s = $[
        $[1,    "per"],
        $[1,    "rez"],
        $[1,    "gar"],
        $[1,    "cia"],
        $[1,    "lo"],
        $[1,    "pez"],
        $[1,    "san"],
        $[1,    "chez"],
        $[1,    "rodri"],
        $[1,    "guez"],
        $[1,    "mar"],
        $[1,    "tin"],
        $[1,    "diaz"],
        $[1,    "fer"],
        $[1,    "nandez"],
        $[1,    "gon"],
        $[1,    "zalez"],
        $[1,    "del"],
        $[1,    "gado"],
        $[1,    "mar"],
        $[1,    "tinez"],
        $[1,    "lina"],
        $[1,    "rano"],
        $[1,    "ser"],
        $[1,    "gi"],
        $[1,    "menez"],
    ],
};

!@export name_set ${
    a = $[ # assorted
        $[1, "vccvc"],
        $[1, "cvccvc"],
        $[1, "cvcvc"],
        $[1, "cvc"],
        $[1, "cvvc"],
        $[1, "cvccvcvc"],
    ],
    C = $[ # japanese first name
        $[3, "jj"],
        $[2, "jjj"],
    ],
    c = $[ # japanese
        $[2, "jj"],
        $[3, "jjj"],
        $[1, "jjjjj"],
        $[3, "jjjj"],
        $[2, "jjjc"],
    ],
    d = $[ # japanese assorted
        $[1, "jjjvcvc"],
        $[1, "vcvcjjj"],
        $[1, "vcjj"],
        $[1, "jvcj"],
        $[1, "jvvj"],
    ],
    f = $[ # amerijap
        $[1, "ja"],
        $[2, "avc"],
    ],
    b = $[ # american
        $[1, "aa"],
        $[4, "aaa"],
    ],
    g = $[ # german
        $[1, "gg"],
        $[1, "ggg"],
    ],
    s = $[ # spanish
        $[1, "ss"],
        $[1, "ss"],
    ],
};
