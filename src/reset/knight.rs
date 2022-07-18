use crate::reset::Reset;

pub const KNIGHT_CAN_MOVE_0100: u64 = 0x0000fefefefefefe;
pub const KNIGHT_CAN_MOVE_0200: u64 = 0x00fcfcfcfcfcfcfc;
pub const KNIGHT_CAN_MOVE_0400: u64 = 0xfcfcfcfcfcfcfc00;
pub const KNIGHT_CAN_MOVE_0500: u64 = 0xfefefefefefe0000;
pub const KNIGHT_CAN_MOVE_0700: u64 = 0x7f7f7f7f7f7f0000;
pub const KNIGHT_CAN_MOVE_0800: u64 = 0x3f3f3f3f3f3f3f00;
pub const KNIGHT_CAN_MOVE_1000: u64 = 0x003f3f3f3f3f3f3f;
pub const KNIGHT_CAN_MOVE_1100: u64 = 0x00007f7f7f7f7f7f;

impl Reset {

    /// Generate the next possible knight move
    ///
    /// Returns `true` if a move was suggested, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// ```
    pub fn generate_next_knight_move(&mut self, child: &mut Reset) -> bool {

        let b_available_moves: u64 = 0;
        if self.white_to_move() {
            let b_available_moves = !self.b_white;
        } else {
            let b_available_moves = !self.b_black;
        }

        if self.move_id == 10 && (self.b_current_piece & KNIGHT_CAN_MOVE_0100 != 0) {
                let b_destination = self.b_current_piece << 6;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.move_id += 10;
                    return true;
                }
        }
        if self.move_id == 20 && (self.b_current_piece & KNIGHT_CAN_MOVE_0200 != 0) {
                let b_destination = self.b_current_piece << 15;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.move_id += 10;
                    return true;
                }
        }
        if self.move_id == 30 && (self.b_current_piece & KNIGHT_CAN_MOVE_0400 != 0) {
                let b_destination = self.b_current_piece << 17;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.move_id += 10;
                    return true;
                }
        }
        if self.move_id == 40 && (self.b_current_piece & KNIGHT_CAN_MOVE_0500 != 0) {
                let b_destination = self.b_current_piece << 10;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.move_id += 10;
                    return true;
                }
        }
        if self.move_id == 50 && (self.b_current_piece & KNIGHT_CAN_MOVE_0700 != 0) {
                let b_destination = self.b_current_piece >> 6;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.move_id += 10;
                    return true;
                }
        }
        if self.move_id == 60 && (self.b_current_piece & KNIGHT_CAN_MOVE_0800 != 0) {
                let b_destination = self.b_current_piece >> 15;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.move_id += 10;
                    return true;
                }
        }
        if self.move_id == 70 && (self.b_current_piece & KNIGHT_CAN_MOVE_1000 != 0) {
                let b_destination = self.b_current_piece >> 17;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.move_id += 10;
                    return true;
                }
        }
        if self.move_id == 80 && (self.b_current_piece & KNIGHT_CAN_MOVE_1100 != 0) {
                let b_destination = self.b_current_piece >> 10;
                if (b_available_moves & b_destination != 0) 
                    && (self.add_move_if_valid(child, b_destination)) 
                {
                    self.move_id += 10;
                    return true;
                }
        }
        self.consider_next_moveable_piece();
        self.move_id = 10;
        false
    }

}
