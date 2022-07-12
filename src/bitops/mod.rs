// This builds on Kernighan's algorithm for counting ones in a bitstring
pub fn lowest_bit(bitstring: u64) -> u64 {
    if bitstring == 0 {
        0
    } else {
        (bitstring & (bitstring - 1)) ^ bitstring
    }
}

pub fn next_lowest_bit(bitstring: u64, singlebit: u64) -> u64 {
    lowest_bit(!((singlebit << 1) - 1) & bitstring)
}

// I think O(log(log(n)) is the best I can do here
pub fn get_bit_number(singlebit: u64) -> u32 {
    let mut result: u32 = 1;
    if singlebit & 0xffffffff00000000 > 0 {
        result += 32;
    }
    if singlebit & 0xffff0000ffff0000 > 0 {
        result += 16;
    }
    if singlebit & 0xff00ff00ff00ff00 > 0 {
        result += 8;
    }
    if singlebit & 0xf0f0f0f0f0f0f0f0 > 0 {
        result += 4;
    }
    if singlebit & 0xcccccccccccccccc > 0 {
        result += 2;
    }
    if singlebit & 0xaaaaaaaaaaaaaaaa > 0 {
        result += 1;
    }
    result
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