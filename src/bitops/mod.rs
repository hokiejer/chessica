pub fn lowest_bit(bitstring: u64) -> u64 {
    if (bitstring == 0) {
        0
    } else {
        (bitstring & (bitstring - 1)) ^ bitstring
    }
}

pub fn next_lowest_bit(bitstring: u64, singlebit: u64) -> u64 {
    lowest_bit(!((singlebit << 1) - 1) & bitstring)
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
        assert_eq!(bitops::next_lowest_bit(0x1000000000000000,0x1000000000000000),
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

}
