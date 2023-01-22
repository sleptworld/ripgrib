use nom::{
    bytes::complete::tag,
    sequence::{terminated, tuple},
    IResult,
};

use crate::{ProductRecord, Raw};

use super::{
    bds::{bds_parser, BDS},
    bms::{bms_parser, BMS},
    gds::{gds_parser, GDS},
    is::{is_parser, IS},
    pds::{pds_parser, PDS},
};

#[derive(Debug, Default)]
pub struct Grib1Product<'a> {
    is: IS,
    pds: PDS,
    gds: Option<GDS<'a>>,
    bms: Option<BMS<'a>>,
    bds: BDS<'a>,
}

impl<'a> ProductRecord for Grib1Product<'a> {
    fn header_parse(&mut self) {}
    fn data_parse(&mut self) {}
}

pub fn test_mth<'a>(input: &'a [u8]) -> IResult<&'a [u8], Grib1Product<'a>> {
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
    let (next, bds): (&[u8], BDS) = terminated(bds_parser, tag([55, 55, 55, 55]))(next)?;
    Ok((
        next,
        Grib1Product {
            is,
            pds,
            gds,
            bms,
            bds,
        },
    ))
}
