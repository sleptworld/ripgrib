use super::Parm;
pub const DWD_201: [Parm; 256] = [
    /* 0 */ Parm("var0", "undefined"),
    /* 1 */
    Parm(
        "dw sw flux",
        "downward shortwave radiant flux density [W/m**2]",
    ),
    /* 2 */
    Parm(
        "uw sw flux",
        "upward shortwave radiant flux density [W/m**2]",
    ),
    /* 3 */
    Parm(
        "dw lw flux",
        "downward longwave radiant flux density [W/m**2]",
    ),
    /* 4 */
    Parm(
        "uw lw flux",
        "upward longwave radiant flux density [W/m**2]",
    ),
    /* 5 */
    Parm(
        "APAB_S",
        "downwd photosynthetic active radiant flux density [W/m**2]",
    ),
    /* 6 */ Parm("net s flux", "net shortwave flux [W/m**2]"),
    /* 7 */ Parm("net l flux", "net longwave flux [W/m**2]"),
    /* 8 */ Parm("net flux", "total net radiative flux density [W/m**2]"),
    /* 9 */
    Parm(
        "dw sw clfr",
        "downw shortw radiant flux density, cloudfree part [W/m**2]",
    ),
    /* 10 */
    Parm(
        "uw sw cldy",
        "upw shortw radiant flux density, cloudy part [W/m**2]",
    ),
    /* 11 */
    Parm(
        "dw lw clfr",
        "downw longw radiant flux density, cloudfree part [W/m**2]",
    ),
    /* 12 */
    Parm(
        "uw lw cldy",
        "upw longw radiant flux density, cloudy part [W/m**2]",
    ),
    /* 13 */ Parm("SOHR_RAD", "shortwave radiative heating rate [K/s]"),
    /* 14 */ Parm("THHR_RAD", "longwave radiative heating rate [K/s]"),
    /* 15 */ Parm("rad heat", "total radiative heating rate [K/s]"),
    /* 16 */ Parm("soilheat S", "soil heat flux, surface [W/m**2]"),
    /* 17 */ Parm("soilheat L", "soil heat flux, bottom of layer [W/m**2]"),
    /* 18 */ Parm("var18", "undefined"),
    /* 19 */ Parm("var19", "undefined"),
    /* 20 */ Parm("var20", "undefined"),
    /* 21 */ Parm("var21", "undefined"),
    /* 22 */ Parm("var22", "undefined"),
    /* 23 */ Parm("var23", "undefined"),
    /* 24 */ Parm("var24", "undefined"),
    /* 25 */ Parm("var25", "undefined"),
    /* 26 */ Parm("var26", "undefined"),
    /* 27 */ Parm("var27", "undefined"),
    /* 28 */ Parm("var28", "undefined"),
    /* 29 */ Parm("CLC", "cloud cover, grid scale + convective [1]"),
    /* 30 */ Parm("clc gr sc", "cloud cover, grid scale  (0...1) [1]"),
    /* 31 */ Parm("QC", "specific cloud water content, grid scale [kg/kg]"),
    /* 32 */
    Parm(
        "clw gs vi",
        "cloud water content, grid scale, vert integrated [kg/m**2]",
    ),
    /* 33 */ Parm("QI", "specific cloud ice content, grid scale [kg/kg]"),
    /* 34 */
    Parm(
        "cli gs vi",
        "cloud ice content, grid scale, vert integrated [kg/m**2]",
    ),
    /* 35 */ Parm("QR", "specific rainwater content, grid scale [kg/kg]"),
    /* 36 */ Parm("QS", "specific snow content, grid scale [kg/kg]"),
    /* 37 */
    Parm(
        "src gs vi",
        "specific rainwater content, gs, vert. integrated [kg/m**2]",
    ),
    /* 38 */
    Parm(
        "ssc gs vi",
        "specific snow content, gs, vert. integrated [kg/m**2]",
    ),
    /* 39 */ Parm("QG", "specific graupel content, grid scale [kg/kg]"),
    /* 40 */ Parm("var40", "undefined"),
    /* 41 */
    Parm(
        "TWATER",
        "vert. integral of humidity, cloud water (and ice) [kg/(m**2)]",
    ),
    /* 42 */
    Parm(
        "TDIV_HUM",
        "vert. integral of divergence of tot. water content [kg/(m**2)]",
    ),
    /* 43 */ Parm("var43", "undefined"),
    /* 44 */ Parm("var44", "undefined"),
    /* 45 */ Parm("var45", "undefined"),
    /* 46 */ Parm("var46", "undefined"),
    /* 47 */ Parm("var47", "undefined"),
    /* 48 */ Parm("var48", "undefined"),
    /* 49 */ Parm("var49", "undefined"),
    /* 50 */ Parm("CH_CM_CL", "cloud covers CH_CM_CL (000...888) [1]"),
    /* 51 */ Parm("cl cov. CH", "cloud cover CH (0..8) [1]"),
    /* 52 */ Parm("cl cov. CM", "cloud cover CM (0..8) [1]"),
    /* 53 */ Parm("cl cov. CL", "cloud cover CL (0..8) [1]"),
    /* 54 */ Parm("cloud cov.", "total cloud cover (0..8) [1]"),
    /* 55 */ Parm("fog", "fog (0..8) [1]"),
    /* 56 */ Parm("fog", "fog [1]"),
    /* 57 */ Parm("var57", "undefined"),
    /* 58 */ Parm("var58", "undefined"),
    /* 59 */ Parm("var59", "undefined"),
    /* 60 */ Parm("clc con ci", "cloud cover, convective cirrus  (0...1) [1]"),
    /* 61 */
    Parm(
        "CLW_CON",
        "specific cloud water content, convective clouds [kg/kg]",
    ),
    /* 62 */
    Parm(
        "clw con vi",
        "cloud water content, conv clouds, vert integrated [kg/m**2]",
    ),
    /* 63 */
    Parm(
        "cli con",
        "specific cloud ice content, convective clouds [kg/kg]",
    ),
    /* 64 */
    Parm(
        "cli con vi",
        "cloud ice content, conv clouds, vert integrated [kg/m**2]",
    ),
    /* 65 */ Parm("mass fl co", "convective mass flux [kg/(s*m**2)]"),
    /* 66 */ Parm("upd vel co", "updraft velocity, convection [m/s]"),
    /* 67 */ Parm("entr p co", "entrainment parameter, convection [m**(-1)]"),
    /* 68 */ Parm("HBAS_CON", "cloud base, convective clouds (above msl) [m]"),
    /* 69 */ Parm("HTOP_CON", "cloud top, convective clouds (above msl) [m]"),
    /* 70 */ Parm("con layers", "convective layers (00...77)  (BKE) [1]"),
    /* 71 */ Parm("KO-index", "KO-index [1]"),
    /* 72 */ Parm("BAS_CON", "convection base index [1]"),
    /* 73 */ Parm("TOP_CON", "convection top index [1]"),
    /* 74 */ Parm("DT_CON", "convective temperature tendency [K/s]"),
    /* 75 */
    Parm(
        "DQV_CON",
        "convective tendency of specific humidity [s**(-1)]",
    ),
    /* 76 */ Parm("H ten co", "convective tendency of total heat [J/(kg*s)]"),
    /* 77 */ Parm("QDW ten co", "convective tendency of total water [s**(-1)]"),
    /* 78 */
    Parm(
        "DU_CON",
        "convective momentum tendency (X-component) [m/s**2]",
    ),
    /* 79 */
    Parm(
        "DV_CON",
        "convective momentum tendency (Y-component) [m/s**2]",
    ),
    /* 80 */ Parm("vor ten co", "convective vorticity tendency [s**(-2)]"),
    /* 81 */ Parm("div ten co", "convective divergence tendency [s**(-2)]"),
    /* 82 */ Parm("HTOP_DC", "top of dry convection (above msl) [m]"),
    /* 83 */ Parm("top ind dc", "dry convection top index [1]"),
    /* 84 */
    Parm(
        "HZEROCL",
        "height of 0 degree Celsius isotherm above msl [m]",
    ),
    /* 85 */ Parm("SNOWLMT", "height of snowfall limit above msl [m]"),
    /* 86 */ Parm("var86", "undefined"),
    /* 87 */ Parm("var87", "undefined"),
    /* 88 */ Parm("var88", "undefined"),
    /* 89 */ Parm("var89", "undefined"),
    /* 90 */ Parm("var90", "undefined"),
    /* 91 */ Parm("var91", "undefined"),
    /* 92 */ Parm("var92", "undefined"),
    /* 93 */ Parm("var93", "undefined"),
    /* 94 */ Parm("var94", "undefined"),
    /* 95 */ Parm("var95", "undefined"),
    /* 96 */ Parm("var96", "undefined"),
    /* 97 */ Parm("var97", "undefined"),
    /* 98 */ Parm("var98", "undefined"),
    /* 99 */
    Parm(
        "QRS_GSP",
        "spec water cont of rain/snow needed for w loading [kg/kg]",
    ),
    /* 100 */
    Parm(
        "PRR_GSP",
        "surface precipitation rate, rain, grid scale [kg/(s*m**2)]",
    ),
    /* 101 */
    Parm(
        "PRS_GSP",
        "surface precipitation rate, snow, grid scale [kg/(s*m**2)]",
    ),
    /* 102 */
    Parm(
        "RAIN_GSP",
        "surface precipitation amount, rain, grid scale [kg/m**2]",
    ),
    /* 103 */ Parm("condens gs", "condensation rate, grid scale [kg/(kg*s)]"),
    /* 104 */
    Parm(
        "autocon gs",
        "autoconversion rate, grid scale   (C+C  --> R) [kg/(kg*s)]",
    ),
    /* 105 */
    Parm(
        "accret gs",
        "accretion rate, grid scale        (R+C  --> R) [kg/(kg*s)]",
    ),
    /* 106 */
    Parm(
        "nucleat gs",
        "nucleation rate, grid scale       (C+C  --> S) [kg/(kg*s)]",
    ),
    /* 107 */
    Parm(
        "riming gs",
        "riming rate, grid scale           (S+C  --> S) [kg/(kg*s)]",
    ),
    /* 108 */
    Parm(
        "deposit gs",
        "deposition rate, grid scale       (S+V <--> S) [kg/(kg*s)]",
    ),
    /* 109 */
    Parm(
        "melting gs",
        "melting rate, grid scale          (S    --> R) [kg/(kg*s)]",
    ),
    /* 110 */
    Parm(
        "evapor gs",
        "evaporation rate, grid scale      (R+V <--  R) [kg/(kg*s)]",
    ),
    /* 111 */
    Parm(
        "PRR_CON",
        "surface precipitation rate, rain, convective [kg/(s*m**2)]",
    ),
    /* 112 */
    Parm(
        "PRS_CON",
        "surface precipitation rate, snow, convective [kg/(s*m**2)]",
    ),
    /* 113 */
    Parm(
        "RAIN_CON",
        "surface precipitation amount, rain, convective [kg/m**2]",
    ),
    /* 114 */ Parm("condens co", "condensation rate, convective [kg/(kg*s)]"),
    /* 115 */ Parm("autocon co", "autoconversion rate, convective [kg/(kg*s)]"),
    /* 116 */ Parm("accret co", "accretion rate, convective [kg/(kg*s)]"),
    /* 117 */ Parm("nucleat co", "nucleation rate, convective [kg/(kg*s)]"),
    /* 118 */ Parm("riming co", "riming rate, convective [kg/(kg*s)]"),
    /* 119 */ Parm("sublim co", "sublimation rate, convective [kg/(kg*s)]"),
    /* 120 */ Parm("melting co", "melting rate, convective [kg/(kg*s)]"),
    /* 121 */ Parm("evapor co", "evaporation rate, convective [kg/(kg*s)]"),
    /* 122 */
    Parm(
        "rain am",
        "rain amount, grid-scale plus convective [kg/m**2]",
    ),
    /* 123 */
    Parm(
        "snow am",
        "snow amount, grid-scale plus convective [kg/m**2]",
    ),
    /* 124 */
    Parm(
        "DT_GSP",
        "temperature tendency, grid-scale condensation [K/s]",
    ),
    /* 125 */
    Parm(
        "DQV_GSP",
        "tendency of specific humidity, grid-scale condens [s**(-1)]",
    ),
    /* 126 */
    Parm(
        "H ten gs",
        "tendency of total heat, grid-scale condensation [J/(kg*s)]",
    ),
    /* 127 */
    Parm(
        "DQC_GSP",
        "tendency of total water, grid-scale condensation [s**(-1)]",
    ),
    /* 128 */ Parm("snowfall", "snowfall  (dimension"),
    /* 129 */ Parm("FRESHSNW", "fresh snow factor [1]"),
    /* 130 */
    Parm(
        "DQI_GSP",
        "tend of the sp cl ice cont due to gs precipitation [kg/(kg*s)]",
    ),
    /* 131 */
    Parm(
        "PRG_GSP",
        "surface precipitation rate, graupel, grid scale [kg/(s*m**2)]",
    ),
    /* 132 */
    Parm(
        "GRAU_GSP",
        "surface precipitation amount, graupel, grid scale [kg/(m**2)]",
    ),
    /* 133 */ Parm("RHO_SNOW", "snow density [kg/m**3"),
    /* 134 */ Parm("var134", "undefined"),
    /* 135 */ Parm("var135", "undefined"),
    /* 136 */ Parm("var136", "undefined"),
    /* 137 */ Parm("var137", "undefined"),
    /* 138 */ Parm("var138", "undefined"),
    /* 139 */ Parm("PP", "deviation of pressure from reference value [Pa]"),
    /* 140 */ Parm("var140", "undefined"),
    /* 141 */ Parm("var141", "undefined"),
    /* 142 */ Parm("var142", "undefined"),
    /* 143 */ Parm("var143", "undefined"),
    /* 144 */ Parm("var144", "undefined"),
    /* 145 */ Parm("var145", "undefined"),
    /* 146 */ Parm("var146", "undefined"),
    /* 147 */ Parm("var147", "undefined"),
    /* 148 */ Parm("var148", "undefined"),
    /* 149 */ Parm("KE", "kinetic energy ((u**2 + v**2) / 2) [(m**2/s**2)]"),
    /* 150 */ Parm("hdi coeff", "coefficient of horizontal diffusion [m**2/s]"),
    /* 151 */ Parm("dissp rate", "dissipation rate [W/(Pa*m**2)]"),
    /* 152 */ Parm("TKE", "turbulent kinetic energy [(m/s)**2]"),
    /* 153 */
    Parm(
        "TKVM",
        "coefficient of vertical diffusion, momentum [m**2/s]",
    ),
    /* 154 */ Parm("TKVH", "coefficient of vertical diffusion, heat [m**2/s]"),
    /* 155 */
    Parm(
        "vdi coe cw",
        "coefficient of vertical diffusion, cloud water [m**2/s]",
    ),
    /* 156 */
    Parm(
        "vdi coe ci",
        "coefficient of vertical diffusion, cloud ice [m**2/s]",
    ),
    /* 157 */
    Parm(
        "vdi coe vp",
        "coefficient of vertical diffusion, water vapour [m**2/s]",
    ),
    /* 158 */ Parm("dis len m", "turbulent dissipation length for momentum [m]"),
    /* 159 */ Parm("dis len h", "turbulent dissipation length for heat [m]"),
    /* 160 */
    Parm(
        "var u mom",
        "variance of u-component of momentum [(m/s)**2]",
    ),
    /* 161 */
    Parm(
        "var v mom",
        "variance of v-component of momentum [(m/s)**2]",
    ),
    /* 162 */
    Parm(
        "var w mom",
        "variance of w-component of momentum [(m/s)**2]",
    ),
    /* 163 */ Parm("var temp", "variance of temperature [K**2]"),
    /* 164 */
    Parm(
        "var cl wat",
        "variance of specific cloud water content [(kg/kg)**2]",
    ),
    /* 165 */
    Parm(
        "var cl ice",
        "variance of specific cloud ice content [(kg/kg)**2]",
    ),
    /* 166 */
    Parm(
        "var vap mr",
        "variance of water vapour mixing ratio [(kg/kg)**2]",
    ),
    /* 167 */
    Parm(
        "c wat flux",
        "turbulent vertical flux of spec cloud water [m/s]",
    ),
    /* 168 */
    Parm(
        "c ice flux",
        "turbulent vertical flux of spec cloud ice [m/s]",
    ),
    /* 169 */
    Parm(
        "w vap flux",
        "turbulent vertical flux of water vapour mix ratio [m/s]",
    ),
    /* 170 */ Parm("TCM", "drag coefficient CD [1]"),
    /* 171 */ Parm("TCH", "transfer coefficient CH (sensible heat) [1]"),
    /* 172 */ Parm("tr coef CQ", "transfer coefficient CQ (latent heat) [1]"),
    /* 173 */ Parm("PBL-top h", "PBL-top h [m]"),
    /* 174 */ Parm("T-jump  h", "temperature jump at PBL-top [K]"),
    /* 175 */ Parm("q-jump  h", "specific humidity jump at PBL-top [kg/kg]"),
    /* 176 */ Parm("entr at h", "entrainment at PBL-top [kg/(s*m**2)]"),
    /* 177 */ Parm("mass fl h", "upward mass flux at PBL-top [kg/(s*m**2)]"),
    /* 178 */ Parm("cl cov PBL", "cloud cover of PBL-clouds (0...1) [1]"),
    /* 179 */
    Parm(
        "cl wat PBL",
        "specific cloud water content of PBL-clouds [kg/kg]",
    ),
    /* 180 */ Parm("cl top PBL", "cloud top of PBL-clouds [m]"),
    /* 181 */ Parm("cl bas PBL", "cloud base of PBL-clouds [m]"),
    /* 182 */
    Parm(
        "moun wav X",
        "vertical mountain wave momentum flux (X component) [kg/(m*s**2)]",
    ),
    /* 183 */
    Parm(
        "moun wav Y",
        "vertical mountain wave momentum flux (Y component) [kg/(m*s**2)]",
    ),
    /* 184 */ Parm("wave Ri", "wave Richardson number [1]"),
    /* 185 */
    Parm(
        "wav div X",
        "mountain wave momentum flux divergence (X comp) [m/s**2]",
    ),
    /* 186 */
    Parm(
        "wav div Y",
        "mountain wave momentum flux divergence (Y comp) [m/s**2]",
    ),
    /* 187 */ Parm("VMAX_10M", "maximum wind velocity [m/s]"),
    /* 188 */
    Parm(
        "wav dis vi",
        "mountain wave dissipation, vert integrated [W/m**2]",
    ),
    /* 189 */ Parm("wv en flux", "vertical wave energy flux [kg*m/s**4]"),
    /* 190 */ Parm("var190", "undefined"),
    /* 191 */ Parm("var191", "undefined"),
    /* 192 */ Parm("var192", "undefined"),
    /* 193 */ Parm("var193", "undefined"),
    /* 194 */ Parm("var194", "undefined"),
    /* 195 */ Parm("var195", "undefined"),
    /* 196 */ Parm("var196", "undefined"),
    /* 197 */ Parm("T_SO", "temperature of soil layers [K]"),
    /* 198 */ Parm("W_SO", "water + ice content of soil layers [kg/(m**2)]"),
    /* 199 */ Parm("W_SO_ICE", "ice content of soil layers [kg/(m**2)]"),
    /* 200 */ Parm("W_I", "water content of interception store [kg/(m**2)]"),
    /* 201 */ Parm("interc ice", "icebit for interception store [1]"),
    /* 202 */ Parm("snow fract", "snow fraction [1]"),
    /* 203 */ Parm("T_SNOW", "snow temperature [K]"),
    /* 204 */ Parm("foliag tem", "foliage temperature [K]"),
    /* 205 */ Parm("infiltrat", "infiltration [m/s]"),
    /* 206 */ Parm("runoff", "runoff [m/s]"),
    /* 207 */ Parm("soil evap", "bare soil evaporation [m/s]"),
    /* 208 */ Parm("plant tran", "plant transpiration [m/s]"),
    /* 209 */ Parm("inter evap", "interception store evaporation [m/s]"),
    /* 210 */ Parm("water evap", "evaporation from water surfaces [m/s]"),
    /* 211 */ Parm("aero resis", "aerodynamic resistance [s/m]"),
    /* 212 */ Parm("plant res", "plant resistance [s/m]"),
    /* 213 */ Parm("soil res", "soil resistance [s/m]"),
    /* 214 */
    Parm(
        "total evap",
        "total evaporation (water, soil, plants) [m/s]",
    ),
    /* 215 */ Parm("T_ICE", "temperature of sea ice [K]"),
    /* 216 */ Parm("var216", "undefined"),
    /* 217 */ Parm("max wind m", "maximum wind velocity (modified) [m/s]"),
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
    /* 230 */ Parm("XYZ", "S1 [1]"),
    /* 231 */ Parm("RHS_SI", "S2 [1]"),
    /* 232 */ Parm("DTTDIV", "S3 [1]"),
    /* 233 */ Parm("SOTR_RAD", "effective transmissivity of solar rad. [1]"),
    /* 234 */ Parm("GEN_TEN1", "averaged tendencies [x/s]"),
    /* 235 */ Parm("GEN_TEN2", "averaged tendencies [x/s]"),
    /* 236 */ Parm("S7", "S7 [1]"),
    /* 237 */ Parm("S8", "S8 [1]"),
    /* 238 */ Parm("S9", "S9 [1]"),
    /* 239 */ Parm("S10", "S10 [1]"),
    /* 240 */ Parm("MFLX_CON", "cloud base mass flux kg/(s*m**2)"),
    /* 241 */ Parm("CAPE_CON", "convective available potential energy [J/kg]"),
    /* 242 */
    Parm(
        "QCVG_CON",
        "moisture convergence for Kuo-type closure [1/s]",
    ),
    /* 243 */ Parm("TKE_CON", "convective turbulent energy [J/kg]"),
    /* 244 */
    Parm(
        "MOS pTS fq",
        "MOS Gewitter-Wahrscheinlichkeit (frequent) [1]",
    ),
    /* 245 */
    Parm(
        "MOS TS cov",
        "MOS Gewitteranteil (occasional - frequent (1 - 2)) [1]",
    ),
    /* 246 */ Parm("S17", "S17 [1]"),
    /* 247 */ Parm("S18", "S18 [1]"),
    /* 248 */ Parm("S19", "S19 [1]"),
    /* 249 */ Parm("S20", "S20 [1]"),
    /* 250 */
    Parm(
        "MOS TSISO1",
        "MOS Wahrscheinlichkeit mindestens ein Blitz [1]",
    ),
    /* 251 */
    Parm(
        "MOS TSISO2",
        "MOS Wahrscheinlichkeit mindestens zehn Blitze [1]",
    ),
    /* 252 */
    Parm(
        "MOS TSISO3",
        "MOS Wahrscheinlichkeit mindestens hundert Blitze [1]",
    ),
    /* 253 */ Parm("MOS TS DEN", "MOS Vorhersage der Blitzanzahl [1]"),
    /* 254 */
    Parm(
        "MOS TS OCC",
        "MOS Gewitter-Wahrscheinlichkeit (occasional) [1]",
    ),
    /* 255 */
    Parm(
        "MOS TS FRQ",
        "MOS Gewitter-Wahrscheinlichkeit (frequent) [1]",
    ),
];
