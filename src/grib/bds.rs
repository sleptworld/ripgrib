use bitflags::bitflags;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_i16, le_u24, u8},
    sequence::tuple,
    IResult,
};

use super::{bms::BMS, gds::GDS};
const UNDIFINED: u32 = 0;

bitflags! {

    pub struct BdsFlag:u8{
        const SPHERICAL_HARMONIC = 0b10000000;
        const COMPLEX_PACKING = 0b01000000;
        const INTEGER_NUMBERIC = 0b00100000;
        const ADDITIONAL_FLAG = 0b00010000;
    }

}

#[derive(Debug)]
pub struct BDS<'a> {
    length: usize,
    flag: BdsFlag,
    scale: i16,
    refrence_value: f32,
    // Number of bits into which a datum point is packed.
    bit_number: usize,
    data: PackedData<'a>,
}

#[derive(Debug)]
enum PackedData<'a> {
    Simple(&'a [u8]),
    Complex {
        n1: u16,
        extension_flag: u8,
        n2: u16,
        p1: u16,
        p2: u16,
        width: &'a [u8],
        secondary_bitmap: &'a [u8],
        first_order_packed_values: &'a [u8],
        second_order_packed_values: &'a [u8],
    },
}

pub fn bds_parser<'a>(input: &'a [u8]) -> IResult<&'a [u8], BDS<'a>> {
    let (next, (length, flag_bit, scale, refrence, number)) =
        tuple((le_u24, u8, le_i16, le_f32, u8))(input)?;
    let flags = BdsFlag::from_bits(flag_bit).unwrap();

    if flags.contains(BdsFlag::COMPLEX_PACKING) {
        panic!();
    } else {
        let (next, value) = take(length - 11)(next)?;
        Ok((
            next,
            BDS {
                length: length as usize,
                flag: flags,
                scale,
                refrence_value: refrence,
                bit_number: number as usize,
                data: PackedData::Simple(value),
            },
        ))
    }
    // // Grid-point data - Simple packing
    // let g_s = BdsFlag::DATA_TYPE | BdsFlag::PACKING_TYPE;
    // let s_s = !BdsFlag::DATA_TYPE | BdsFlag::PACKING_TYPE;
    // let g_c = BdsFlag::DATA_TYPE | !BdsFlag::PACKING_TYPE;
    // // total length
    // let total_length: usize = gds.representation.get_total();

    // if flags.contains(g_s) {
    //     if let Some(b) = bms.as_ref() {
    //         for v in bms {}
    //     }
    // }
}
