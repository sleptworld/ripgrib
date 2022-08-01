use nom::{character::complete::u8, number::complete::u24, IResult};

struct GDS {
    nv: usize,
    pv_or_pl: usize,
    representation: DataRepresentation,
    ni: usize,
    nj: usize,
    pv: u8,
    pl: u8,
}

type Lat = i32;
type Lon = i32;

type LatPair = (Lat, Lat);
type LonPair = (Lon, Lon);

#[derive(Clone, Copy)]
enum DataRepresentation {
    LatLon {
        lat: LatPair,
        lon_1: LonPair,
        di: u32,
        regular_grib: u32,
        scan_mode: u8,
    },
    Mercator {
        lon: LonPair,
        lat: LatPair,
        latin: u32,
        di: u32,
        dj: u32,
        scan_mode: u8,
    },
    Gnomonic,
    LambertConformal {
        lat_1: Lat,
        lon_1: Lon,
        lov: u32,
        dx: u32,
        dy: u32,
        projection_center: u32,
        scan_mode: u32,
        latin_1: u32,
        latin_2: u32,
        southern_pole_lon: u32,
        southern_pole_lat: u32,
    },
    Gaussian {
        lat: LatPair,
        lon_1: LonPair,
        di: u32,
        gaussian_grid: u32,
        scan_mode: u8,
    },
    PolarStereographic {
        lat_1: Lat,
        lon_1: Lon,
        lov: u32,
        dx: u32,
        dy: u32,
        projection_center: u32,
        scan_mode: u32,
    },

    // NCEP
    SemiStaggered {
        lat: LatPair,
        lon: LonPair,
        di: u32,
        dj: u32,
        scan_mode: u8,
    },
    FilledStaggered,
}

fn gds_parser(input: &[u8]) -> IResult<&[u8], GDS> {
    let (next, gds_length) = u24(nom::number::Endianness::Little)(input)?;

    let nv_p = u8;
    let pv_or_pl_p = u8;

    let r_type_p = u8;
}
