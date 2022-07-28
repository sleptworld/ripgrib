use super::Parm;

pub const NCEP_130: [Parm; 256] = [
    /* 0 */ Parm("var0", "undefined"),
    /* 1 */ Parm("PRES", "Pressure [Pa]"),
    /* 2 */ Parm("PRMSL", "Pressure reduced to MSL [Pa]"),
    /* 3 */ Parm("PTEND", "Pressure tendency [Pa/s]"),
    /* 4 */ Parm("PVORT", "Pot. vorticity [km^2/kg/s]"),
    /* 5 */ Parm("ICAHT", "ICAO Standard Atmosphere Reference Height [M]"),
    /* 6 */ Parm("GP", "Geopotential [m^2/s^2]"),
    /* 7 */ Parm("HGT", "Geopotential height [gpm]"),
    /* 8 */ Parm("DIST", "Geometric height [m]"),
    /* 9 */ Parm("HSTDV", "Std dev of height [m]"),
    /* 10 */ Parm("TOZNE", "Total ozone [Dobson]"),
    /* 11 */ Parm("TMP", "Temp. [K]"),
    /* 12 */ Parm("VTMP", "Virtual temp. [K]"),
    /* 13 */ Parm("POT", "Potential temp. [K]"),
    /* 14 */ Parm("EPOT", "Pseudo-adiabatic pot. temp. [K]"),
    /* 15 */ Parm("TMAX", "Max. temp. [K]"),
    /* 16 */ Parm("TMIN", "Min. temp. [K]"),
    /* 17 */ Parm("DPT", "Dew point temp. [K]"),
    /* 18 */ Parm("DEPR", "Dew point depression [K]"),
    /* 19 */ Parm("LAPR", "Lapse rate [K/m]"),
    /* 20 */ Parm("VIS", "Visibility [m]"),
    /* 21 */ Parm("RDSP1", "Radar spectra (1) [non-dim]"),
    /* 22 */ Parm("RDSP2", "Radar spectra (2) [non-dim]"),
    /* 23 */ Parm("RDSP3", "Radar spectra (3) [non-dim]"),
    /* 24 */ Parm("PLI", "Parcel lifted index (to 500 hPa) [K]"),
    /* 25 */ Parm("TMPA", "Temp. anomaly [K]"),
    /* 26 */ Parm("PRESA", "Pressure anomaly [Pa]"),
    /* 27 */ Parm("GPA", "Geopotential height anomaly [gpm]"),
    /* 28 */ Parm("WVSP1", "Wave spectra (1) [non-dim]"),
    /* 29 */ Parm("WVSP2", "Wave spectra (2) [non-dim]"),
    /* 30 */ Parm("WVSP3", "Wave spectra (3) [non-dim]"),
    /* 31 */ Parm("WDIR", "Wind direction [deg]"),
    /* 32 */ Parm("WIND", "Wind speed [m/s]"),
    /* 33 */ Parm("UGRD", "u wind [m/s]"),
    /* 34 */ Parm("VGRD", "v wind [m/s]"),
    /* 35 */ Parm("STRM", "Stream function [m^2/s]"),
    /* 36 */ Parm("VPOT", "Velocity potential [m^2/s]"),
    /* 37 */ Parm("MNTSF", "Montgomery stream function [m^2/s^2]"),
    /* 38 */ Parm("SGCVV", "Sigma coord. vertical velocity [/s]"),
    /* 39 */ Parm("VVEL", "Pressure vertical velocity [Pa/s]"),
    /* 40 */ Parm("DZDT", "Geometric vertical velocity [m/s]"),
    /* 41 */ Parm("ABSV", "Absolute vorticity [/s]"),
    /* 42 */ Parm("ABSD", "Absolute divergence [/s]"),
    /* 43 */ Parm("RELV", "Relative vorticity [/s]"),
    /* 44 */ Parm("RELD", "Relative divergence [/s]"),
    /* 45 */ Parm("VUCSH", "Vertical u shear [/s]"),
    /* 46 */ Parm("VVCSH", "Vertical v shear [/s]"),
    /* 47 */ Parm("DIRC", "Direction of current [deg]"),
    /* 48 */ Parm("SPC", "Speed of current [m/s]"),
    /* 49 */ Parm("UOGRD", "u of current [m/s]"),
    /* 50 */ Parm("VOGRD", "v of current [m/s]"),
    /* 51 */ Parm("SPFH", "Specific humidity [kg/kg]"),
    /* 52 */ Parm("RH", "Relative humidity [%]"),
    /* 53 */ Parm("MIXR", "Humidity mixing ratio [kg/kg]"),
    /* 54 */ Parm("PWAT", "Precipitable water [kg/m^2]"),
    /* 55 */ Parm("VAPP", "Vapor pressure [Pa]"),
    /* 56 */ Parm("SATD", "Saturation deficit [Pa]"),
    /* 57 */ Parm("EVP", "Evaporation [kg/m^2]"),
    /* 58 */ Parm("CICE", "Cloud Ice [kg/m^2]"),
    /* 59 */ Parm("PRATE", "Precipitation rate [kg/m^2/s]"),
    /* 60 */ Parm("TSTM", "Thunderstorm probability [%]"),
    /* 61 */ Parm("APCP", "Total precipitation [kg/m^2]"),
    /* 62 */ Parm("NCPCP", "Large scale precipitation [kg/m^2]"),
    /* 63 */ Parm("ACPCP", "Convective precipitation [kg/m^2]"),
    /* 64 */ Parm("SRWEQ", "Snowfall rate water equiv. [kg/m^2/s]"),
    /* 65 */ Parm("WEASD", "Accum. snow [kg/m^2]"),
    /* 66 */ Parm("SNOD", "Snow depth [m]"),
    /* 67 */ Parm("MIXHT", "Mixed layer depth [m]"),
    /* 68 */ Parm("TTHDP", "Transient thermocline depth [m]"),
    /* 69 */ Parm("MTHD", "Main thermocline depth [m]"),
    /* 70 */ Parm("MTHA", "Main thermocline anomaly [m]"),
    /* 71 */ Parm("TCDC", "Total cloud cover [%]"),
    /* 72 */ Parm("CDCON", "Convective cloud cover [%]"),
    /* 73 */ Parm("LCDC", "Low level cloud cover [%]"),
    /* 74 */ Parm("MCDC", "Mid level cloud cover [%]"),
    /* 75 */ Parm("HCDC", "High level cloud cover [%]"),
    /* 76 */ Parm("CWAT", "Cloud water [kg/m^2]"),
    /* 77 */ Parm("BLI", "Best lifted index (to 500 hPa) [K]"),
    /* 78 */ Parm("SNOC", "Convective snow [kg/m^2]"),
    /* 79 */ Parm("SNOL", "Large scale snow [kg/m^2]"),
    /* 80 */ Parm("WTMP", "Water temp. [K]"),
    /* 81 */ Parm("LAND", "Land cover (land=1;sea=0) [fraction]"),
    /* 82 */ Parm("DSLM", "Deviation of sea level from mean [m]"),
    /* 83 */ Parm("SFCR", "Surface roughness [m]"),
    /* 84 */ Parm("ALBDO", "Albedo [%]"),
    /* 85 */ Parm("TSOIL", "Soil temp. [K]"),
    /* 86 */ Parm("SOILM", "Soil moisture content [kg/m^2]"),
    /* 87 */ Parm("VEG", "Vegetation [%]"),
    /* 88 */ Parm("SALTY", "Salinity [kg/kg]"),
    /* 89 */ Parm("DEN", "Density [kg/m^3]"),
    /* 90 */ Parm("WATR", "Water runoff [kg/m^2]"),
    /* 91 */ Parm("ICEC", "Ice concentration (ice=1;no ice=0) [fraction]"),
    /* 92 */ Parm("ICETK", "Ice thickness [m]"),
    /* 93 */ Parm("DICED", "Direction of ice drift [deg]"),
    /* 94 */ Parm("SICED", "Speed of ice drift [m/s]"),
    /* 95 */ Parm("UICE", "u of ice drift [m/s]"),
    /* 96 */ Parm("VICE", "v of ice drift [m/s]"),
    /* 97 */ Parm("ICEG", "Ice growth rate [m/s]"),
    /* 98 */ Parm("ICED", "Ice divergence [/s]"),
    /* 99 */ Parm("SNOM", "Snow melt [kg/m^2]"),
    /* 100 */ Parm("HTSGW", "Sig height of wind waves and swell [m]"),
    /* 101 */ Parm("WVDIR", "Direction of wind waves [deg]"),
    /* 102 */ Parm("WVHGT", "Sig height of wind waves [m]"),
    /* 103 */ Parm("WVPER", "Mean period of wind waves [s]"),
    /* 104 */ Parm("SWDIR", "Direction of swell waves [deg]"),
    /* 105 */ Parm("SWELL", "Sig height of swell waves [m]"),
    /* 106 */ Parm("SWPER", "Mean period of swell waves [s]"),
    /* 107 */ Parm("DIRPW", "Primary wave direction [deg]"),
    /* 108 */ Parm("PERPW", "Primary wave mean period [s]"),
    /* 109 */ Parm("DIRSW", "Secondary wave direction [deg]"),
    /* 110 */ Parm("PERSW", "Secondary wave mean period [s]"),
    /* 111 */ Parm("NSWRS", "Net short wave (surface) [W/m^2]"),
    /* 112 */ Parm("NLWRS", "Net long wave (surface) [W/m^2]"),
    /* 113 */ Parm("NSWRT", "Net short wave (top) [W/m^2]"),
    /* 114 */ Parm("NLWRT", "Net long wave (top) [W/m^2]"),
    /* 115 */ Parm("LWAVR", "Long wave [W/m^2]"),
    /* 116 */ Parm("SWAVR", "Short wave [W/m^2]"),
    /* 117 */ Parm("GRAD", "Global radiation [W/m^2]"),
    /* 118 */ Parm("BRTMP", "Brightness temperature [K]"),
    /* 119 */ Parm("LWRAD", "Radiance with respect to wave no. [W/m/sr]"),
    /* 120 */ Parm("SWRAD", "Radiance with respect ot wave len. [W/m^3/sr]"),
    /* 121 */ Parm("LHTFL", "Latent heat flux [W/m^2]"),
    /* 122 */ Parm("SHTFL", "Sensible heat flux [W/m^2]"),
    /* 123 */ Parm("BLYDP", "Boundary layer dissipation [W/m^2]"),
    /* 124 */ Parm("UFLX", "Zonal momentum flux [N/m^2]"),
    /* 125 */ Parm("VFLX", "Meridional momentum flux [N/m^2]"),
    /* 126 */ Parm("WMIXE", "Wind mixing energy [J]"),
    /* 127 */ Parm("IMGD", "Image data []"),
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
    /* 143 */ Parm("var143", "undefined 143"),
    /* 144 */
    Parm(
        "SOILW",
        "Volumetric soil moisture (frozen + liquid) [fraction]",
    ),
    /* 145 */ Parm("PEVPR", "Potential evaporation rate [W/m^2]"),
    /* 146 */ Parm("VEGT", "Vegetation canopy temperature [K]"),
    /* 147 */ Parm("BARET", "Bare soil surface skin temperature [K]"),
    /* 148 */ Parm("AVSFT", "Average surface skin temperature [K]"),
    /* 149 */ Parm("RADT", "Effective radiative skin temperature [K]"),
    /* 150 */ Parm("SSTOR", "Surface water storage [Kg/m^2]"),
    /* 151 */
    Parm(
        "LSOIL",
        "Liquid soil moisture content (non-frozen) [Kg/m^2]",
    ),
    /* 152 */ Parm("EWATR", "Open water evaporation (standing water) [W/m^2]"),
    /* 153 */ Parm("var153", "undefined"),
    /* 154 */ Parm("LSPA", "Land Surface Precipitation Accumulation [kg/m^2]"),
    /* 155 */ Parm("GFLUX", "Ground Heat Flux [W/m^2]"),
    /* 156 */ Parm("CIN", "Convective inhibition [J/Kg]"),
    /* 157 */ Parm("CAPE", "Convective available potential energy [J/Kg]"),
    /* 158 */ Parm("TKE", "Turbulent Kinetic Energy [J/Kg]"),
    /* 159 */ Parm("MXSALB", "Maximum snow albedo [%]"),
    /* 160 */
    Parm(
        "SOILL",
        "Liquid volumetric soil moisture (non-frozen) [fraction]",
    ),
    /* 161 */ Parm("ASNOW", "Frozen precipitation (e.g. snowfall) [Kg/m^2]"),
    /* 162 */ Parm("ARAIN", "Liquid precipitation (rainfall) [Kg/m^2]"),
    /* 163 */ Parm("GWREC", "Groundwater recharge [Kg/m^2]"),
    /* 164 */ Parm("QREC", "Flood plain recharge [Kg/m^2]"),
    /* 165 */ Parm("SNOWT", "Snow temperature, depth-avg [K]"),
    /* 166 */ Parm("VBDSF", "Visible beam downward solar flux [W/m^2]"),
    /* 167 */ Parm("VDDSF", "Visible diffuse downward solar flux [W/m^2]"),
    /* 168 */ Parm("NBDSF", "Near IR beam downward solar flux [W/m^2]"),
    /* 169 */ Parm("NDDSF", "Near IR diffuse downward solar flux [W/m^2]"),
    /* 170 */ Parm("SNFALB", "Snow-free albedo [%]"),
    /* 171 */ Parm("RLYRS", "Number of soil layers in root zone [non-dim]"),
    /* 172 */ Parm("MFLX", "Momentum flux [N/m^2]"),
    /* 173 */ Parm("var173", "undefined"),
    /* 174 */ Parm("var174", "undefined"),
    /* 175 */ Parm("var175", "undefined"),
    /* 176 */ Parm("NLAT", "Latitude (-90 to +90) [deg]"),
    /* 177 */ Parm("ELON", "East longitude (0-360) [deg]"),
    /* 178 */ Parm("FLDCAP", "Field capacity [fraction]"),
    /* 179 */ Parm("ACOND", "Aerodynamic conductance [m/s]"),
    /* 180 */ Parm("SNOAG", "Snow age [s]"),
    /* 181 */ Parm("CCOND", "Canopy conductance [m/s]"),
    /* 182 */ Parm("LAI", "Leaf area index (0-9) [non-dim]"),
    /* 183 */ Parm("SFCRH", "Roughness length for heat [m]"),
    /* 184 */ Parm("SALBD", "Snow albedo (over snow cover area only) [%]"),
    /* 185 */ Parm("var185", "undefined"),
    /* 186 */ Parm("var186", "undefined"),
    /* 187 */ Parm("NDVI", "Normalized Difference Vegetation Index []"),
    /* 188 */ Parm("DRIP", "Canopy drip [Kg/m^2]"),
    /* 189 */ Parm("VBSALB", "Visible, black sky albedo [%]"),
    /* 190 */ Parm("VWSALB", "Visible, white sky albedo [%]"),
    /* 191 */ Parm("NBSALB", "Near IR, black sky albedo [%]"),
    /* 192 */ Parm("NWSALB", "Near IR, white sky albedo [%]"),
    /* 193 */ Parm("var193", "undefined"),
    /* 194 */ Parm("var194", "undefined"),
    /* 195 */ Parm("var195", "undefined"),
    /* 196 */ Parm("var196", "undefined"),
    /* 197 */ Parm("var197", "undefined"),
    /* 198 */ Parm("SBSNO", "Sublimation (evaporation from snow) [W/m^2]"),
    /* 199 */ Parm("EVBS", "Direct evaporation from bare soil [W/m^2]"),
    /* 200 */ Parm("EVCW", "Canopy water evaporation [W/m^2]"),
    /* 201 */ Parm("var201", "undefined"),
    /* 202 */ Parm("var202", "undefined"),
    /* 203 */ Parm("RSMIN", "Minimal stomatal resistance [s/m]"),
    /* 204 */ Parm("DSWRF", "Downward shortwave radiation flux [W/m^2]"),
    /* 205 */ Parm("DLWRF", "Downward longwave radiation flux [W/m^2]"),
    /* 206 */ Parm("var206", "undefined"),
    /* 207 */ Parm("MSTAV", "Moisture availability [%]"),
    /* 208 */ Parm("SFEXC", "Exchange coefficient [(Kg/m^3)(m/s)]"),
    /* 209 */ Parm("var209", "undefined"),
    /* 210 */ Parm("TRANS", "Transpiration [W/m^2]"),
    /* 211 */ Parm("USWRF", "Upward short wave radiation flux [W/m^2]"),
    /* 212 */ Parm("ULWRF", "Upward long wave radiation flux [W/m^2]"),
    /* 213 */ Parm("var213", "undefined"),
    /* 214 */ Parm("var214", "undefined"),
    /* 215 */ Parm("var215", "undefined"),
    /* 216 */ Parm("var216", "undefined"),
    /* 217 */ Parm("var217", "undefined"),
    /* 218 */ Parm("var218", "undefined"),
    /* 219 */ Parm("WILT", "Wilting point [fraction]"),
    /* 220 */ Parm("FLDCP", "Field Capacity [fraction]"),
    /* 221 */ Parm("HPBL", "Planetary boundary layer height [m]"),
    /* 222 */ Parm("SLTYP", "Surface slope type [Index]"),
    /* 223 */ Parm("CNWAT", "Plant canopy surface water [Kg/m^2]"),
    /* 224 */ Parm("SOTYP", "Soil type [Index]"),
    /* 225 */ Parm("VGTYP", "Vegetation type [Index]"),
    /* 226 */ Parm("BMIXL", "Blackadars mixing length scale [m]"),
    /* 227 */ Parm("AMIXL", "Asymptotic mixing length scale [m]"),
    /* 228 */ Parm("PEVAP", "Potential evaporation [Kg/m^2]"),
    /* 229 */ Parm("SNOHF", "Snow phase-change heat flux [W/m^2]"),
    /* 230 */
    Parm(
        "SMREF",
        "Transpiration stress-onset (soil moisture) [fraction]",
    ),
    /* 231 */
    Parm(
        "SMDRY",
        "Direct evaporation cease (soil moisture) [fraction]",
    ),
    /* 232 */ Parm("var232", "undefined"),
    /* 233 */ Parm("var233", "undefined"),
    /* 234 */ Parm("BGRUN", "Subsurface runoff (baseflow) [Kg/m^2]"),
    /* 235 */ Parm("SSRUN", "Surface runoff (non-infiltrating) [Kg/m^2]"),
    /* 236 */ Parm("var236", "undefined"),
    /* 237 */ Parm("var237", "undefined"),
    /* 238 */ Parm("SNOWC", "Snow cover [%]"),
    /* 239 */ Parm("SNOT", "Snow temperature [K]"),
    /* 240 */ Parm("POROS", "Soil porosity [fraction]"),
    /* 241 */ Parm("var241", "undefined"),
    /* 242 */ Parm("var242", "undefined"),
    /* 243 */ Parm("var243", "undefined"),
    /* 244 */ Parm("var244", "undefined"),
    /* 245 */ Parm("var245", "undefined"),
    /* 246 */ Parm("RCS", "Solar parameter in canopy conductance [fraction]"),
    /* 247 */
    Parm(
        "RCT",
        "Temperature parameter in canopy conductance [fraction]",
    ),
    /* 248 */ Parm("RCQ", "Humidity parameter in canopy conductance [fraction]"),
    /* 249 */
    Parm(
        "RCSOL",
        "Soil moisture parameter in canopy conductance [fraction]",
    ),
    /* 250 */ Parm("var250", "undefined"),
    /* 251 */ Parm("var251", "undefined"),
    /* 252 */ Parm("CD", "Surface drag coefficient [non-dim]"),
    /* 253 */ Parm("FRICV", "Surface friction velocity [m/s]"),
    /* 254 */ Parm("RI", "Richardson number [non-dim]"),
    /* 255 */ Parm("var255", "undefined"),
];
