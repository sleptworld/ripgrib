use super::Parm;
pub const DWD_002: [Parm; 256] = [
    /* 0 */ Parm("var0", "undefined"),
    /* 1 */ Parm("PS", "pressure [Pa]"),
    /* 2 */ Parm("PMSL", "pressure reduced to MSL [Pa]"),
    /* 3 */ Parm("DPSDT", "pressure tendency [Pa/s]"),
    /* 4 */ Parm("var4", "undefined"),
    /* 5 */ Parm("var5", "undefined"),
    /* 6 */ Parm("FI", "geopotential [(m**2)/(s**2)]"),
    /* 7 */ Parm("geopot h", "geopotential height [gpm]"),
    /* 8 */ Parm("HH", "geometrical height [m]"),
    /* 9 */ Parm("dev of h", "standard deviation of height [m]"),
    /* 10 */ Parm("TO3", "total ozone [Dobson Units]"),
    /* 11 */ Parm("T", "temperature [K]"),
    /* 12 */ Parm("virt.temp.", "virtual temperature [K]"),
    /* 13 */ Parm("pot. temp.", "potential temperature [K]"),
    /* 14 */ Parm("pseudo-pot", "pseudo-adiabatic potential temperature [K]"),
    /* 15 */ Parm("TMAX", "maximum temperature [K]"),
    /* 16 */ Parm("TMIN", "minimum temperature [K]"),
    /* 17 */ Parm("TD", "dew-point temperature [K]"),
    /* 18 */ Parm("dew-pnt de", "dew-point depression (or deficit) [K]"),
    /* 19 */ Parm("lapse rate", "laps rate [K/m]"),
    /* 20 */ Parm("visibility", "visibility [m]"),
    /* 21 */ Parm("radar sp 1", "radar spectra (1) [non-dim]"),
    /* 22 */ Parm("radar sp 2", "radar spectra (2) [non-dim]"),
    /* 23 */ Parm("radar sp 3", "radar spectra (3) [non-dim]"),
    /* 24 */ Parm("pli to 500", "parcel lifted index (to 500 hPa) [K]"),
    /* 25 */ Parm("temp anom", "temperature anomaly [K]"),
    /* 26 */ Parm("pres anom", "pressure anomaly [Pa]"),
    /* 27 */ Parm("geop anom", "geopotential height anomaly [gpm]"),
    /* 28 */ Parm("wave sp 1", "wave spaectra(1) [non-dim]"),
    /* 29 */ Parm("wave sp 2", "wave spaectra(2) [non-dim]"),
    /* 30 */ Parm("wave sp 3", "wave spaectra(3) [non-dim]"),
    /* 31 */ Parm("DD", "wind direction [degree true]"),
    /* 32 */ Parm("FF", "wind speed [m/s]"),
    /* 33 */ Parm("U", "u-component (zonal) of wind [m/s]"),
    /* 34 */ Parm("V", "v-component (merdional) of wind [m/s]"),
    /* 35 */ Parm("stream fun", "stream function [(m**2)/s]"),
    /* 36 */ Parm("vel potent", "velocity potential [(m**2)/s]"),
    /* 37 */ Parm("M.stream f", "Montgomery stream function [(m**2)/(s**2)]"),
    /* 38 */ Parm("sigma vert", "sigma co-ordinate vertical velocity [1/s]"),
    /* 39 */ Parm("OMEGA", "vertical velocity [Pa/s]"),
    /* 40 */ Parm("W", "vertical velocity [m/s]"),
    /* 41 */ Parm("abs vortic", "absolute vorticity [1/s]"),
    /* 42 */ Parm("abs diverg", "absolute divergence [1/s]"),
    /* 43 */ Parm("rel vortic", "relative vorticity [1/s]"),
    /* 44 */ Parm("rel diverg", "relative divergence [1/s]"),
    /* 45 */ Parm("vert.u-shr", "vertical u-component shear [1/s]"),
    /* 46 */ Parm("vert.v-shr", "vertical v-component shear [1/s]"),
    /* 47 */ Parm("dir of cur", "direction of current [degree true]"),
    /* 48 */ Parm("spd of cur", "speed of current [m/s]"),
    /* 49 */ Parm("currcomp U", "u-component of current [m/s]"),
    /* 50 */ Parm("currcomp V", "v-component of current [m/s]"),
    /* 51 */ Parm("QV", "specific humidity [kg/kg]"),
    /* 52 */ Parm("RELHUM", "relative humidity [%]"),
    /* 53 */ Parm("hum mixrat", "humidity mixing ratio [kg/kg]"),
    /* 54 */ Parm("TQV", "total precipitable water [kg/m**2]"),
    /* 55 */ Parm("vapor pres", "vapor pressure [Pa]"),
    /* 56 */ Parm("sat.defic.", "saturation deficit [Pa]"),
    /* 57 */ Parm("AEVAP_S", "evaporation [kg/(m**2)]"),
    /* 58 */ Parm("TQI", "total cloud ice content [kg/m**2]"),
    /* 59 */ Parm("prec. rate", "precipitation rate [kg/((m**2)*s)]"),
    /* 60 */ Parm("thunderst.", "thunderstorm probability [%]"),
    /* 61 */ Parm("TOT_PREC", "total precipitation [kg/(m**2)]"),
    /* 62 */ Parm("PREC_GSP", "large scale precipitation [kg/(m**2)]"),
    /* 63 */ Parm("PREC_CON", "convective precipitation [kg/(m**2)]"),
    /* 64 */
    Parm(
        "snowf.rate",
        "snowfall rate water equivalent [kg/((m**2)*s)]",
    ),
    /* 65 */
    Parm(
        "W_SNOW",
        "water equivalent of accumulated snow depth [kg/(m**2)]",
    ),
    /* 66 */ Parm("H_SNOW", "snow depth [m]"),
    /* 67 */ Parm("mix lay de", "mixed layer depth [m]"),
    /* 68 */ Parm("tr therm d", "transient thermocline depth [m]"),
    /* 69 */ Parm("ma therm d", "main thermocline depth [m]"),
    /* 70 */ Parm("m therm da", "main thermocline depth anomaly [m]"),
    /* 71 */ Parm("CLCT", "total cloud cover [%]"),
    /* 72 */ Parm("CLC_CON", "convective cloud cover [%]"),
    /* 73 */ Parm("CLCL", "low cloud cover [%]"),
    /* 74 */ Parm("CLCM", "medium cloud cover [%]"),
    /* 75 */ Parm("CLCH", "high cloud cover [%]"),
    /* 76 */ Parm("TQC", "total cloud water content [kg/m**2]"),
    /* 77 */ Parm("bli to 500", "best lifted index (to 500 hPa) [K]"),
    /* 78 */ Parm("SNOW_CON", "convective snow [kg/(m**2)]"),
    /* 79 */ Parm("SNOW_GSP", "large scale snow [kg/(m**2)]"),
    /* 80 */ Parm("water temp", "water temperature [K]"),
    /* 81 */ Parm("FR_LAND", "land cover (1=land, 0=sea) [1]"),
    /* 82 */ Parm("dev sea-le", "deviation of sea-level from mean [m]"),
    /* 83 */ Parm("Z0", "surface roughness [m]"),
    /* 84 */ Parm("ALB_RAD", "albedo [%]"),
    /* 85 */ Parm("T_soil", "soil temperature [K]"),
    /* 86 */ Parm("W_soil", "soil moisture content [kg/(m**2)]"),
    /* 87 */ Parm("PLCOV", "vegetation (plant cover) [%]"),
    /* 88 */ Parm("salinity", "salinity [kg/kg]"),
    /* 89 */ Parm("density", "density [kg/(m**3)]"),
    /* 90 */ Parm("RUNOFF", "water run-off [kg/(m**2)]"),
    /* 91 */ Parm("FR_ICE", "ice cover (1=ice, 0=no ice) [1]"),
    /* 92 */ Parm("H_ICE", "ice thickness [m]"),
    /* 93 */ Parm("dir ice dr", "direction of ice drift [degree true]"),
    /* 94 */ Parm("sp ice dr", "speed of ice drift [m/s]"),
    /* 95 */ Parm("ice dr u", "u-component of ice drift [m/s]"),
    /* 96 */ Parm("ice dr v", "v-component of ice drift [m/s]"),
    /* 97 */ Parm("ice growth", "ice growth rate [m/s]"),
    /* 98 */ Parm("ice diverg", "ice divergence [1/s]"),
    /* 99 */ Parm("snow melt", "snow melt [kg/(m**2)]"),
    /* 100 */
    Parm(
        "winwav/swe",
        "significant height of comb. wind waves and swell [m]",
    ),
    /* 101 */ Parm("dir of wav", "direction of wind waves [degree true]"),
    /* 102 */ Parm("hei of wav", "significant height of wind waves [m]"),
    /* 103 */ Parm("MP of wiwa", "mean period of wind waves [s]"),
    /* 104 */ Parm("dir of swe", "direction of swell [degree true]"),
    /* 105 */ Parm("hei of swe", "significant height of swell [m]"),
    /* 106 */ Parm("MP of swel", "mean period of swell [s]"),
    /* 107 */ Parm("pr wave di", "primary wave direction [degree true]"),
    /* 108 */ Parm("pr wave pe", "primary wave period [s]"),
    /* 109 */ Parm("se wave di", "secondary wave direction [degree true]"),
    /* 110 */ Parm("se wave pe", "secondary wave period [s]"),
    /* 111 */ Parm("ASOB_S", "net short-wave radiation (surface) [W/(m**2)]"),
    /* 112 */ Parm("ATHB_S", "net long-wave radiation (surface) [W/(m**2)]"),
    /* 113 */
    Parm(
        "ASOB_T",
        "net short-wave radiation (top of atmosphere) [W/(m**2)]",
    ),
    /* 114 */
    Parm(
        "ATHB_T",
        "net long-wave radiation (top of atmosphere) [W/(m**2)]",
    ),
    /* 115 */ Parm("l-w rad.", "long-wave radiation [W/(m**2)]"),
    /* 116 */ Parm("s-w rad.", "short-wave radiation [W/(m**2)]"),
    /* 117 */ Parm("global rad", "global radiation [W/(m**2)]"),
    /* 118 */ Parm("var118", "undefined"),
    /* 119 */ Parm("var119", "undefined"),
    /* 120 */ Parm("var120", "undefined"),
    /* 121 */ Parm("ALHFL_S", "latent heat flux [W/(m**2)]"),
    /* 122 */ Parm("ASHFL_S", "sensible heat flux [W/(m**2)]"),
    /* 123 */ Parm("bound l di", "boundary layer dissipation [W/(m**2)]"),
    /* 124 */ Parm("AUMFL_S", "momentum flux, u component [N/(m**2)]"),
    /* 125 */ Parm("AVMFL_S", "momentum flux, v component [N/(m**2)]"),
    /* 126 */ Parm("wind mix e", "wind mixing energy [J]"),
    /* 127 */ Parm("image data", "image data []"),
    /* 128 */ Parm("var128", "undefined"),
    /* 129 */ Parm("geopot h", "geopotential height (ECMF) [gpm]"),
    /* 130 */ Parm("temperatur", "temperature (ECMF) [K]"),
    /* 131 */ Parm("wind compU", "u-component of wind (ECMF) [m/s]"),
    /* 132 */ Parm("wind compV", "v-component of wind (ECMF) [m/s]"),
    /* 133 */ Parm("var133", "undefined"),
    /* 134 */ Parm("var134", "undefined"),
    /* 135 */ Parm("var135", "undefined"),
    /* 136 */ Parm("var136", "undefined"),
    /* 137 */ Parm("var137", "undefined"),
    /* 138 */ Parm("var138", "undefined"),
    /* 139 */ Parm("soil temp.", "soil temperature (ECMF) [K]"),
    /* 140 */ Parm("var140", "undefined"),
    /* 141 */ Parm("var141", "undefined"),
    /* 142 */ Parm("ls precip.", "large scale precipitation (ECMF) [kg/(m**2)]"),
    /* 143 */ Parm("conv prec.", "convective precipitation (ECMF) [kg/(m**2)]"),
    /* 144 */ Parm("snowfall", "snowfall (ECMF) [m of water equivalent]"),
    /* 145 */ Parm("var145", "undefined"),
    /* 146 */ Parm("var146", "undefined"),
    /* 147 */ Parm("var147", "undefined"),
    /* 148 */ Parm("var148", "undefined"),
    /* 149 */ Parm("var149", "undefined"),
    /* 150 */ Parm("var150", "undefined"),
    /* 151 */ Parm("pressure", "pressure reduced to MSL (ECMF) [Pa]"),
    /* 152 */ Parm("var152", "undefined"),
    /* 153 */ Parm("var153", "undefined"),
    /* 154 */ Parm("var154", "undefined"),
    /* 155 */ Parm("var155", "undefined"),
    /* 156 */ Parm("geopot h", "geopotential height (ECMF) [gpm]"),
    /* 157 */ Parm("rel. humid", "relative humidity (ECMF) [%]"),
    /* 158 */ Parm("var158", "undefined"),
    /* 159 */ Parm("var159", "undefined"),
    /* 160 */ Parm("var160", "undefined"),
    /* 161 */ Parm("var161", "undefined"),
    /* 162 */ Parm("var162", "undefined"),
    /* 163 */ Parm("var163", "undefined"),
    /* 164 */ Parm("cloud cov.", "total cloud cover (ECMF) [%]"),
    /* 165 */ Parm("10m-wind U", "u-component of 10m-wind (ECMF) [m/s]"),
    /* 166 */ Parm("10m-wind V", "v-component of 10m-wind (ECMF) [m/s]"),
    /* 167 */ Parm("2m temper", "2m temperature (ECMF) [K]"),
    /* 168 */ Parm("2m due-p.", "2m due-point temperature (ECMF) [K]"),
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
    /* 228 */ Parm("total prec", "total precipitation (ECMF) [m]"),
    /* 229 */ Parm("seaway 01", "seaway 01 (ECMF) []"),
    /* 230 */ Parm("seaway 02", "seaway 02 (ECMF) []"),
    /* 231 */ Parm("seaway 03", "seaway 03 (ECMF) []"),
    /* 232 */ Parm("seaway 04", "seaway 04 (ECMF) []"),
    /* 233 */ Parm("seaway 05", "seaway 05 (ECMF) []"),
    /* 234 */ Parm("seaway 06", "seaway 06 (ECMF) []"),
    /* 235 */ Parm("seaway 07", "seaway 07 (ECMF) []"),
    /* 236 */ Parm("seaway 08", "seaway 08 (ECMF) []"),
    /* 237 */ Parm("seaway 09", "seaway 09 (ECMF) []"),
    /* 238 */ Parm("seaway 10", "seaway 10 (ECMF) []"),
    /* 239 */ Parm("seaway 11", "seaway 11 (ECMF) []"),
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