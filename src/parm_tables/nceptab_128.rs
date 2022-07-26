use super::Parm;

pub const NCEP_128: [Parm; 256] = [
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
    /* 128 */ Parm("AVDEPTH", "Ocean depth - mean [m]"),
    /* 129 */ Parm("DEPTH", "Ocean depth - instantaneous [m]"),
    /* 130 */ Parm("ELEV", "Ocean surface elevation relative to geoid [m]"),
    /* 131 */ Parm("MXEL24", "Max ocean surface elevation in last 24 hours [m]"),
    /* 132 */ Parm("MNEL24", "Min ocean surface elevation in last 24 hours [m]"),
    /* 133 */ Parm("var133", "undefined"),
    /* 134 */ Parm("var134", "undefined"),
    /* 135 */ Parm("O2", "Oxygen (O2 (aq)) []"),
    /* 136 */ Parm("PO4", "PO4 [Mol/kg]"),
    /* 137 */ Parm("NO3", "NO3 [Mol/kg]"),
    /* 138 */ Parm("SiO4", "SiO4 [Mol/kg]"),
    /* 139 */ Parm("CO2aq", "CO2 (aq) [Mol/kg]"),
    /* 140 */ Parm("HCO3", "HCO3 - [Mol/kg]"),
    /* 141 */ Parm("CO3", "CO3 -- [Mol/kg]"),
    /* 142 */ Parm("TCO2", "TCO2 [Mol/kg]"),
    /* 143 */ Parm("TALK", "TALK [Mol/kg]"),
    /* 144 */ Parm("var144", "undefined"),
    /* 145 */ Parm("var145", "undefined"),
    /* 146 */ Parm("S11", "S11 - 1,1 component of ice stress tensor []"),
    /* 147 */ Parm("S12", "S12 - 1,2 component of ice stress tensor []"),
    /* 148 */ Parm("S22", "S22 - 2,2 component of ice stress tensor []"),
    /* 149 */ Parm("INV1", "T1 - First invariant of stress tensor []"),
    /* 150 */ Parm("INV2", "T2 - Second invariant of stress tensor []"),
    /* 151 */ Parm("var151", "undefined"),
    /* 152 */ Parm("var152", "undefined"),
    /* 153 */ Parm("var153", "undefined"),
    /* 154 */ Parm("var154", "undefined"),
    /* 155 */ Parm("WVRGH", "Wave Roughness[ ]"),
    /* 156 */ Parm("WVSTRS", "Wave Stresses []"),
    /* 157 */ Parm("WHITE", "Whitecap coverage []"),
    /* 158 */ Parm("SWDIRWID", "Swell direction width []"),
    /* 159 */ Parm("SWFREWID", "Swell frequency width []"),
    /* 160 */ Parm("WVAGE", "Wave age []"),
    /* 161 */ Parm("PWVAGE", "Physical Wave age []"),
    /* 162 */ Parm("var162", "undefined"),
    /* 163 */ Parm("var163", "undefined"),
    /* 164 */ Parm("var164", "undefined"),
    /* 165 */ Parm("LTURB", "Master length scale (turbulence) [m]"),
    /* 166 */ Parm("var166", "undefined"),
    /* 167 */ Parm("var167", "undefined"),
    /* 168 */ Parm("var168", "undefined"),
    /* 169 */ Parm("var169", "undefined"),
    /* 170 */ Parm("AIHFLX", "Net Air-Ice heat flux [W/m^2]"),
    /* 171 */ Parm("AOHFLX", "Net Air-Ocean heat flux [W/m^2]"),
    /* 172 */ Parm("IOHFLX", "Net Ice-Ocean heat flux [W/m^2]"),
    /* 173 */ Parm("IOSFLX", "Net Ice-Ocean salt flux kg/s]"),
    /* 174 */ Parm("var174", "undefined"),
    /* 175 */ Parm("OMLT", "Ocean Mixed Layer Temperature [K]"),
    /* 176 */ Parm("OMLS", "Ocean Mixed Layer Salinity [kg/kg]"),
    /* 177 */
    Parm(
        "OMLPOTDEN",
        "Ocean Mixed Layer Potential density (Referenced to 2000m) [kg/m^3]",
    ),
    /* 178 */ Parm("OMLU", "U Velocity in mixed layer [m/s]"),
    /* 179 */ Parm("OMLV", "V Velocity in mixed layer [m/s]"),
    /* 180 */ Parm("ASHFL", "Assimilative Heat Flux [W/m^2]"),
    /* 181 */ Parm("ASSFL", "Assimilative Salt Flux [mm/day]"),
    /* 182 */ Parm("BOTLD", "Bottom Layer Depth [m]"),
    /* 183 */ Parm("UBARO", "Barotropic U Velocity [m/s]"),
    /* 184 */ Parm("VBARO", "Barotropic V Velocity [m/s]"),
    /* 185 */ Parm("INTFD", "Interface Depth [m]"),
    /* 186 */ Parm("WTMPC", "Temperature [C]"),
    /* 187 */ Parm("SALIN", "Salinity [psu]"),
    /* 188 */ Parm("EMNP", "Evaporation - Precipitation [cm/day]"),
    /* 189 */ Parm("var189", "undefined"),
    /* 190 */ Parm("KENG", "Kinetic Energy [J/kg]"),
    /* 191 */ Parm("var191", "undefined"),
    /* 192 */ Parm("LAYTH", "Layer Thickness[m]"),
    /* 193 */ Parm("SSTT", "Surface Temperature Trend [K/day]"),
    /* 194 */ Parm("SSST", "Surface Salinity Trend [psu/day]"),
    /* 195 */ Parm("OVHD", "Ocean vertical heat diffusivity [m^2/s]"),
    /* 196 */ Parm("OVSD", "Ocean vertical salt diffusivity [m^2/s]"),
    /* 197 */ Parm("OVMD", "Ocean vertical momementum diffusivity [m^2/s]"),
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
    /* 254 */ Parm("RERRVAR", "Relative Error Variance [pure number]"),
    /* 255 */ Parm("var255", "undefined"),
];
