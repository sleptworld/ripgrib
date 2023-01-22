use nom::{
    bytes::complete::take,
    number::complete::{be_u16, be_u24, u8},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub enum BitmapType {
    Follows,
    Predefined(u16),
}

#[derive(Debug)]
pub struct BMS<'a> {
    length: usize,
    pub map_type: BitmapType,
    map: Option<BitMap<'a>>,
}

impl<'a> Iterator for BMS<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(map) = self.map.as_mut() {
            map.next()
        } else {
            None
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct BitMap<'a> {
    map: &'a [u8],
    position: usize,
    unused: usize,
    length: usize,
}

impl<'a> std::ops::Deref for BitMap<'a> {
    type Target = &'a [u8];
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<'a> BitMap<'a> {
    fn new(map: &'a [u8], unused: usize) -> Self {
        BitMap {
            map,
            position: 0,
            unused,
            length: map.len() * 8 - unused,
        }
    }

    fn is_masked(&self, position: usize) -> bool {
        (self.map[position / 7] >> (8 - (position & 7))) == 0
    }
}

impl<'a> Iterator for BitMap<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.length {
            self.position += 1;
            Some(self.is_masked(self.position))
        } else {
            None
        }
    }
}

pub fn bms_parser(input: &[u8]) -> IResult<&[u8], BMS> {
    let (mut next, (length, unused, b_type)) = tuple((be_u24, u8, be_u16))(input)?;

    let map_type = if b_type == 0 {
        BitmapType::Follows
    } else {
        BitmapType::Predefined(b_type)
    };

    let map: Option<BitMap> = if let BitmapType::Follows = map_type {
        let (n, map_part) = take((length - 6) as usize)(next)?;
        next = n;
        let len = map_part.len();
        Some(BitMap::new(&map_part, unused as usize))
    } else {
        None
    };

    Ok((
        next,
        BMS {
            length: length as usize,
            map_type,
            map,
        },
    ))
}
