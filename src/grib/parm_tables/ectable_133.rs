use super::Parm;
pub const ECMWF_133: [Parm; 256] = [
    /* 0 */ Parm("var0", "undefined"),
    /* 1 */ Parm("2TPLM10", "2m temperature probability less than -10 C [%]"),
    /* 2 */ Parm("2TPLM5", "2m temperature probability less than -5 C [%]"),
    /* 3 */ Parm("2TPL0", "2m temperature probability less than 0 C [%]"),
    /* 4 */ Parm("2TPL5", "2m temperature probability less than 5 C [%]"),
    /* 5 */ Parm("2TPL10", "2m temperature probability less than 10 C [%]"),
    /* 6 */ Parm("2TPG25", "2m temperature probability greater than 25 C [%]"),
    /* 7 */ Parm("2TPG30", "2m temperature probability greater than 30 C [%]"),
    /* 8 */ Parm("2TPG35", "2m temperature probability greater than 35 C [%]"),
    /* 9 */ Parm("2TPG40", "2m temperature probability greater than 40 C [%]"),
    /* 10 */ Parm("2TPG45", "2m temperature probability greater than 45 C [%]"),
    /* 11 */
    Parm(
        "MN2TPLM10",
        "Minimum 2 metre temperature probability less than -10 C [%]",
    ),
    /* 12 */
    Parm(
        "MN2TPLM5",
        "Minimum 2 metre temperature probability less than -5 C [%]",
    ),
    /* 13 */
    Parm(
        "MN2TPL0",
        "Minimum 2 metre temperature probability less than 0 C [%]",
    ),
    /* 14 */
    Parm(
        "MN2TPL5",
        "Minimum 2 metre temperature probability less than 5 C [%]",
    ),
    /* 15 */
    Parm(
        "MN2TPL10",
        "Minimum 2 metre temperature probability less than 10 C [%]",
    ),
    /* 16 */
    Parm(
        "MX2TPG25",
        "Maximum 2 metre temperature probability greater than 25 C [%]",
    ),
    /* 17 */
    Parm(
        "MX2TPG30",
        "Maximum 2 metre temperature probability greater than 30 C [%]",
    ),
    /* 18 */
    Parm(
        "MX2TPG35",
        "Maximum 2 metre temperature probability greater than 35 C [%]",
    ),
    /* 19 */
    Parm(
        "MX2TPG40",
        "Maximum 2 metre temperature probability greater than 40 C [%]",
    ),
    /* 20 */
    Parm(
        "MX2TPG45",
        "Maximum 2 metre temperature probability greater than 45 C [%]",
    ),
    /* 21 */
    Parm(
        "10SPG10",
        "10 metre wind speed probability of at least 10 m/s [%]",
    ),
    /* 22 */
    Parm(
        "10SPG15",
        "10 metre wind speed probability of at least 15 m/s [%]",
    ),
    /* 23 */
    Parm(
        "10SPG20",
        "10 metre wind speed probability of at least 20 m/s [%]",
    ),
    /* 24 */
    Parm(
        "10SPG35",
        "10 metre wind speed probability of at least 35 m/s [%]",
    ),
    /* 25 */
    Parm(
        "10SPG50",
        "10 metre wind speed probability of at least 50 m/s [%]",
    ),
    /* 26 */
    Parm(
        "10GPG20",
        "10 metre wind gust probability of at least 20 m/s [%]",
    ),
    /* 27 */
    Parm(
        "10GPG35",
        "10 metre wind gust probability of at least 35 m/s [%]",
    ),
    /* 28 */
    Parm(
        "10GPG50",
        "10 metre wind gust probability of at least 50 m/s [%]",
    ),
    /* 29 */
    Parm(
        "10GPG75",
        "10 metre wind gust probability of at least 75 m/s [%]",
    ),
    /* 30 */
    Parm(
        "10GPG100",
        "10 metre wind gust probability of at least 100 m/s [%]",
    ),
    /* 31 */
    Parm(
        "TPPG1",
        "Total precipitation probability of at least 1 mm [%]",
    ),
    /* 32 */
    Parm(
        "TPPG5",
        "Total precipitation probability of at least 5 mm [%]",
    ),
    /* 33 */
    Parm(
        "TPPG10",
        "Total precipitation probability of at least 10 mm [%]",
    ),
    /* 34 */
    Parm(
        "TPPG20",
        "Total precipitation probability of at least 20 mm [%]",
    ),
    /* 35 */
    Parm(
        "TPPG40",
        "Total precipitation probability of at least 40 mm [%]",
    ),
    /* 36 */
    Parm(
        "TPPG60",
        "Total precipitation probability of at least 60 mm [%]",
    ),
    /* 37 */
    Parm(
        "TPPG80",
        "Total precipitation probability of at least 80 mm [%]",
    ),
    /* 38 */
    Parm(
        "TPPG100",
        "Total precipitation probability of at least 100 mm [%]",
    ),
    /* 39 */
    Parm(
        "TPPG150",
        "Total precipitation probability of at least 150 mm [%]",
    ),
    /* 40 */
    Parm(
        "TPPG200",
        "Total precipitation probability of at least 200 mm [%]",
    ),
    /* 41 */
    Parm(
        "TPPG300",
        "Total precipitation probability of at least 300 mm [%]",
    ),
    /* 42 */ Parm("SFPG1", "Snowfall probability of at least 1 mm [%]"),
    /* 43 */ Parm("SFPG5", "Snowfall probability of at least 5 mm [%]"),
    /* 44 */ Parm("SFPG10", "Snowfall probability of at least 10 mm [%]"),
    /* 45 */ Parm("SFPG20", "Snowfall probability of at least 20 mm [%]"),
    /* 46 */ Parm("SFPG40", "Snowfall probability of at least 40 mm [%]"),
    /* 47 */ Parm("SFPG60", "Snowfall probability of at least 60 mm [%]"),
    /* 48 */ Parm("SFPG80", "Snowfall probability of at least 80 mm [%]"),
    /* 49 */ Parm("SFPG100", "Snowfall probability of at least 100 mm [%]"),
    /* 50 */ Parm("SFPG150", "Snowfall probability of at least 150 mm [%]"),
    /* 51 */ Parm("SFPG200", "Snowfall probability of at least 200 mm [%]"),
    /* 52 */ Parm("SFPG300", "Snowfall probability of at least 300 mm [%]"),
    /* 53 */
    Parm(
        "TCCPG10",
        "Total Cloud Cover probability greater than 10% [%]",
    ),
    /* 54 */
    Parm(
        "TCCPG20",
        "Total Cloud Cover probability greater than 20% [%]",
    ),
    /* 55 */
    Parm(
        "TCCPG30",
        "Total Cloud Cover probability greater than 30% [%]",
    ),
    /* 56 */
    Parm(
        "TCCPG40",
        "Total Cloud Cover probability greater than 40% [%]",
    ),
    /* 57 */
    Parm(
        "TCCPG50",
        "Total Cloud Cover probability greater than 50% [%]",
    ),
    /* 58 */
    Parm(
        "TCCPG60",
        "Total Cloud Cover probability greater than 60% [%]",
    ),
    /* 59 */
    Parm(
        "TCCPG70",
        "Total Cloud Cover probability greater than 70% [%]",
    ),
    /* 60 */
    Parm(
        "TCCPG80",
        "Total Cloud Cover probability greater than 80% [%]",
    ),
    /* 61 */
    Parm(
        "TCCPG90",
        "Total Cloud Cover probability greater than 90% [%]",
    ),
    /* 62 */
    Parm(
        "TCCPG99",
        "Total Cloud Cover probability greater than 99% [%]",
    ),
    /* 63 */
    Parm(
        "HCCPG10",
        "High Cloud Cover probability greater than 10% [%]",
    ),
    /* 64 */
    Parm(
        "HCCPG20",
        "High Cloud Cover probability greater than 20% [%]",
    ),
    /* 65 */
    Parm(
        "HCCPG30",
        "High Cloud Cover probability greater than 30% [%]",
    ),
    /* 66 */
    Parm(
        "HCCPG40",
        "High Cloud Cover probability greater than 40% [%]",
    ),
    /* 67 */
    Parm(
        "HCCPG50",
        "High Cloud Cover probability greater than 50% [%]",
    ),
    /* 68 */
    Parm(
        "HCCPG60",
        "High Cloud Cover probability greater than 60% [%]",
    ),
    /* 69 */
    Parm(
        "HCCPG70",
        "High Cloud Cover probability greater than 70% [%]",
    ),
    /* 70 */
    Parm(
        "HCCPG80",
        "High Cloud Cover probability greater than 80% [%]",
    ),
    /* 71 */
    Parm(
        "HCCPG90",
        "High Cloud Cover probability greater than 90% [%]",
    ),
    /* 72 */
    Parm(
        "HCCPG99",
        "High Cloud Cover probability greater than 99% [%]",
    ),
    /* 73 */
    Parm(
        "MCCPG10",
        "Medium Cloud Cover probability greater than 10% [%]",
    ),
    /* 74 */
    Parm(
        "MCCPG20",
        "Medium Cloud Cover probability greater than 20% [%]",
    ),
    /* 75 */
    Parm(
        "MCCPG30",
        "Medium Cloud Cover probability greater than 30% [%]",
    ),
    /* 76 */
    Parm(
        "MCCPG40",
        "Medium Cloud Cover probability greater than 40% [%]",
    ),
    /* 77 */
    Parm(
        "MCCPG50",
        "Medium Cloud Cover probability greater than 50% [%]",
    ),
    /* 78 */
    Parm(
        "MCCPG60",
        "Medium Cloud Cover probability greater than 60% [%]",
    ),
    /* 79 */
    Parm(
        "MCCPG70",
        "Medium Cloud Cover probability greater than 70% [%]",
    ),
    /* 80 */
    Parm(
        "MCCPG80",
        "Medium Cloud Cover probability greater than 80% [%]",
    ),
    /* 81 */
    Parm(
        "MCCPG90",
        "Medium Cloud Cover probability greater than 90% [%]",
    ),
    /* 82 */
    Parm(
        "MCCPG99",
        "Medium Cloud Cover probability greater than 99% [%]",
    ),
    /* 83 */
    Parm(
        "LCCPG10",
        "Low Cloud Cover probability greater than 10% [%]",
    ),
    /* 84 */
    Parm(
        "LCCPG20",
        "Low Cloud Cover probability greater than 20% [%]",
    ),
    /* 85 */
    Parm(
        "LCCPG30",
        "Low Cloud Cover probability greater than 30% [%]",
    ),
    /* 86 */
    Parm(
        "LCCPG40",
        "Low Cloud Cover probability greater than 40% [%]",
    ),
    /* 87 */
    Parm(
        "LCCPG50",
        "Low Cloud Cover probability greater than 50% [%]",
    ),
    /* 88 */
    Parm(
        "LCCPG60",
        "Low Cloud Cover probability greater than 60% [%]",
    ),
    /* 89 */
    Parm(
        "LCCPG70",
        "Low Cloud Cover probability greater than 70% [%]",
    ),
    /* 90 */
    Parm(
        "LCCPG80",
        "Low Cloud Cover probability greater than 80% [%]",
    ),
    /* 91 */
    Parm(
        "LCCPG90",
        "Low Cloud Cover probability greater than 90% [%]",
    ),
    /* 92 */
    Parm(
        "LCCPG99",
        "Low Cloud Cover probability greater than 99% [%]",
    ),
    /* 93 */ Parm("var93", "undefined"),
    /* 94 */ Parm("var94", "undefined"),
    /* 95 */ Parm("var95", "undefined"),
    /* 96 */ Parm("var96", "undefined"),
    /* 97 */ Parm("var97", "undefined"),
    /* 98 */ Parm("var98", "undefined"),
    /* 99 */ Parm("var99", "undefined"),
    /* 100 */ Parm("var100", "undefined"),
    /* 101 */ Parm("var101", "undefined"),
    /* 102 */ Parm("var102", "undefined"),
    /* 103 */ Parm("var103", "undefined"),
    /* 104 */ Parm("var104", "undefined"),
    /* 105 */ Parm("var105", "undefined"),
    /* 106 */ Parm("var106", "undefined"),
    /* 107 */ Parm("var107", "undefined"),
    /* 108 */ Parm("var108", "undefined"),
    /* 109 */ Parm("var109", "undefined"),
    /* 110 */ Parm("var110", "undefined"),
    /* 111 */ Parm("var111", "undefined"),
    /* 112 */ Parm("var112", "undefined"),
    /* 113 */ Parm("var113", "undefined"),
    /* 114 */ Parm("var114", "undefined"),
    /* 115 */ Parm("var115", "undefined"),
    /* 116 */ Parm("var116", "undefined"),
    /* 117 */ Parm("var117", "undefined"),
    /* 118 */ Parm("var118", "undefined"),
    /* 119 */ Parm("var119", "undefined"),
    /* 120 */ Parm("var120", "undefined"),
    /* 121 */ Parm("var121", "undefined"),
    /* 122 */ Parm("var122", "undefined"),
    /* 123 */ Parm("var123", "undefined"),
    /* 124 */ Parm("var124", "undefined"),
    /* 125 */ Parm("var125", "undefined"),
    /* 126 */ Parm("var126", "undefined"),
    /* 127 */ Parm("var127", "undefined"),
    /* 128 */ Parm("var128", "undefined"),
    /* 129 */ Parm("var129", "undefined"),
    /* 130 */ Parm("var130", "undefined"),
    /* 131 */ Parm("var131", "undefined"),
    /* 132 */ Parm("var132", "undefined"),
    /* 133 */ Parm("var133", "undefined"),
    /* 134 */ Parm("var134", "undefined"),
    /* 135 */ Parm("var135", "undefined"),
    /* 136 */ Parm("var136", "undefined"),
    /* 137 */ Parm("var137", "undefined"),
    /* 138 */ Parm("var138", "undefined"),
    /* 139 */ Parm("var139", "undefined"),
    /* 140 */ Parm("var140", "undefined"),
    /* 141 */ Parm("var141", "undefined"),
    /* 142 */ Parm("var142", "undefined"),
    /* 143 */ Parm("var143", "undefined"),
    /* 144 */ Parm("var144", "undefined"),
    /* 145 */ Parm("var145", "undefined"),
    /* 146 */ Parm("var146", "undefined"),
    /* 147 */ Parm("var147", "undefined"),
    /* 148 */ Parm("var148", "undefined"),
    /* 149 */ Parm("var149", "undefined"),
    /* 150 */ Parm("var150", "undefined"),
    /* 151 */ Parm("var151", "undefined"),
    /* 152 */ Parm("var152", "undefined"),
    /* 153 */ Parm("var153", "undefined"),
    /* 154 */ Parm("var154", "undefined"),
    /* 155 */ Parm("var155", "undefined"),
    /* 156 */ Parm("var156", "undefined"),
    /* 157 */ Parm("var157", "undefined"),
    /* 158 */ Parm("var158", "undefined"),
    /* 159 */ Parm("var159", "undefined"),
    /* 160 */ Parm("var160", "undefined"),
    /* 161 */ Parm("var161", "undefined"),
    /* 162 */ Parm("var162", "undefined"),
    /* 163 */ Parm("var163", "undefined"),
    /* 164 */ Parm("var164", "undefined"),
    /* 165 */ Parm("var165", "undefined"),
    /* 166 */ Parm("var166", "undefined"),
    /* 167 */ Parm("var167", "undefined"),
    /* 168 */ Parm("var168", "undefined"),
    /* 169 */ Parm("var169", "undefined"),
    /* 170 */ Parm("var170", "undefined"),
    /* 171 */ Parm("var171", "undefined"),
    /* 172 */ Parm("var172", "undefined"),
    /* 173 */ Parm("var173", "undefined"),
    /* 174 */ Parm("var174", "undefined"),
    /* 175 */ Parm("var175", "undefined"),
    /* 176 */ Parm("var176", "undefined"),
    /* 177 */ Parm("var177", "undefined"),
    /* 178 */ Parm("var178", "undefined"),
    /* 179 */ Parm("var179", "undefined"),
    /* 180 */ Parm("var180", "undefined"),
    /* 181 */ Parm("var181", "undefined"),
    /* 182 */ Parm("var182", "undefined"),
    /* 183 */ Parm("var183", "undefined"),
    /* 184 */ Parm("var184", "undefined"),
    /* 185 */ Parm("var185", "undefined"),
    /* 186 */ Parm("var186", "undefined"),
    /* 187 */ Parm("var187", "undefined"),
    /* 188 */ Parm("var188", "undefined"),
    /* 189 */ Parm("var189", "undefined"),
    /* 190 */ Parm("var190", "undefined"),
    /* 191 */ Parm("var191", "undefined"),
    /* 192 */ Parm("var192", "undefined"),
    /* 193 */ Parm("var193", "undefined"),
    /* 194 */ Parm("var194", "undefined"),
    /* 195 */ Parm("var195", "undefined"),
    /* 196 */ Parm("var196", "undefined"),
    /* 197 */ Parm("var197", "undefined"),
    /* 198 */ Parm("var198", "undefined"),
    /* 199 */ Parm("var199", "undefined"),
    /* 200 */ Parm("var200", "undefined"),
    /* 201 */ Parm("var201", "undefined"),
    /* 202 */ Parm("var202", "undefined"),
    /* 203 */ Parm("var203", "undefined"),
    /* 204 */ Parm("var204", "undefined"),
    /* 205 */ Parm("var205", "undefined"),
    /* 206 */ Parm("var206", "undefined"),
    /* 207 */ Parm("var207", "undefined"),
    /* 208 */ Parm("var208", "undefined"),
    /* 209 */ Parm("var209", "undefined"),
    /* 210 */ Parm("var210", "undefined"),
    /* 211 */ Parm("var211", "undefined"),
    /* 212 */ Parm("var212", "undefined"),
    /* 213 */ Parm("var213", "undefined"),
    /* 214 */ Parm("var214", "undefined"),
    /* 215 */ Parm("var215", "undefined"),
    /* 216 */ Parm("var216", "undefined"),
    /* 217 */ Parm("var217", "undefined"),
    /* 218 */ Parm("var218", "undefined"),
    /* 219 */ Parm("var219", "undefined"),
    /* 220 */ Parm("var220", "undefined"),
    /* 221 */ Parm("var221", "undefined"),
    /* 222 */ Parm("var222", "undefined"),
    /* 223 */ Parm("var223", "undefined"),
    /* 224 */ Parm("var224", "undefined"),
    /* 225 */ Parm("var225", "undefined"),
    /* 226 */ Parm("var226", "undefined"),
    /* 227 */ Parm("var227", "undefined"),
    /* 228 */ Parm("var228", "undefined"),
    /* 229 */ Parm("var229", "undefined"),
    /* 230 */ Parm("var230", "undefined"),
    /* 231 */ Parm("var231", "undefined"),
    /* 232 */ Parm("var232", "undefined"),
    /* 233 */ Parm("var233", "undefined"),
    /* 234 */ Parm("var234", "undefined"),
    /* 235 */ Parm("var235", "undefined"),
    /* 236 */ Parm("var236", "undefined"),
    /* 237 */ Parm("var237", "undefined"),
    /* 238 */ Parm("var238", "undefined"),
    /* 239 */ Parm("var239", "undefined"),
    /* 240 */ Parm("var240", "undefined"),
    /* 241 */ Parm("var241", "undefined"),
    /* 242 */ Parm("var242", "undefined"),
    /* 243 */ Parm("var243", "undefined"),
    /* 244 */ Parm("var244", "undefined"),
    /* 245 */ Parm("var245", "undefined"),
    /* 246 */ Parm("var246", "undefined"),
    /* 247 */ Parm("var247", "undefined"),
    /* 248 */ Parm("var248", "undefined"),
    /* 249 */ Parm("var249", "undefined"),
    /* 250 */ Parm("var250", "undefined"),
    /* 251 */ Parm("var251", "undefined"),
    /* 252 */ Parm("var252", "undefined"),
    /* 253 */ Parm("var253", "undefined"),
    /* 254 */ Parm("var254", "undefined"),
    /* 255 */ Parm("var255", "undefined"),
];
