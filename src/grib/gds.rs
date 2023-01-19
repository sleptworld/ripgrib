use lazy_static::lazy_static;

use nom::{
    bytes::complete::take,
    character::complete::u8,
    multi::count,
    number::complete::{be_u16, le_i16, le_i24, le_u24},
    sequence::tuple,
    IResult,
};
use std::{borrow::Borrow, collections::HashMap};

use bitflags::bitflags;

// Deal with data representation type and Projection
// ....
lazy_static! {
    pub static ref E: DP = DP::default();
}

// Type declare
#[derive(Debug)]
pub struct GDS<'a> {
    pub nv: usize,
    pub representation: DataRepresentation,
    pv: Option<&'a [u8]>,
    pl: Option<&'a [u8]>,
}

pub type Lat = i32;
pub type Lon = i32;

pub type LatPair = (Lat, Lat);
pub type LonPair = (Lon, Lon);

#[derive(Clone, Copy, Debug)]
pub struct DataRepresentation {
    row_length: usize,
    col_length: usize,
    projection: Projection,
}

impl DataRepresentation {
    pub fn get_total(&self) -> usize {
        self.row_length * self.col_length
    }

    pub fn get_row_length(&self) -> usize {
        self.row_length
    }

    pub fn get_col_length(&self) -> usize {
        self.col_length
    }
}

impl Default for DataRepresentation {
    fn default() -> Self {
        DataRepresentation {
            row_length: 0,
            col_length: 0,
            projection: Projection::FilledStaggered,
        }
    }
}

// Projections
#[derive(Debug, Clone, Copy)]
pub enum Projection {
    // Plate Carree Projecion(including GAUSSIAN)
    LatLon {
        ni: usize,
        nj: usize,
        lat: LatPair,
        lon: LonPair,
        res_com_flag: ResComFlags,
        di: i16,
        regular_grib: i16,
        scan_mode: ScanningFlags,
    },
    Mercator {
        ni: usize,
        nj: usize,
        lon: LonPair,
        lat: LatPair,
        latin: u32,
        di: i32,
        dj: i32,
        scan_mode: ScanningFlags,
    },
    Gnomonic,
    LambertConformal {
        nx: usize,
        ny: usize,
        lat_1: Lat,
        lon_1: Lon,

        resolution: ResComFlags,
        lov: i32,
        dx: i32,
        dy: i32,
        projection_center: u8,
        scan_mode: ScanningFlags,
        latin_1: i32,
        latin_2: i32,
        southern_pole_lon: i32,
        southern_pole_lat: i32,
    },
    Gaussian {
        lat: LatPair,
        lon_1: LonPair,
        di: u32,
        gaussian_grid: u32,
        scan_mode: u8,
    },
    PolarStereographic {
        nx: usize,
        ny: usize,
        lat_1: Lat,
        lon_1: Lon,
        resolution: ResComFlags,
        lov: i32,
        dx: i32,
        dy: i32,
        projection_center: u8,
        scan_mode: ScanningFlags,
    },

    // NCEP
    SemiStaggered {
        ni: usize,
        nj: usize,
        lat: LatPair,
        lon: LonPair,
        resolution: ResComFlags,
        di: i32,
        dj: i32,
        scan_mode: ScanningFlags,
    },
    FilledStaggered,
}

bitflags! {
    pub struct ScanningFlags:u8 {
        const IINCREASE = 0b10000000;
        const JINCREASE = 0b01000000;
        const FORTRAN_MODE = 0b00100000;
    }
    // Resolution and Component flags
    // GDS Octet 17
    pub struct ResComFlags:u8 {
        const DIRE_INCRES = 0b10000000;
        // If earth assumed spherical with radius = 6367.47km
        const EARTH_RADIUS = 0b01000000;
        // If u/v components of vector quantities resolved relative to easterly and northerly directions
        const U_V_RESO = 0b00001000;
    }

}

pub struct DP {
    events: HashMap<u8, Box<dyn Fn(&[u8]) -> IResult<&[u8], DataRepresentation> + Sync>>,
}

impl Default for DP {
    fn default() -> Self {
        let mut events: HashMap<
            u8,
            Box<dyn Fn(&[u8]) -> IResult<&[u8], DataRepresentation> + Sync>,
        > = HashMap::new();
        events.insert(
            0,
            Box::new(|next| {
                let (next, ni) = be_u16(next)?;
                let (next, nj) = be_u16(next)?;
                let (next, (lat1, lon1, reso, lat2, lon2, di, dj, scan_mode, _)) =
                    tuple((
                        le_i24,
                        le_i24,
                        u8,
                        le_i24,
                        le_i24,
                        le_i16,
                        le_i16,
                        u8,
                        take(4usize),
                    ))(next)?;

                return Ok((
                    next,
                    DataRepresentation {
                        col_length: ni as usize,
                        row_length: nj as usize,
                        projection: Projection::LatLon {
                            ni: ni as usize,
                            nj: nj as usize,
                            lat: (lat1, lat2),
                            lon: (lon1, lon2),
                            res_com_flag: ResComFlags::from_bits(reso).unwrap(),
                            di,
                            regular_grib: dj,
                            scan_mode: ScanningFlags::from_bits(scan_mode).unwrap(),
                        },
                    },
                ));
            }),
        );
        events.insert(
            1,
            Box::new(|next| {
                let (next, ni) = be_u16(next)?;
                let (next, nj) = be_u16(next)?;
                let (next, (lat1, lon1, reso, lat2, lon2, latin, _, scan_mode, dij, _)) =
                    tuple((
                        le_i24,
                        le_i24,
                        u8,
                        le_i24,
                        le_i24,
                        le_u24,
                        take(1usize),
                        u8,
                        count(le_i24, 2),
                        take(8usize),
                    ))(next)?;

                return Ok((
                    next,
                    DataRepresentation {
                        col_length: ni as usize,
                        row_length: nj as usize,
                        projection: Projection::Mercator {
                            ni: ni as usize,
                            nj: nj as usize,
                            lon: (lon1, lon2),
                            lat: (lat1, lat2),
                            latin: (latin),
                            di: (dij[0]),
                            dj: (dij[1]),
                            scan_mode: ScanningFlags::from_bits(scan_mode).unwrap(),
                        },
                    },
                ));
            }),
        );
        events.insert(
            3,
            Box::new(|next| {
                let (next, nx) = be_u16(next)?;
                let (next, ny) = be_u16(next)?;
                let (next, (lat1, lon1, reso, lxy, center, scan, lls, _)) =
                    tuple((
                        le_i24,
                        le_i24,
                        u8,
                        count(le_i24, 3),
                        u8,
                        u8,
                        count(le_i24, 4),
                        take(2usize),
                    ))(next)?;

                return Ok((
                    next,
                    DataRepresentation {
                        col_length: nx as usize,
                        row_length: ny as usize,
                        projection: Projection::LambertConformal {
                            nx: nx as usize,
                            ny: ny as usize,
                            lat_1: lat1,
                            lon_1: lon1,
                            resolution: ResComFlags::from_bits(reso).unwrap(),
                            lov: lxy[0],
                            dx: lxy[1],
                            dy: lxy[2],
                            projection_center: center,
                            scan_mode: ScanningFlags::from_bits(scan).unwrap(),
                            latin_1: lls[0],
                            latin_2: lls[1],
                            southern_pole_lon: lls[2],
                            southern_pole_lat: lls[3],
                        },
                    },
                ));
            }),
        );

        events.insert(
            5,
            Box::new(|next| {
                let (next, nx) = be_u16(next)?;
                let (next, ny) = be_u16(next)?;

                let (next, (lat1, lon1, reso, lxy, center, scan, _)) =
                    tuple((le_i24, le_i24, u8, count(le_i24, 3), u8, u8, take(4usize)))(next)?;

                return Ok((
                    next,
                    DataRepresentation {
                        col_length: nx as usize,
                        row_length: ny as usize,
                        projection: Projection::PolarStereographic {
                            nx: nx as usize,
                            ny: ny as usize,
                            lat_1: lat1,
                            lon_1: lon1,
                            resolution: ResComFlags::from_bits(reso).unwrap(),
                            lov: lxy[0],
                            dx: lxy[1],
                            dy: lxy[2],
                            projection_center: center,
                            scan_mode: ScanningFlags::from_bits(scan).unwrap(),
                        },
                    },
                ));
            }),
        );

        events.insert(
            201,
            Box::new(|next| {
                let (next, ni) = be_u16(next)?;
                let (next, nj) = be_u16(next)?;

                let (next, (lat1, lon1, reso, lij, scan, _)) =
                    tuple((le_i24, le_i24, u8, count(le_i24, 4), u8, take(4usize)))(next)?;

                return Ok((
                    next,
                    DataRepresentation {
                        col_length: ni as usize,
                        row_length: nj as usize,
                        projection: Projection::SemiStaggered {
                            ni: ni as usize,
                            nj: nj as usize,
                            lat: (lat1, lij[0]),
                            lon: (lon1, lij[1]),
                            resolution: ResComFlags::from_bits(reso).unwrap(),
                            di: lij[2],
                            dj: lij[3],
                            scan_mode: ScanningFlags::from_bits(scan).unwrap(),
                        },
                    },
                ));
            }),
        );

        DP { events }
    }
}

impl DP {
    fn run_event<'a>(&self, next: &'a [u8], e: u8) -> IResult<&'a [u8], DataRepresentation> {
        if let Some(func) = self.events.get(&e).borrow() {
            return func(next);
        } else {
            return Ok((next, DataRepresentation::default()));
        }
    }

    pub fn open(&mut self) {}
}

fn representation_parser(input: &[u8]) -> IResult<&[u8], DataRepresentation> {
    let (next, projection_type_mask) = u8(input)?;
    return E.run_event(next, projection_type_mask);
}

pub fn gds_parser(input: &[u8]) -> IResult<&[u8], GDS> {
    let (next, gds_length) = le_u24(input)?;
    let nv_p = u8;
    let pv_or_pl_p = u8;

    let (next, (nv, pv_pl, data_repr)) = tuple((nv_p, pv_or_pl_p, representation_parser))(next)?;

    if nv == 0 {
        assert!(pv_pl != 255);
        let (next, pl_list) = take(data_repr.row_length * 2)(next)?;
        return Ok((
            next,
            GDS {
                nv: nv as usize,
                representation: data_repr,
                pv: None,
                pl: Some(pl_list),
            },
        ));
    } else {
        let (next, (pv_list, pl_list)) =
            tuple((take(nv * 4), take(data_repr.row_length * 2)))(next)?;
        return Ok((
            next,
            GDS {
                nv: nv as usize,
                representation: data_repr,
                pv: Some(pv_list),
                pl: Some(pl_list),
            },
        ));
    }
}
