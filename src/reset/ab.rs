use crate::reset::Reset;
use crate::reset::r#const::SCORE_STALEMATE;
use crate::reset::r#const::SCORE_BLACK_CHECKMATE;
use crate::reset::r#const::SCORE_WHITE_CHECKMATE;

impl Reset {

    pub fn alpha_beta_in_place(&mut self, depth: u8, mut min: i32, mut max: i32) -> i32 {
        let mut moves_generated: bool = false;
        if depth == 0 {
            self.score()
        } else {
            let mut child = crate::reset::new();
            self.conditionally_complete_move_initialization();
            while self.generate_next_move(&mut child) {
                moves_generated = true;
                let mut temp_score: i32 = child.alpha_beta_in_place(depth-1,min,max);
                if self.white_to_move() {
                    if temp_score > max {
                        max = temp_score;
                    }
                } else {
                    if temp_score < min {
                        min = temp_score;
                    }
                }
                if min <= max {
                    break;
                }
            }
            if moves_generated {
                if self.white_to_move() {
                    max
                } else {
                    min
                }
            } else {
                if self.in_check != 0 {
                    if self.white_to_move() {
                        SCORE_BLACK_CHECKMATE
                    } else {
                        SCORE_WHITE_CHECKMATE
                    }
                } else {
                    SCORE_STALEMATE
                }
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::utils;
    use crate::reset::r#const::SCORE_MIN;
    use crate::reset::r#const::SCORE_MAX;
    use crate::reset::r#const::SCORE_STALEMATE;
    use crate::reset::r#const::SCORE_WHITE_CHECKMATE;
    use crate::reset::r#const::SCORE_BLACK_CHECKMATE;


    fn prep_board(fen: &str) -> Reset {
        let mut r = reset::new();
        let fen = String::from(fen);
        r.init_from_fen(fen);
        r
    }

    #[test]
    fn ab_stalemate_test() {
        let mut r = prep_board("8/8/8/8/8/3K4/3B4/3k4 b - - 0 1");
        let score = r.alpha_beta_in_place(8, SCORE_MAX, SCORE_MIN);
        assert_eq!(score,SCORE_STALEMATE);

        let mut r = prep_board("7K/5k2/p4n2/Pp2b3/1P6/8/8/8 w - - 0 1");
        let score = r.alpha_beta_in_place(8, SCORE_MAX, SCORE_MIN);
        assert_eq!(score,SCORE_STALEMATE);
    }

    #[test]
    fn ab_checkmate_test() {
        let mut r = prep_board("r1bqkbnr/pppp1Qpp/8/4p3/2BnP3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 1");
        let score = r.alpha_beta_in_place(8, SCORE_MAX, SCORE_MIN);
        assert_eq!(score,SCORE_WHITE_CHECKMATE);

        let mut r = prep_board("8/7P/5n2/1P6/2P2p2/4k3/8/r3K3 w - - 0 1");
        let score = r.alpha_beta_in_place(8, SCORE_MAX, SCORE_MIN);
        assert_eq!(score,SCORE_BLACK_CHECKMATE);
    }
}
