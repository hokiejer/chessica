
/// Convert a chess "square" (e.g., "b6") to a position number
/// 
/// # Examples
///
/// ```
/// let mysquare = String::from("b6");
/// let squarenumber = convert_square_to_number(square: mysquare);
/// assert_eq!(bitstring,47,"b6");
/// ```
pub fn convert_square_to_number(square: String) -> u8 {
    let mut squarenumber: u8 = 1;
    for c in square.chars() {
        match c {
            'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h' => {
                squarenumber += 'h' as u8 - c as u8;
            },
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8' => {
                squarenumber += 8 * ((c as u8) - ('1' as u8));
                println!("squarenumber = {}",squarenumber);
            },
            _ => println!("I don't know what to do with {}",c),
        }
    }
    squarenumber
}

/// Convert a chess position number to a "square" (e.g., "b6")
/// 
/// # Examples
///
/// ```
/// let square = convert_number_to_square("b6");
/// assert_eq!(square,47,"b6");
/// ```
pub fn convert_number_to_square(number: u8) -> String {
    let mut square = String::from("");
    let mut file: u8 = 'a' as u8;
    file += 7 - ((number - 1) % 8);
    square.push(file as char);
    let mut rank: u8 = '1' as u8;
    rank += (number - 1) / 8;
    square.push(rank as char);
    square
}

/// Convert a chess "square" (e.g., "b6") to a bitstring
/// 
/// # Examples
///
/// ```
/// let mysquare = String::from("b6");
/// let bitstring = convert_square_to_bitstring(square: mysquare);
/// assert_eq!(bitstring,0x0000400000000000,"b6");
/// ```
pub fn convert_square_to_bitstring(square: String) -> u64 {
    let bitstring: u64 = 1;
    let squarenumber = convert_square_to_number(square);
    bitstring << (squarenumber - 1)
}

/// Convert a bitstring to a "square" (e.g., "b6")
/// 
/// # Examples
///
/// ```
/// let bitstring = 0x0000000000010000;
/// let square = convert_bitstring_to_square(bitstring);
/// assert_eq!(square,"b6","b6");
/// ```
pub fn convert_bitstring_to_square(bitstring: u64) -> String {
    use crate::bitops;
    let number = bitops::get_bit_number(bitstring);
    convert_number_to_square(number)
}

#[cfg(test)]
mod tests {
    use crate::utils;
    #[test]
    fn utils_convert_square_to_bitstring() {
        let test_data = [
            ("a1", 0x0000000000000080u64),
            ("a8", 0x8000000000000000u64),
            ("h1", 0x0000000000000001u64),
            ("h8", 0x0100000000000000u64),
            ("c2", 0x0000000000002000u64),
            ("g5", 0x0000000200000000u64),
        ];
        for tuple in test_data.iter() {
          let (square,bitstring) = tuple;
          let result = utils::convert_square_to_bitstring(square.to_string());
          assert_eq!(result,*bitstring,"{}",square.to_string());
          let result = utils::convert_bitstring_to_square(*bitstring);
          assert_eq!(result,*square,"{}",square.to_string());
        }
    }

    #[test]
    fn utils_convert_square_to_number_and_back() {
        let test_data = [
            (8u8,  "a1"),
            (64u8, "a8"),
            (1u8,  "h1"),
            (57u8, "h8"),
            (14u8, "c2"),
            (34u8, "g5"),
        ];
        for tuple in test_data.iter() {
            let (number,square) = tuple;
            let result = utils::convert_square_to_number(square.to_string());
            assert_eq!(result,*number,"{}",square);
            let result = utils::convert_number_to_square(*number);
            assert_eq!(result,*square,"{}",square);
        }
    }
}

