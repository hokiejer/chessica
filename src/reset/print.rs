use crate::reset::Reset;
use crate::utils::convert_bitstring_to_square;

/// Prints a Reset
/// 
/// # Examples
///
/// ```
/// # use chessica::reset::Reset;
/// let mut r = chessica::reset::new();
/// r.print();
/// ```
impl Reset {
    pub fn print(&mut self) -> String {
        let piece_text = if self.b_to & self.b_pawns != 0 {
            if self.white_to_move() {
                "p"
            } else {
                "P"
            }
        } else if self.b_to & self.b_knights != 0 {
            if self.white_to_move() {
                "n"
            } else {
                "N"
            }
        } else if self.b_to & self.b_bishops != 0 {
            if self.white_to_move() {
                "b"
            } else {
                "B"
            }
        } else if self.b_to & self.b_rooks != 0 {
            if self.white_to_move() {
                "r"
            } else {
                "R"
            }
        } else if self.b_to & self.b_kings != 0 {
            if self.white_to_move() {
                "k"
            } else {
                "K"
            }
        } else if self.white_to_move() {
            "q"
        } else {
            "Q"
        };
        let from_text = convert_bitstring_to_square(self.b_from);
        let to_text = convert_bitstring_to_square(self.b_to);
        println!("{}:{}-{} => {}",piece_text,from_text,to_text,self.to_fen());
        self.to_fen()
    }

    pub fn print_board_small(&mut self) {
        let mut b_index: u64 = 0x8000000000000000;
        let mut increment: u8 = 0;
        let mut print_string: String = "".to_string();
        self.print();
        while b_index > 0 {
            if b_index & self.b_all == 0 {
                print_string.push_str(&". ".to_string());
            } else {
                if b_index & self.b_white != 0 {
                    if b_index & self.b_pawns != 0 {
                        print_string.push_str(&"P ".to_string());
                    } else if b_index & self.b_knights != 0 {
                        print_string.push_str(&"N ".to_string());
                    } else if b_index & self.b_bishops != 0 {
                        print_string.push_str(&"B ".to_string());
                    } else if b_index & self.b_rooks != 0 {
                        print_string.push_str(&"R ".to_string());
                    } else if b_index & self.b_kings != 0 {
                        print_string.push_str(&"K ".to_string());
                    } else {
                        print_string.push_str(&"Q ".to_string());
                    }
                } else {
                    if b_index & self.b_pawns != 0 {
                        print_string.push_str(&"p ".to_string());
                    } else if b_index & self.b_knights != 0 {
                        print_string.push_str(&"n ".to_string());
                    } else if b_index & self.b_bishops != 0 {
                        print_string.push_str(&"b ".to_string());
                    } else if b_index & self.b_rooks != 0 {
                        print_string.push_str(&"r ".to_string());
                    } else if b_index & self.b_kings != 0 {
                        print_string.push_str(&"k ".to_string());
                    } else {
                        print_string.push_str(&"q ".to_string());
                    }
                }
            }
            b_index >>= 1;
            increment += 1;
            if increment % 8 == 0 {
                println!("{}",print_string);
                print_string = "".to_string();
            }
        }
    }

    pub fn print_board_big(&mut self) {
        use ansi_term::Colour;
        let mut b_index: u64 = 0x8000000000000000;
        let mut increment: u8 = 0;
        let mut row: u8 = 8;
        let mut col: u8 = 1;
        let mut level: u8 = 1;
        self.print();
        while row > 0 {
            let mut style;
            if col == 1 {
            }
            if (row + col) % 2 == 0 { // Black Square
                style = Colour::RGB(0,0,0).on(Colour::RGB(110,110,110));
                print!("{}", style.paint(" "));
            } else { // White Square
                style = Colour::RGB(0,0,0).on(Colour::RGB(60,60,60));
                print!("{}", style.paint(" "));
            }
            if level == 2 {
                if b_index & self.b_all == 0 {
                    if b_index & self.b_from != 0 {
                        if (row + col) % 2 == 0 { // Black Square
                            style = Colour::RGB(0,0,0).on(Colour::RGB(120,120,100));
                        } else { // White Square
                            style = Colour::RGB(0,0,0).on(Colour::RGB(70,70,50));
                        }
                    }
                    print!("{}", style.paint("   "));
                } else {
                    if b_index & self.b_white != 0 {
                        if b_index & self.b_to != 0 {
                            style = Colour::RGB(0,0,0).on(Colour::RGB(255,255,255)).blink();
                        } else {
                            style = Colour::RGB(0,0,0).on(Colour::RGB(255,255,255));
                        }
                        if b_index & self.b_pawns != 0 {
                            print!("{}", style.paint(" P "));
                        } else if b_index & self.b_knights != 0 {
                            print!("{}", style.paint(" N "));
                        } else if b_index & self.b_bishops != 0 {
                            print!("{}", style.paint(" B "));
                        } else if b_index & self.b_rooks != 0 {
                            print!("{}", style.paint(" R "));
                        } else if b_index & self.b_kings != 0 {
                            print!("{}", style.paint(" K "));
                        } else {
                            print!("{}", style.paint(" Q "));
                        }
                    } else {
                        if b_index & self.b_to != 0 {
                            style = Colour::RGB(255,255,255).on(Colour::RGB(0,0,0)).blink();
                        } else {
                            style = Colour::RGB(255,255,255).on(Colour::RGB(0,0,0));
                        }
                        if b_index & self.b_pawns != 0 {
                            print!("{}", style.paint(" P "));
                        } else if b_index & self.b_knights != 0 {
                            print!("{}", style.paint(" N "));
                        } else if b_index & self.b_bishops != 0 {
                            print!("{}", style.paint(" B "));
                        } else if b_index & self.b_rooks != 0 {
                            print!("{}", style.paint(" R "));
                        } else if b_index & self.b_kings != 0 {
                            print!("{}", style.paint(" K "));
                        } else {
                            print!("{}", style.paint(" Q "));
                        }
                    }
                }
                b_index >>= 1;
            } else {
                print!("{}", style.paint("   "));
            }
            if (row + col) % 2 == 0 { // Black Square
                style = Colour::RGB(0,0,0).on(Colour::RGB(110,110,110));
                print!("{}", style.paint(" "));
            } else { // White Square
                style = Colour::RGB(0,0,0).on(Colour::RGB(60,60,60));
                print!("{}", style.paint(" "));
            }
            increment += 1;
            col += 1;
            if increment % 8 == 0 {
                println!("");
                level += 1;
                if level > 3 {
                    level = 1;
                    row -= 1;
                }
                col = 1;
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::reset;

    #[test]
    fn test_print_board_small() {
        let mut r = reset::new();
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        r.init_from_fen(fen);
        r.print_board_small();
        assert!(true);
    }

}
