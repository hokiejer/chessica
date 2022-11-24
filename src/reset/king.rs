use crate::reset::Reset;
use crate::reset::r#const::B_NOT_N_EDGE;
use crate::reset::r#const::B_NOT_NE_EDGE;
use crate::reset::r#const::B_NOT_E_EDGE;
use crate::reset::r#const::B_NOT_SE_EDGE;
use crate::reset::r#const::B_NOT_S_EDGE;
use crate::reset::r#const::B_NOT_SW_EDGE;
use crate::reset::r#const::B_NOT_W_EDGE;
use crate::reset::r#const::B_NOT_NW_EDGE;
use crate::reset::r#const::B_WHITE_CASTLEK_SAFETY;
use crate::reset::r#const::B_WHITE_CASTLEQ_SAFETY;
use crate::reset::r#const::B_BLACK_CASTLEK_SAFETY;
use crate::reset::r#const::B_BLACK_CASTLEQ_SAFETY;
use crate::reset::r#const::B_WHITE_CASTLEK_EMPTY;
use crate::reset::r#const::B_WHITE_CASTLEQ_EMPTY;
use crate::reset::r#const::B_BLACK_CASTLEK_EMPTY;
use crate::reset::r#const::B_BLACK_CASTLEQ_EMPTY;
use crate::reset::r#const::B_WHITE_CASTLEK_DESTINATION;
use crate::reset::r#const::B_WHITE_CASTLEQ_DESTINATION;
use crate::reset::r#const::B_BLACK_CASTLEK_DESTINATION;
use crate::reset::r#const::B_BLACK_CASTLEQ_DESTINATION;

impl Reset {
    pub fn generate_next_king_move(&mut self, child: &mut Reset) -> bool {

        let b_available_moves: u64 = if self.white_to_move() {
            !self.b_white
        } else {
            self.b_white | !self.b_all
        };

        // North
        if self.move_id < 20 && (self.b_current_piece & B_NOT_N_EDGE != 0) {
            let b_destination = self.b_current_piece << 8;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_full_safety_check(child, b_destination)) 
            {
                self.move_id = 20;
                self.valid_child_post_processing(child);
                return true;
            }
        }

        // Northeast
        if self.move_id < 30 && (self.b_current_piece & B_NOT_NE_EDGE != 0) {
            let b_destination = self.b_current_piece << 7;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_full_safety_check(child, b_destination)) 
            {
                self.move_id = 30;
                self.valid_child_post_processing(child);
                return true;
            }
        }

        // East
        if self.move_id < 40 && (self.b_current_piece & B_NOT_E_EDGE != 0) {
            let b_destination = self.b_current_piece >> 1;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_full_safety_check(child, b_destination)) 
            {
                self.move_id = 40;
                self.valid_child_post_processing(child);
                return true;
            }
        }

        // Southeast
        if self.move_id < 50 && (self.b_current_piece & B_NOT_SE_EDGE != 0) {
            let b_destination = self.b_current_piece >> 9;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_full_safety_check(child, b_destination)) 
            {
                self.move_id = 50;
                self.valid_child_post_processing(child);
                return true;
            }
        }

        // South
        if self.move_id < 60 && (self.b_current_piece & B_NOT_S_EDGE != 0) {
            let b_destination = self.b_current_piece >> 8;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_full_safety_check(child, b_destination)) 
            {
                self.move_id = 60;
                self.valid_child_post_processing(child);
                return true;
            }
        }

        // Southwest
        if self.move_id < 70 && (self.b_current_piece & B_NOT_SW_EDGE != 0) {
            let b_destination = self.b_current_piece >> 7;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_full_safety_check(child, b_destination)) 
            {
                self.move_id = 70;
                self.valid_child_post_processing(child);
                return true;
            }
        }

        // West
        if self.move_id < 80 && (self.b_current_piece & B_NOT_W_EDGE != 0) {
            let b_destination = self.b_current_piece << 1;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_full_safety_check(child, b_destination)) 
            {
                self.move_id = 80;
                self.valid_child_post_processing(child);
                return true;
            }
        }

        // Northwest
        if self.move_id < 90 && (self.b_current_piece & B_NOT_NW_EDGE != 0) {
            let b_destination = self.b_current_piece << 9;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_full_safety_check(child, b_destination)) 
            {
                self.move_id = 90;
                self.valid_child_post_processing(child);
                return true;
            }
        }

        if self.white_to_move() {

            // White Castle Kingside
            if self.move_id < 100 && 
                self.white_castle_k() &&
                (self.b_all & B_WHITE_CASTLEK_EMPTY == 0) &&
                self.white_is_safe(B_WHITE_CASTLEK_SAFETY)
            {
                self.add_move_unconditional(child, B_WHITE_CASTLEK_DESTINATION);
                child.b_all &= 0xfffffffffffffffe;
                child.b_white &= 0xfffffffffffffffe;
                child.b_rooks &= 0xfffffffffffffffe;
                child.b_all |= 0x0000000000000004;
                child.b_white |= 0x0000000000000004;
                child.b_rooks |= 0x0000000000000004;
                if !child.black_is_safe(child.b_kings & child.b_black()) {
                    child.in_check = 1;
                }
                self.move_id = 100;
                self.valid_child_post_processing(child);
                return true;
            }

            // White Castle Queenside
            if self.move_id < 110 && 
                self.white_castle_q() &&
                (self.b_all & B_WHITE_CASTLEQ_EMPTY == 0) &&
                self.white_is_safe(B_WHITE_CASTLEQ_SAFETY)
            {
                self.add_move_unconditional(child, B_WHITE_CASTLEQ_DESTINATION);
                child.b_all &= 0xffffffffffffff7f;
                child.b_white &= 0xffffffffffffff7f;
                child.b_rooks &= 0xffffffffffffff7f;
                child.b_all |= 0x0000000000000010;
                child.b_white |= 0x0000000000000010;
                child.b_rooks |= 0x0000000000000010;
                if !child.black_is_safe(child.b_kings & child.b_black()) {
                    child.in_check = 1;
                }
                self.consider_next_moveable_piece();
                self.valid_child_post_processing(child);
                return true;
            }

        } else {

            // Black Castle Kingside
            if self.move_id < 100 && 
                self.black_castle_k() &&
                (self.b_all & B_BLACK_CASTLEK_EMPTY == 0) &&
                self.black_is_safe(B_BLACK_CASTLEK_SAFETY)
            {
                self.add_move_unconditional(child, B_BLACK_CASTLEK_DESTINATION);
                child.b_all &= 0xfeffffffffffffff;
                child.b_rooks &= 0xfeffffffffffffff;
                child.b_all |= 0x0400000000000000;
                child.b_rooks |= 0x0400000000000000;
                if !child.white_is_safe(child.b_kings & child.b_white) {
                    child.in_check = 1;
                }
                self.move_id = 100;
                self.valid_child_post_processing(child);
                return true;
            }

            // Black Castle Queenside
            if self.move_id < 110 && 
                self.black_castle_q() &&
                (self.b_all & B_BLACK_CASTLEQ_EMPTY == 0) &&
                self.black_is_safe(B_BLACK_CASTLEQ_SAFETY)
            {
                self.add_move_unconditional(child, B_BLACK_CASTLEQ_DESTINATION);
                child.b_all &= 0x7fffffffffffffff;
                child.b_rooks &= 0x7fffffffffffffff;
                child.b_all |= 0x1000000000000000;
                child.b_rooks |= 0x1000000000000000;
                if !child.white_is_safe(child.b_kings & child.b_white) {
                    child.in_check = 1;
                }
                self.consider_next_moveable_piece();
                self.valid_child_post_processing(child);
                return true;
            }
        }

        self.consider_next_moveable_piece();
        false
    }

}


#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::utils;

    pub fn prep_board(fen: &str) -> Reset {
        let mut r = reset::new();
        let fen = String::from(fen);
        r.init_from_fen(fen);
        r
    }

    #[test]
    fn white_king_moves_basic() {
        let mut r = prep_board("8/8/3k4/8/8/2K5/8/8 w - - 0 1");
        let mut child = reset::new();
        r.current_piece_init("c3");

        // c3 to c4
        let fen = String::from("8/8/3k4/8/2K5/8/8/8 b - - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // c3 to d4
        let fen = String::from("8/8/3k4/8/3K4/8/8/8 b - - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,30);
        assert_eq!(child.capture,0);

        // c3 to d3
        let fen = String::from("8/8/3k4/8/8/3K4/8/8 b - - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,0);

        // c3 to d2
        let fen = String::from("8/8/3k4/8/8/8/3K4/8 b - - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,50);
        assert_eq!(child.capture,0);

        // c3 to c2
        let fen = String::from("8/8/3k4/8/8/8/2K5/8 b - - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,60);
        assert_eq!(child.capture,0);

        // c3 to b2
        let fen = String::from("8/8/3k4/8/8/8/1K6/8 b - - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,70);
        assert_eq!(child.capture,0);

        // c3 to c2
        let fen = String::from("8/8/3k4/8/8/1K6/8/8 b - - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,80);
        assert_eq!(child.capture,0);

        // c3 to d2
        let fen = String::from("8/8/3k4/8/1K6/8/8/8 b - - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,90);
        assert_eq!(child.capture,0);

        let retval = r.generate_next_king_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,0);
    }

    #[test]
    fn black_king_moves_basic() {
        let mut r = prep_board("8/8/3k4/8/8/2K5/8/8 b - - 0 1");
        let mut child = reset::new();
        r.current_piece_init("d6");

        // d6 to d7
        let fen = String::from("8/3k4/8/8/8/2K5/8/8 w - - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d6".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // d6 to e7
        let fen = String::from("8/4k3/8/8/8/2K5/8/8 w - - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d6".to_string()));
        assert_eq!(r.move_id,30);
        assert_eq!(child.capture,0);

        // d6 to e6
        let fen = String::from("8/8/4k3/8/8/2K5/8/8 w - - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d6".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,0);

        // d6 to e5
        let fen = String::from("8/8/8/4k3/8/2K5/8/8 w - - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d6".to_string()));
        assert_eq!(r.move_id,50);
        assert_eq!(child.capture,0);

        // d6 to d5
        let fen = String::from("8/8/8/3k4/8/2K5/8/8 w - - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d6".to_string()));
        assert_eq!(r.move_id,60);
        assert_eq!(child.capture,0);

        // d6 to c5
        let fen = String::from("8/8/8/2k5/8/2K5/8/8 w - - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d6".to_string()));
        assert_eq!(r.move_id,70);
        assert_eq!(child.capture,0);

        // d6 to c6
        let fen = String::from("8/8/2k5/8/8/2K5/8/8 w - - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d6".to_string()));
        assert_eq!(r.move_id,80);
        assert_eq!(child.capture,0);

        // d6 to c7
        let fen = String::from("8/2k5/8/8/8/2K5/8/8 w - - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d6".to_string()));
        assert_eq!(r.move_id,90);
        assert_eq!(child.capture,0);

        let retval = r.generate_next_king_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,0);
    }

    #[test]
    fn white_king_castle_kingside_valid() {
        let mut r = prep_board("r1bqkbnr/pppp2pp/2n2p2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");
        let mut child = reset::new();
        r.current_piece_init("e1");

        // e1 to e2
        let fen = String::from("r1bqkbnr/pppp2pp/2n2p2/1B2p3/4P3/5N2/PPPPKPPP/RNBQ3R b kq - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e1".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // e1 to f1
        let fen = String::from("r1bqkbnr/pppp2pp/2n2p2/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1K1R b kq - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e1".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,0);

        // e1 castle kingside
        let fen = String::from("r1bqkbnr/pppp2pp/2n2p2/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e1".to_string()));
        assert_eq!(r.move_id,100);
        assert_eq!(child.capture,0);

        let retval = r.generate_next_king_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d1".to_string()));
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn white_king_castle_queenside_valid() {
        let mut r = prep_board("rnb1kb1r/ppp1qppp/4pn2/3p4/3P1B2/2N5/PPPQPPPP/R3KBNR w KQkq - 0 1");
        let mut child = reset::new();
        r.current_piece_init("e1");

        // e1 to d1
        let fen = String::from("rnb1kb1r/ppp1qppp/4pn2/3p4/3P1B2/2N5/PPPQPPPP/R2K1BNR b kq - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e1".to_string()));
        assert_eq!(r.move_id,80);

        // e1 castle queenside
        let fen = String::from("rnb1kb1r/ppp1qppp/4pn2/3p4/3P1B2/2N5/PPPQPPPP/2KR1BNR b kq - 1 1");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a1".to_string()));
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn white_king_castle_queenside_invalid() {
        let mut r = prep_board("r3kr2/8/7b/8/8/8/8/R3K3 w Qq - 0 1");
        let mut child = reset::new();
        r.current_piece_init("e1");
        r.move_id = 90;

        let retval = r.generate_next_king_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a1".to_string()));
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn black_king_castle_kingside_valid() {
        let mut r = prep_board("rnbqk2r/ppppbppp/5n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 1");
        let mut child = reset::new();
        r.current_piece_init("e8");

        // e8 to f8
        let fen = String::from("rnbq1k1r/ppppbppp/5n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQ - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e8".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,0);

        // e1 castle kingside
        let fen = String::from("rnbq1rk1/ppppbppp/5n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQ - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e8".to_string()));
        assert_eq!(r.move_id,100);
        assert_eq!(child.capture,0);

        let retval = r.generate_next_king_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d8".to_string()));
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn black_king_castle_kingside_invalid() {
        let mut r = prep_board("4k2r/8/8/8/B7/8/8/R3K3 b Qk - 0 1");
        let mut child = reset::new();
        r.current_piece_init("e8");
        r.move_id = 90;

        let retval = r.generate_next_king_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,0);
    }

    #[test]
    fn black_king_castle_queenside_valid() {
        let mut r = prep_board("r3kb1r/pppbqppp/2n1pn2/3p4/3P1B2/2N3P1/PPPQPPBP/2KR2NR b kq - 0 1");
        let mut child = reset::new();
        r.current_piece_init("e8");

        // e8 to d8
        let fen = String::from("r2k1b1r/pppbqppp/2n1pn2/3p4/3P1B2/2N3P1/PPPQPPBP/2KR2NR w - - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e8".to_string()));
        assert_eq!(r.move_id,80);

        // e8 castle queenside
        let fen = String::from("2kr1b1r/pppbqppp/2n1pn2/3p4/3P1B2/2N3P1/PPPQPPBP/2KR2NR w - - 1 2");
        let retval = r.generate_next_king_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a8".to_string()));
        assert_eq!(r.move_id,10);
    }

}

