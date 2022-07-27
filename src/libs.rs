use chrono::prelude::*;
use chrono::{Duration, TimeZone};
use nom::bytes::complete::tag;
use nom::number::complete::{u24, u8};
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
    length: usize,
    center_identification: Center,
    generating_process_id: u8,
    grid_identification: GridID,
    pub gds: bool,
    pub bms: bool,
    units: u8,
    level_type_and_value: String,
    datetime: DateTime<Utc>,
    forecast_time_unit: Duration,
    time_range: usize,
    average_number: u16,
    missing_number: u8,
    decimal_scale: u16,
}

enum Center {
    WMC(u8),
    RSMC(u8),
    RAFC(u8),
    INPE(u8),
    NMC(u8),
    NCAR(u8),
    Other(u8),
}

enum GridID {
    Lambert,
    Mocator,
}

fn pdsParser(input: &[u8]) -> IResult<&[u8], IS> {
    let length = u24(Little);
}

struct GDS {
    length: usize,
    nv: u8,
    pv_or_pl: u8,
    r_type: u8,
    description: u8,
}
