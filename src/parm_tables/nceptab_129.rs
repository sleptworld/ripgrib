use super::Parm;

pub const NCEP_129: [Parm; 256] = [
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
    /* 128 */ Parm("PAOT", "Probability anomaly of temp [%]"),
    /* 129 */ Parm("PAOP", "Probability anomaly of precip [%]"),
    /* 130 */ Parm("CWR", "Probability of wetting rain > 0.1 in [%]"),
    /* 131 */ Parm("FRAIN", "Rain fraction of total liquid water []"),
    /* 132 */ Parm("FICE", "Ice fraction of total condensate []"),
    /* 133 */ Parm("FRIME", "Rime factor []"),
    /* 134 */ Parm("CUEFI", "Convective cloud efficiency []"),
    /* 135 */ Parm("TCOND", "Total condensate [kg/kg]"),
    /* 136 */ Parm("TCOLW", "Total column cloud water [kg/m/m]"),
    /* 137 */ Parm("TCOLI", "Total column cloud ice [kg/m/m]"),
    /* 138 */ Parm("TCOLR", "Total column rain [kg/m/m]"),
    /* 139 */ Parm("TCOLS", "Total column snow [kg/m/m]"),
    /* 140 */ Parm("TCOLC", "Total column condensate [kg/m/m]"),
    /* 141 */
    Parm(
        "PLPL",
        "Pressure of level from which parcel was lifted [Pa]",
    ),
    /* 142 */ Parm("HLPL", "Height of level from which parcel was lifted [m]"),
    /* 143 */ Parm("CEMS", "Cloud Emissivity [fraction]"),
    /* 144 */ Parm("COPD", "Cloud Optical Depth [non-dim]"),
    /* 145 */ Parm("PSIZ", "Effective Particle size [microns]"),
    /* 146 */ Parm("TCWAT", "Total Water Cloud [%]"),
    /* 147 */ Parm("TCICE", "Total Ice Cloud [%]"),
    /* 148 */ Parm("WDIF", "Wind Difference [m/s]"),
    /* 149 */ Parm("WSTP", "Wave Steepness [non-dim]"),
    /* 150 */ Parm("PTAN", "Probability of Temp. above normal [%]"),
    /* 151 */ Parm("PTNN", "Probability of Temp. near normal [%]"),
    /* 152 */ Parm("PTBN", "Probability of Temp. below normal [%]"),
    /* 153 */ Parm("PPAN", "Probability of Precip. above normal [%]"),
    /* 154 */ Parm("PPNN", "Probability of Precip. near normal [%]"),
    /* 155 */ Parm("PPBN", "Probability of Precip. below normal [%]"),
    /* 156 */ Parm("PMTC", "Particulate matter (coarse) [ug/m^3]"),
    /* 157 */ Parm("PMTF", "Particulate matter (fine) [ug/m^3]"),
    /* 158 */ Parm("AETMP", "Analysis Error of Temperature [K]"),
    /* 159 */ Parm("AEDPT", "Analysis Error of Dew Point [K]"),
    /* 160 */ Parm("AESPH", "Analysis Error of Specific Humidity [kg/kg] wne"),
    /* 161 */ Parm("AEUWD", "Analysis Error of U-wind [m/s]"),
    /* 162 */ Parm("AEVWD", "Analysis Error of V-wind [m/s]"),
    /* 163 */ Parm("LPMTF", "Particulate matter (fine) [log10(ug/m^3)]"),
    /* 164 */
    Parm(
        "LIPMF",
        "Integrated Column Particulate matter (fine) [log10(ug/m^2)] wne",
    ),
    /* 165 */
    Parm(
        "REFZR",
        "Derived radar reflectivity backscatter from rain [mm^6/m^3]",
    ),
    /* 166 */
    Parm(
        "REFZI",
        "Derived radar reflectivity backscatter from ice [mm^6/m^3]",
    ),
    /* 167 */
    Parm(
        "REFZC",
        "Derived radar reflectivity backscatter from parameterized convection [mm^6/m^3]",
    ),
    /* 168 */ Parm("TCLSW", "Integrated supercooled liquid water [kg/m^2]"),
    /* 169 */ Parm("TCOLM", "Total Column Integrated Melting Ice [kg/m^2]"),
    /* 170 */ Parm("ELRDI", "Ellrod Index [non-dim]"),
    /* 171 */ Parm("TSEC", "Seconds prior to initial reference time [sec]"),
    /* 172 */ Parm("TSECA", "Seconds after initial reference time [sec]"),
    /* 173 */ Parm("NUM", "Number of samples/observations [non-dim]"),
    /* 174 */ Parm("AEPRS", "Analysis Error of Pressure [Pa]"),
    /* 175 */ Parm("ICSEV", "Icing Severity [non-dim]"),
    /* 176 */ Parm("ICPRB", "Icing Probability [non-dim]"),
    /* 177 */ Parm("LAVNI", "Low-level Aviation Interest [non-dim]"),
    /* 178 */ Parm("HAVNI", "High-level Aviation Interest [non-dim]"),
    /* 179 */ Parm("FLGHT", "Flight Category [non-dim]"),
    /* 180 */ Parm("OZCON", "Ozone concentration [ppb]"),
    /* 181 */ Parm("OZCAT", "Categorical ozone concentration [?]"),
    /* 182 */ Parm("VEDH", "vertical heat eddy diffusivity [m^2/s]"),
    /* 183 */ Parm("SIGV", "Sigma level value [non-dim]"),
    /* 184 */ Parm("EWGT", "Ensemble Weight [non-dim]"),
    /* 185 */ Parm("CICEL", "Confidence indicator - Ceiling [non-dim]"),
    /* 186 */ Parm("CIVIS", "Confidence indicator - Visibility [non-dim] 186"),
    /* 187 */ Parm("var187", "undefined"),
    /* 188 */ Parm("LAVV", "Latitude of V wind component of velocity [deg]"),
    /* 189 */ Parm("LOVV", "Longitude of V wind component of velocity [deg]"),
    /* 190 */ Parm("USCT", "Scatterometer est. U wind component [m/s]"),
    /* 191 */ Parm("VSCT", "Scatterometer est. V wind component [m/s]"),
    /* 192 */ Parm("LAUV", "Latitude of U wind component of velocity [deg]"),
    /* 193 */ Parm("LOUV", "Longitude of U wind component of velocity [deg]"),
    /* 194 */ Parm("TCHP", "Tropical Cyclone Heat Potential [J/m^2]"),
    /* 195 */ Parm("DBSS", "Geometric Depth Below Sea Surface [m]"),
    /* 196 */ Parm("ODHA", "Ocean Dynamic Heat Anomaly [dynamic m]"),
    /* 197 */ Parm("OHC", "Ocean Heat Content [J/m^2]"),
    /* 198 */ Parm("SSHG", "Sea Surface Height Relative to Geoid [m]"),
    /* 199 */ Parm("SLTFL", "Salt flux [g/cm^2/s]"),
    /* 200 */ Parm("DUVB", "UV-B Downward Solar Flux [W/m^2]"),
    /* 201 */ Parm("CDUVB", "Clear Sky UV-B Downward Solar Flux [W/m^2]"),
    /* 202 */ Parm("THFLX", "Total downward heat flux at surface [W/m^2]"),
    /* 203 */ Parm("UVAR", "U velocity variance [m^2/s^2]"),
    /* 204 */ Parm("VVAR", "V velocity variance [m^2/s^2]"),
    /* 205 */ Parm("UVVCC", "UV Velocity Cross Correlation [m^2/s^2]"),
    /* 206 */ Parm("MCLS", "Meteorological Correlation Length Scale [m]"),
    /* 207 */ Parm("LAPP", "Latitude of pressure point [deg]"),
    /* 208 */ Parm("LOPP", "Longitude of pressure point [deg]"),
    /* 209 */ Parm("var209", "undefined"),
    /* 210 */ Parm("REFO", "Observed radar reflectivity [dbZ]"),
    /* 211 */ Parm("REFD", "Derived radar reflectivity [dbZ]"),
    /* 212 */ Parm("REFC", "Maximum/Composite radar reflectivity [dbZ]"),
    /* 213 */
    Parm(
        "SBT122",
        "Simulated Brightness Temperature for GOES12, Channel 2 [K]",
    ),
    /* 214 */
    Parm(
        "SBT123",
        "Simulated Brightness Temperature for GOES12, Channel 3 [K]",
    ),
    /* 215 */
    Parm(
        "SBT124",
        "Simulated Brightness Temperature for GOES12, Channel 4 [K]",
    ),
    /* 216 */
    Parm(
        "SBT125",
        "Simulated Brightness Temperature for GOES12, Channel 5 [K]",
    ),
    /* 217 */ Parm("MINRH", "Minimum Relative Humumidity [%]"),
    /* 218 */ Parm("MAXRH", "Maximum Relative Humumidity [%]"),
    /* 219 */ Parm("CEIL", "Ceiling [m]"),
    /* 220 */ Parm("PBLREG", "Planetary boundary layer regime []"),
    /* 221 */
    Parm(
        "SBC123",
        "Simulated brightness counts for GOES12, Channel 3 [byte]",
    ),
    /* 222 */
    Parm(
        "SBC124",
        "Simulated brightness counts for GOES12, Channel 4 [byte]",
    ),
    /* 223 */ Parm("RPRATE", "Rain precipitation rate [kg/m^2/s]"),
    /* 224 */ Parm("SPRATE", "Snow precipitation rate [kg/m^2/s]"),
    /* 225 */ Parm("FPRATE", "Freezing rain precipitation rate [kg/m^2/s]"),
    /* 226 */ Parm("IPRATE", "Ice pellets precipitation rate [kg/m^2/s]"),
    /* 227 */ Parm("UPHL", "Updraft Helicity [m^2/s^2]"),
    /* 228 */ Parm("SURGE", "Storm Surge [m]"),
    /* 229 */ Parm("ETSRG", "Extra-tropical storm Surge [m]"),
    /* 230 */ Parm("RHPW", "Relative humidity with respect to precip water [%]"),
    /* 231 */ Parm("OZMAX1", "Ozone daily max from 1-hour ave [ppbV]"),
    /* 232 */ Parm("OZMAX8", "Ozone daily max from 8-hour ave [ppbV]"),
    /* 233 */ Parm("var233", "undefined"),
    /* 234 */ Parm("var234", "undefined"),
    /* 235 */ Parm("var235", "undefined"),
    /* 236 */ Parm("var236", "undefined"),
    /* 237 */ Parm("var237", "undefined"),
    /* 238 */ Parm("var238", "undefined"),
    /* 239 */ Parm("var239", "undefined"),
    /* 240 */ Parm("var240", "undefined"),
    /* 241 */ Parm("var241", "undefined"),
    /* 242 */ Parm("TCSRG20", "20% tropical cyclone storm exceedance [m]"),
    /* 243 */ Parm("TCSRG30", "30% tropical cyclone storm exceedance [m]"),
    /* 244 */ Parm("TCSRG40", "40% tropical cyclone storm exceedance [m]"),
    /* 245 */ Parm("TCSRG50", "50% tropical cyclone storm exceedance [m]"),
    /* 246 */ Parm("TCSRG60", "60% tropical cyclone storm exceedance [m]"),
    /* 247 */ Parm("TCSRG70", "70% tropical cyclone storm exceedance [m]"),
    /* 248 */ Parm("TCSRG80", "80% tropical cyclone storm exceedance [m]"),
    /* 249 */ Parm("TCSRG90", "90% tropical cyclone storm exceedance [m]"),
    /* 250 */ Parm("RETOP", "Radar echo top (18.3 DBZ) [m]"),
    /* 251 */ Parm("TENV", "Total energy norm variance []"),
    /* 252 */ Parm("var252", "undefined"),
    /* 253 */ Parm("var253", "undefined"),
    /* 254 */ Parm("var254", "undefined"),
    /* 255 */ Parm("var255", "undefined"),
];
