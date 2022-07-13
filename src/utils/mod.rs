
/// Convert a chess "square" (e.g., "b6") to a number
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
                squarenumber += ('h' as u8 - c as u8);
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

/// Convert a chess "square" (e.g., "b6") to a bitstring
/// 
/// # Examples
///
/// ```
/// let mysquare = String::from("b6");
/// let bitstring = convert_square_to_bitstring(square: mysquare);
/// assert_eq!(bitstring,0x0000000000400000,"b6");
/// ```
pub fn convert_square_to_bitstring(square: String) -> u64 {
    let mut bitstring: u64 = 1;
    let squarenumber = convert_square_to_number(square);
    bitstring << (squarenumber - 1)
}

#[cfg(test)]
mod tests {
    use crate::utils;
    #[test]
    fn utils_convert_square_to_bitstring() {
        let result = utils::convert_square_to_bitstring("a1".to_string());
        assert_eq!(result,0x0000000000000080,"a1");
        let result = utils::convert_square_to_bitstring("a8".to_string());
        assert_eq!(result,0x8000000000000000,"a8");
        let result = utils::convert_square_to_bitstring("h1".to_string());
        assert_eq!(result,0x0000000000000001,"h1");
        let result = utils::convert_square_to_bitstring("h8".to_string());
        assert_eq!(result,0x0100000000000000,"h8");
        let result = utils::convert_square_to_bitstring("c2".to_string());
        assert_eq!(result,0x0000000000002000,"c2");
        let result = utils::convert_square_to_bitstring("g5".to_string());
        assert_eq!(result,0x0000000200000000,"g5");
    }

    #[test]
    fn utils_convert_square_to_number() {
        let result = utils::convert_square_to_number("a1".to_string());
        assert_eq!(result,8,"a1");
        let result = utils::convert_square_to_number("a8".to_string());
        assert_eq!(result,64,"a8");
        let result = utils::convert_square_to_number("h1".to_string());
        assert_eq!(result,1,"h1");
        let result = utils::convert_square_to_number("h8".to_string());
        assert_eq!(result,57,"h8");
        let result = utils::convert_square_to_number("c2".to_string());
        assert_eq!(result,14,"c2");
        let result = utils::convert_square_to_number("g5".to_string());
        assert_eq!(result,34,"g5");
    }
}

