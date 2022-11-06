use crate::reset::Reset;
use crate::reset::pinned::PinDimension;
use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0100;
use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0200;
use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0400;
use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0500;
use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0700;
use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0800;
use crate::reset::r#const::B_KNIGHT_CAN_MOVE_1000;
use crate::reset::r#const::B_KNIGHT_CAN_MOVE_1100;
use crate::reset::r#const::BLACK;
use crate::reset::r#const::WHITE;

impl Reset {

    /// Generate the next possible knight move
    ///
    /// Returns `true` if a move was suggested, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// ```
    pub fn generate_next_knight_move(&mut self, child: &mut Reset) -> bool {

        if self.pin_dimension == PinDimension::None {
            let b_available_moves: u64 = if self.white_to_move() {
                !self.b_white
            } else {
                self.b_white | !self.b_all
            };

            if self.move_id < 20 && (self.b_current_piece & B_KNIGHT_CAN_MOVE_0100 != 0) {
                self.move_id = 20;
                let b_destination = self.b_current_piece << 15;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.valid_child_post_processing(child);
                    return true;
                }
            }
            if self.move_id < 30 && (self.b_current_piece & B_KNIGHT_CAN_MOVE_0200 != 0) {
                self.move_id = 30;
                let b_destination = self.b_current_piece << 6;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.valid_child_post_processing(child);
                    return true;
                }
            }
            if self.move_id < 40 && (self.b_current_piece & B_KNIGHT_CAN_MOVE_0400 != 0) {
                self.move_id = 40;
                let b_destination = self.b_current_piece >> 10;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.valid_child_post_processing(child);
                    return true;
                }
            }
            if self.move_id < 50 && (self.b_current_piece & B_KNIGHT_CAN_MOVE_0500 != 0) {
                self.move_id = 50;
                let b_destination = self.b_current_piece >> 17;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.valid_child_post_processing(child);
                    return true;
                }
            }
            if self.move_id < 60 && (self.b_current_piece & B_KNIGHT_CAN_MOVE_0700 != 0) {
                self.move_id = 60;
                let b_destination = self.b_current_piece >> 15;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.valid_child_post_processing(child);
                    return true;
                }
            }
            if self.move_id < 70 && (self.b_current_piece & B_KNIGHT_CAN_MOVE_0800 != 0) {
                self.move_id = 70;
                let b_destination = self.b_current_piece >> 6;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.valid_child_post_processing(child);
                    return true;
                }
            }
            if self.move_id < 80 && (self.b_current_piece & B_KNIGHT_CAN_MOVE_1000 != 0) {
                self.move_id = 80;
                let b_destination = self.b_current_piece << 10;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.valid_child_post_processing(child);
                    return true;
                }
            }
            if self.move_id < 90 && (self.b_current_piece & B_KNIGHT_CAN_MOVE_1100 != 0) {
                let b_destination = self.b_current_piece << 17;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.consider_next_moveable_piece();
                    self.valid_child_post_processing(child);
                    return true;
                }
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

    fn prep_board(fen: &str) -> Reset {
        let mut r = reset::new();
        let fen = String::from(fen);
        r.init_from_fen(fen);
        r.set_current_piece_pin_dimension();
        r
    }

    #[test]
    fn knight_moves_white_first_starting_position() {
        let mut r = prep_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut child = reset::new();
        r.b_current_piece = 0x0000000000000002;

        // First Move: 0100 (10)
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 1 1");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000000000000002);
        assert_eq!(r.move_id,20);

        // Second Move: 1100 (80)
        let fen2 = String::from("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen2);
        assert_eq!(r.b_current_piece,0x0000000000000004);
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn knight_moves_white_second_starting_position() {
        let mut r = prep_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut child = reset::new();
        r.b_current_piece = 0x0000000000000040;

        // First Move: 0100 (10)
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000000000000040);
        assert_eq!(r.move_id,20);

        // Second Move: 1100 (80)
        let fen2 = String::from("rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 1 1");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen2);
        assert_eq!(r.b_current_piece,0x0000000000000080);
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn knight_moves_black_first_starting_position() {
        let mut r = prep_board("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
        let mut child = reset::new();
        r.b_current_piece = 0x0200000000000000;

        // First Move: 0500 (40)
        let fen = String::from("rnbqkb1r/pppppppp/7n/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0200000000000000);
        assert_eq!(r.move_id,50);

        // Second Move: 0700 (50)
        let fen2 = String::from("rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen2);
        assert_eq!(r.b_current_piece,0x0200000000000000);
        assert_eq!(r.move_id,60);

        // No Third Move
        let retval = r.generate_next_knight_move(&mut child);
        assert_eq!(retval,false);
        assert_eq!(r.b_current_piece,0x0400000000000000);
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn knight_moves_black_second_starting_position() {
        let mut r = prep_board("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
        let mut child = reset::new();
        r.b_current_piece = 0x4000000000000000;

        // First Move: 0500 (40)
        let fen = String::from("r1bqkbnr/pppppppp/2n5/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x4000000000000000);
        assert_eq!(r.move_id,50);

        // Second Move: 0700 (50)
        let fen2 = String::from("r1bqkbnr/pppppppp/n7/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen2);
        assert_eq!(r.b_current_piece,0x4000000000000000);
        assert_eq!(r.move_id,60);

        // No Third Move
        let retval = r.generate_next_knight_move(&mut child);
        assert_eq!(retval,false);
        assert_eq!(r.b_current_piece,0x8000000000000000);
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn knight_moves_black_simple_center() {
        let mut r = prep_board("4k3/8/8/3n4/8/8/8/4K3 b - - 0 34");
        let mut child = reset::new();
        r.b_current_piece = 0x0000001000000000;

        // First Move: 0100 (10)
        let fen = String::from("4k3/4n3/8/8/8/8/8/4K3 w - - 1 35");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000001000000000);
        assert_eq!(r.move_id,20);

        // Second Move: 0200 (20)
        let fen = String::from("4k3/8/5n2/8/8/8/8/4K3 w - - 1 35");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000001000000000);
        assert_eq!(r.move_id,30);

        // Third Move: 0400 (30)
        let fen = String::from("4k3/8/8/8/5n2/8/8/4K3 w - - 1 35");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000001000000000);
        assert_eq!(r.move_id,40);

        // Fourth Move: 0500 (40)
        let fen = String::from("4k3/8/8/8/8/4n3/8/4K3 w - - 1 35");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000001000000000);
        assert_eq!(r.move_id,50);

        // Fifth Move: 0700 (50)
        let fen = String::from("4k3/8/8/8/8/2n5/8/4K3 w - - 1 35");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000001000000000);
        assert_eq!(r.move_id,60);

        // Sixth Move: 0800 (60)
        let fen = String::from("4k3/8/8/8/1n6/8/8/4K3 w - - 1 35");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000001000000000);
        assert_eq!(r.move_id,70);

        // Seventh Move: 1000 (70)
        let fen = String::from("4k3/8/1n6/8/8/8/8/4K3 w - - 1 35");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000001000000000);
        assert_eq!(r.move_id,80);

        // Eighth Move: 1100 (80)
        let fen = String::from("4k3/2n5/8/8/8/8/8/4K3 w - - 1 35");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0800000000000000);
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn knight_move_following_illegal_attempt() {
        let mut r = prep_board("1r2k3/p7/2n5/p7/1p1p4/8/4R3/4K3 b - - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("c6".to_string());

        // First Move: 0100 (will fail because black still in check)
        // Second Move: 0200 (20)
        let fen = String::from("1r2k3/p3n3/8/p7/1p1p4/8/4R3/4K3 w - - 1 2");
        let retval = r.generate_next_knight_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c6".to_string()));
        assert_eq!(r.move_id,30);
    }

}
