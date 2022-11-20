pub mod r#const;

// This builds on Kernighan's algorithm for counting ones in a bitstring
pub fn lowest_bit(bitstring: u64) -> u64 {
    if bitstring == 0 {
        0
    } else {
        (bitstring & (bitstring - 1)) ^ bitstring
    }
}

pub fn next_lowest_bit(bitstring: u64, singlebit: u64) -> u64 {
    if singlebit == 0x8000000000000000 {
        0
    } else {
        lowest_bit(!((singlebit << 1) - 1) & bitstring)
    }
}

// This will convert into a single popcnt instruction!!
// This will return 65 (gibberish) if passed 0
pub fn get_bit_number(singlebit: u64) -> u8 {
    let count_u32: u32 = singlebit.trailing_zeros() + 1;
    count_u32.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::bitops;

    #[test]
    fn bitops_lowest_bit() {
        assert_eq!(bitops::lowest_bit(0x0000000000000000),0x0000000000000000,
            "empty string");
        assert_eq!(bitops::lowest_bit(0x1000000000000000),0x1000000000000000,
            "left-most");
        assert_eq!(bitops::lowest_bit(0x0000000000000001),0x0000000000000001,
            "right-most sparse");
        assert_eq!(bitops::lowest_bit(0x1001110011011011),0x0000000000000001,
            "right-most dense");
        assert_eq!(bitops::lowest_bit(0x1111111100000000),0x0000000100000000,
            "middle");
    }

    #[test]
    fn bitops_next_lowest_bit() {
        assert_eq!(bitops::next_lowest_bit(0x0000000000000000,0x0000000100000000),
            0x0000000000000000, "empty string");
        assert_eq!(bitops::next_lowest_bit(0x8000000000000000,0x8000000000000000),
            0x0000000000000000, "left-most");
        assert_eq!(bitops::next_lowest_bit(0x0000000000000001,0x0000000000000001),
            0x0000000000000000, "right-most");
        assert_eq!(bitops::next_lowest_bit(0x1001110011011011,0x0000000000000001),
            0x0000000000000010, "dense 1");
        assert_eq!(bitops::next_lowest_bit(0x1001110011011011,0x0000000000001000),
            0x0000000000010000, "dense 2");
        assert_eq!(bitops::next_lowest_bit(0x1111111100000000,0x0000000100000000),
            0x0000001000000000, "middle");
        assert_eq!(bitops::next_lowest_bit(0x1000000000000001,0x0000000000000001),
            0x1000000000000000, "split");
    }

    #[test]
    fn bitops_get_bit_number() {
        assert_eq!(bitops::get_bit_number(0x0000000000000001),1);
        assert_eq!(bitops::get_bit_number(0x0000000000000002),2);
        assert_eq!(bitops::get_bit_number(0x0000000000040000),19);
        assert_eq!(bitops::get_bit_number(0x0000008000000000),40);
        assert_eq!(bitops::get_bit_number(0x0001000000000000),49);
        assert_eq!(bitops::get_bit_number(0x8000000000000000),64);
    }
}
