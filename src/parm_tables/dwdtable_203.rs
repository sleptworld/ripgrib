use super::Parm;
pub const DWD_203: [Parm; 256] = [
    /* 0 */ Parm("var0", "undefined"),
    /* 1 */ Parm("pressure", "pressure [hPa]"),
    /* 2 */ Parm("geopot h", "geopotential height [10 * gpm]"),
    /* 3 */ Parm("var3", "undefined"),
    /* 4 */ Parm("temperatur", "temperature [1*degree Celsius]"),
    /* 5 */ Parm("dew-pnt te", "dew-point temperature [1*degree Celsius]"),
    /* 6 */
    Parm(
        "windcompXY",
        "wind components X/Y (X*100000 + ((Y*10)+5000)) [m/s]",
    ),
    /* 7 */ Parm("geomet h", "geometrical height [kft]"),
    /* 8 */ Parm("geomet h", "geometrical height [hft]"),
    /* 9 */
    Parm(
        "wind di/sp",
        "wind direction and speed (dd*1000 + ff) [1*degree, 1*kt]",
    ),
    /* 10 */ Parm("3 h pr cha", "3 hour pressure change [Pa/(3*h)]"),
    /* 11 */ Parm("Schnee-Mge", "Schneemenge [mm]"),
    /* 12 */ Parm("var12", "undefined"),
    /* 13 */ Parm("Bod-Wass-G", "Bodenwassergehalt [mm]"),
    /* 14 */ Parm("var14", "undefined"),
    /* 15 */ Parm("stab. ind.", "stability index [K]"),
    /* 16 */ Parm("var16", "undefined"),
    /* 17 */ Parm("var17", "undefined"),
    /* 18 */ Parm("max wind", "maximum wind velocity [km/h]"),
    /* 19 */ Parm("max wind", "maximum wind velocity [kt]"),
    /* 20 */
    Parm(
        "wind di/sp",
        "wind direction and speed (dd*1000 + ff) [5*degrees, 1*(m/s)]",
    ),
    /* 21 */
    Parm(
        "wind di/sp",
        "wind direction and speed (dd*1000 + ff) [5*degrees, 1*kt]",
    ),
    /* 22 */
    Parm(
        "wave di/he",
        "direction and height of wind waves (dd*1000 + h) [1*degree, 1*cm]",
    ),
    /* 23 */
    Parm(
        "swe. di/he",
        "direction and height of swell (dd*1000 + h) [1*degree, 1*cm]",
    ),
    /* 24 */
    Parm(
        "wave m d/h",
        "mean direction and height of waves (dd*1000 + h) [1*degree, 1*cm]",
    ),
    /* 25 */ Parm("wind speed", "wind speed [kt]"),
    /* 26 */ Parm("var26", "undefined"),
    /* 27 */ Parm("wind compX", "wind component X-direction [kt]"),
    /* 28 */ Parm("wind compY", "wind component Y-direction [kt]"),
    /* 29 */ Parm("var29", "undefined"),
    /* 30 */ Parm("var30", "undefined"),
    /* 31 */ Parm("var31", "undefined"),
    /* 32 */ Parm("var32", "undefined"),
    /* 33 */ Parm("abs voradv", "absolute vorticity advection [1/(s**2)]"),
    /* 34 */ Parm("var34", "undefined"),
    /* 35 */ Parm("var35", "undefined"),
    /* 36 */ Parm("var36", "undefined"),
    /* 37 */ Parm("var37", "undefined"),
    /* 38 */ Parm("var38", "undefined"),
    /* 39 */ Parm("var39", "undefined"),
    /* 40 */ Parm("var40", "undefined"),
    /* 41 */ Parm("var41", "undefined"),
    /* 42 */ Parm("vert. vel.", "vertical velocity [hPa/h]"),
    /* 43 */ Parm("var43", "undefined"),
    /* 44 */ Parm("var44", "undefined"),
    /* 45 */ Parm("var45", "undefined"),
    /* 46 */ Parm("var46", "undefined"),
    /* 47 */ Parm("var47", "undefined"),
    /* 48 */ Parm("var48", "undefined"),
    /* 49 */ Parm("var49", "undefined"),
    /* 50 */ Parm("var50", "undefined"),
    /* 51 */ Parm("var51", "undefined"),
    /* 52 */ Parm("var52", "undefined"),
    /* 53 */ Parm("var53", "undefined"),
    /* 54 */ Parm("var54", "undefined"),
    /* 55 */ Parm("max. temp.", "maximum temperature [1*degree Celsius]"),
    /* 56 */ Parm("min. temp.", "minimum temperature [1*degree Celsius]"),
    /* 57 */ Parm("sul_prob", "probability to perceive sultriness [1]"),
    /* 58 */ Parm("clo", "value of isolation of clothes [1]"),
    /* 59 */ Parm("pmva", "predected mean vote (angepasst) [1]"),
    /* 60 */ Parm("feeled t", "feeled temperature [1*degree Celsius]"),
    /* 61 */ Parm("sea temper", "sea temperature [1*degree Celsius]"),
    /* 62 */ Parm("var62", "undefined"),
    /* 63 */ Parm("var63", "undefined"),
    /* 64 */ Parm("var64", "undefined"),
    /* 65 */ Parm("var65", "undefined"),
    /* 66 */ Parm("var66", "undefined"),
    /* 67 */ Parm("var67", "undefined"),
    /* 68 */ Parm("var68", "undefined"),
    /* 69 */ Parm("var69", "undefined"),
    /* 70 */ Parm("var70", "undefined"),
    /* 71 */ Parm("var71", "undefined"),
    /* 72 */ Parm("var72", "undefined"),
    /* 73 */ Parm("var73", "undefined"),
    /* 74 */ Parm("var74", "undefined"),
    /* 75 */ Parm("var75", "undefined"),
    /* 76 */ Parm("var76", "undefined"),
    /* 77 */ Parm("var77", "undefined"),
    /* 78 */ Parm("var78", "undefined"),
    /* 79 */ Parm("var79", "undefined"),
    /* 80 */ Parm("var80", "undefined"),
    /* 81 */ Parm("var81", "undefined"),
    /* 82 */ Parm("var82", "undefined"),
    /* 83 */ Parm("var83", "undefined"),
    /* 84 */ Parm("var84", "undefined"),
    /* 85 */ Parm("var85", "undefined"),
    /* 86 */
    Parm(
        "Globalstr.",
        "Summe der Globalstrahlung ueber einen Zeitraum [kWh/m**2]",
    ),
    /* 87 */
    Parm(
        "Nied-GW-GE",
        "Niederschlagsart+Gewitter+Glatteis (T23-i) (0..99) [1]",
    ),
    /* 88 */
    Parm(
        "NiedGW-Art",
        "Niederschlagsart+Gewitter (T23-intern)     (0..99) [1]",
    ),
    /* 89 */
    Parm(
        "NiedGE-Art",
        "Niederschlagsart+Glatteis (T23-intern)     (0..99) [1]",
    ),
    /* 90 */
    Parm(
        "NiedBewArt",
        "Kombination Niederschl.-Bew.-Blautherm. (283..407) [1]",
    ),
    /* 91 */
    Parm(
        "Konv.U-Gr.",
        "Hoehe der Konvektionsuntergrenze ueber Grund [m]",
    ),
    /* 92 */
    Parm(
        "Nied.-Art",
        "Niederschlagsart -ww- (T23-intern)         (0..99) [1]",
    ),
    /* 93 */
    Parm(
        "Konv.-Art",
        "Konvektionsart                              (0..4) [1]",
    ),
    /* 94 */
    Parm(
        "Konv.UG-nn",
        "Hoehe der Konvektionsuntergrenze ueber nn [m]",
    ),
    /* 95 */ Parm("var95", "undefined"),
    /* 96 */ Parm("var96", "undefined"),
    /* 97 */ Parm("var97", "undefined"),
    /* 98 */ Parm("var98", "undefined"),
    /* 99 */ Parm("WW", "Wetter (verschluesselt nach ww-Tabelle"),
    /* 100 */ Parm("geostr Vor", "geostrophische Vorticity [1/s]"),
    /* 101 */ Parm("Geo VorAdv", "geostrophische  Vorticityadvektion [1/s**2]"),
    /* 102 */
    Parm(
        "VerGraVoAd",
        "vert. Gradient der geostr. Vorticityadvektion [m/(kg*s)]",
    ),
    /* 103 */
    Parm(
        "Geo TemAdv",
        "geostrophische Schichtdickenadvektion [m**3/(kg*s)]",
    ),
    /* 104 */
    Parm(
        "Lap TemAdv",
        "Kruemmung der geostr. Schichtdickenadvektion [m/(kg*s)]",
    ),
    /* 105 */
    Parm(
        "Omega Forc",
        "Forcing rechte Seite Omegagleichung [m/(kg*s)]",
    ),
    /* 106 */ Parm("var106", "undefined"),
    /* 107 */ Parm("Schichtd.A", "Schichtdicken-Advektion [m**3/(kg*s)]"),
    /* 108 */
    Parm(
        "AdGeVoThWi",
        "Advektion von geostr. Vorticity mit dem therm Wind [m/(kg*s)]",
    ),
    /* 109 */ Parm("Wind-Div.", "Winddivergenz [1/s]"),
    /* 110 */
    Parm(
        "Q",
        "Q-vector direction and speed (dd*1000 + fff*1E13) [5*deg,1E13*m**2/kg/s]",
    ),
    /* 111 */ Parm("Qx", "Q-Vektor X-Komponente [m**2/(kg*s)]"),
    /* 112 */ Parm("Qy", "Q-Vektor Y-Komponente [m**2/(kg*s)]"),
    /* 113 */ Parm("Div Q", "Divergenz Q [m/(kg*s)]"),
    /* 114 */
    Parm(
        "FrontoGeQn",
        "Frontogenesefunktion, Q isother-senkrecht-Kompon. [m**2/(kg*s)]",
    ),
    /* 115 */
    Parm(
        "Qs (geo)",
        "Qs (geo),Komp. Q-Vektor parallel zu den Isothermen [m**2/(kg*s)]",
    ),
    /* 116 */ Parm("DivQn(geo)", "Divergenz Qn  geostrophisch [m/(kg*s)]"),
    /* 117 */ Parm("DivQs(geo)", "Divergenz Qs  geostrophisch [m/(kg*s)]"),
    /* 118 */ Parm("Fronto Gen", "Frontogenesefunktion [K**2/(m**2*s)]"),
    /* 119 */ Parm("var119", "undefined"),
    /* 120 */ Parm("var120", "undefined"),
    /* 121 */ Parm("var121", "undefined"),
    /* 122 */ Parm("var122", "undefined"),
    /* 123 */ Parm("var123", "undefined"),
    /* 124 */ Parm("FrontoGenP", "Frontogenese-Parameter [1]"),
    /* 125 */
    Parm(
        "Qs-Vektor",
        "Qs, Komp. Q-Vektor parallel zu den Isothermen [m**2/(kg*s)]",
    ),
    /* 126 */ Parm("var126", "undefined"),
    /* 127 */ Parm("Div Qs", "Divergenz Qs [m/(kg*s)]"),
    /* 128 */ Parm("var128", "undefined"),
    /* 129 */ Parm("var129", "undefined"),
    /* 130 */ Parm("IPV", "Isentrope potentielle Vorticity [K*m**2/(s*kg)]"),
    /* 131 */
    Parm(
        "Wind KompX",
        "Wind X-Komponente auf isentropen Flaechen [m/s]",
    ),
    /* 132 */
    Parm(
        "Wind KompY",
        "Wind Y-Komponente auf isentropen Flaechen [m/s]",
    ),
    /* 133 */ Parm("Druck-Ise.", "Druck einer isentropen Flaeche [hPa]"),
    /* 134 */ Parm("var134", "undefined"),
    /* 135 */ Parm("var135", "undefined"),
    /* 136 */ Parm("var136", "undefined"),
    /* 137 */ Parm("var137", "undefined"),
    /* 138 */ Parm("var138", "undefined"),
    /* 139 */ Parm("var139", "undefined"),
    /* 140 */ Parm("KO-Index", "KO-Index [K]"),
    /* 141 */ Parm("TT-Index", "Totals-Totals-Index [K]"),
    /* 142 */ Parm("S-Index", "S-Index [K]"),
    /* 143 */ Parm("Stein-Ind", "Steinbeck-Index [1]"),
    /* 144 */ Parm("Baily-Ind", "Baily-Index [1]"),
    /* 145 */ Parm("Microburst", "Microburst-Index [1]"),
    /* 146 */ Parm("Cat-Index", "Clear Air Turbulence Index [1/s]"),
    /* 147 */ Parm("var147", "undefined"),
    /* 148 */ Parm("Lab-Energ", "Labilit{tsenergie [J/g]"),
    /* 149 */ Parm("var149", "undefined"),
    /* 150 */ Parm("Virt T", "Virtuelle Temperatur [K]"),
    /* 151 */ Parm("Pseudo T", "Pseudo-Temperatur [K]"),
    /* 152 */ Parm("Pseudo Pot", "Pseudopotentielle Temperatur [K]"),
    /* 153 */ Parm("Aequi T", "Aequivalent-Temperatur [K]"),
    /* 154 */ Parm("Aequi Pot", "Aequivalentpotentielle Temperatur [K]"),
    /* 155 */ Parm("var155", "undefined"),
    /* 156 */ Parm("var156", "undefined"),
    /* 157 */ Parm("var157", "undefined"),
    /* 158 */ Parm("var158", "undefined"),
    /* 159 */ Parm("var159", "undefined"),
    /* 160 */ Parm("Bas St Wol", "Untergrenze strat. Bew|lkung [hft]"),
    /* 161 */ Parm("Bas St Wol", "Untergrenze strat. Bew|lkung [hPa]"),
    /* 162 */ Parm("Bas Cu Wol", "Untergrenze cumul. Bew|lkung [hft]"),
    /* 163 */ Parm("Bas Cu Wol", "Untergrenze cumul. Bew|lkung [hPa]"),
    /* 164 */ Parm("Top St Wol", "Obergrenze strat. Bew|lkung [hft]"),
    /* 165 */ Parm("Top St Wol", "Obergrenze strat. Bew|lkung [hPa]"),
    /* 166 */ Parm("Top Cu Wol", "Obergrenze cumul. Bew|lkung [hft]"),
    /* 167 */ Parm("Top Cu Wol", "Obergrenze cumul. Bew|lkung [hPa]"),
    /* 168 */ Parm("var168", "undefined"),
    /* 169 */ Parm("var169", "undefined"),
    /* 170 */ Parm("Bas Tur Wo", "Untergrenze Wolkenturbulenz [hft]"),
    /* 171 */ Parm("Bas Tur Wo", "Untergrenze Wolkenturbulenz [hPa]"),
    /* 172 */ Parm("Top Tur Wo", "Obergrenze Wolkenturbulenz [hft]"),
    /* 173 */ Parm("Top Tur Wo", "Obergrenze Wolkenturbulenz [hPa]"),
    /* 174 */ Parm("Bas Eis Wo", "Untergrenze Vereisung in Wolken [hft]"),
    /* 175 */ Parm("Bas Eis Wo", "Untergrenze Vereisung in Wolken [hPa]"),
    /* 176 */ Parm("Top Eis Wo", "Obergrenze Vereisung in Wolken [hft]"),
    /* 177 */ Parm("Top Eis Wo", "Obergrenze Vereisung in Wolken [hPa]"),
    /* 178 */
    Parm(
        "Int Tur Wo",
        "Intensitaet der Turbulenz in Wolken  (0..4) [1]",
    ),
    /* 179 */ Parm("Int Eis Wo", "Intensitaet der Vereisung  (0..4) [1]"),
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
    /* 190 */ Parm("Sichtweite", "Sichtweite [m]"),
    /* 191 */ Parm("PIP_degree", "Prognostic Icing"),
    /* 192 */ Parm("PIP_scenar", "Prog Icing"),
    /* 193 */ Parm("DIP_degree", "Diagnostic Icing"),
    /* 194 */ Parm("DIP_scenar", "Diag Icing"),
    /* 195 */
    Parm(
        "IcingGuess",
        "Icing Regime 1.Guess(1=gen,2=conv,3=strat,4=freez) [1]",
    ),
    /* 196 */ Parm("IcingGrade", "Icing Grade (1=LGT,2=MOD,3=SEV) [1]"),
    /* 197 */
    Parm(
        "IcingRegim",
        "Icing Regime(1=general,2=convect,3=strat,4=freez) [1]",
    ),
    /* 198 */ Parm("var198", "undefined"),
    /* 199 */ Parm("var199", "undefined"),
    /* 200 */ Parm("Gru Wetter", "Wetter - Grundzustand   (ww"),
    /* 201 */ Parm("Lok Wetter", "Wetter - 1. lokale Abweichung  (ww"),
    /* 202 */ Parm("Lok Wetter", "Wetter - 2. lokale Abweichung  (ww"),
    /* 203 */ Parm("CLDEPTH", "cloud depth (grey scale"),
    /* 204 */ Parm("CLCT_MOD", "modified total cloud cover  (0..1) [1]"),
    /* 205 */ Parm("curr weath", "current weather (symbol number"),
    /* 206 */ Parm("var206", "undefined"),
    /* 207 */ Parm("var207", "undefined"),
    /* 208 */ Parm("var208", "undefined"),
    /* 209 */ Parm("var209", "undefined"),
    /* 210 */ Parm("var210", "undefined"),
    /* 211 */ Parm("Cu", "Cumulus  (0..1) [1]"),
    /* 212 */ Parm("Cb", "Cumulimbus  (0..1) [1]"),
    /* 213 */ Parm("Sc", "Stratocumulus  (0..1) [1]"),
    /* 214 */ Parm("Ac", "Altocumulus  (0..1) [1]"),
    /* 215 */ Parm("Ci", "Cirrus  (0..1) [1]"),
    /* 216 */ Parm("St", "Stratus  (0..1) [1]"),
    /* 217 */ Parm("As", "Altostratus  (0..1) [1]"),
    /* 218 */ Parm("var218", "undefined"),
    /* 219 */ Parm("var219", "undefined"),
    /* 220 */ Parm("var220", "undefined"),
    /* 221 */ Parm("Bedeckung", "Bedeckung in Stufen [1]"),
    /* 222 */ Parm("Konvektion", "Konvektion  ja/nein [1]"),
    /* 223 */ Parm("MN >90%", "Gesamtbedeckung > 90%  ja/nein [1]"),
    /* 224 */ Parm("RF700 >89%", "relative Feuchte 700 hPa >= 90%  ja/nein [1]"),
    /* 225 */ Parm("RR12 zentr", "Niederschlag 12 std. zentriert [mm]"),
    /* 226 */
    Parm(
        "RR12 <=0.5",
        "Niederschlag 12 std. zentriert, Werte <= 0.5mm [mm]",
    ),
    /* 227 */
    Parm(
        "RR12 SA>60",
        "RR12 zentriert, Schneeanteil > 60%  ja/nein [1]",
    ),
    /* 228 */
    Parm(
        "RR12 Kv>60",
        "RR12 zentriert, konvektiver Anteil > 60%  ja/nein [1]",
    ),
    /* 229 */
    Parm(
        "SRR12ff",
        "Starkniederschlag in Stufen (12 std. Folgezeitr) [1]",
    ),
    /* 230 */ Parm("RRMAX/STD", "Maximaler Starkniederschlag / std [mm/h]"),
    /* 231 */ Parm("RRMAX/MIN", "Maximaler Starkniederschlag / min [mm/min]"),
    /* 232 */
    Parm(
        "SN12ff >15",
        "Schneefall (12std. Folgezeitraum) > 15 mm  ja/nein [1]",
    ),
    /* 233 */
    Parm(
        "RRgefr12ff",
        "gefrierender Regen (12std. Folgezeitraum)  ja/nein [1]",
    ),
    /* 234 */ Parm("FFboe", "Boeenstaerke in Stufen [1]"),
    /* 235 */ Parm("Gewitter", "Gewitter in Stufen [1]"),
    /* 236 */
    Parm(
        "Tx2m12h ze",
        "2m Maximumtemperatur 12h zentriert [Grad Celsius]",
    ),
    /* 237 */
    Parm(
        "Tn2m12h ze",
        "2m Minimumtemperatur 12h zentriert [Grad Celsius]",
    ),
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
    /* 251 */ Parm("SCHWUELIND", "Schwuele-Index [1]"),
    /* 252 */ Parm("SMOGSTUFEN", "Smog-Intensitaetsstufen [1]"),
    /* 253 */ Parm("var253", "undefined"),
    /* 254 */ Parm("SMOGHOEHE", "Obergrenze Smog  ( Inversionshoehe ) [m]"),
    /* 255 */ Parm("var255", "undefined"),
];