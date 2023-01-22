use super::Parm;

pub const NCEP_133: [Parm; 256] = [
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
    /* 81 */ Parm("LAND", "Land-sea coverage (land=1;sea=0) [fraction]"),
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
    /* 139 */ Parm("POZT", "Ozone production from T term"),
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
    /* 154 */ Parm("OMGALF", "omega divided by density"),
    /* 155 */ Parm("var155", "undefined"),
    /* 156 */ Parm("var156", "undefined"),
    /* 157 */ Parm("var157", "undefined"),
    /* 158 */ Parm("var158", "undefined"),
    /* 159 */ Parm("var159", "undefined"),
    /* 160 */ Parm("var160", "undefined"),
    /* 161 */ Parm("var161", "undefined"),
    /* 162 */ Parm("var162", "undefined"),
    /* 163 */ Parm("var163", "undefined"),
    /* 164 */ Parm("COVZZ", "Covariance between u and u"),
    /* 165 */ Parm("COVMM", "Covariance between v and v"),
    /* 166 */ Parm("COVQZ", "Covariance between q and u"),
    /* 167 */ Parm("COVQM", "Covariance between q and v"),
    /* 168 */ Parm("COVTVV", "Covariance between T and omega"),
    /* 169 */ Parm("COVQVV", "Covariance between q and omega"),
    /* 170 */ Parm("var170", "undefined"),
    /* 171 */ Parm("var171", "undefined"),
    /* 172 */ Parm("var172", "undefined"),
    /* 173 */ Parm("LRGMR", "Large scale moistening rate"),
    /* 174 */ Parm("VDFOZ", "Ozone vertical diffusion"),
    /* 175 */ Parm("POZ", "Ozone production"),
    /* 176 */ Parm("var176", "undefined"),
    /* 177 */ Parm("var177", "undefined"),
    /* 178 */ Parm("var178", "undefined"),
    /* 179 */ Parm("var179", "undefined"),
    /* 180 */ Parm("var180", "undefined"),
    /* 181 */ Parm("GWDU", "Gravity wave drag u acceleration"),
    /* 182 */ Parm("GWDV", "Gravity wave drag v acceleration"),
    /* 183 */ Parm("CNVU", "Convective u momentum mixing acceleration"),
    /* 184 */ Parm("CNVV", "Convective v momentum mixing acceleration"),
    /* 185 */ Parm("AKHS", "Sfc exchange coeff for T and Q divided by delta z"),
    /* 186 */ Parm("AKMS", "Sfc exchange coeff for U and V divided by delta z"),
    /* 187 */ Parm("var187", "undefined"),
    /* 188 */ Parm("TOZ", "Ozone tendency"),
    /* 189 */ Parm("var189", "undefined"),
    /* 190 */ Parm("var190", "undefined"),
    /* 191 */ Parm("SUNSD", "Sunshine duration [s]"),
    /* 192 */ Parm("MOSF", "Meridional overturning stream function [10^6m^3/s]"),
    /* 193 */ Parm("EPSR", "Radiative emiissivity"),
    /* 194 */ Parm("var194", "undefined"),
    /* 195 */ Parm("QZ0", "q at top of viscous sublayer"),
    /* 196 */ Parm("CNGWDU", "Convective gravity wave drag zonal acceleration"),
    /* 197 */
    Parm(
        "CNGWDV",
        "Convective gravity wave drag meridional acceleration",
    ),
    /* 198 */ Parm("var198", "undefined"),
    /* 199 */ Parm("var199", "undefined"),
    /* 200 */ Parm("var200", "undefined"),
    /* 201 */ Parm("THZ0", "Theta at top of viscous sublayer"),
    /* 202 */ Parm("CNVUMF", "Convective updraft mass flux"),
    /* 203 */ Parm("COVPSPS", "Covariance between psfc and psfc"),
    /* 204 */ Parm("QMAX", "Maximum specific humidity at 2m"),
    /* 205 */ Parm("QMIN", "Minimum specific humidity at 2m"),
    /* 206 */ Parm("COVQQ", "Covariance between q and q"),
    /* 207 */ Parm("var207", "undefined"),
    /* 208 */ Parm("var208", "undefined"),
    /* 209 */ Parm("CNVDMF", "Convective downdraft mass flux"),
    /* 210 */ Parm("var210", "undefined"),
    /* 211 */ Parm("var211", "undefined"),
    /* 212 */ Parm("var212", "undefined"),
    /* 213 */ Parm("var213", "undefined"),
    /* 214 */ Parm("var214", "undefined"),
    /* 215 */ Parm("var215", "undefined"),
    /* 216 */ Parm("var216", "undefined"),
    /* 217 */ Parm("var217", "undefined"),
    /* 218 */ Parm("var218", "undefined"),
    /* 219 */ Parm("CNVDEMF", "Convective detrainment mass flux"),
    /* 220 */ Parm("COVVVVV", "Covariance between omega and omega"),
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
    /* 234 */ Parm("COVTT", "Covariance between T and T"),
    /* 235 */ Parm("var235", "undefined"),
    /* 236 */ Parm("WTEND", "Tendency of vertical velocity"),
    /* 237 */ Parm("var237", "undefined"),
    /* 238 */ Parm("var238", "undefined"),
    /* 239 */ Parm("POZO", "Ozone production from col ozone term"),
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