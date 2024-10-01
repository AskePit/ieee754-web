use core::ops::Add;
use std::fmt::Display;
use std::ops::AddAssign;

#[derive(Copy, Clone)]
pub struct BitField {
    data: [u32; 8],
    size: usize,
}

impl Default for BitField {
    fn default() -> Self {
        BitField {
            data: [0u32; 8],
            size: 0,
        }
    }
}

impl Display for BitField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        for i in (0..self.size).rev() {
            res += if self.get_bit(i) { "1" } else { "0" };
        }

        write!(f, "{}", res)
    }
}

impl Add for BitField {
    type Output = BitField;

    fn add(self, rhs: Self) -> Self::Output {
        self.concat(&rhs)
    }
}

impl Add for &BitField {
    type Output = BitField;

    fn add(self, rhs: Self) -> Self::Output {
        self.concat(&rhs)
    }
}

impl AddAssign for BitField {
    fn add_assign(&mut self, rhs: Self) {
        self.concat_in_place(&rhs)
    }
}

impl BitField {
    pub const fn max_size() -> usize {
        256usize
    }

    pub const fn block_size() -> usize {
        32usize
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new(size: usize) -> Self {
        BitField {
            size,
            ..Self::default()
        }
    }

    pub fn make_all_zeroes(size: usize) -> BitField {
        BitField::new(size)
    }

    pub fn make_all_ones(size: usize) -> BitField {
        assert!(size < BitField::max_size());

        let mut data = [0u32; 8];

        let blocks_count = size / BitField::block_size();
        let last_ones = size % BitField::block_size();

        if blocks_count > 0 {
            for i in 0..blocks_count {
                let all_ones = u32::MAX;
                data[i] = all_ones;
            }
            data[blocks_count] = (1u32 << last_ones) - 1;
        } else {
            data[0] = (1u32 << last_ones) - 1;
        }

        BitField { data, size }
    }

    pub fn make_u8(number: u8, size: usize) -> BitField {
        assert!(size < BitField::max_size());
        let data = [number as u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
        BitField { data, size }
    }

    pub fn make_u16(number: u16, size: usize) -> BitField {
        assert!(size < BitField::max_size());
        let data = [number as u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
        BitField { data, size }
    }

    pub fn make_u32(number: u32, size: usize) -> BitField {
        assert!(size < BitField::max_size());
        let data = [number, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
        BitField { data, size }
    }

    pub fn make_u64(number: u64, size: usize) -> BitField {
        assert!(size < BitField::max_size());
        let data = [
            (number & 0x00000000FFFFFFFF) as u32,
            (number >> 32) as u32,
            0u32,
            0u32,
            0u32,
            0u32,
            0u32,
            0u32,
        ];
        BitField { data, size }
    }

    pub fn make_u128(number: u128, size: usize) -> BitField {
        assert!(size < BitField::max_size());
        let data = [
            (number & 0x000000000000000000000000FFFFFFFF) as u32,
            ((number & 0x0000000000000000FFFFFFFF00000000) >> 32) as u32,
            ((number & 0x00000000FFFFFFFF0000000000000000) >> 64) as u32,
            (number >> 96) as u32,
            0u32,
            0u32,
            0u32,
            0u32,
        ];
        BitField { data, size }
    }

    pub fn set_bit(&mut self, pos: usize, value: bool) {
        if pos >= self.size {
            return;
        }
        let block_index = pos / BitField::block_size();
        let bit_position = pos % BitField::block_size();
        if value {
            self.data[block_index] |= 1 << bit_position;
        } else {
            self.data[block_index] &= !(1 << bit_position);
        }
    }

    pub fn get_bit(&self, pos: usize) -> bool {
        if pos >= self.size {
            return false;
        }
        let block_index = pos / BitField::block_size();
        let bit_position = pos % BitField::block_size();
        (self.data[block_index] & (1 << bit_position)) != 0
    }

    pub fn concat(&self, other: &BitField) -> BitField {
        let total_bits = self.size + other.size;
        assert!(total_bits < BitField::max_size());

        let mut result = BitField::new(total_bits);

        // Copy bits from `other` (they are the lowest bits now)
        for i in 0..other.size {
            result.set_bit(i, other.get_bit(i));
        }

        // Copy bits from `self` with an offset of `other.size`
        for i in 0..self.size {
            result.set_bit(other.size + i, self.get_bit(i));
        }

        result
    }

    pub fn concat_in_place(&mut self, other: &BitField) {
        *self = self.concat(other);
    }

    pub fn parse(s: &str) -> Option<BitField> {
        BitField::parse_with_size(s, s.len())
    }

    pub fn parse_with_size(s: &str, size: usize) -> Option<BitField> {
        assert!(size <= BitField::max_size());

        let mut result = BitField::new(size);

        for (i, c) in s.chars().rev().enumerate() {
            if c == '1' {
                result.set_bit(i, true);
            } else if c != '0' {
                return None;
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        {
            "0 01111111 00000000000000000000000";
            let sign = BitField::make_all_zeroes(1);
            let mut exponent = BitField::make_all_ones(8);
            exponent.set_bit(7, false);
            let mantissa = BitField::make_all_zeroes(23);

            assert_eq!(
                sign.concat(&exponent).concat(&mantissa).to_string(),
                "00111111100000000000000000000000"
            );
        }
        {
            let sum = BitField::make_u8(0, 1) + BitField::make_all_zeroes(11);
            assert_eq!(sum.to_string(), "000000000000");
        }
        {
            let sum = BitField::make_all_zeroes(12) + BitField::make_all_ones(52);
            assert_eq!(
                sum.to_string(),
                "0000000000001111111111111111111111111111111111111111111111111111"
            );
        }
        {
            let sum = BitField::make_u8(0, 1)
                + BitField::make_all_zeroes(11)
                + BitField::make_all_ones(52);
            assert_eq!(
                sum.to_string(),
                "0000000000001111111111111111111111111111111111111111111111111111"
            );
        }
    }

    #[test]
    fn test_parse() {
        {
            let bit_field = BitField::parse("001111011001").unwrap();
            assert_eq!(bit_field.size(), 12);
            assert_eq!(bit_field.get_bit(0), true);
            assert_eq!(bit_field.get_bit(1), false);
            assert_eq!(bit_field.get_bit(2), false);
            assert_eq!(bit_field.get_bit(3), true);
            assert_eq!(bit_field.get_bit(4), true);
            assert_eq!(bit_field.get_bit(5), false);
            assert_eq!(bit_field.get_bit(6), true);
            assert_eq!(bit_field.get_bit(7), true);
            assert_eq!(bit_field.get_bit(8), true);
            assert_eq!(bit_field.get_bit(9), true);
            assert_eq!(bit_field.get_bit(10), false);
            assert_eq!(bit_field.get_bit(11), false);
        }
        {
            let bit_field = BitField::parse_with_size("001111011001", 4).unwrap();
            assert_eq!(bit_field.size(), 4);
            assert_eq!(bit_field.get_bit(0), true);
            assert_eq!(bit_field.get_bit(1), false);
            assert_eq!(bit_field.get_bit(2), false);
            assert_eq!(bit_field.get_bit(0), true);
        }
        {
            let bit_field = BitField::parse_with_size("101111", 10).unwrap();
            assert_eq!(bit_field.size(), 10);
            assert_eq!(bit_field.get_bit(0), true);
            assert_eq!(bit_field.get_bit(1), true);
            assert_eq!(bit_field.get_bit(2), true);
            assert_eq!(bit_field.get_bit(3), true);
            assert_eq!(bit_field.get_bit(4), false);
            assert_eq!(bit_field.get_bit(5), true);
            assert_eq!(bit_field.get_bit(6), false);
            assert_eq!(bit_field.get_bit(7), false);
            assert_eq!(bit_field.get_bit(8), false);
            assert_eq!(bit_field.get_bit(9), false);
        }
    }
}
