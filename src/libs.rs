use super::parm_tables::Parm;
use crate::parm_tables;
use chrono::prelude::*;
use nom::bytes::complete::{tag, take};
use nom::multi::count;
use nom::number::complete::{i16, u16, u24, u8};
use nom::number::Endianness::Little;
use nom::sequence::{preceded, tuple};
use nom::IResult;

struct IS {
    total_length: usize,
    version_number: u8,
}
fn isParser(input: &[u8]) -> IResult<&[u8], IS> {
    let grib = tag([]);
    let total_length = u24(Little);
    let editon = u8;

    let (input, (len, edition_number)) = preceded(grib, tuple((total_length, editon)))(input)?;

    Ok((
        input,
        IS {
            total_length: len as usize,
            version_number: edition_number,
        },
    ))
}

struct PDS {
    center_identification: Center,
    generating_process_id: u8,
    grid_identification: u8,
    gds_or_bms: (bool, bool),
    unit: Parm,
    level_type_and_value: String,
    datetime: DateTime<Utc>,
    time_range: String,
    average_or_missing_number: i16,
    decimal_scale: i32,
    sub_center: SubCenter,
    missing: u8,
}

#[derive(Clone)]
enum Center {
    WMC(u8),
    RSMC(u8),
    RAFC(u8),
    RsmcAndRafc(u8),
    WAFC(u8),
    NMC(u8),
    Other(u8),
}

#[derive(Clone)]
enum SubCenter {
    NCEPReAnalysis,
    NCEPEnsemble,
    NCEPCentral,
    EnvModelingCenter,
    WeatherPredictionCenter,
    OceanPredictionCenter,
    ClimatePredictionCenter,
    AviationWeatherCenter,
    StormPredictionCenter,
    NationalHurricaneCenter,
    NWSTechDevLab,
    NESDIS,
    FederalAviationAdministration,
    NWSMeteorologialDevLab,
    NorthAmericaRegionalRA,
    SpaceWeather,
    ESRLGlobalSystem,
    Other,
}

enum GridID {
    Lambert,
    Mocator,
}

fn center_parser(input: &[u8]) -> IResult<&[u8], (Center, PreConfParaTable)> {
    let (next, value) = u8(input)?;
    match value {
        // WMC
        1..=6 => Ok((next, (Center::WMC(value), PreConfParaTable::NONE))),
        7 => Ok((next, (Center::WMC(value), PreConfParaTable::NMC))),
        8..=9 => Ok((next, (Center::WMC(value), PreConfParaTable::NONE))),

        // RSMC/RAFC
        10..=15 | 28 | 29 | 41..=44 | 51 | 69 | 70 => {
            Ok((next, (Center::RsmcAndRafc(value), PreConfParaTable::NONE)))
        }

        // RSMC
        16..=19 | 21 | 24..=27 | 30..=35 | 38 | 39 | 53 | 65..=67 | 71 | 74 | 75 | 79..=81 => {
            Ok((next, (Center::RSMC(value), PreConfParaTable::NONE)))
        }
        54 => Ok((next, (Center::RSMC(value), PreConfParaTable::CMC))),
        78 => Ok((next, (Center::RSMC(value), PreConfParaTable::DWD))),

        // NMC
        23
        | 47..=50
        | 73
        | 102..=109
        | 111..=122
        | 124..=136
        | 140..=144
        | 150..=153
        | 156..=159
        | 162..=165
        | 167..=171
        | 190..=198
        | 201..=203
        | 222..=232
        | 234..=238
        | 240
        | 242..=246 => Ok((next, (Center::NMC(value), PreConfParaTable::NONE))),
        200 => Ok((next, (Center::NMC(value), PreConfParaTable::LAMI))),

        // WAFC
        93 => Ok((next, (Center::WAFC(value), PreConfParaTable::NONE))),

        // Other
        146 => Ok((next, (Center::Other(value), PreConfParaTable::CHM))),
        46 => Ok((next, (Center::Other(value), PreConfParaTable::CPTEC))),
        20 | 92 => Ok((next, (Center::RAFC(value), PreConfParaTable::NONE))),
        _ => Ok((next, (Center::Other(value), PreConfParaTable::NONE))),
    }
}

fn gds_or_bms_parser(input: &[u8]) -> IResult<&[u8], (bool, bool)> {
    let (next, value) = u8(input)?;
    let gds = if value & 128 != 0 { true } else { false };
    let bms = if value & 64 != 0 { true } else { false };

    Ok((next, (gds, bms)))
}

fn levels(layer_indicator: u8, center: &PreConfParaTable, key_value: u16) -> String {
    let o11 = key_value / 256;
    let o12 = key_value % 256;

    return match layer_indicator {
        1 => String::from("sfc"),
        2 => String::from("cloud base"),
        3 => String::from(String::from("cloud top")),
        4 => String::from("0 deg isotherm"),
        5 => String::from("cond level"),
        6 => String::from("max wind level"),
        7 => String::from("tropopause"),
        8 => String::from("nominal top of atmosphere"),
        9 => String::from("sea bottom"),

        100 => format!("{} mb", key_value),
        101 => format!("{}-{} mb", o11 + 10, o12 + 10),
        102 => String::from("mean sea level (MSL)"),
        103 => format!("{} m above MSL", key_value),
        104 => format!("{}-{} m above MSL", o11 * 100, o12 * 100),
        105 => format!("{} m above gnd", key_value),
        106 => format!("{}-{} m above gnd", o11 * 100, o12 * 100),
        107 => format!("sigma={:.4}", key_value as f32 / 10000.0),
        108 => format!("sigma {:.2}-{:.2}", o11 as f32 / 100.0, o12 as f32 / 100.0),
        109 => format!("hybrid level {}", key_value),
        110 => format!("hybrid {}-{}", o11, o12),
        111 => format!("{} cm down", key_value),
        112 => format!("{}-{} cm down", o11, o12),
        113 => format!("pot-temp={}K", key_value),
        114 => format!("{}-{}K", 475 - o11, 475 - o12),
        115 => format!("{} mb above gnd", key_value),
        116 => format!("{}-{} mb above gnd", o11, o12),
        117 => format!(
            "{} pv units",
            (1 - ((o11 & 0x80) >> 6) as u32) * ((o11 & 0x7f) << 8 + o12) as u32
        ),
        119 => format!("{:.5} (ETA level)", key_value as f32 / 10000.0),
        120 => format!(
            "{:.2}-{:.2} (ETA levels)",
            o11 as f32 / 100.0,
            o12 as f32 / 100.0
        ),
        121 => format!("{}-{} mb", 1100 - o11, 1100 - o12),
        125 => format!("{} cm above gnd", key_value),
        126 => match center {
            PreConfParaTable::NMC => format!("{:.2} mb", key_value as f32 * 0.01),
            _ => String::from("None"),
        },
        128 => format!(
            "{:.3}-{:.3} (sigma)",
            1.1 - o11 as f32 / 1000.0,
            1.1 - o12 as f32 / 1000.0
        ),
        141 => format!("{}-{} mb", o11 * 10, 1100 - o12),
        160 => format!("{} m below sea level", key_value),

        200 => String::from("atmos col"),
        201 => String::from("ocean column"),

        204 => String::from("high trop freezing lvl"),
        206 => String::from("grid-scale cloud bottom"),
        207 => String::from("grid-scale cloud top"),
        209 => String::from("boundary layer cloud bottom"),

        210 => match center {
            PreConfParaTable::NMC => String::from("boundary-layer cloud top"),
            _ => format!("{:.2} mb", key_value as f32 * 0.01),
        },

        211 => String::from("boundary layer cloud layer"),
        212 => String::from("low cloud bottom"),
        213 => String::from("low cloud top"),
        214 => String::from("low cloud layer"),
        215 => String::from("cloud ceiling"),
        216 => String::from("Cb base"),
        217 => String::from("Cb top"),
        220 => String::from("planetary boundary layer(from Richardson no.)"),

        222 => String::from("middle cloud bottom"),
        223 => String::from("middle cloud top"),
        224 => String::from("middle cloud layer"),

        232 => String::from("high cloud bottom"),
        233 => String::from("high cloud top"),
        234 => String::from("high cloud layer"),

        235 => {
            if key_value % 10 == 0 {
                format!("{}C ocean isotherm level", key_value / 10)
            } else {
                format!("{:.1}C ocean isotherm level", key_value as f32 / 10.0)
            }
        }
        236 => format!("{}-{}m ocean layer", o11 * 10, o12 * 10),

        237 => String::from("ocean mixed layer bottom"),
        238 => String::from("ocean isothermal layer bottom"),
        239 => String::from("surface-26C ocean layer"),
        240 => String::from("ocean mixed layer"),
        241 => String::from("ordered sequence of data"),
        242 => String::from("convect-cloud bottom"),
        243 => String::from("convect-cloud top"),
        244 => String::from("convect-cloud layer"),
        245 => String::from("lowest level of wet bulb zero"),
        246 => String::from("max e-pot-temp level"),
        247 => String::from("equilibrium level"),
        248 => String::from("shallow convect-cloud bottom"),
        249 => String::from("shallow convect-cloud top"),
        251 => String::from("deep convect-cloud bottom"),
        252 => String::from("deep convect-cloud top"),
        253 => String::from("lowest bottom level of supercooled liquid water layer"),
        254 => String::from("highest top level of supercooled liquid water layer"),
        _ => String::from(""),
    };
}

#[derive(Clone)]
enum PreConfParaTable {
    NMC,
    ECMWF,
    DWD,
    CMC,
    CPTEC,
    CHM,
    LAMI,
    NONE,
}

fn unit_parser(
    value: u8,
    p_table: u8,
    center: &PreConfParaTable,
    sub_center: &SubCenter,
    process: u8,
) -> Parm {
    let mut para_table: &'static [Parm; 256] = &parm_tables::nceptable_opn::NCEP_OPN_PARM_TABLE;

    match *center {
        PreConfParaTable::NMC => {
            if p_table <= 3 {
                match sub_center {
                    SubCenter::NCEPReAnalysis => {
                        para_table = &parm_tables::nceptable_reanal::NCEP_REANAL_PARM_TABLE
                    }

                    SubCenter::NWSMeteorologialDevLab => {
                        para_table = &parm_tables::nceptable_mdl::NCEP_TABLE_MDL_PARM_TABLE
                    }

                    _ => (),
                }

                if (process != 80 && process != 180) || (p_table != 1 && p_table != 2) {
                    para_table = &parm_tables::nceptable_opn::NCEP_OPN_PARM_TABLE;
                }
            } else {
                match p_table {
                    128 => para_table = &parm_tables::nceptab_128::NCEP_128,
                    129 => para_table = &parm_tables::nceptab_129::NCEP_129,
                    130 => para_table = &parm_tables::nceptab_130::NCEP_130,
                    131 => para_table = &parm_tables::nceptab_131::NCEP_131,
                    133 => para_table = &parm_tables::nceptab_133::NCEP_133,
                    140 => para_table = &parm_tables::nceptab_140::NCEP_140,
                    141 => para_table = &parm_tables::nceptab_141::NCEP_141,
                    _ => (),
                }
            }
        }

        PreConfParaTable::ECMWF => {}

        PreConfParaTable::DWD | PreConfParaTable::CHM => {}

        PreConfParaTable::CPTEC => {}

        _ => {}
    }

    para_table[value as usize]
}

fn time_unit(input: u8) -> String {
    // MINUTE  0
    // HOUR    1
    // DAY     2
    // MONTH   3
    // YEAR    4
    // DECADE  5
    // NORMAL  6
    // CENTURY 7
    // HOURS3  10
    // HOURS6  11
    // HOURS12  12
    // MINUTES15 13
    // MINUTES30 14
    // SECOND  254
    match input {
        0 => String::from("Minute"),
        1 => String::from("Hour"),
        2 => String::from("Day"),
        3 => String::from("Month"),
        4 => String::from("Year"),
        5 => String::from("Decade"),
        6 => String::from("Normal"),
        7 => String::from("Century"),
        10 => String::from("3 Hours"),
        11 => String::from("6 Hours"),
        12 => String::from("12 Hours"),
        13 => String::from("15 Minutes"),
        14 => String::from("30 Minutes"),
        254 => String::from("Second"),
        _ => String::from("Undefined"),
    }
}

fn subcenter_parser(input: &[u8]) -> IResult<&[u8], SubCenter> {
    let (next, value) = u8(input)?;

    Ok((
        next,
        match value {
            1 => SubCenter::NCEPReAnalysis,
            2 => SubCenter::NCEPEnsemble,
            3 => SubCenter::NCEPCentral,
            4 => SubCenter::EnvModelingCenter,
            5 => SubCenter::WeatherPredictionCenter,
            6 => SubCenter::OceanPredictionCenter,
            7 => SubCenter::ClimatePredictionCenter,
            8 => SubCenter::AviationWeatherCenter,
            9 => SubCenter::StormPredictionCenter,
            10 => SubCenter::NationalHurricaneCenter,
            11 => SubCenter::NWSTechDevLab,
            12 => SubCenter::NESDIS,
            13 => SubCenter::FederalAviationAdministration,
            14 => SubCenter::NWSMeteorologialDevLab,
            15 => SubCenter::NorthAmericaRegionalRA,
            16 => SubCenter::SpaceWeather,
            17 => SubCenter::ESRLGlobalSystem,
            _ => SubCenter::Other,
        },
    ))
}

fn init_time(input: &[u8], century: u8) -> DateTime<Utc> {
    let year = 100 * (century as i32 - 1) + input[0] as i32;
    let month = input[1] as u32;
    let day = input[2] as u32;
    let hour = input[3] as u32;

    let minute = input[4] as u32;

    Utc.ymd(year, month, day).and_hms(hour, minute, 0)
}

fn time_range_parser(input: &[u8]) -> IResult<&[u8], String> {
    let (next, value) = count(u8, 4)(input)?;

    let unit = time_unit(value[0]);

    let time_range = value[1];
    let p1 = value[2];
    let p2 = value[3];

    Ok((
        next,
        match time_range {
            0 | 1 | 10 => format!(""),
            2 => format!("valid {}-{}{}:", p1, p2, unit),

            3 => format!("{}-{}{} ave:", p1, p2, unit),

            4 => format!("{}-{}{} acc:", p1, p2, unit),

            5 => format!("{}-{}{} diff:", p1, p2, unit),

            6 => format!("-{} to -{} {} ave:", p1, p2, unit),

            7 => format!("-{} to {} {} ave:", p1, p2, unit),

            11 => {
                if p1 > 0 {
                    format!("init fcst {}{}:", p1, unit)
                } else {
                    format!("time?:")
                }
            }

            13 => format!("nudge ana {}{}:", p1, unit),

            14 => format!("rel. fcst {}{}:", p1, unit),

            51 => {
                if p1 == 0 {
                    /* format!("clim {}{}:",p2,unit), */
                    format!("0-{}{} product:ave@1yr:", p2, unit)
                } else if p1 == 1 {
                    /* format!("clim (diurnal) {}{}:",p2,unit), */
                    format!("0-{}{} product:same-hour,ave@1yr:", p2, unit)
                } else {
                    format!("clim? p1={}? {}{}?:", p1, p2, unit)
                }
            }

            113 | 123 => format!("ave@{}{}:", p2, unit),

            114 | 124 => format!("acc@{}{}:", p2, unit),

            115 => format!("ave of fcst:{} to {}{}:", p1, p2, unit),

            116 => format!("acc of fcst:{} to {}{}:", p1, p2, unit),

            118 => format!("var@{}{}:", p2, unit),

            128 => format!("{}-{}{} fcst acc:ave@24hr:", p1, p2, unit),

            129 => format!("{}-{}{} fcst acc:ave@{}{}:", p1, p2, unit, p2 - p1, unit),

            130 => format!("{}-{}{} fcst ave:ave@24hr:", p1, p2, unit),

            131 => format!("{}-{}{} fcst ave:ave@{}{}:", p1, p2, unit, p2 - p1, unit),

            /* for CFS */
            132 => format!("{}-{}{} anl:ave@1yr:", p1, p2, unit),

            133 => format!("{}-{}{} fcst:ave@1yr:", p1, p2, unit),

            134 => format!("{}-{}{} fcst-anl:rms@1yr:", p1, p2, unit),

            135 => format!("{}-{}{} fcst-fcst_mean:rms@1yr:", p1, p2, unit),

            136 => format!("{}-{}{} anl-anl_mean:rms@1yr:", p1, p2, unit),

            137 => format!("{}-{}{} fcst acc:ave@6hr:", p1, p2, unit),

            138 => format!("{}-{}{} fcst ave:ave@6hr:", p1, p2, unit),

            139 => format!("{}-{}{} fcst acc:ave@12hr:", p1, p2, unit),

            140 => format!("{}-{}{} fcst ave:ave@12hr:", p1, p2, unit),

            _ => format!("time?:"),
        },
    ))
}

fn decimal_scale_parser(input: &[u8]) -> IResult<&[u8], i32> {
    let (next, value) = take(2usize)(input)?;

    let result = 1 - ((value[0] & 0x80) >> 6) as i32 * (((value[0] & 0x7f) << 8) + value[1]) as i32;

    Ok((next, result))
}

fn pds_parser(input: &[u8]) -> IResult<&[u8], PDS> {
    let date_time_parser = take(5usize);

    let length = u24(Little);

    let (
        next,
        (
            ll,
            p_table,
            center_and_preconf,
            process,
            grid_id,
            flag,
            raw_unit,
            level_type,
            raw_level,
            date_time,
            time_range,
            average_or_acc,
            missing,
            init_century,
            subcenter,
            decimal_factor,
        ),
    ) = tuple((
        length,
        u8,
        center_parser,
        u8,
        u8,
        gds_or_bms_parser,
        u8,
        u8,
        u16(Little),
        date_time_parser,
        time_range_parser,
        i16(Little),
        u8,
        u8,
        subcenter_parser,
        decimal_scale_parser,
    ))(input)?;

    let unit = unit_parser(
        raw_unit,
        p_table,
        &center_and_preconf.1,
        &subcenter,
        process,
    );
    let levels = levels(level_type, &center_and_preconf.1, raw_level);

    let init_time = init_time(date_time, init_century);

    Ok((
        next,
        PDS {
            center_identification: center_and_preconf.0,
            generating_process_id: process,
            grid_identification: grid_id,
            gds_or_bms: flag,
            unit,
            time_range,
            level_type_and_value: levels,
            datetime: init_time,
            average_or_missing_number: average_or_acc,
            decimal_scale: decimal_factor,
            sub_center: subcenter,
            missing,
        },
    ))
}

struct GDS {
    length: usize,
    nv: u8,
    pv_or_pl: u8,
    r_type: u8,
    description: u8,
}
