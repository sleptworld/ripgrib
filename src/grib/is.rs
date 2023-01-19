use nom::{
    bytes::complete::tag,
    number::{complete::le_u24, complete::u8},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
pub struct IS {
    total_length: usize,
    version_number: u8,
}

pub fn is_parser(input: &[u8]) -> IResult<&[u8], IS> {
    let grib = tag([]);
    let total_length = le_u24;
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
