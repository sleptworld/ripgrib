use grib::{
    bds::{self, bds_parser, BDS},
    bms::{bms_parser, BMS},
    gds::{gds_parser, GDS},
    is::{is_parser, IS},
    pds::{pds_parser, PDS},
};
use nom::{sequence::tuple, IResult};

pub mod grib;

#[derive(Debug)]
pub struct Project<'a> {
    is: IS,
    pds: PDS,
    gds: Option<GDS<'a>>,
    bms: Option<BMS<'a>>,
    bds: BDS<'a>,
}

pub fn test_mth<'a>(input: &'a [u8]) -> IResult<&'a [u8], Project<'a>> {
    let (mut next, (is, pds)) = tuple((is_parser, pds_parser))(input)?;

    let mut gds: Option<GDS> = None;
    let mut bms: Option<BMS> = None;

    if pds.gds_or_bms.0 {
        let (n, g) = gds_parser(next)?;
        next = n;
        gds = Some(g);
    }

    if pds.gds_or_bms.1 {
        let (n, b) = bms_parser(next)?;
        next = n;
        bms = Some(b);
    }

    let (next, bds) = bds_parser(next)?;

    Ok((
        next,
        Project {
            is,
            pds,
            gds,
            bms,
            bds,
        },
    ))
}
