use crate::reset::Reset;

impl Reset {
    pub fn generate_next_king_move(&mut self, child: &mut Reset) -> bool {

        let b_available_moves: u64 = if self.white_to_move() {
            !self.b_white
        } else {
            !self.b_black
        };

        // Up
        if self.move_id < 20 && (self.b_current_piece & B_NOT_TOP_EDGE != 0) {
            self.move_id = 20;
            let b_destination = self.b_current_piece << 8;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_if_valid(child, b_destination)) 
            {
                return true;
            }
        }

        // Up Right
        if self.move_id < 30 && (self.b_current_piece & B_NOT_UR_EDGE != 0) {
            self.move_id = 30;
            let b_destination = self.b_current_piece << 7;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_if_valid(child, b_destination)) 
            {
                return true;
            }
        }

        // Right
        if self.move_id < 40 && (self.b_current_piece & B_NOT_RIGHT_EDGE != 0) {
            self.move_id = 40;
            let b_destination = self.b_current_piece << 1;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_if_valid(child, b_destination)) 
            {
                return true;
            }
        }

        // Down Right
        if self.move_id < 50 && (self.b_current_piece & B_NOT_DR_EDGE != 0) {
            self.move_id = 50;
            let b_destination = self.b_current_piece >> 9;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_if_valid(child, b_destination)) 
            {
                return true;
            }
        }

        // Down
        if self.move_id < 60 && (self.b_current_piece & B_NOT_DR_EDGE != 0) {
            self.move_id = 60;
            let b_destination = self.b_current_piece >> 8;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_if_valid(child, b_destination)) 
            {
                return true;
            }
        }

        // Down Left
        if self.move_id < 70 && (self.b_current_piece & B_NOT_DL_EDGE != 0) {
            self.move_id = 70;
            let b_destination = self.b_current_piece >> 8;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_if_valid(child, b_destination)) 
            {
                return true;
            }
        }

        // Left
        if self.move_id < 80 && (self.b_current_piece & B_NOT_LEFT_EDGE != 0) {
            self.move_id = 80;
            let b_destination = self.b_current_piece << 1;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_if_valid(child, b_destination)) 
            {
                return true;
            }
        }

        // Up Left
        if self.move_id < 90 && (self.b_current_piece & B_NOT_UL_EDGE != 0) {
            self.move_id = 90;
            let b_destination = self.b_current_piece << 9;
            if (b_available_moves & b_destination != 0) 
                && (self.add_move_if_valid(child, b_destination)) 
            {
                return true;
            }
        }

        true
    }

}


