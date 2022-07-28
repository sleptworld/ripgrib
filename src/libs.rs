use crate::parm_tables::{self, *};
use chrono::prelude::*;
use chrono::{Duration, TimeZone};
use nom::bytes::complete::tag;
use nom::number::complete::{u24, u8};
use nom::number::Endianness::Little;
use nom::sequence::{preceded, tuple};
use nom::IResult;

use super::parm_tables::Parm;

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
    length: usize,
    center_identification: Center,
    generating_process_id: u8,
    grid_identification: GridID,
    pub gds: bool,
    pub bms: bool,
    unit: Parm,
    level_type_and_value: String,
    datetime: DateTime<Utc>,
    forecast_time_unit: Duration,
    time_range: usize,
    average_number: u16,
    missing_number: u8,
    decimal_scale: u16,
    sub_center: SubCenter,
}

#[derive(Eq)]
enum Center {
    WMC(u8),
    RSMC(u8),
    RAFC(u8),
    RsmcAndRafc(u8),
    WAFC(u8),
    NMC(u8),
    Other(u8),
}

#[derive(Eq)]
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
}

enum GridID {
    Lambert,
    Mocator,
}

fn center_parser(input: &[u8]) -> IResult<&[u8], Center> {
    let (next, value) = u8(input)?;
    match value {
        1..=9 => Ok((next, Center::WMC(value))),
        10..=15 | 28 | 29 | 41..=44 | 51 | 69 | 70 => Ok((next, Center::RsmcAndRafc(value))),
        16..=19 | 21 | 24..=27 | 30..=35 | 38 | 39 | 53 | 54 | 65..=67 | 71 | 74 | 75 | 78..=81 => {
            Ok((next, Center::RSMC(value)))
        }
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
        | 200..=203
        | 222..=232
        | 234..=238
        | 240
        | 242..=246 => Ok((next, Center::NMC(value))),

        93 => Ok((next, Center::WAFC(value))),
        20 | 92 => Ok((next, Center::RAFC(value))),
        _ => Ok((next, Center::Other(value))),
    }
}

fn gds_or_bms_parser(input: &[u8]) -> IResult<&[u8], (bool, bool)> {
    let (next, value) = u8(input)?;
    let gds = if value & 128 != 0 { true } else { false };
    let bms = if value & 64 != 0 { true } else { false };

    Ok((next, (gds, bms)))
}

fn levels(layer_indicator: u8, center: u8, key_value: u32) -> String {
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
        126 => {
            if center == 7 {
                format!("{:.2} mb", key_value as f32 * 0.01)
            } else {
                String::from("None")
            }
        }
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

        210 => {
            if center == 7 {
                String::from("boundary-layer cloud top")
            } else {
                format!("{:.2} mb", key_value as f32 * 0.01)
            }
        }

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

fn unit_parser(
    input: &[u8],
    p_table: u8,
    center: Center,
    sub_center: SubCenter,
    para_version: u8,
    process: u8,
) -> IResult<&[u8], Parm> {
    let (next, value) = u8(input)?;

    let para_table = match center {
        Center::WMC(value) => {
            // figure out if NCEP opn or reanalysis
            if value == 7 && p_table <= 3 {
                if sub_center == SubCenter::NCEPReAnalysis {
                    parm_tables::nceptable_reanal::NCEP_REANAL_PARM_TABLE
                } else if sub_center == SubCenter::NWSMeteorologialDevLab {
                    parm_tables::nceptable_mdl::NCEP_TABLE_MDL_PARM_TABLE
                } else if (process != 80 && process != 180) || (p_table != 1 && p_table != 2) {
                    parm_tables::nceptable_opn::NCEP_OPN_PARM_TABLE
                }
            }
        },
    }
}

fn pds_parser(input: &[u8]) -> IResult<&[u8], IS> {
    let length = u24(Little);
}

struct GDS {
    length: usize,
    nv: u8,
    pv_or_pl: u8,
    r_type: u8,
    description: u8,
}
