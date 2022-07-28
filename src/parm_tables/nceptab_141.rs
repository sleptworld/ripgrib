use super::Parm;

pub const NCEP_141: [Parm; 256] = [
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
    /* 128 */ Parm("EXTNC", "Aerosol Extinction Coefficient [1/km]"),
    /* 129 */ Parm("AOD", "Aerosol Optical Depth [-]"),
    /* 130 */ Parm("ASFTR", "Aerosol Asymmetry Factor [-]"),
    /* 131 */ Parm("SSALBD", "Aerosol Single Scatter Albedo [-]"),
    /* 132 */ Parm("BSCTRS", "Aerosol Back Scattering [1/km/sr]"),
    /* 133 */ Parm("var133", "undefined"),
    /* 134 */ Parm("var134", "undefined"),
    /* 135 */ Parm("var135", "undefined"),
    /* 136 */ Parm("var136", "undefined"),
    /* 137 */ Parm("var137", "undefined"),
    /* 138 */ Parm("var138", "undefined"),
    /* 139 */ Parm("var139", "undefined"),
    /* 140 */ Parm("NOy", "Total Inorganic and Organic Nitrates [ppbV]"),
    /* 141 */ Parm("NO", "Nitrogen Oxide [ppbV]"),
    /* 142 */ Parm("NO2", "Nitrogen Dioxide [ppbV]"),
    /* 143 */ Parm("N2O5", "Nitrogen Pentoxide [ppbV]"),
    /* 144 */ Parm("HNO3", "Nitric Acid [ppbV]"),
    /* 145 */ Parm("NO3", "Nitrogen Trioxide [ppbV]"),
    /* 146 */ Parm("PNA", "Peroxynitric Acid [ppbV]"),
    /* 147 */ Parm("HONO", "Nitrous Acid [ppbV]"),
    /* 148 */ Parm("CO", "Carbon Monoxide [ppbV]"),
    /* 149 */ Parm("NH3", "Ammonia [ppbV]"),
    /* 150 */ Parm("HCL", "Hydrogen Chloride [ppbV]"),
    /* 151 */ Parm("var151", "undefined"),
    /* 152 */ Parm("var152", "undefined"),
    /* 153 */ Parm("var153", "undefined"),
    /* 154 */ Parm("var154", "undefined"),
    /* 155 */ Parm("var155", "undefined"),
    /* 156 */ Parm("var156", "undefined"),
    /* 157 */ Parm("var157", "undefined"),
    /* 158 */ Parm("var158", "undefined"),
    /* 159 */ Parm("PAR", "Lumped Single-Bond Carbon Specie [ppbV]"),
    /* 160 */ Parm("ETHE", "Ethene [ppbV]"),
    /* 161 */
    Parm(
        "OLE",
        "Lumped Double-Bond Carbon Species Less Ethene [ppbV]",
    ),
    /* 162 */ Parm("TOL", "Toluene [ppbV]"),
    /* 163 */ Parm("XYL", "Xylene [ppbV]"),
    /* 164 */ Parm("ISOP", "Isoprene [ppbV]"),
    /* 165 */ Parm("var165", "undefined"),
    /* 166 */ Parm("FORM", "Formaldehyde [ppbV]"),
    /* 167 */ Parm("ALD2", "Acetaldehyde & Higher Aldehydes [ppbV]"),
    /* 168 */ Parm("MGLY", "Methyl Glyoxal [ppbV]"),
    /* 169 */ Parm("CRES", "Cresol and Higher Molecular Weight Phenols [ppbV]"),
    /* 170 */ Parm("var170", "undefined"),
    /* 171 */ Parm("var171", "undefined"),
    /* 172 */ Parm("PAN", "Peroxyacyl Nitrate [ppbV]"),
    /* 173 */ Parm("NTR", "Lumped Gaseous Organic Nitrate [ppbV]"),
    /* 174 */ Parm("var174", "undefined"),
    /* 175 */ Parm("var175", "undefined"),
    /* 176 */ Parm("var176", "undefined"),
    /* 177 */ Parm("ROOH", "Esters [ppbV]"),
    /* 178 */ Parm("ETHOH", "Ethanol [ppbV]"),
    /* 179 */ Parm("METHOH", "Methanol [ppbV]"),
    /* 180 */ Parm("var180", "undefined"),
    /* 181 */ Parm("var181", "undefined"),
    /* 182 */ Parm("var182", "undefined"),
    /* 183 */ Parm("var183", "undefined"),
    /* 184 */ Parm("var184", "undefined"),
    /* 185 */ Parm("var185", "undefined"),
    /* 186 */ Parm("H2O2", "Hydrogen Peroxide [ppbV]"),
    /* 187 */ Parm("OH", "Hydroxyl Radical [ppbV]"),
    /* 188 */ Parm("HO2", "Hydroperoxyl Radical [ppbV]"),
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
    /* 200 */
    Parm(
        "ASO4",
        "Sulfate (SO4) Particulates ≤ 2.5 μm Diameter [μg/m^3]",
    ),
    /* 201 */
    Parm(
        "ANH4",
        "Ammonia (NH4) Particulates ≤ 2.5 μm Diameter [μg/m^3]",
    ),
    /* 202 */
    Parm(
        "ANO3",
        "Nitrate (NO3) Particulates ≤ 2.5 μm Diameter [μg/m^3]",
    ),
    /* 203 */ Parm("AORGA", "Organic Particulates ≤ 2.5 μm Diameter [μg/m^3]"),
    /* 204 */
    Parm(
        "AORGPA",
        "Primarily Organic Particulates ≤ 2.5 μm Diameter [μg/m^3]",
    ),
    /* 205 */
    Parm(
        "AORGB",
        "Biogenically Originated Particulates ≤ 2.5 μm Diameter [μg/m^3]",
    ),
    /* 206 */
    Parm(
        "AEC",
        "Elemental Carbon Particulates ≤ 2.5 μm Diameter [μg/m^3]",
    ),
    /* 207 */
    Parm(
        "A25",
        "Unspecified Anthropogenic Particulates ≤ 2.5 μm Diameter [μg/m^3]",
    ),
    /* 208 */ Parm("AH2O", "Water Particulates ≤ 2.5 μm Diameter [μg/m^3]"),
    /* 209 */ Parm("ANA", "Sodium Particulates ≤ 2.5 μm Diameter [μg/m^3]"),
    /* 210 */ Parm("ACL", "Chloride Particulates ≤ 2.5 μm Diameter [μg/m^3]"),
    /* 211 */ Parm("var211", "undefined"),
    /* 212 */ Parm("var212", "undefined"),
    /* 213 */ Parm("var213", "undefined"),
    /* 214 */ Parm("var214", "undefined"),
    /* 215 */ Parm("var215", "undefined"),
    /* 216 */
    Parm(
        "ASO4K",
        "Sulfate (SO4) Particulates between 2.5 and 10 μm Diameter [μg/m^3]",
    ),
    /* 217 */
    Parm(
        "ANAK",
        "Sodium (NA) Particulates between 2.5 and 10 μm Diameter [μg/m^3]",
    ),
    /* 218 */
    Parm(
        "ACLK",
        "Chloride (CL) Particulates between 2.5 and 10 μm Diameter [μg/m^3]",
    ),
    /* 219 */
    Parm(
        "ASEAS",
        "Sea Salt Originated Particulates between 2.5 and 10 μm Diameter [μg/m^3]",
    ),
    /* 220 */
    Parm(
        "ASOIL",
        "Crustal Material Orginated Particulates between 2.5 and 10 μm Diameter [μg/m^3]",
    ),
    /* 221 */
    Parm(
        "ACORS",
        "Particulates between 2.5 and 10 μm Diameter [μg/m^3]",
    ),
    /* 222 */
    Parm(
        "NUMATKN",
        "Number Concentration Particulates between 2.5 and 0.1 μm Diameter [number/m^3]",
    ),
    /* 223 */
    Parm(
        "NUMACC",
        "Number Concentration Particulates between 2.5 and 2.5 μm Diameter [number/m^3]",
    ),
    /* 224 */
    Parm(
        "NUMCOR",
        "Number Concentration Particulates between 2.5 and 10 μm Diameter [number/m^3]",
    ),
    /* 225 */ Parm("var225", "undefined"),
    /* 226 */ Parm("var226", "undefined"),
    /* 227 */ Parm("var227", "undefined"),
    /* 228 */
    Parm(
        "SRFATKN",
        "Surface Area Contributed by Particulates ≤ 0.1 μm Diameter [m2/m^3]",
    ),
    /* 229 */
    Parm(
        "SRFACC",
        "Surface Area Contributed by Particulates between 0.1 and 2.5 μm Diameter [m2/m^3]",
    ),
    /* 230 */ Parm("var230", "undefined"),
    /* 231 */ Parm("var231", "undefined"),
    /* 232 */ Parm("SO2", "Sulfur Dioxide [ppbV]"),
    /* 233 */ Parm("MSA", "Methanesulfonic Acid [Kg/Kg]"),
    /* 234 */
    Parm(
        "TSO4",
        "Total Sulfate Particulates (Fine ands Coarse) [μg/m^3]",
    ),
    /* 235 */ Parm("DMS", "Dimethylsulfide [Kg/Kg]"),
    /* 236 */ Parm("var236", "undefined"),
    /* 237 */ Parm("var237", "undefined"),
    /* 238 */ Parm("var238", "undefined"),
    /* 239 */ Parm("var239", "undefined"),
    /* 240 */
    Parm(
        "DU1",
        "Dust Particulates between 0.2 - 2.0 μm Diameter [Kg/Kg]",
    ),
    /* 241 */
    Parm(
        "DU2",
        "Dust Particulates between 2.0 - 3.6 μm Diameter [Kg/Kg]",
    ),
    /* 242 */
    Parm(
        "DU3",
        "Dust Particulates between 3.6 - 6.0 μm Diameter [Kg/Kg]",
    ),
    /* 243 */
    Parm(
        "DU4",
        "Dust Particulates between 6.0 - 12.0 μm Diameter [Kg/Kg]",
    ),
    /* 244 */
    Parm(
        "DU5",
        "Dust Particulates between 12.0 - 20.0 μm Diameter [Kg/Kg]",
    ),
    /* 245 */
    Parm(
        "SS1",
        "Sea Salt Particulates between 0.2 - 1.0 μm Diameter [Kg/Kg]",
    ),
    /* 246 */
    Parm(
        "SS2",
        "Sea Salt Particulates between 1.0 - 3.0 μm Diameter [Kg/Kg]",
    ),
    /* 247 */
    Parm(
        "SS3",
        "Sea Salt Particulates between 3.0 - 10.0 μm Diameter [Kg/Kg]",
    ),
    /* 248 */
    Parm(
        "SS4",
        "Sea Salt Particulates between 10.0 - 20.0 μm Diameter [Kg/Kg]",
    ),
    /* 249 */ Parm("OCDRY", "Hydrophobic Organic Carbon [Kg/Kg]"),
    /* 250 */ Parm("OCWET", "Hydrophilic Organic Carbon [Kg/Kg]"),
    /* 251 */ Parm("BCDRY", "Hydrophobic Black Carbon [Kg/Kg]"),
    /* 252 */ Parm("BCWET", "Hydrophilic Black Carbon [Kg/Kg]"),
    /* 253 */ Parm("var253", "undefined"),
    /* 254 */ Parm("var254", "undefined"),
    /* 255 */ Parm("var255", "undefined"),
];
