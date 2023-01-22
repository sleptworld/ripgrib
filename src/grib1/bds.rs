use bitflags::bitflags;
use nom::{
    bytes::complete::take,
    number::complete::{be_i16, be_u24, be_u32, u8},
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

        const RESERVE = 0b00001000;
        const MATRIX_VALUE = 0b00000100;
        const SECONDARY_BIT_MAP = 0b00000010;
        const DIFFERENT_WIDTH = 0b00000001;
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

impl<'a> Default for BDS<'a> {
    fn default() -> Self {
        BDS {
            length: 0,
            flag: BdsFlag::empty(),
            scale: 0,
            refrence_value: 0.0,
            bit_number: 0,
            data: PackedData::Other,
        }
    }
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

    Other,
}

fn float_transer(input: u32) -> f32 {
    ibmfloat::F32::from_bits(input).into()
}

pub fn bds_parser<'a>(input: &'a [u8]) -> IResult<&'a [u8], BDS<'a>> {
    let (next, (length, flag_bit, scale, refrence, number)) =
        tuple((be_u24, u8, be_i16, be_u32, u8))(input)?;

    println!("length:{},flag:{:#b}", length, flag_bit);
    let flags = BdsFlag::from_bits(flag_bit).unwrap();

    if flags.contains(BdsFlag::COMPLEX_PACKING) {
        panic!();
    } else {
        let (next, value) = take((length - 11) as usize)(next)?;

        Ok((
            next,
            BDS {
                length: length as usize,
                flag: flags,
                scale,
                refrence_value: float_transer(refrence),
                bit_number: number as usize,
                data: PackedData::Simple(value),
            },
        ))
    }
}
